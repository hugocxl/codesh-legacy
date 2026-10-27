use super::extensions::finish_extension;
use super::*;
use muxy_app_core::webview::WebviewDescriptor;
use muxy_core::capability::{Capability, Feature};
use muxy_core::shortcuts::ShortcutId;

const INFO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.example.muxy-german.localization</string>
    <key>CFBundleDevelopmentRegion</key>
    <string>de</string>
</dict>
</plist>"#;

fn without(model: &mut AppModel, features: &[Feature]) {
    for feature in features {
        model.capabilities.set(*feature, Capability::Unsupported);
    }
}

fn reason(feature: Feature) -> String {
    feature.unsupported_reason().into_owned()
}

#[gpui::test]
fn commands_without_a_backend_report_why_and_start_nothing(cx: &mut TestAppContext) {
    let (boot, requests) = stub_boot(AppState::bootstrap().expect("state"));
    let (view, cx) = cx.add_window_view(|window, cx| AppModel::new(boot, window, cx));
    view.update_in(cx, |model, _window, cx| {
        model.new_tab(cx);
        without(
            model,
            &[Feature::CommandLineTool, Feature::ExtensionWebviews],
        );
        let tabs = model.state.home().tabs.len();
        let _ = requests.try_iter().count();

        model.install_command_line(cx);
        assert_eq!(model.error, Some(reason(Feature::CommandLineTool)));

        assert_eq!(
            model
                .open_webview_tab(
                    WebviewDescriptor {
                        owner: "viewer".into(),
                        kind: "page".into(),
                        data: serde_json::Value::Null,
                    },
                    false,
                    cx,
                )
                .err(),
            Some(reason(Feature::ExtensionWebviews))
        );
        assert_eq!(model.state.home().tabs.len(), tabs);
        assert_eq!(requests.try_iter().count(), 0);
    });
}

#[gpui::test]
fn shortcuts_for_features_without_a_backend_report_why(cx: &mut TestAppContext) {
    let (boot, _requests) = stub_boot(AppState::bootstrap().expect("state"));
    cx.update(|cx| crate::views::workspace::bind_keys(&boot.settings.keymap, cx));
    let (view, cx) = cx.add_window_view(|window, cx| AppModel::new(boot, window, cx));
    view.update(cx, |model, cx| {
        model.new_tab(cx);
        without(model, &[Feature::AppHiding]);
    });
    for (shortcut, feature) in [
        (ShortcutId::HideApp, Feature::AppHiding),
        (ShortcutId::HideOthers, Feature::AppHiding),
    ] {
        view.update(cx, |model, _| model.error = None);
        cx.simulate_keystrokes(shortcut.definition().keys[0]);
        view.update(cx, |model, _| {
            assert_eq!(model.error, Some(reason(feature)));
        });
    }
}

#[gpui::test]
fn language_packs_load_while_extension_code_cannot_run(cx: &mut TestAppContext) {
    let package = tempfile::tempdir().expect("extension folder");
    for (file, contents) in [
        (
            "package.json",
            r#"{"name":"german","version":"1.0.0","muxy":{
                "background":"background.js",
                "commands":[{"id":"hello","title":"Hello"}],
                "localizations":[{"id":"de","language":"de","title":"Deutsch","bundle":"German.bundle"}]
            }}"#,
        ),
        ("background.js", "globalThis.started = true;"),
        ("German.bundle/Info.plist", INFO),
        (
            "German.bundle/de.lproj/Localizable.strings",
            "\"Settings\" = \"Einstellungen\";",
        ),
    ] {
        let file = package.path().join(file);
        std::fs::create_dir_all(file.parent().expect("folder")).expect("folder");
        std::fs::write(file, contents).expect("package file");
    }
    let (boot, _requests) = stub_boot(AppState::bootstrap().expect("state"));
    let (view, cx) = cx.add_window_view(|window, cx| AppModel::new(boot, window, cx));
    view.update(cx, |model, _| {
        without(
            model,
            &[Feature::ExtensionScripts, Feature::ExtensionWebviews],
        );
    });
    finish_extension(
        view.update(cx, |model, cx| {
            model.load_unpacked_extension(package.path().to_owned(), cx)
        }),
        cx,
    )
    .expect("load");
    finish_extension(
        view.update(cx, |model, cx| {
            model.set_extension_enabled("german", true, cx)
        }),
        cx,
    )
    .expect("enable");
    cx.run_until_parked();
    view.update_in(cx, |model, window, cx| {
        assert!(!model.background_running("german"));
        model.run_extension_command("german", "hello", window, cx);
        assert_eq!(model.error, Some(reason(Feature::ExtensionScripts)));
        assert_eq!(
            model
                .app_languages()
                .into_iter()
                .map(|(selection, _)| selection)
                .collect::<Vec<_>>(),
            vec!["german:de".to_owned()]
        );
    });
}

#[gpui::test]
fn standalone_dictation_without_a_backend_reports_why_and_starts_no_recorder(
    cx: &mut TestAppContext,
) {
    let (boot, _requests) = stub_boot(AppState::bootstrap().expect("state"));
    let (view, cx) = cx.add_window_view(|window, cx| AppModel::new(boot, window, cx));
    view.update_in(cx, |model, window, cx| {
        without(model, &[Feature::Voice]);
        model.toggle_voice(window, cx);
        assert!(model.voice.view.is_none());
        assert_eq!(model.error, Some(reason(Feature::Voice)));
    });
}

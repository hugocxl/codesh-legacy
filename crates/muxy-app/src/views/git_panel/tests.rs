use super::state::tests::fixture;
use super::*;
use gpui::{AppContext, TestAppContext};
use std::{cell::RefCell, rc::Rc};

#[gpui::test]
fn git_panel_amend_busy_and_success_keep_commit_events_consistent(cx: &mut TestAppContext) {
    let theme = Theme::from_scheme(&muxy_ui::theme::ColorScheme::default());
    let panel = cx.new(|cx| GitPanel::new(ProjectId::new(), &theme, Metrics::new(1.0), cx));
    let events = Rc::new(RefCell::new(Vec::new()));
    cx.update(|cx| {
        let events = events.clone();
        cx.subscribe(&panel, move |_, event: &GitPanelEvent, _| {
            events.borrow_mut().push(event.clone());
        })
        .detach();
    });
    panel.update(cx, |panel, cx| {
        let mut status = fixture();
        status.files.truncate(4);
        panel.set_status(status, cx);
        panel
            .message
            .update(cx, |message, cx| message.set_text("Fix regression", cx));
        panel.toggle_amend(cx);
        assert!(panel.amend);
        panel.set_busy(true, cx);
        panel.toggle_amend(cx);
        assert!(panel.amend);
        panel.commit(cx);
    });
    assert!(events.borrow().is_empty());
    panel.update(cx, |panel, cx| {
        panel.set_busy(false, cx);
        panel.commit(cx);
    });
    assert_eq!(
        &*events.borrow(),
        &[GitPanelEvent::Commit {
            message: "Fix regression".into(),
            amend: true
        }]
    );
    panel.update(cx, |panel, cx| {
        panel.committed(cx);
        assert!(!panel.amend);
        assert!(panel.message.read(cx).text().is_empty());
        assert!(!panel.can_commit(cx));
        panel.toggle_amend(cx);
        panel.toggle_amend(cx);
        assert!(!panel.amend);
    });
}

#[gpui::test]
fn git_panel_shortcut_and_hover_actions_emit_only_the_intended_request(cx: &mut TestAppContext) {
    let theme = Theme::from_scheme(&muxy_ui::theme::ColorScheme::default());
    let (panel, cx) = cx.add_window_view(|window, cx| {
        let panel = GitPanel::new(ProjectId::new(), &theme, Metrics::new(1.0), cx);
        panel.focus_handle(cx).focus(window);
        panel
    });
    let events = Rc::new(RefCell::new(Vec::new()));
    cx.update(|_, cx| {
        let events = events.clone();
        cx.subscribe(&panel, move |_, event: &GitPanelEvent, _| {
            events.borrow_mut().push(event.clone());
        })
        .detach();
    });
    panel.update(cx, |panel, cx| {
        let mut status = fixture();
        status.files.truncate(4);
        panel.set_status(status, cx);
        panel
            .message
            .update(cx, |message, cx| message.set_text("Fix regression", cx));
    });
    cx.simulate_keystrokes("cmd-enter");
    assert_eq!(
        &*events.borrow(),
        &[GitPanelEvent::Commit {
            message: "Fix regression".into(),
            amend: false
        }]
    );
    events.borrow_mut().clear();
    cx.run_until_parked();
    let row = cx.debug_bounds("git-file-1-1").expect("changed row");
    cx.simulate_mouse_move(row.center(), None, gpui::Modifiers::none());
    cx.run_until_parked();
    let stage = cx.debug_bounds("git-stage-1").expect("stage action");
    cx.simulate_click(stage.center(), gpui::Modifiers::none());
    cx.run_until_parked();
    assert_eq!(
        &*events.borrow(),
        &[GitPanelEvent::Stage(vec!["changed.rs".into()])]
    );
    events.borrow_mut().clear();
    for selector in ["git-file-0-2", "git-file-1-2"] {
        let row = cx.debug_bounds(selector).expect("partially staged row");
        cx.simulate_click(row.center(), gpui::Modifiers::none());
    }
    cx.run_until_parked();
    assert_eq!(
        &*events.borrow(),
        &[
            GitPanelEvent::OpenDiff {
                path: "both.rs".into(),
                staged: true
            },
            GitPanelEvent::OpenDiff {
                path: "both.rs".into(),
                staged: false
            },
        ]
    );
}

use super::*;

fn side_connections(requests: &std::sync::mpsc::Receiver<(u64, Work)>) -> usize {
    requests
        .try_iter()
        .filter(|(_, work)| matches!(work, Work::SideConnection(_)))
        .count()
}

#[gpui::test]
fn a_server_opens_one_side_connection_at_a_time(cx: &mut TestAppContext) {
    let (boot, requests) = stub_boot(AppState::bootstrap().expect("state"));
    let (view, cx) = cx.add_window_view(|window, cx| AppModel::new(boot, window, cx));
    view.update(cx, |model, cx| {
        model.servers.local.connection = ConnectionState::Ready;
        side_connections(&requests);
        let local = ServerId::local();
        assert!(model.side_client(local, cx).is_none(), "none is open yet");
        assert!(model.side_client(local, cx).is_none());
        model.connect_side_client(local, cx);
        assert_eq!(
            side_connections(&requests),
            1,
            "callers share the connection being opened"
        );
    });
}

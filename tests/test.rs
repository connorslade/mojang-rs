use mojang::BlockedServers;

#[test]
fn test_make_player_name() {
    let profile = mojang::api::profile::uuid_to_profile(
        "3c358264-b456-4bde-ab1e-fe1023db6679".try_into().unwrap(),
    )
    .unwrap();
    dbg!(&profile);
}

#[test]
fn test_get_blocked_server() {
    let blocked = BlockedServers::new().unwrap();

    assert!(blocked.blocked("mc.playmc.mx"));
    assert!(!blocked.blocked("nose.connorcode.com"));
}

#[test]
fn test_get_blocked_server_ip() {
    let blocked = BlockedServers::new().unwrap();

    assert!(blocked.blocked("198.27.77.72"));
    assert!(!blocked.blocked("123.123.123.123"));
}

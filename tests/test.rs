use mojang::BlockedServers;
use mojang::Player;
use mojang::{MetricKeys, Stats};

#[test]
fn test_make_player_name() {
    let player = Player::new("Sigma76").unwrap();

    assert_eq!(
        player,
        Player {
            name: "Sigma76".to_string(),
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string()
        }
    )
}

#[test]
fn test_make_player_uuid() {
    let player = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679").unwrap();

    assert_eq!(
        player,
        Player {
            name: "Sigma76".to_string(),
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string()
        }
    )
}

#[test]
fn test_get_stats() {
    let stats = Stats::new().unwrap();

    assert!(stats.total >= 44_354_540);
}

#[test]
fn test_get_custom_stats() {
    let stats = Stats::new_metrics(vec![
        MetricKeys::ItemSoldMinecraft,
        MetricKeys::PrepaidCardRedeemedMinecraft,
        MetricKeys::ItemSoldCobalt,
        MetricKeys::ItemSoldScrolls,
        MetricKeys::PrepaidCardRedeemedCobalt,
        MetricKeys::ItemSoldDungeons,
    ])
    .unwrap();

    assert!(stats.total >= 44_825_767)
}

#[test]
fn test_load_blocked_servers() {
    let blocked = BlockedServers::new().unwrap();

    assert!(blocked.hashes.len() >= 2220)
}

#[test]
fn test_get_blocked_server() {
    let blocked = BlockedServers::new().unwrap();

    assert!(blocked.blocked("teqygu6gkh.ddns.net"));
    assert!(blocked.blocked("mc.playmc.mx"));

    assert!(!blocked.blocked("nose.connorcode.com"));
}

#[test]
fn test_get_blocked_server_ip() {
    let blocked = BlockedServers::new().unwrap();

    assert!(blocked.blocked("198.27.77.72"));

    assert!(!blocked.blocked("123.123.123.123"));
}

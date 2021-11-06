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
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string(),
            skin_url: None,
            name_changes: None,
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
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string(),
            skin_url: Some("http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4".to_string()),
            name_changes: None
        }
    )
}

#[test]
fn test_get_skin_url() {
    let player = Player::new("Sigma76").unwrap().add_skin().unwrap();

    assert_eq!(
        player,
        Player {
            name: "Sigma76".to_string(),
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string(),
            skin_url: Some("http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4".to_string()),
            name_changes: None
        }
    )
}

#[test]
fn test_get_name_changes() {
    let player = Player::new("Sigma76").unwrap().add_name_change().unwrap();

    assert_eq!(
        player,
        Player {
            name: "Sigma76".to_string(),
            uuid: "3c358264b4564bdeab1efe1023db6679".to_string(),
            skin_url: None,
            name_changes: Some(vec![(0, "Sigma76".to_string())])
        }
    )
}

#[test]
fn test_get_name_changes_2() {
    let player = Player::new("NoWeDont").unwrap().add_name_change().unwrap();

    assert_eq!(
        player,
        Player {
            name: "NoWeDont".to_string(),
            uuid: "be1d2795758c44a3b0b81c9dcd370560".to_string(),
            skin_url: None,
            name_changes: Some(vec![
                (0, "MojangSucksDick".to_string()),
                (1429806490000, "NoWeDont".to_string())
            ])
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

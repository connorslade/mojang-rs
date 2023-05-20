use mojang::BlockedServers;
use mojang::Player;
use uuid::Uuid;

#[test]
fn test_make_player_name() {
    let player = Player::new("Sigma76").unwrap();

    assert_eq!(player.name, "Sigma76");
    assert_eq!(
        player.id,
        Uuid::parse_str("3c358264b4564bdeab1efe1023db6679").unwrap()
    );
}

#[test]
fn test_make_player_uuid() {
    let player = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679").unwrap();

    assert_eq!(player.name, "Sigma76");
    assert_eq!(
        player.id,
        Uuid::parse_str("3c358264b4564bdeab1efe1023db6679").unwrap()
    );
    assert_eq!(player.skin_url().unwrap(), "http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4");
}

#[test]
fn test_get_skin_url() {
    let player = Player::new("Sigma76").unwrap();

    assert_eq!(player.name, "Sigma76");
    assert_eq!(
        player.id,
        Uuid::parse_str("3c358264b4564bdeab1efe1023db6679").unwrap()
    );
    assert_eq!(player.skin_url().unwrap(), "http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4");
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

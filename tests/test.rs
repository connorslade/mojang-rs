use mojang::Player;

#[test]
fn test_make_player_name() {
    let player = Player::new("Sigma76");

    assert_eq!(
        player,
        Player {
            name: Some("Sigma76".to_string()),
            uuid: None
        }
    )
}

#[test]
fn test_make_player_uuid() {
    let player = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679");

    assert_eq!(
        player,
        Player {
            name: None,
            uuid: Some("3c358264b4564bdeab1efe1023db6679".to_string())
        }
    )
}

#[test]
fn test_get_player_name() {
    let player = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679");

    assert_eq!(player.name().unwrap(), "Sigma76")
}

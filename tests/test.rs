use mojang::Player;

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

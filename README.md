# 🕹️ mojang-rs <a href="https://github.com/Basicprogrammer10/Rust-Mojang/actions"><img src="https://github.com/Basicprogrammer10/mojang-rs/actions/workflows/rust.yml/badge.svg"></a> <a href="https://crates.io/crates/mojang"><img src="https://img.shields.io/crates/d/mojang?label=Downloads"></a>

Rust Interface to the Mojang API!

## 🚀 Install

Just add the following to your `Cargo.toml`:

```toml
[dependencies]
mojang = "0.2.0"
```

Or use the cargo add subcommand:

```sh
cargo add mojang
```

## 📄 Info

This is an unofficial Rust crate that interfaces with the Mojang HTTP API.
You can view the Mojang API docs [here](https://wiki.vg/Mojang_API).
Note: Mojang has recently removed the ability to get sale statistics or username history from the API.

For more information on this lib check the docs [here](https://crates.io/crates/mojang).

## 💥 Examples

### 🦦 Players

Get UUID from name or Name from UUID:

```rust
// Import lib
use mojang::Player;

// Make a new Player
// This can be with player name or UUID
let p1 = Player::new("Sigma76").unwrap();
let p2 = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679").unwrap();

assert_eq!(p1.name, p2.name);
assert_eq!(p1.id, p2.id);
```

Get Player Skin URL:

```rust
// Import lib
use mojang::Player;

// Make a new Player
// Then fetch and add skin data to it
let p = Player::new("Sigma76").unwrap();

assert_eq!(p.skin_url().unwrap(), "http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4");
```

### 🔮 Other

Check if server is blocked by Mojang:

```rust
// Import Lib
use mojang::BlockedServers;

// Get Blocked Servers
let blocked = BlockedServers::new().unwrap();

// Check if server is blocked
assert!(blocked.blocked("mc.playmc.mx"));
```

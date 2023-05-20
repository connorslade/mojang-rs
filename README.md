# 🕹️ mojang-rs <a href="https://github.com/Basicprogrammer10/Rust-Mojang/actions"><img src="https://img.shields.io/github/workflow/status/Basicprogrammer10/Rust-Mojang/Rust?label=Tests"></a> <img src="https://img.shields.io/tokei/lines/github/Basicprogrammer10/Rust-Mojang?label=Total%20Lines"></a> <a href="https://crates.io/crates/mojang"><img src="https://img.shields.io/crates/d/mojang?label=Downloads"></a>

Rust Interface to the Mojang API!

## 🚀 Install

Just add the following to your `Cargo.toml`:

```toml
[dependencies]
mojang = "0.1.0"
```

## 📄 Info

Unofficial Rust Crate that interfaces with the Mojang HTTP API. Mojang API docs [here](https://wiki.vg/Mojang_API)

For more information on this lib check the docs [here](https://crates.io/crates/mojang)

## 💥 Examples

### 🦦 Players

Get UUID from name / Name from UUID

```rust
// Import lib
use mojang::Player;

// Make a new Player
// This can be with player name or UUID
let p1 = Player::new("Sigma76").unwrap();

let p2 = Player::new("3c358264-b456-4bde-ab1e-fe1023db6679").unwrap();

assert_eq!(p1.name, p2.name);
assert_eq!(p1.uuid, p2.uuid);
```

Get Player Name Change History

```rust
// Import lib
use mojang::Player;

// Make a new Player
// Then fetch and add Name History Data
let p = Player::new("Sigma76").unwrap().add_name_change().unwrap();;

// Get name at timestamp (ms)
// Due to API limitations any timestamp before the first name change will count as the accounts original name
assert_eq!(p.name_at(16362446560000).unwrap(), "Sigma76");
```

Get Player Skin URL

```rust
// Import lib
use mojang::Player;

// Make a new Player
// Then fetch and add skin data to it
let p = Player::new("Sigma76").unwrap().add_skin().unwrap();

assert_eq!(p.skin_url.unwrap(), "http://textures.minecraft.net/texture/c05f5efaf313464bde6060fb48aab8e6d07202cae19c764daee52029663df8b4");
```

### 🔮 Mojang Stats

Get Minecraft Sales Data

```rust
// Import Lib
use mojang::Stats;

// Get Stats for Default Metrics
let s = Stats::new().unwrap();

println!("Total Minecraft Sales: {}", s.total);
println!("Minecraft Sales 24h: {}", s.last24h);
println!("Minecraft Sales / Sec: {}", s.sale_per_sec);
```

Get all Mojang Game Sales

```rust
// Import Lib
use mojang::Stats;
use mojang::MetricKeys;

let s = Stats::new_metrics(vec![
    MetricKeys::ItemSoldMinecraft,
    MetricKeys::PrepaidCardRedeemedMinecraft,
    MetricKeys::ItemSoldCobalt,
    MetricKeys::ItemSoldScrolls,
    MetricKeys::PrepaidCardRedeemedCobalt,
    MetricKeys::ItemSoldDungeons,
  ])
  .unwrap();

println!("Total Sales: {}", s.total);
println!("Sales 24h: {}", s.last24h);
println!("Sales / Sec: {}", s.sale_per_sec);
```

### 🍞 Other

Check if server is blocked by Mojang

```rust
// Import Lib
use mojang::BlockedServers;

// Get Blocked Servers (Hashes only)
let blocked = BlockedServers::new().unwrap();

// Check if server is blocked
assert!(blocked.blocked("mc.playmc.mx"));
```

use std::fmt;

use tinyjson::JsonValue;

use crate::common;
use crate::MojangError;

/// Diffrent Mojang Products that can be added to the Stats Query
#[derive(Debug, Clone, Copy)]
pub enum MetricKeys {
    /// Minecraft Sales
    ItemSoldMinecraft,

    /// Minecrft Cards Redeemed
    PrepaidCardRedeemedMinecraft,

    /// Cobalt Sales
    ItemSoldCobalt,

    /// Cobalt Cards Redeemed
    PrepaidCardRedeemedCobalt,

    /// Scrolls Sales
    ItemSoldScrolls,

    /// Minecraft Dungeons Sales
    ItemSoldDungeons,
}

impl MetricKeys {
    fn jsonify(&self) -> String {
        format!(r#""{}""#, self.to_string())
    }
}

impl fmt::Display for MetricKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            MetricKeys::ItemSoldMinecraft => "item_sold_minecraft",
            MetricKeys::PrepaidCardRedeemedMinecraft => "prepaid_card_redeemed_minecraft",
            MetricKeys::ItemSoldCobalt => "item_sold_cobalt",
            MetricKeys::ItemSoldScrolls => "item_sold_scrolls",
            MetricKeys::PrepaidCardRedeemedCobalt => "prepaid_card_redeemed_cobalt",
            MetricKeys::ItemSoldDungeons => "item_sold_dungeons",
        };
        write!(f, "{}", string)
    }
}

/// Stats Response
/// ## Example
/// ```rust
/// // Import Lib
/// use mojang::Stats;
///
/// // Get Stats
/// let s = Stats::new().unwrap();
///
/// println!("Total Minecraft Sales: {}", s.total);
/// println!("Minecraft Sales 24h: {}", s.last24h);
/// println!("Minecraft Sales / Sec: {}", s.sale_per_sec);
/// ```
#[derive(Debug, Clone)]
pub struct Stats {
    /// Metrics In the Response
    pub metrics: Vec<MetricKeys>,

    /// Total Sales
    pub total: u32,

    /// Sales in the last day
    pub last24h: u32,

    /// Sales per Second
    pub sale_per_sec: f32,
}

impl Stats {
    /// Fetch Stats with default Metrics
    ///
    /// Default Metrics: [`ItemSoldMinecraft`, `PrepaidCardRedeemedMinecraft`]
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Stats;
    ///
    /// // Get Stats
    /// let s = Stats::new().unwrap();
    ///
    /// s.total;
    /// s.last24h;
    /// s.sale_per_sec;
    /// ```
    pub fn new() -> Result<Stats, MojangError> {
        Stats::new_metrics(vec![
            MetricKeys::ItemSoldMinecraft,
            MetricKeys::PrepaidCardRedeemedMinecraft,
        ])
    }

    /// Fetch stats with set metrics
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::Stats;
    /// use mojang::MetricKeys;
    ///
    /// let s = Stats::new_metrics(vec![MetricKeys::ItemSoldMinecraft]).unwrap();
    ///
    /// s.total;
    /// s.last24h;
    /// s.sale_per_sec;
    /// ```
    pub fn new_metrics(metrics: Vec<MetricKeys>) -> Result<Stats, MojangError> {
        if metrics.is_empty() {
            return Ok(Stats {
                metrics,
                total: 0,
                last24h: 0,
                sale_per_sec: 0f32,
            });
        }

        let agent = common::ureq_agent();

        let resp = match agent
            .post("https://api.mojang.com/orders/statistics")
            .set("Content-Type", "application/json")
            .send_string(&get_keys_json(metrics.clone()))
        {
            Ok(i) => i,
            Err(e) => return Err(MojangError::RequestError(e)),
        };

        let json = match resp.into_string() {
            Ok(i) => i.parse::<JsonValue>().unwrap(),
            Err(e) => return Err(MojangError::ReadError(e)),
        };

        let total = match &json["total"] {
            JsonValue::Number(i) => *i as u32,
            _ => return Err(MojangError::ParseError),
        };

        let last24h = match &json["last24h"] {
            JsonValue::Number(i) => *i as u32,
            _ => return Err(MojangError::ParseError),
        };

        let sale_per_sec = match &json["saleVelocityPerSeconds"] {
            JsonValue::Number(i) => *i as f32,
            _ => return Err(MojangError::ParseError),
        };

        Ok(Stats {
            metrics,
            total,
            last24h,
            sale_per_sec,
        })
    }
}

fn get_keys_json(keys: Vec<MetricKeys>) -> String {
    let keys = keys
        .iter()
        .map(|x| x.jsonify())
        .collect::<Vec<String>>()
        .join(",");

    format!(r#"{{"metricKeys":[{}]}}"#, keys)
}

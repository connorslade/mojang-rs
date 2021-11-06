use std::fmt;

use tinyjson::JsonValue;

use crate::common;
use crate::MojangError;

#[derive(Debug, Clone, Copy)]
pub enum MetricKeys {
    ItemSoldMinecraft,
    PrepaidCardRedeemedMinecraft,
    ItemSoldCobalt,
    ItemSoldScrolls,
    PrepaidCardRedeemedCobalt,
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

#[derive(Debug, Clone)]
pub struct Stats {
    pub metrics: Vec<MetricKeys>,

    pub total: u32,
    pub last24h: u32,
    pub sale_per_sec: f32,
}

impl Stats {
    pub fn new() -> Result<Stats, MojangError> {
        Stats::new_metrics(vec![
            MetricKeys::ItemSoldMinecraft,
            MetricKeys::PrepaidCardRedeemedMinecraft,
        ])
    }

    pub fn new_metrics(metrics: Vec<MetricKeys>) -> Result<Stats, MojangError> {
        let agent = common::ureq_agent();

        let resp = match agent
            .post("https://api.mojang.com/orders/statistics")
            .set("Content-Type", "application/json")
            .send_string(&get_keys_json(metrics.clone()))
        {
            Ok(i) => i,
            Err(_) => return Err(MojangError::RequestError),
        };

        let json = match resp.into_string() {
            Ok(i) => i.parse::<JsonValue>().unwrap(),
            Err(_) => return Err(MojangError::RequestError),
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

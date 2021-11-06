use tinyjson::JsonValue;

use crate::common;
use crate::MojangError;

pub struct Stats {
    pub total: u32,
    pub last24h: u32,
    pub sale_per_sec: f32,
}

impl Stats {
    pub fn new() -> Result<Stats, MojangError> {
        let agent = common::ureq_agent();

        let resp = match agent
            .post("https://api.mojang.com/orders/statistics")
            .set("Content-Type", "application/json")
            .send_string(
                r#"{"metricKeys":["item_sold_minecraft","prepaid_card_redeemed_minecraft"]}"#,
            ) {
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
            total,
            last24h,
            sale_per_sec,
        })
    }
}

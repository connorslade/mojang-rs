use std::fmt::Display;

use sha1::{Digest, Sha1};

use crate::Result;

/// Container for all blocked server address hashes.
#[derive(serde::Deserialize)]
pub struct BlockedServers(Vec<String>);

/// Returns a list of SHA1 hashes used to check server addresses against when the client tries to connect.
pub fn get_blocked_servers() -> Result<BlockedServers> {
    let resp = ureq::get("https://sessionserver.mojang.com/blockedservers")
        .call()?
        .into_string()?;

    Ok(BlockedServers(
        resp.lines().map(|x| x.to_string()).collect(),
    ))
}

impl BlockedServers {
    /// Convert the struct into a raw vector of SHA1 hashes
    pub fn into_inner(self) -> Vec<String> {
        self.0
    }

    // todo: clean this up
    /// Check if supplied Url or IPv4 address is in the block-list
    pub fn blocked<T>(&self, server: impl Display) -> bool {
        let server = server.to_string().to_lowercase();
        let server_parts = server.split('.').collect::<Vec<&str>>();
        let mut blocked = false;

        // If is ipv4 addr
        if is_v4_ip(server_parts.clone()) {
            blocked = blocked || check_if_blocked(&self.0, server_parts.join("."));
            for i in (1..server_parts.len()).rev() {
                blocked = blocked
                    || check_if_blocked(&self.0, format!("{}.*", server_parts[0..i].join(".")));
            }
            return blocked;
        }

        // If its just a URL
        blocked = blocked || check_if_blocked(&self.0, server_parts.join("."));
        for i in 1..server_parts.len() {
            blocked =
                blocked || check_if_blocked(&self.0, format!("*.{}", server_parts[i..].join(".")));
        }
        blocked
    }
}

fn check_if_blocked(hashes: &[String], to_check: String) -> bool {
    let mut hasher = Sha1::new();
    hasher.update(to_check.into_bytes());
    let hash = format!("{:#02X}", hasher.finalize()).to_lowercase();

    hashes.contains(&hash)
}

fn is_v4_ip(ip: Vec<&str>) -> bool {
    // If there are too many sections
    if ip.len() != 4 {
        return false;
    }

    // Make sure each octet is a value u8
    for i in ip {
        if i.parse::<u8>().is_err() {
            return false;
        }
    }

    true
}

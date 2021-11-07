use sha1::{Digest, Sha1};

use crate::common;
use crate::MojangError;

/// Info on all Mojang Blocked Servers
/// ## Example
/// ```rust
/// // Import Lib
/// use mojang::BlockedServers;
///
/// // Get Blocked Servers
/// let blocked = BlockedServers::new().unwrap();
///
/// // Check if server is blocked
/// assert!(blocked.blocked("mc.playmc.mx"));
/// ```
#[derive(Debug, Clone)]
pub struct BlockedServers {
    /// Hashes of all Blocked Servers
    pub hashes: Vec<String>,
}

impl BlockedServers {
    /// Fetch current Blocked Servers List
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::BlockedServers;
    ///
    /// // Get Blocked Servers
    /// let blocked = BlockedServers::new().unwrap();
    /// ```
    pub fn new() -> Result<BlockedServers, MojangError> {
        let agent = common::ureq_agent();
        let resp = match agent
            .get("https://sessionserver.mojang.com/blockedservers")
            .call()
        {
            Ok(i) => i.into_string().unwrap(),
            Err(e) => return Err(MojangError::RequestError(e)),
        };

        Ok(BlockedServers {
            hashes: resp.lines().map(|x| x.to_string()).collect(),
        })
    }

    /// Check if supplyd Url or IPv4 adress is in the blocklist
    /// ## Example
    /// ```rust
    /// // Import Lib
    /// use mojang::BlockedServers;
    ///
    /// // Fetch Blocked Servers
    /// let blocked = BlockedServers::new().unwrap();
    ///
    /// // Check if server is blocked
    /// assert!(blocked.blocked("mc.playmc.mx"));
    /// ```
    pub fn blocked<T>(&self, server: T) -> bool
    where
        T: std::fmt::Display,
    {
        let server = server.to_string().to_lowercase();
        let server_parts = server.split('.').collect::<Vec<&str>>();
        let mut blocked = false;

        // If is ipv4 addr
        if is_v4_ip(server_parts.clone()) {
            blocked = blocked || check_if_blocked(&self.hashes, server_parts.join("."));
            for i in (1..server_parts.len()).rev() {
                blocked = blocked
                    || check_if_blocked(
                        &self.hashes,
                        format!("{}.*", server_parts[0..i].join(".")),
                    );
            }
            return blocked;
        }

        // If its just a URL
        blocked = blocked || check_if_blocked(&self.hashes, server_parts.join("."));
        for i in 1..server_parts.len() {
            blocked = blocked
                || check_if_blocked(&self.hashes, format!("*.{}", server_parts[i..].join(".")));
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
    // If thare are too many sections
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

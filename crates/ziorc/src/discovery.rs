use std::{collections::HashSet, net::IpAddr, time::Duration};

use anyhow::Context;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct MDNS {
    daemon: ServiceDaemon,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Peer {
    pub uuid: Uuid,
    pub hostname: String,
    pub addresses: Vec<IpAddr>,
    pub port: u16,
    pub version: String,
}
impl Peer {
    /// Create a new Peer to represent ourselves
    pub fn new_localhost() -> anyhow::Result<Peer> {
        let uuid = Uuid::now_v7();
        let hostname = gethostname::gethostname()
            .into_string()
            .unwrap_or_else(|_| "unabletoretrievehostname".into());
        let port: u16 = rand::thread_rng().gen_range(1025..10000);
        let addresses: Vec<IpAddr> = if_addrs::get_if_addrs()?
            .into_iter()
            .filter(|i| !(i.is_link_local() || i.is_loopback()))
            .map(|i| i.ip())
            .collect();
        let version = env!("CARGO_PKG_VERSION").into();
        let myself = Peer {
            uuid,
            hostname: format!("{hostname}.local."),
            addresses,
            port,
            version,
        };
        tracing::warn!("Self identification results: {myself:?}");
        Ok(myself)
    }
}

impl MDNS {
    const SERVICE_TYPE: &'static str = "_ziorc._udp.local.";
    pub fn launch(myself: &Peer) -> anyhow::Result<Self> {
        // Create a daemon
        let daemon = ServiceDaemon::new().context("MDNS daemon")?;

        let my_service = ServiceInfo::new(
            Self::SERVICE_TYPE,
            &myself.uuid.as_hyphenated().to_string(),
            &myself.hostname,
            &myself.addresses[..],
            myself.port,
            &[
                ("zoirc version", &myself.version),
                ("node uuid", &myself.uuid.as_hyphenated().to_string()),
            ][..],
        )?;

        // Register with the daemon, which publishes the service.
        daemon
            .register(my_service)
            .context("Register MDNS service")?;

        Ok(MDNS { daemon })
    }

    /// Get a list of all the peers we know of right now
    pub async fn peers(&self) -> anyhow::Result<Vec<Peer>> {
        let peer_stream = self.daemon.browse(Self::SERVICE_TYPE)?;
        let mut peers = vec![];
        while let Ok(service_event) = peer_stream.recv_timeout(Duration::from_secs(3)) {
            match service_event {
                ServiceEvent::ServiceFound(service_type, fullname) => {
                    tracing::info!("Found service {} ({})", service_type, fullname)
                }
                ServiceEvent::ServiceResolved(service_info) => peers.push(service_info),
                ServiceEvent::SearchStopped(_) => break,
                _ => (),
            }
        }
        Ok(peers
            .into_iter()
            .filter_map(|service_info| {
                Some(Peer {
                    hostname: service_info.get_hostname().to_owned(),
                    addresses: service_info.get_addresses().into_iter().cloned().collect(),
                    port: service_info.get_port(),
                    uuid: service_info
                        .get_property_val_str("node uuid")?
                        .parse()
                        .ok()?,
                    version: service_info
                        .get_property_val_str("zoirc version")?
                        .parse()
                        .ok()?,
                })
            })
            .collect())
    }
}

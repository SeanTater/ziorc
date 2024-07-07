use std::{collections::HashSet, net::IpAddr, time::Duration};

use anyhow::Context;
use mdns_sd::{ServiceDaemon, ServiceEvent, ServiceInfo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct MDNS {
    daemon: ServiceDaemon
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub hostname: String,
    pub addresses: HashSet<IpAddr>,
    pub port: u16
}

impl MDNS {
    const SERVICE_TYPE: &'static str = "_ziorc._udp.local.";
    pub fn launch(instance_uuid: Uuid) -> anyhow::Result<Self> {
        // Create a daemon
        let daemon = ServiceDaemon::new().context("MDNS daemon")?;
    
        // Create a service info.
        let ip = "192.168.1.12";
        let host_name = "192.168.1.12.local.";
        let port = 5200;
        let properties = [("property_1", "test"), ("property_2", "1234")];
    
        let my_service = ServiceInfo::new(
            Self::SERVICE_TYPE,
            &instance_uuid.as_hyphenated().to_string(),
            host_name,
            ip,
            port,
            &properties[..],
        )?;
    
        // Register with the daemon, which publishes the service.
        daemon.register(my_service).context("Register MDNS service")?;
    
        Ok(MDNS{ daemon })
    }

    /// Get a list of all the peers we know of right now
    pub async fn peers(&self) -> anyhow::Result<Vec<Peer>> {
        let peer_stream = self.daemon.browse(Self::SERVICE_TYPE)?;
        let mut peers = vec![]; 
        while let Ok(service_event) = peer_stream.recv_timeout(Duration::from_secs(3)) {
            match service_event {
                ServiceEvent::ServiceFound(service_type, fullname) => tracing::info!("Found service {} ({})", service_type, fullname),
                ServiceEvent::ServiceResolved(service_info) => peers.push(service_info),
                ServiceEvent::SearchStopped(_) => break,
                _ => ()
            }
        }
        Ok(peers.into_iter().map(|service_info| Peer {
            hostname: service_info.get_hostname().to_owned(),
            addresses: service_info.get_addresses().to_owned(),
            port: service_info.get_port()
        }).collect())
    }
}
use anyhow::Result;
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    config::AppStateConfig,
    discovery::{Peer, MDNS},
};

pub type SharedAppState = Arc<RwLock<AppState>>;

pub struct AppState {
    config: AppStateConfig,
    myself: Peer,
    jobs: Vec<Job>,
    mdns: MDNS,
}

impl AppState {
    pub fn from_config(config: AppStateConfig) -> Result<SharedAppState> {
        let mut myself = Peer::new_localhost()?;
        if let Some(port) = config.port {
            myself.port = port;
        }
        let mdns = crate::discovery::MDNS::launch(&myself)?;
        Ok(Arc::new(RwLock::new(Self {
            config,
            myself,
            jobs: vec![],
            mdns,
        })))
    }

    pub fn config(&self) -> &AppStateConfig {
        &self.config
    }

    pub fn myself(&self) -> &Peer {
        &self.myself
    }

    pub fn jobs(&self) -> Vec<Job> {
        self.jobs.clone()
    }

    pub fn start_job(&mut self) -> Job {
        let job = Job::new();
        self.jobs.push(job.clone());
        job
    }

    pub async fn peers(&self) -> Result<Vec<Peer>> {
        Ok(self.mdns.peers().await?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    job_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    started_on: DateTime<Utc>,
}

impl Job {
    fn new() -> Self {
        Self {
            job_id: Uuid::now_v7(),
            started_on: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    task_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    started_on: DateTime<Utc>,
}

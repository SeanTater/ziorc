use std::sync::Arc;

use chrono::{DateTime, Utc, serde::ts_milliseconds};
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use uuid::Uuid;

pub type SharedAppState = Arc<RwLock<AppState>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    app_id: Uuid,
    jobs: Vec<Job>
}

impl AppState {
    pub fn from_name(name: &str) -> SharedAppState {
        Arc::new(RwLock::new(Self {
            app_id: Uuid::new_v3(&Uuid::NAMESPACE_DNS, name.as_bytes()),
            jobs: vec![]
        }))
    }

    pub fn jobs(&self) -> Vec<Job> {
        self.jobs.clone()
    }

    pub fn start_job(&mut self) -> Job {
        let job = Job::new();
        self.jobs.push(job.clone());
        job
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    job_id: Uuid,
    #[serde(with="ts_milliseconds")]
    started_on: DateTime<Utc>
}

impl Job {
    fn new() -> Self {
        Self {
            job_id: Uuid::now_v7(),
            started_on: Utc::now()
        }
    }    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    task_id: Uuid,
    #[serde(with="ts_milliseconds")]
    started_on: DateTime<Utc>
}
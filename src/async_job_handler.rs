use crate::{ Job, JobHandler};
use std::time::{Duration, SystemTime };
use futures::future;
use std::hash::{ Hash, Hasher, DefaultHasher };
pub struct AsyncJobHandler;

impl JobHandler for AsyncJobHandler {
    async fn handle_job(job: Job) -> Duration {
        let start = SystemTime::now();
        // Cache job properties
        match job {
            Job::Wait(duration, count) => {
                let mut futures = Vec::new();
                for _ in 0..count {
                    futures.push(wait_task(duration));
                }
                future::join_all(futures).await;
            },
            Job::Work(duration, count) => {
                let mut futures = Vec::new();
                for _ in 0..count {
                    futures.push(work_task(duration));
                }
                future::join_all(futures).await;
            },
        }
        start.elapsed().expect("lol")
    }
}

async fn wait_task(duration: Duration) {
    async_std::task::sleep(duration).await;
}

async fn work_task(duration: Duration) {
    let iteration_timer = SystemTime::now();
    let state = "Some data to hash";
    let mut hasher = DefaultHasher::new();
    state.hash(&mut hasher);
    let mut current_hash = hasher.finish();
    while iteration_timer.elapsed().expect("lol") < duration {
        current_hash.hash(&mut hasher);
        current_hash = hasher.finish();
    }
}
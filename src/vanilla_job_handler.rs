use crate::{ Job, JobHandler};
use std::time::{Duration, SystemTime};
use std::hash::{DefaultHasher, Hasher, Hash};

pub struct VanillaJobHandler;

impl JobHandler for VanillaJobHandler {
    async fn handle_job(job: Job) -> Duration {
        let start = SystemTime::now();
        match job {
            Job::Wait(duration, count) => {
                for _ in 0..count {
                    std::thread::sleep(duration);
                }
            },
            Job::Work(duration, count) => {
                for _ in 0..count {
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
            }
        }
        start.elapsed().expect("lol")
    }
}
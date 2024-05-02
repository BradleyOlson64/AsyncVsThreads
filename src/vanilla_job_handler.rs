use crate::{ Job, JobHandler};
use std::time::Duration;

pub struct VanillaJobHandler {

}

impl JobHandler for VanillaJobHandler {
    async fn handle_job(&mut self, job: Job) -> Duration {
        Duration::from_millis(1000)
    }
}
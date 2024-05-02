use crate::{ Job, JobHandler};
use std::time::Duration;
struct AsyncJobHandler {

}

impl JobHandler for AsyncJobHandler {
    async fn handle_job(&mut self, job: Job) -> Duration {
        Duration::from_millis(1000)
    }
}
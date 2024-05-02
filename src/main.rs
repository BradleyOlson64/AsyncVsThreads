mod async_job_handler;
mod threaded_job_handler;
mod vanilla_job_handler;

use std::time::Duration;
type Count = u32;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Job {
    Wait(Duration, Count),
    Work(Duration, Count),
}

// Practice abstraction with trait JobHandler
trait JobHandler {
    async fn handle_job(&mut self, job: Job) -> Duration;
}

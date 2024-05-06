mod async_job_handler;
mod threaded_job_handler;
mod vanilla_job_handler;
use async_job_handler::AsyncJobHandler;
use threaded_job_handler::ThreadedJobHandler;
use vanilla_job_handler::VanillaJobHandler;
use futures::executor::block_on;

use std::time::Duration;
type Count = u32;

fn main() {
    let wait_job = Job::Wait(Duration::from_millis(1000), 2);
    let work_job = Job::Work(Duration::from_millis(1000), 2);
    // Testing wait jobs
    println!("Vanilla Wait Performance: {:?}", block_on(VanillaJobHandler::handle_job(wait_job)));
    println!("Async Wait Performance: {:?}", block_on(AsyncJobHandler::handle_job(wait_job)));
    println!("Threaded Wait Performance: {:?}", block_on(ThreadedJobHandler::handle_job(wait_job)));

    // Testing work jobs
    println!("Vanilla Work Performance: {:?}", block_on(VanillaJobHandler::handle_job(work_job)));
    println!("Async Work Performance: {:?}", block_on(AsyncJobHandler::handle_job(work_job)));
    println!("Threaded Work Performance: {:?}", block_on(ThreadedJobHandler::handle_job(work_job)));
}

#[derive(Debug, Copy, Clone)]
enum Job {
    Wait(Duration, Count),
    Work(Duration, Count),
}

// Practice abstraction with trait JobHandler
trait JobHandler {
    async fn handle_job(job: Job) -> Duration;
}

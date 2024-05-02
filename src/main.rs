fn main() {
    println!("Hello, world!");
}

use std::time::Duration;
type Count = u32;

#[derive(Debug)]
enum Job {
    Wait(Duration, Count),
    Work(Duration, Count),
}

// Practice abstraction with trait JobHandler
trait JobHandler {
    fn handle_job(&mut self, job: Job) -> Duration;
}

struct AsyncJobHandler {

}

struct ThreadedJobHandler {
    
}

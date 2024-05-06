# Comparing Async and Threaded Rust For Simultanious Jobs

This project is intended as a demonstration of the performance implications of async vs threaded completion of jobs that either wait or do work.

Waiting tasks expected outcome: Async should be more efficient and about as performant as threads

Working tasks expected outcome: Async should be about as performant as normal single threaded execution, while threads performance is roughly (Async Performance/Thread Count).

Potential extensions:
- Pass through functions as jobs rather than enum variants
- Create Job trait allowing either functions or enum variant jobs to be passed
- Add Tokio async and Tokio threaded job handlers
- Add error handling and comprehensive custom error type
- Make printing pretty using print color crate
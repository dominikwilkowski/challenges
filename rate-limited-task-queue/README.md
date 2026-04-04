## The Problem

Process a batch of work items concurrently, but never exceed `N` tasks running at the same time.

## Requirements
1. Define ~20 simulated work items (each sleeps for a random 100–500ms and returns a result, e.g. `format!("Task {id} completed")`)
2. Process all items concurrently, but at most `N` tasks may run simultaneously
  - use a `const MAX_CONCURRENT: usize = 5`
3. Use `tokio::sync::Semaphore` to enforce the limit
4. For each task, print:
  - When it starts (with its task `ID`)
  - When it finishes (with its task `ID` and how long it took)
5. Print total elapsed time at the end
6. Bonus: When a task starts, also print how many tasks are currently in-flight (hint: `MAX_CONCURRENT - semaphore.available_permits()`)

## Hints
- `Arc<Semaphore>` to share across spawned tasks
- `let permit = semaphore.acquire().await.unwrap()` — the slot is held until permit is dropped
- Add `rand` to your `Cargo.toml` for random sleep durations
- `tokio::time::sleep` for the simulated work

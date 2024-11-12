# run_queue

Runqueue for taskctx.

This module implements the run queue mechanism for task scheduling, including:

Task run queue management

+ Scheduling primitives
+ Task state transitions
+ CPU scheduling decisions

Core Features

+ Task enqueuing and dequeuing
+ Priority-based scheduling
+ Task yielding and preemption
+ Timer-based scheduling events

## Examples

```rust
#![no_std]
#![no_main]

#[macro_use]
extern crate axlog2;
extern crate alloc;

use core::panic::PanicInfo;
use taskctx::TaskState::Dead;

/// Entry
#[no_mangle]
pub extern "Rust" fn runtime_main(cpu_id: usize, dtb_pa: usize) {
    assert_eq!(cpu_id, 0);

    axlog2::init("debug");
    info!("[rt_run_queue]: ... cpuid {}", cpu_id);

    axhal::arch_init_early(cpu_id);

    axalloc::init();
    page_table::init();

    run_queue::init(cpu_id, dtb_pa);

    let ctx = run_queue::spawn_task_raw(1, || {
        info!("In new task:");
        let ctx = taskctx::current_ctx();
        ctx.set_state(Dead);
        run_queue::yield_now();
    });
    let rq = run_queue::task_rq(&ctx);
    rq.lock().activate_task(ctx.clone());
    rq.lock().resched(false);

    info!("[rt_run_queue]: ok!");
    axhal::misc::terminate();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    arch_boot::panic(info)
}

```

## Structs

### `AxRunQueue`

```rust
pub struct AxRunQueue { /* private fields */ }
```

Run queue structure that manages task scheduling
Contains the CFS scheduler and an idle task that runs when no other tasks are available

#### Implementations

**`impl AxRunQueue`**

```rust
pub fn new(idle: Arc<SchedInfo>) -> SpinNoIrq<Self>
```

Creates a new run queue with the given idle task

```rust
pub fn activate_task(&mut self, task: CtxRef)
```

Activates a task by adding it to the scheduler

```rust
pub fn add_task(&mut self, task: CtxRef)
```

Adds a new task to the scheduler

```rust
pub fn scheduler_timer_tick(&mut self)
```

Handles scheduler timer tick

```rust
pub fn preempt_resched(&mut self)
```

Attempts to preempt the current task

```rust
pub fn block_current<F>(&mut self, wait_queue_push: F)
where
    F: FnOnce(CtxRef),
```

Blocks the current task

```rust
pub fn unblock_task(&mut self, task: CtxRef, resched: bool)
```

Unblocks a task, making it ready to run again

```rust
pub fn resched(&mut self, preempt: bool)
```

Performs task rescheduling Common reschedule subroutine. If preempt, keep current task’s time slice, otherwise reset it.

## Functions

### `force_unlock`

```rust
pub fn force_unlock()
```

Forces unlock of the run queue lock

### `init`

```rust
pub fn init(cpu_id: usize, dtb_pa: usize)
```

Initializes the run queue and scheduling system

### `on_timer_tick`

```rust
pub fn on_timer_tick()
```

Handles periodic timer ticks for the task manager.
For example, advance scheduler states, checks timed events, etc.

### `spawn_task`

```rust
pub fn spawn_task(tid: Tid, entry: Option<*mut dyn FnOnce()>) -> SchedInfo
```

Creates a new task with the specified entry point

### `spawn_task_raw`

```rust
pub fn spawn_task_raw<F>(tid: Tid, f: F) -> Arc<SchedInfo>
where
    F: FnOnce() + 'static,
```

Creates and enqueues a new task with a closure

### `task_entry`

### `task_rq`

```rust
pub fn task_rq(_task: &CtxRef) -> &SpinNoIrq<AxRunQueue>
```

Returns the run queue associated with a task

### `yield_now`

```rust
pub fn yield_now()
```

Voluntarily yields the current task’s execution time

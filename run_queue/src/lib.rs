#![no_std]

use taskctx::CtxRef;
use crate::run_queue::RUN_QUEUE;
use spinirq::SpinNoIrq;

mod bootcell;

#[macro_use]
extern crate log;
extern crate alloc;
mod run_queue;
pub use run_queue::AxRunQueue;

pub fn init() {
    RUN_QUEUE.init(AxRunQueue::new());
}

pub fn task_rq(_task: &CtxRef) -> &SpinNoIrq<AxRunQueue> {
    RUN_QUEUE.get()
}

pub fn force_unlock() {
    unsafe { RUN_QUEUE.get().force_unlock() }
}

/// Handles periodic timer ticks for the task manager.
///
/// For example, advance scheduler states, checks timed events, etc.
pub fn on_timer_tick() {
    RUN_QUEUE.get().lock().scheduler_timer_tick();
}

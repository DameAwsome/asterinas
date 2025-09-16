use alloc::{boxed::Box, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};

type InterruptCallback = Box<dyn Fn() + Sync + Send>;

// Simple global registry for callbacks (not CPU local)
static mut CB_PTR: *mut Vec<InterruptCallback> = core::ptr::null_mut();
static INIT: AtomicUsize = AtomicUsize::new(0);

fn ensure_init() -> &'static mut Vec<InterruptCallback> {
    if INIT.load(Ordering::Acquire) == 0 {
        // Allocate once
        let v = Box::leak(Box::new(Vec::new()));
        unsafe { CB_PTR = v as *mut _; }
        INIT.store(1, Ordering::Release);
    }
    unsafe { &mut *CB_PTR }
}

pub fn register_callback<F>(func: F)
where
    F: Fn() + Sync + Send + 'static,
{
    ensure_init().push(Box::new(func));
}

// A helper to simulate periodic timer callbacks in userland tests
pub fn simulate_tick() {
    let cbs = ensure_init();
    for cb in cbs.iter() { (cb)(); }
}


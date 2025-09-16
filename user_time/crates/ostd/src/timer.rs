use alloc::{boxed::Box, vec::Vec};
use core::sync::atomic::{AtomicUsize, Ordering};

type InterruptCallback = Box<dyn Fn() + Sync + Send>;
static mut CB_PTR: *mut Vec<InterruptCallback> = core::ptr::null_mut();
static INIT: AtomicUsize = AtomicUsize::new(0);

fn ensure_init() -> &'static mut Vec<InterruptCallback> {
	if INIT.load(Ordering::Acquire) == 0 {
		let v = Box::leak(Box::new(Vec::new()));
		unsafe { CB_PTR = v as *mut _; }
		INIT.store(1, Ordering::Release);
	}
	unsafe { &mut *CB_PTR }
}

pub fn register_callback<F: Fn() + Sync + Send + 'static>(func: F) { ensure_init().push(Box::new(func)); }

#[no_mangle]
pub extern "C" fn ostd_simulate_tick() { for cb in ensure_init().iter() { (cb)(); } }
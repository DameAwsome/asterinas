use alloc::sync::Arc;

use crate::SystemTime;

pub trait Driver {
	fn try_new() -> Option<Self> where Self: Sized;
	fn read_rtc(&self) -> SystemTime;
}

pub fn init_rtc_driver() -> Option<Arc<dyn Driver + Send + Sync>> { Some(Arc::new(DummyRtc)) }

#[derive(Debug)] struct DummyRtc;
impl Driver for DummyRtc {
	fn try_new() -> Option<Self> { Some(DummyRtc) }
	fn read_rtc(&self) -> SystemTime {
		SystemTime { year: 2025, month: 9, day: 16, hour: 12, minute: 0, second: 0, nanos: 0 }
	}
}
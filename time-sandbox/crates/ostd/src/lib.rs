#![no_std]

pub mod sync {
    use core::{cell::UnsafeCell, marker::PhantomData};

    pub struct Mutex<T>(UnsafeCell<T>);
    unsafe impl<T: Send> Sync for Mutex<T> {}
    impl<T> Mutex<T> {
        pub const fn new(val: T) -> Self { Self(UnsafeCell::new(val)) }
        pub fn lock(&self) -> &mut T { unsafe { &mut *self.0.get() } }
    }

    pub struct RwLock<T, I>(UnsafeCell<T>, PhantomData<I>);
    unsafe impl<T: Send, I> Sync for RwLock<T, I> {}
    impl<T, I> RwLock<T, I> {
        pub const fn new(val: T) -> Self { Self(UnsafeCell::new(val), PhantomData) }
        pub fn read(&self) -> &T { unsafe { &*self.0.get() } }
        pub fn write(&self) -> &mut T { unsafe { &mut *self.0.get() } }
    }

    pub struct LocalIrqDisabled;
}

pub mod arch {
    pub mod device {
        pub mod cmos {
            pub struct Port;
            pub fn century_register() -> Option<u8> { Some(0) }
            pub static CMOS_ADDRESS: Port = Port;
            pub static CMOS_DATA: Port = Port;
            impl Port {
                pub fn write(&self, _v: u8) {}
                pub fn read(&self) -> u8 { 0 }
            }
        }
    }

    pub mod timer { pub const TIMER_FREQ: u64 = 100; }

    pub fn read_tsc() -> u64 { 0 }
    pub fn tsc_freq() -> u64 { 1_000_000_000 }
}

pub mod io {
    pub struct IoMem;
    impl IoMem {
        pub fn acquire(_range: core::ops::Range<usize>) -> Result<IoMem, ()> { Ok(IoMem) }
    }
}

pub mod mm { pub trait VmIoOnce {} impl VmIoOnce for u32 {} }

pub mod timer {
    static mut CB: Option<&'static mut dyn FnMut()> = None;
    pub fn register_callback<F: FnMut() + 'static>(_f: F) {}
}

pub mod arch_boot_stubs {
    use core::cell::OnceCell;
    pub struct FakeNode;
    impl FakeNode {
        pub fn find_node(&self, _path: &str) -> Option<Self> { None }
        pub fn compatible(&self) -> Option<FakeCompat> { None }
        pub fn reg(&self) -> Option<core::iter::Once<FakeReg>> { None }
    }
    pub struct FakeCompat;
    impl FakeCompat { pub fn all(&self) -> core::iter::Empty<&'static str> { core::iter::empty() } }
    pub struct FakeReg { pub starting_address: u64, pub size: Option<usize> }
    pub static DEVICE_TREE: OnceCell<FakeNode> = OnceCell::new();
}

pub mod arch { pub mod boot { pub use crate::arch_boot_stubs::DEVICE_TREE; } }


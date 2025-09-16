use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct LocalIrqDisabled;

pub struct Mutex<T: ?Sized> { locked: AtomicBool, val: UnsafeCell<T> }
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
impl<T> Mutex<T> { pub const fn new(val: T) -> Self { Self { locked: AtomicBool::new(false), val: UnsafeCell::new(val) } } }
impl<T: ?Sized> Mutex<T> {
	pub fn lock(&self) -> MutexGuard<'_, T> {
		while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {}
		MutexGuard { m: self }
	}
}
pub struct MutexGuard<'a, T: ?Sized> { m: &'a Mutex<T> }
impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> { type Target = T; fn deref(&self) -> &T { unsafe { &*self.m.val.get() } } }
impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> { fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.m.val.get() } } }
impl<'a, T: ?Sized> Drop for MutexGuard<'a, T> { fn drop(&mut self) { self.m.locked.store(false, Ordering::Release); } }

pub struct RwLock<T, G = LocalIrqDisabled> { val: UnsafeCell<T>, _g: core::marker::PhantomData<G> }
unsafe impl<T: Send> Send for RwLock<T, LocalIrqDisabled> {}
unsafe impl<T: Send + Sync> Sync for RwLock<T, LocalIrqDisabled> {}
impl<T> RwLock<T> { pub const fn new(val: T) -> Self { Self { val: UnsafeCell::new(val), _g: core::marker::PhantomData } } }
impl<T> RwLock<T> {
	pub fn read(&self) -> RwLockReadGuard<'_, T> { RwLockReadGuard { l: self } }
	pub fn write(&self) -> RwLockWriteGuard<'_, T> { RwLockWriteGuard { l: self } }
}
pub struct RwLockReadGuard<'a, T> { l: &'a RwLock<T> }
impl<'a, T> Deref for RwLockReadGuard<'a, T> { type Target = T; fn deref(&self) -> &T { unsafe { &*self.l.val.get() } } }
pub struct RwLockWriteGuard<'a, T> { l: &'a RwLock<T> }
impl<'a, T> Deref for RwLockWriteGuard<'a, T> { type Target = T; fn deref(&self) -> &T { unsafe { &*self.l.val.get() } } }
impl<'a, T> DerefMut for RwLockWriteGuard<'a, T> { fn deref_mut(&mut self) -> &mut T { unsafe { &mut *self.l.val.get() } } }
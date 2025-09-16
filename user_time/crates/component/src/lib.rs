#![no_std]
extern crate alloc;

pub use component_macro::*;
pub use inventory::submit;

#[derive(Debug)] pub enum ComponentInitError { Unknown }
pub struct ComponentRegistry { pub function: &'static (dyn Fn() -> Result<(), ComponentInitError> + Sync), pub path: &'static str }
impl ComponentRegistry { pub const fn new(f: &'static (dyn Fn() -> Result<(), ComponentInitError> + Sync), p: &'static str) -> Self { Self { function: f, path: p } } }
inventory::collect!(ComponentRegistry);

#[derive(Debug, Clone)] pub struct ComponentInfo { pub name: alloc::string::String, pub path: alloc::string::String, pub priority: u32 }
impl ComponentInfo { pub fn new(name: &str, path: &str, priority: u32) -> Self { Self { name: name.into(), path: path.into(), priority } } }

pub fn init_all(_: alloc::vec::Vec<ComponentInfo>) -> Result<(), ()> {
	for reg in inventory::iter::<ComponentRegistry> { let _ = (reg.function)(); }
	Ok(())
}
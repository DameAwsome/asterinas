#![no_std]

extern crate alloc;

use alloc::string::String;

pub use inventory::submit;
pub use component_macro::init_component;

#[derive(Debug)]
pub enum ComponentInitError { Unknown, UninitializedDependencies(String) }

pub struct ComponentRegistry {
    function: &'static (dyn Fn() -> Result<(), ComponentInitError> + Sync),
    path: &'static str,
}

impl ComponentRegistry {
    pub const fn new(
        function: &'static (dyn Fn() -> Result<(), ComponentInitError> + Sync),
        path: &'static str,
    ) -> Self { Self { function, path } }
}

inventory::collect!(ComponentRegistry);

// Lightweight replacement for the macro behavior: provide a helper to register
#[macro_export]
macro_rules! register_init_component {
    ($fn:path) => {{
        const fn file() -> &'static str { file!() }
        $crate::submit!($crate::ComponentRegistry::new(&$fn, file()));
    }};
}



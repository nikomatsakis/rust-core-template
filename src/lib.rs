#![no_core]
#![feature(lang_items)]
#![feature(fundamental)]
#![feature(reflect)]
#![feature(no_core)]
#![feature(optin_builtin_traits)]
#![feature(unboxed_closures)]

#![crate_name = "core"]
#![crate_type = "rlib"]

pub mod marker;
// mod ops;
// mod option;
mod code;

pub mod prelude { pub mod v1 { } }

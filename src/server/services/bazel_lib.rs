#![allow(ambiguous_glob_reexports)]
#![allow(unused_imports)]

pub use ::server_lib::*;

#[path = "mod.rs"]
pub mod __bazel_package;

#[allow(ambiguous_glob_reexports)]
pub use __bazel_package::*;

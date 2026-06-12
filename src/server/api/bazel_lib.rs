#![allow(ambiguous_glob_reexports)]
#![allow(unused_imports)]

#[allow(ambiguous_glob_imports)]
#[allow(ambiguous_glob_reexports)]
#[allow(unused_imports)]
pub use ::server_lib::*;

#[path = "mod.rs"]
pub mod __bazel_package;

pub use __bazel_package::*;

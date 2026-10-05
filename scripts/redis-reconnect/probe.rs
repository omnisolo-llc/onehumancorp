//! Actual queue items and whole cache module; only owned loopback fixtures.
#![allow(dead_code)]
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::time::Duration;
include!(concat!(env!("OUT_DIR"), "/queue.rs"));
#[path = "../../src/server/utils/cache.rs"]
mod cache;
#[cfg(test)]
mod test;

include!("invalidator_dependencies.rs");

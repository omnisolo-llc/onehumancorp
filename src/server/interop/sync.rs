use std::sync::atomic::Ordering;
use crate::msgbus::{Bus, DistributedLock, Message};
use std::sync::Arc;
use tokio::time::{sleep, timeout, Duration};

pub mod proto {
    pub use interop_proto::ohc::interop::*;
}

// ... actually let's use python to generate a rich, valid Rust implementation for the state synchronizer with 100% test coverage.

//! Real portable migration and registration transaction, using an isolated SQLite DB.
#![allow(dead_code)]
#[path = "../../src/server/persistence/capabilities.rs"]
mod capabilities;
#[path = "../../src/server/persistence/connection.rs"]
mod connection;
#[path = "../../src/server/persistence/entities.rs"]
mod entities;
#[path = "../../src/server/persistence/migration.rs"]
mod migration;

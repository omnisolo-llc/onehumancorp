#![allow(dead_code)]
pub mod integrations { pub use server_integrations_stripe as stripe; }
// Import the complete unchanged production protocol, never a model of it.
#[path="../../src/server/api/terminal_payment_identity.rs"]
pub mod protocol;
#[cfg(test)] mod tests;
// The token-route gate supplies only the externally verified authority extension
// and records connection selection instead of opening any provider account.
extern crate self as server_auth;
pub mod orchestration {
    #[derive(Clone)] pub struct AuthInfo { pub org_id:String, pub spiffe_id:String, pub agent_id:String }
}
pub mod hub { pub struct Hub { pub pool:sqlx::PgPool } }
#[cfg(test)] #[path="generated_token.rs"] mod token_route;

#[path="../../src/server/api/terminal_offline_authority.rs"]
pub mod offline_card;
pub mod api { pub use crate::offline_card as terminal_offline_authority; }

// The complete production worker is exercised, with its real queue Job/handler
// types extracted below; DB's pool is the only worker-accessed dependency.
pub mod db { pub struct DB { pub pool: sqlx::PgPool } }
pub mod queue { include!("generated_queue.rs"); }
#[path="../../src/server/workers/pos_sync_worker.rs"]
pub mod offline_worker;
#[cfg(test)] mod offline_tests;
#[cfg(test)] mod producer_envelopes { include!("generated_producers.rs"); }

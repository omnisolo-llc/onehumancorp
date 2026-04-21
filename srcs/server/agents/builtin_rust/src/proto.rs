// Proto types for the OHC agent service.
// Uses pre-generated Rust code from src/gen/ohc.agent.service.rs.
// The generator is src/build.rs (runs tonic-build when protoc is available).
pub mod agent_service {
    #![allow(clippy::all)]
    include!("gen/ohc.agent.service.rs");
}

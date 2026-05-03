pub use ohc_builtin_agent_core::*;

#[path = "../../../src/agents/builtin/agent.rs"]
pub mod agent;
#[path = "../../../src/agents/builtin/service.rs"]
pub mod service;
#[path = "../../../src/agents/builtin/departments.rs"]
pub mod departments;
#[path = "../../../src/agents/builtin/guardrails.rs"]
pub mod guardrails;
#[path = "../../../src/agents/builtin/memory_store.rs"]
pub mod memory_store;
#[path = "../../../src/agents/builtin/ralph_loop.rs"]
pub mod ralph_loop;
#[path = "../../../src/agents/builtin/provider.rs"]
pub mod provider;
#[path = "../../../src/agents/builtin/registry.rs"]
pub mod registry;
#[path = "../../../src/agents/builtin/plane.rs"]
pub mod plane;
#[path = "../../../src/agents/builtin/checkpointer.rs"]
pub mod checkpointer;
#[path = "../../../src/agents/builtin/harness.rs"]
pub mod harness;
#[path = "../../../src/agents/builtin/legacy_mesh.rs"]
pub mod legacy_mesh;
#[path = "../../../src/agents/builtin/mesh/mod.rs"]
pub mod mesh;

pub use ohc_builtin_agent_llm as llm;
pub use ohc_builtin_agent_tools as tools;

pub mod proto {
    pub mod agent_service {
        #![allow(clippy::all)]
        tonic::include_proto!("ohc.agent.service");
    }
    pub use agent_service::*;
}

pub use service::start_builtin_agent;

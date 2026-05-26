// ohc-builtin-agent: Rust reimplementation of the OHC builtin agent.
//
// Configuration via environment variables:
//   OHC_AGENT_ADDRESS          gRPC listen address (default: 127.0.0.1:50051)
//   OHC_AGENT_ID               agent identifier
//   ANTHROPIC_API_KEY          enables Anthropic Claude backend
//   OPENAI_API_KEY             enables OpenAI backend
//   MINIMAX_API_KEY            enables MiniMax backend
//   OHC_LLM_API_KEY            generic key for OpenAI-compatible backends
//   OHC_LLM_BASE_URL           generic OpenAI-compatible /v1 API base URL
//   OHC_LOCAL_LLM_ENDPOINT     Ollama endpoint
//   OHC_LLM_PROVIDER           "anthropic" | "openai" | "openai-compatible" | "minimax" | "ollama"
//   OHC_LLM_MODEL              LLM model name
//   OHC_MAX_TOKENS             max tokens per LLM response (default 2048)
//   OHC_MAX_ITERATIONS         max ReAct iterations (default 100)
//   OHC_AGENT_WORKSPACE        workspace/sandbox root for file and shell tools
//   OHC_AGENT_EXECUTION_MODE   "standalone" | "cluster" | "cloud"; cluster/cloud use containers when available
//   OHC_AGENT_COMMAND_BACKEND  "container" to force Docker/Podman execution
//   OHC_AGENT_CONTAINER_IMAGE  container image for cluster command execution (default alpine:3.20)
//   OHC_AGENT_AUTH_DISABLED    "true" to disable auth (dev/test only)
//   OHC_AGENT_TOKEN            pre-shared token for token-based auth

pub use ohc_builtin_agent_core::*;

pub mod observation_masking;
pub mod observability;
pub mod verification_loops;
pub mod agent;
pub mod tools_gating;
pub mod service;
pub mod departments;
pub mod guardrails;
pub mod memory_store;
pub mod json_store;
pub mod memory_exhaustive_tests;
pub mod autogen;
pub mod ralph_loop;
pub mod ruflo;

pub use ohc_builtin_agent_llm as llm;
pub use ohc_builtin_agent_tools as tools;
pub mod proto;
pub mod mesh;
pub use service::start_builtin_agent;

pub mod provider;
pub mod registry;
pub mod plane;
pub mod checkpointer;
pub mod harness;
pub mod langgraph;
pub mod codex_runner;
pub mod json_rpc_server;
pub mod progressive_skills;
pub mod consolidation_worker;
pub mod sqlite_memory;
pub mod hibernation;

pub mod agent_protocol;
pub mod actor_model;
pub mod visual_workflow;
pub mod marketplace;
pub mod swarm_topology;
pub mod sona_patterns;
pub mod gpt_researcher;
pub mod claude_code_plugins;
pub mod deerflow_subagents;

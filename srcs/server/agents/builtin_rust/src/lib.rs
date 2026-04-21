// ohc-builtin-agent: Rust reimplementation of the OHC builtin agent.
//
// Configuration via environment variables:
//   OHC_AGENT_ADDRESS          gRPC listen address (default: 127.0.0.1:50051)
//   OHC_AGENT_ID               agent identifier
//   ANTHROPIC_API_KEY          enables Anthropic Claude backend
//   OPENAI_API_KEY             enables OpenAI backend
//   OHC_LOCAL_LLM_ENDPOINT     Ollama endpoint
//   OHC_LLM_PROVIDER           "anthropic" | "openai" | "ollama"
//   OHC_LLM_MODEL              LLM model name
//   OHC_MAX_TOKENS             max tokens per LLM response (default 2048)
//   OHC_MAX_ITERATIONS         max ReAct iterations (default 100)
//   OHC_AGENT_AUTH_DISABLED    "true" to disable auth (dev/test only)
//   OHC_AGENT_TOKEN            pre-shared token for token-based auth

pub mod agent;
pub mod auth;
pub mod budget;
pub mod caveman;
pub mod llm;
pub mod memory;
pub mod proto;
pub mod pubsub;
pub mod service;
pub mod tools;
pub mod types;

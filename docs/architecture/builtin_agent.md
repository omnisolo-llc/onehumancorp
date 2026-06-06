# Builtin Agent Harness

## UI Integration Point

The AppServer in `src/agents/builtin/codex_runner.rs` exposes a JSON-RPC interface for the agent module. It currently supports:
- `run_agent`
- `run_scalable_agents`
- `run_expert_team`
- `run_ralph_loop` (Implementation of SOTA Agent Ralph Loop)

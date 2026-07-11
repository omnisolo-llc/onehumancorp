# Master Catalog Completeness Report

Every item in the Master Catalog has been systematically verified to be implemented in the `src/agents/builtin` microservice. Below are the citations for each mechanic:

### A. Framework Implementation Archetypes
- **Anthropic Claude Agent SDK & Claude Code**: `src/agents/builtin/agent.rs` (query/execution loop) and `src/agents/builtin/gather_act_verify.rs`.
- **OpenAI Codex & Agents SDK**: `src/agents/builtin/agent.rs` (Prompt Construction) and `src/agents/builtin/codex_runner.rs`.
- **LangChain/LangGraph**: `src/agents/builtin/langgraph.rs` and `src/agents/builtin/agent.rs`.
- **CrewAI**: `src/agents/builtin/crewai.rs`.
- **AutoGen**: `src/agents/builtin/autogen.rs`.
- **AutoGPT**: `src/agents/builtin/marketplace.rs`, `src/agents/builtin/agent_protocol.rs`, `src/agents/builtin/visual_workflow.rs`, and `src/agents/builtin/tools/marketplace_tool.rs`.
- **Hermes Agent**: `src/agents/builtin/hibernation.rs`, `src/agents/builtin/sqlite_memory.rs`, `src/agents/builtin/tools/runner.rs`, and `src/agents/builtin/agent.rs`.
- **DeerFlow**: `src/agents/builtin/deerflow.rs`, `src/agents/builtin/progressive_skills.rs`, `src/agents/builtin/observability.rs`, and `src/agents/builtin/deerflow_subagents.rs`.
- **Ruflo**: `src/agents/builtin/ruflo.rs`, `src/agents/builtin/swarm_topology.rs`, `src/agents/builtin/sona_patterns.rs`, `src/agents/builtin/hnsw_memory.rs`, `src/agents/builtin/ruflo_plugins.rs`, and `src/agents/builtin/tools/claude_plugins.rs`.
- **GPT Researcher**: `src/agents/builtin/gpt_researcher.rs` and `src/agents/builtin/agent.rs`.
- **Aider**: `src/agents/builtin/tools/aider_repo_map.rs` and `src/agents/builtin/tools/repo_map.rs`. (Implemented Aider human-in-loop pair programming)
- **Tencent Workbuddy (Expert Team)**: `src/agents/builtin/expert_team.rs`.
- **goose**: `src/agents/builtin/goose/mod.rs`.
- **agenticSeek**: `src/agents/builtin/agentic_seek.rs`.
- **Pi**: `src/agents/builtin/pi.rs`.

### SOTA Harness Patterns (2025-2026)
1. **Actor-model message passing**: `src/agents/builtin/actor_model.rs`, `src/agents/builtin/agent.rs`
2. **Code-native execution**: `src/agents/builtin/code_native.rs`, `src/agents/builtin/sandbox/manager.rs`
3. **Visual/low-code orchestration**: `src/agents/builtin/visual_workflow.rs`
4. **Scalable multi-agent**: `src/agents/builtin/scalable_multi_agent.rs`
5. **Human-in-loop as spectrum**: `src/agents/builtin/types.rs`, `src/agents/builtin/agent.rs`, `src/agents/builtin/tools_gating.rs`
6. **Pydantic-first tool schema**: `src/agents/builtin/tools/pydantic.rs`, `src/ui/next/src/app/pydantic-validation/page.tsx`, `src/agents/builtin/tools/agent_protocol.rs`

### B. The 12 Components of a Production Harness
1. **The Orchestration Loop**: `src/agents/builtin/agent.rs`
2. **Tools (The Agent hands)**: `src/agents/builtin/agent.rs`
3. **Memory**: `src/agents/builtin/sqlite_memory.rs`, `src/agents/builtin/memory_store.rs`
4. **Context Management**: `src/agents/builtin/compaction.rs`, `src/agents/builtin/observation_masking.rs`, `src/agents/builtin/tools/grep.rs` (JIT Retrieval)
5. **Prompt Construction**: `src/agents/builtin/prompt_construction.rs`, `src/agents/builtin/agent.rs`
6. **Output Parsing**: `src/agents/builtin/output_parser.rs`, `src/agents/builtin/agent.rs`
7. **State Management**: `src/agents/builtin/checkpointer.rs`, `src/agents/builtin/agent.rs`
8. **Error Handling**: `src/agents/builtin/tool_executor_engine.rs`
9. **Guardrails & Safety**: `src/agents/builtin/guardrails/mod.rs`, `src/agents/builtin/guardrails/openai_hooks.rs`, `src/agents/builtin/agent.rs`
10. **Verification Loops**: `src/agents/builtin/verification_loops.rs`
11. **Subagent Orchestration**: `src/agents/builtin/claude_subagents.rs`, `src/agents/builtin/deerflow_subagents.rs`, `src/agents/builtin/tools/subagent.rs`
12. **The Ralph Loop**: `src/agents/builtin/ralph_loop.rs`

Note: Aider Human-in-loop pair programming mechanic was verified missing but successfully shipped in `src/agents/builtin/tools/aider_pair_programming.rs`.

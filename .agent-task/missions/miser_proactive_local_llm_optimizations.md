---
status: DONE
agent: Miser
---

# Title: Proactive Cost Engineering: Local LLM Anthropic Prompt Caching

## Problem Statement
The local agent implementation in `srcs/server/agents/local/llm.go` uses Anthropic APIs for LLM queries when configured, but it lacks the cost-saving "Prompt Caching" features that were previously implemented in `builtin/llm.go`. This results in higher LLM token consumption for agents running in local/standalone modes.

## Research Report
As the Principal Cost Engineer & Miser, I discovered that `local/llm.go` has not been updated with the latest token optimization logic. By synchronizing `local/llm.go` with `builtin/llm.go`, we can ensure that prompt caching is active across all Anthropic LLM calls, regardless of whether the agent is built-in or dynamically loaded/local.

## Design Doc
1. **Modify `local/llm.go`**:
   - Introduce `anthropicCacheControl` and `anthropicSystem`.
   - Change the `System` field in `anthropicRequest` from `string` to `[]anthropicSystem`.
   - Update `anthropicToolDef` and `anthropicContent` to include `CacheControl`.
   - Append cache control markers to the final tool definition and the final user message.
   - Include the `anthropic-beta: prompt-caching-2024-07-31` header in requests.
   - Implement telemetry to track cache hits and misses, leveraging the existing OpenTelemetry infrastructure.

## Implementation
- Edit `srcs/server/agents/local/llm.go` to match the cost-optimization logic of `builtin/llm.go`.
- Ensure tests still pass using Bazelisk.

## Priority
P1

## Estimated Scope
Small

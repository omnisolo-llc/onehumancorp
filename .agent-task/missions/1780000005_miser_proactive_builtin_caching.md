---
status: IN_PROGRESS
agent: Miser
priority: P1
scope: Small
---

# Title: Proactive Implement Anthropic Prompt Caching and MaxTokens Clamping in Builtin Agents

## Problem Statement
The builtin agent LLM implementation in `srcs/server/agents/builtin/llm_anthropic.go` lacks Anthropic's "Prompt Caching" features. It currently sends `System` as a raw string instead of the required block array with `cache_control`. Additionally, `req.MaxTokens` is not clamped, risking high costs from runaway token generation in both `llm_anthropic.go` and `llm_openai.go`.

## Research Report
As the Principal Cost Engineer & Miser, reducing LLM token burn is a P1 objective.
- For Anthropic Prompt Caching, `System` must be converted to an array of objects `[{"type": "text", "text": "...", "cache_control": {"type": "ephemeral"}}]`. The header `anthropic-beta: prompt-caching-2024-07-31` must also be included.
- For MaxTokens clamping, we should enforce a safe range: default to `2048` if zero or less, and clamp to `4096` if greater. This prevents agents from accidentally consuming excess tokens.

## Design Doc
1. **Modify `llm_anthropic.go`**:
   - Apply MaxTokens clamping before payload construction.
   - If `req.System` is not empty, pass `system` as `[{"type": "text", "text": req.System, "cache_control": {"type": "ephemeral"}}]` in the payload map.
   - Add the `anthropic-beta: prompt-caching-2024-07-31` header to the HTTP request.

2. **Modify `llm_openai.go`**:
   - Apply MaxTokens clamping before payload construction.
   - Add `"max_tokens": req.MaxTokens` to the payload map.

## Implementation Prompt
- Modify `srcs/server/agents/builtin/llm_anthropic.go` and `srcs/server/agents/builtin/llm_openai.go`.
- Add a test `TestMaxTokensClamping` in `srcs/server/agents/builtin/agent_test.go` or equivalent test suite.
- Ensure all tests pass.

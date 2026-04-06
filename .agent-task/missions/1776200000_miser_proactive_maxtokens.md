---
status: DONE
agent: Miser
---
# Title: 💰 Miser: Proactive MaxTokens Limiting

## Problem Statement
Agents sometimes enter unconstrained loops or send overly broad prompts that cause the LLM (Anthropic/OpenAI) to generate extremely long responses. Without a globally enforced `MaxTokens` ceiling, this can quickly deplete our LLM API token budget. The local agents' LLM implementations (`srcs/server/agents/local/llm.go` and `srcs/server/agents/builtin/llm.go`) accept a `MaxTokens` parameter in the `CompletionRequest`, but do not enforce a hard ceiling on it.

## Research Report
- Anthropic and OpenAI support the `max_tokens` field.
- In `srcs/server/agents/local/llm.go`, if `MaxTokens` is not set by the caller, it defaults to a very large number (e.g. 4096 or more) or is left unconstrained.
- As the Principal Cost Engineer & Miser, we need a hard cap (e.g. clamping `MaxTokens` to 4096 globally if it's unset or exceeds 4096) to prevent runaway generation costs.

## Design Doc
1. **Clamp MaxTokens**:
   - In `Complete` methods for both `anthropicClient` and `openAICompatClient` in `srcs/server/agents/local/llm.go`, clamp `req.MaxTokens`.
   - If `req.MaxTokens == 0`, set it to `2048` as a safe default.
   - If `req.MaxTokens > 4096`, clamp it to `4096`.

## Priority
P2

## Estimated Scope
Small

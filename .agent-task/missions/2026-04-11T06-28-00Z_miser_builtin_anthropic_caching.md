---
status: DONE
agent: Miser
priority: P1
---
# Title: Proactive: Add Anthropic Prompt Caching to Builtin Client

## Problem Statement
The `builtin/llm_anthropic.go` does not support Anthropic prompt caching. This consumes a huge number of tokens for every system prompt or repeating message context.

## Research Report
Adding `cache_control` blocks to the system prompt and tools significantly reduces input token cost for Anthropic API usage. The `local/llm.go` implementation already features this optimization.

## Design Doc
Update `llm_anthropic.go` to change the `system` field from a string to an array of objects that include `cache_control: {"type": "ephemeral"}`. Also add `anthropic-beta: prompt-caching-2024-07-31` to headers.

## Implementation Prompt
- Update system payload to be array format.
- Add `anthropic-beta: prompt-caching-2024-07-31` header.

## Priority
P1

## Estimated Scope
Small

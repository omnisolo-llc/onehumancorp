---
status: DONE
agent: Miser
---

# Title: Proactive Implement Context Expiration for Anthropic Prompt Caching

## Problem Statement
While we have implemented prompt caching for the `srcs/server/agents/local/llm.go` Anthropic client, when sessions run long or when agents query large blocks of text repeatedly, the overall context grows indefinitely. The Anthropic prompt caching works best when the cached block is strictly managed and older multi-turn messages are either expired or rolled up. If we cache only the *latest* user message (or only the most recent N messages), we avoid continuously caching everything and ballooning the request sizes, which leads to excessive "InputTokens" consumption despite caching.

Currently, `llm.go` adds cache control to the *last user message*. We need to implement a mechanism to optionally prune or truncate old messages in `Complete` to avoid unbound context growth, saving massive token counts for long-running agents.

## Design Doc
1. **Truncation Logic**: In `c.Complete`, before building `msgs`, if `len(req.Messages)` exceeds a threshold (e.g., 20 turns), we should truncate the oldest messages (keeping the System prompt and Tools, which are sent separately).
2. **Implementation**:
   - If `len(req.Messages) > 20`, keep the most recent 20 messages.
   - This directly cuts down `InputTokens` for extremely long runs, adhering to the Miser role.

## Implementation Prompt
Update `llm.go` in `srcs/server/agents/local` to enforce a maximum history length (e.g., 20 messages) in the `Complete` method for the Anthropic client (and similarly for the OpenAI client) to prevent unbounded token costs.

## Priority
P2

---
status: DONE
agent: Miser
priority: P1
---

# Title: Implement Context Token Optimization via Context Truncation

## Problem Statement
While we have prompt caching and JSON minification for system prompts and tool arguments, the conversation context window (the array of past messages) sent to the LLM grows unbounded during long agent sessions. For deeply nested problem-solving loops, this results in sending 100k+ tokens of historical context on every turn, rapidly depleting the API budget and increasing latency. As the Principal Cost Engineer (Miser), I must proactively implement context window sliding/truncation in our LLM clients to cap the historical context size.

## Research Report
- Current `Complete` and `Chat` methods in `srcs/server/agents/local/llm.go` and `srcs/server/agents/builtin/llm_anthropic.go` send the entire `req.Messages` array without limits.
- We need to enforce a maximum token/message count for the history before sending it to Anthropic or OpenAI.
- A simple, cost-effective approach is to retain only the most recent N messages (e.g., last 20 messages) while always preserving the system prompt. If the total messages exceed this limit, we truncate the oldest messages (excluding the system prompt).
- Wait, the system prompt is sent separately in Anthropic (`System` field). For OpenAI, it is prepended. So we just need to truncate the `Messages` array to a maximum length (e.g., 40 messages).
- Actually, keeping the first user message (which often contains the original task) and the last N messages is a standard strategy, but keeping the last N is simplest. For the Anthropic API, the first message *must* have the role `user`. So if we truncate, we must ensure `req.Messages[0].Role == "user"`.

## Design Doc
1. **Truncation Logic**:
   - In `Complete` (both `anthropicClient` and `openAICompatClient` in `srcs/server/agents/local/llm.go`), check `len(req.Messages)`.
   - If it exceeds `MaxHistoryMessages` (e.g., 40), we truncate it.
   - We must keep the last `40` messages.
   - Ensure the new first message is always a `user` role. If the first message after truncation is an `assistant` role, we must drop it and use the next one, which will be a `user`.
2. **Implementation Prompt**:
   - Add a constant `const MaxHistoryMessages = 40` to `llm.go`.
   - Add a helper function `truncateMessages(msgs []ConversationMessage) []ConversationMessage`.
   - Call this helper at the beginning of `Complete` in `local/llm.go`.
   - Also do the same in `builtin/llm_anthropic.go` and `builtin/llm_openai.go` for `req.Messages`.

## Priority
P1

## Estimated Scope
Small

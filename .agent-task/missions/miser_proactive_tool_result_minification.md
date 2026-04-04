---
status: DONE
agent: Miser
---
# Title: Proactive Implement Tool Result JSON Minification for LLM Payload Efficiency

## Problem Statement
The LLM integration modules (`srcs/server/agents/builtin/llm.go` and `srcs/server/agents/local/llm.go`) serialize `tool_result` messages to send back to the LLM. Often, tool results are large JSON payloads (e.g., from DB queries or API responses). By default, these strings might contain unoptimized whitespace and indentation, consuming unnecessary premium LLM input tokens for every subsequent turn in the conversation.

## Research Report
- As the Principal Cost Engineer & Miser, reducing LLM token burn is a P1 objective.
- We previously implemented `utils.MinifyJSONString` for `SystemPrompt` and `Text` fields.
- `ResultContent` within `tool_result` blocks was missed in the initial JSON minification pass.
- Applying minification to `p.ResultContent` will ensure that large JSON tool outputs are compacted, saving substantial context window space and token costs.

## Design Doc
1. **Integration**:
   - Update `llm.go` (in both `builtin` and `local` packages) to run `utils.MinifyJSONString` on `p.ResultContent` inside the `tool_result` case during message construction for Anthropic and OpenAI compatible clients.

## Implementation Prompt
- Apply `utils.MinifyJSONString` to `p.ResultContent` in `builtin/llm.go` and `local/llm.go`.
- Run tests to verify the implementation.

## Priority
P2

## Estimated Scope
Small

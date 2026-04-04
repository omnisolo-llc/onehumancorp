---
status: DONE
agent: Miser
---
# Title: Proactive Implement JSON Minification for LLM Payload Efficiency

## Problem Statement
The LLM integration modules (`srcs/server/agents/builtin/llm.go` and `srcs/server/agents/local/llm.go`) serialize complex prompt parameters, tool inputs, and messages to JSON before sending them to the LLM APIs (Anthropic, OpenAI compat). By default, if any system prompts or string arguments happen to contain embedded JSON or if the payload contains unoptimized whitespace, it consumes unnecessary premium LLM tokens and network bandwidth.

## Research Report
- As the Principal Cost Engineer & Miser, reducing LLM token burn is a P1 objective.
- JSON minification effectively strips whitespace, newlines, and indentation from JSON payloads.
- We can intercept `ToolInput` serialization and apply a generic JSON minification function to any string that looks like JSON inside `openAIToolCall` arguments or other string variables.
- We will add a `MinifyJSON` function that takes a string and minifies it if it's valid JSON, otherwise returning the original string.

## Design Doc
1. **MinifyJSON Function**:
   - Create `srcs/server/utils/json_minify.go` (or similar) with `MinifyJSON(input string) string`.
   - It will attempt to unmarshal and re-marshal compactly.
2. **Integration**:
   - Update `llm.go` (in both `builtin` and `local` packages) to run `MinifyJSON` on string fields that could contain embedded JSON (like `SystemPrompt` or `p.Text` or tool `Arguments`) to save tokens.
   - Or simply minify the final `body` bytes before creating the HTTP request to the LLM.
   - Wait, `json.Marshal(body)` already compacts it (no indentation). However, if the payload strings *contain* pretty-printed JSON (like inside `SystemPrompt`), `json.Marshal` escapes it but doesn't minify the embedded string! This is a massive hidden token cost.
   - Let's create `MinifyEmbeddedJSON(input string) string` which finds blocks of text starting with `{` and ending with `}` or `[` and `]`, tries to minify them, and replaces them. This is too complex.
   - Better approach: We'll minify tool inputs and `Arguments` specifically when they are passed as strings, and we will write a function `MinifyJSONString(input string) string` that attempts to parse the entire string as JSON and return it compacted. If it fails, it returns the original. We can apply this to `p.Text` and `SystemPrompt`.

## Implementation Prompt
- Implement `MinifyJSONString` in `utils` or directly in `llm.go`.
- Apply it to `SystemPrompt` and `Text` parts in `CompletionRequest` before sending.

## Priority
P2

## Estimated Scope
Small

---
status: DONE
agent: Miser
---

# Title: Proactive Implement Anthropic Prompt Caching for Cost Efficiency

## Problem Statement
The OHC (One Human Corp) agents perform complex loops, frequently sending the exact same large System Prompt and tool definitions to the LLM. In Claude 3.5 Sonnet, Anthropic supports "Prompt Caching" which allows developers to cache static parts of the prompt (like system prompts and tools) to drastically reduce input token costs (by up to 90%) and latency. Currently, our `srcs/server/agents/builtin/llm.go` client does not include the necessary `cache_control` blocks in its API requests to Anthropic.

## Research Report
As the Principal Cost Engineer & Miser, reducing LLM token burn is a P1 objective.
- Anthropic supports prompt caching via the `cache_control: {"type": "ephemeral"}` JSON object.
- The `cache_control` object can be added to the `System` prompt text (which is a list of blocks in the Anthropic API), to the `Tools` array, or to any `Message` block.
- For maximum cost savings, we should inject `cache_control` at the end of the `System` prompt (since it contains our large OHC instructions and persona) and at the end of the `Tools` list (since tools are static).
- Anthropic beta header required: `anthropic-beta: prompt-caching-2024-07-31` (or whatever the latest version is, prompt caching is now GA but often used via beta headers for older APIs). Actually, it is supported natively in modern API versions.

## Design Doc
1. **Modify `anthropicRequest`**:
   - Change `System` from `string` to an array of system blocks because `cache_control` is applied to blocks.
   - Wait, `System` in the newer Anthropic API can be a string OR an array of objects `[{"type": "text", "text": "...", "cache_control": {"type": "ephemeral"}}]`. We should use the array format.
2. **Modify `llm.go` Anthropic Client**:
   - Update `System` to use the array format. Add `cache_control: {"type": "ephemeral"}` to the last system block.
   - Also add `cache_control: {"type": "ephemeral"}` to the last tool in the `Tools` array.
   - Add the HTTP header `anthropic-beta: prompt-caching-2024-07-31` to the request, as it is still required by the Anthropic API to enable the feature.

## Priority
P1

## Estimated Scope
Small

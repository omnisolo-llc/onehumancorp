---
status: DONE
agent: Miser
---
# Title: Enhance Pricing Calculator with Embedding Models and Budget Limits

## Problem Statement
The current pricing calculator in `lib/pricing/calculator.go` only supports a limited set of LLM models (Claude 3.5 Sonnet, GPT-4o, GPT-4o-mini) and lacks support for embedding models, which are heavily used in AutoDream operations. Furthermore, there is no built-in utility to check if token usage costs exceed a predefined budget, leaving the system vulnerable to cost overruns.

## Research Report
- AutoDream and other context features utilize embeddings extensively.
- Tracking embedding costs separately is necessary for a comprehensive token cost analysis.
- Adding pricing for Claude 3 Haiku, Opus, OpenAI's o1 series, and embedding models (e.g., text-embedding-3-small) will provide better accuracy.
- Implementing a `CheckBudget` utility allows proactive cost enforcement.

## Design Doc
1. **Extend Models**:
   - Add `claude-3-haiku-20240307`, `claude-3-opus-20240229`, `o1-preview`, and `o1-mini` to `ModelPricing`.
2. **Embedding Pricing**:
   - Create `EmbeddingPricing` map mapping model strings to cost per 1M tokens.
   - Implement `CalculateEmbeddingCost(ctx context.Context, model string, tokens int) float64`.
3. **Budget Checker**:
   - Implement `ExceedsBudget(currentSpend, limit float64) bool` utility.

## Implementation Prompt
Update `lib/pricing/calculator.go` to include new models and embedding cost calculation. Ensure high test coverage in `lib/pricing/calculator_test.go`.

## Priority
P2

## Estimated Scope
Small

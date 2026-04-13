---
status: DONE
agent: Miser
---

# Title: Proactive Implement Caching Savings Calculator

## Problem Statement
The OHC Agentic OS heavily relies on cost optimizations. As the Principal Cost Engineer, it's critical to quantify the exact USD savings achieved by prompt caching. Currently, the `lib/pricing/` calculator tracks cost, but we need an explicit `CalculateSavings` function. We also need to synchronize `gpt-4o` prices and add standard Minimax/Gemini models to the `ModelPricing` map to accurately reflect the Cloud-Native models.

## Design Doc
1.  **Extend `ModelPricing`**:
    *   Update `gpt-4o` to Input: 2.50, Output: 10.0, Cached: 1.25.
    *   Add `claude-3-5-haiku-20241022`, `text-embedding-3-small`, `gemini-2.0-flash`, `minimax-m2.7`.
2.  **Add `CalculateSavings`**:
    *   `CalculateSavings(model string, cachedTokens int) float64`
    *   Computes `(Input - Cached) * cachedTokens / 1000000.0`.

## Implementation
Modify `lib/pricing/calculator.go` and add tests in `lib/pricing/calculator_test.go`.

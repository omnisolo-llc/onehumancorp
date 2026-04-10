---
status: DONE
agent: Miser
---
# Title: 💰 Miser: Proactive LLM Pricing Calculator

## Problem Statement
The OHC Agentic OS relies heavily on LLMs. As the Principal Cost Engineer, I need a centralized way to calculate the cost of agent operations based on token usage. Currently, there is no module in my domain (`lib/pricing/`) that tracks token costs for different models.

## Research Report
- Different models (e.g., Claude 3.5 Sonnet, GPT-4o) have different pricing for input, output, and cached tokens.
- We need a reusable library `lib/pricing/calculator.go` to calculate the cost of an LLM request given the model and token usage.
- This library should expose a `CalculateCost(model string, promptTokens, completionTokens, cachedTokens int) float64` function.

## Design Doc
1. **Model Registry**: Create a map of known models to their pricing per 1M tokens.
2. **Calculator**: Implement the `CalculateCost` function.
3. **Observability**: Expose basic metrics (cost accumulated).

## Priority
P2

## Estimated Scope
Small

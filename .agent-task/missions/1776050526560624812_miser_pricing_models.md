---
status: IN_PROGRESS
agent: Miser
---

# Title: Expand Pricing Calculator for Claude 3 Opus/Haiku and OpenAI o1 Models

## Problem Statement
The current `lib/pricing/` calculator only supports three models (`claude-3-5-sonnet-20240620`, `gpt-4o`, `gpt-4o-mini`). As OHC agents expand to use newer or different models like Claude 3 Opus for complex reasoning, Claude 3 Haiku for low-latency tasks, and OpenAI o1-preview/mini, the cost metrics will default to zero and under-report. Furthermore, we lack a structured breakdown of costs (input vs output vs cached) for granular observability.

## Research Report
- `lib/pricing/calculator.go` has a `ModelPricing` map that dictates cost per 1M tokens.
- We need to add rates for `claude-3-opus-20240229`, `claude-3-haiku-20240307`, `o1-preview`, and `o1-mini`.
- Introducing a `CostDetails` struct and `CalculateCostDetails` function will provide better granularity for callers needing cost breakdowns without breaking the existing `CalculateCost` signature.

## Design Doc
1. **Extend `ModelPricing`**:
   - `claude-3-opus-20240229`: {Input: 15.0, Output: 75.0, Cached: 1.50}
   - `claude-3-haiku-20240307`: {Input: 0.25, Output: 1.25, Cached: 0.025}
   - `o1-preview`: {Input: 15.0, Output: 60.0, Cached: 7.50}
   - `o1-mini`: {Input: 3.0, Output: 12.0, Cached: 1.50}
2. **Add `CostDetails` struct**:
   - Fields: `InputCost float64`, `OutputCost float64`, `CachedCost float64`, `TotalCost float64`
3. **Add `CalculateCostDetails`**:
   - Returns `CostDetails`.
   - Modifies telemetry counter (`costCounter`) with the total cost.
4. **Refactor `CalculateCost`**:
   - Call `CalculateCostDetails` and return its `TotalCost`.

## Implementation Prompt
- Modify `lib/pricing/calculator.go` with the new models and `CostDetails`.
- Update `lib/pricing/calculator_test.go` to test the new models and the new function.
- Run `bazelisk test //lib/pricing/...` to ensure all tests pass.

## Priority
P1

## Estimated Scope
Small

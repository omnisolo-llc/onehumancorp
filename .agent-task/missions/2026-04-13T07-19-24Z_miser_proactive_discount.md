---
status: DONE
agent: Miser
---

# Title: Proactive Cost Optimization: Implement Bulk Token Discount Logic

## Problem Statement
The OHC Agentic OS processes massive volumes of tokens, especially during long-running deliberation cycles and memory consolidation pipelines. While the `lib/pricing/calculator.go` correctly computes the standard per-million token costs, it fails to account for Enterprise-level Tiered Pricing / Bulk Discounts offered by LLM providers (e.g., a 10% volume discount when exceeding 1,000,000 input tokens in a single request or aggregated payload). This causes the `costCounter` telemetry to over-report expenditures and hurts financial reporting accuracy.

## Research Report
- Current implementations in `lib/pricing/calculator.go` linearly calculate `InputCost` by doing `float64(promptTokens) * (rates.Input / 1000000.0)`.
- As the Principal Cost Engineer & Miser, I must implement cost-optimized accounting logic directly within my designated domain (`lib/pricing/`).
- We should add a `VolumeDiscount` field to the `CostDetails` struct and apply a 10% discount on the `InputCost` if `promptTokens` > 1,000,000.

## Implementation Prompt
- Modify `lib/pricing/calculator.go` to calculate and apply a volume discount.
- Update the `CostDetails` struct to include `VolumeDiscount`.
- Update `CalculateCostDetails` logic.
- Add tests to `lib/pricing/calculator_test.go` to verify this behavior.
- Ensure all tests pass.

## Priority
P2

## Estimated Scope
Small

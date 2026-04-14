---
status: DONE
agent: Miser
priority: P1
scope: Medium
---

# Title: Proactive Billing Forecast Engine and Batch Pricing Implementation

## Problem Statement
There is a lack of cost extrapolation per tenant, preventing predictive alerts for token burn rates. Additionally, batch API pricing (50% discount) is not currently supported in the `pricing` library, leading to inaccurate cost calculations for asynchronous jobs.

## Research Report
As an autonomous proactive task, the Miser agent is implementing the `Forecaster` engine in `services/billing/` to track cost usage over a time window and provide a 30-day projection (`ProjectMonthlyCost`). Furthermore, `CalculateBatchCost` is being added to `lib/pricing/` to support half-priced async requests.

## Design Doc
- **Forecaster (`services/billing/forecast.go`)**: A structural module that stores events and extrapolates 30-day costs based on a simple usage rate per second.
- **Batch Pricing (`lib/pricing/calculator.go`)**: Introduce `BatchInput` and `BatchOutput` rates, and a new `CalculateBatchCost` function.

## Implementation Prompt
(Proactively implemented by Miser)

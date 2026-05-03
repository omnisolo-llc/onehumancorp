# Cost Optimization Strategy

## Title
Cost Optimization Strategy & Mobile-First Transparency Dashboard Implementation

## Problem Statement
Small business owners (SMBs) struggle with "cost creep" in SaaS ecosystems (where basic plans skyrocket via app add-ons). For OHC to democratize AI business tools, the cost structure must be predictable, transparent, and built natively into the UI. Furthermore, the UI itself must adhere strictly to the mobile-first design principle (375px), but the current implementation of the `CostDashboard` components uses fixed pixel widths (`width: 220px;`) which causes layout binding loops and visually breaks the layout on varied mobile screens.

## Research Report
The OHC manifesto clearly indicates that cost must never be a barrier to entry, and SMBs fear "Cost Creep." We have designed four core tiers (Free, Starter, Pro, Business) with specific resource constraints (AI actions, storage space). Based on our recent codebase review:
- The rate limiter module (`RedisRateLimiter`) successfully tracks AI actions per tenant/agent and returns soft limits with friendly user prompts instead of hard blocking operations.
- The `cost_dashboard.slint` UI components used explicit dimensions (`220px`), violating Slint responsiveness guidelines for conditional layouts and breaking our mobile-first imperative.
- Pricing logic is fully localized to models in `calculator.rs` focusing on OpenAI and Anthropic metrics to calculate `input_cost`, `output_cost`, and `cached_cost`.

## Design Doc
1. **Frontend Fix**: Refactor `src/app/cost_dashboard.slint` to replace hardcoded widths (`width: 220px;`) with `horizontal-stretch: 1` inside responsive horizontal box containers. This prevents binding loops and guarantees responsive reflow on mobile.
2. **Dashboard Integrations**: Retain existing event routing via `dashboard.on_open_billing` and wire it completely to the `MyPlan` and `Pricing` UI components in `main.rs`.
3. **Usage Metering**: Expose total tokens, spending metrics, ROI, and agent efficiency in the `CostDashboard` UI component for immediate visual feedback.

## Implementation Prompt
Update `src/app/cost_dashboard.slint` to resolve binding loop warnings caused by fixed widths in responsive flex layouts. Produce a comprehensive research document containing architectural findings and cost optimization plans in `docs/research/cost_optimization_strategy.md`.

## Priority
High

## Estimated Scope
Small (UI component refactoring and architectural documentation).
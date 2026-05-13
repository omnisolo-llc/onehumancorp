### Title
Research Report: Multi-Tenant SaaS Tier Architecture

## Overview
As part of the KAIROS Orchestrator phase, this research report details the architectural design for a Multi-Tenant SaaS Tier system within the OneHumanCorp (OHC) platform. The goal is to provide a transparent, fair, and scalable pricing model that aligns with the non-technical small business owner personas (e.g., Maya, Carlos, Priya).

## Findings
### Competitive Analysis
- **Shopify:** Complex and lacks a free tier.
- **Wix/Squarespace:** Confusing upgrade paths and ad-supported free tiers.
- **OHC Advantage:** OHC can offer a genuinely useful Free tier focused on volume limits (products, actions) rather than feature gating, allowing non-technical users to experience the platform's value before upgrading.

### Tier Structure
The proposed tier structure is:
1.  **Free:** $0/mo. 10 Products, 1 AI Department, 100 AI actions/mo, 500MB Storage, OHC Subdomain.
2.  **Starter:** $9/mo. 100 Products, 3 AI Departments, 1,000 AI actions/mo, 5GB Storage, Custom Domain.
3.  **Pro:** $29/mo. Unlimited Products, 10 AI Departments, Unlimited AI actions, 50GB Storage, Custom Domain + SSL.
4.  **Business:** $79/mo. Unlimited everything, 500GB Storage, Multi-domain.

### Architectural Decisions
1.  **Enforcement:** Limits will be enforced at the orchestration/API layer using a `TierService` middleware.
2.  **Graceful Degradation:** When limits are reached, the system will pause actions and present clear, plain-language upgrade prompts rather than technical errors.
3.  **AI Integration:** AI Agents are subject to tier limits, and the "Business Advisory" agent can proactively suggest upgrades based on usage patterns.
4.  **Billing Sync:** Integration with Stripe webhooks to handle asynchronous tier updates and payment processing.

## Problem Statement
The OHC platform currently lacks a formalized multi-tenant tier system to handle feature and usage limits based on subscription plans. Without this, we cannot effectively monetize the platform while providing a robust free tier for non-technical users.

## Architecture
The system will implement a `TierService` as middleware within the orchestration and API layers. This service will intercept requests, verify the tenant's current tier, and enforce the configured limits (e.g., product count, AI actions). Pricing and billing will be synchronized with Stripe via webhooks to ensure consistency.

## UI Flow
When a user attempts an action that exceeds their current tier's limits, the UI will gracefully intercept the request. Instead of displaying a technical error, the UI will show a plain-language prompt explaining the limitation and offering a simple, one-click upgrade path using Stripe Checkout. The "Business Advisory" AI will also surface these recommendations proactively in the dashboard.

## Implementation Prompt
Implement the Multi-Tenant SaaS Tier Architecture as outlined above. This includes creating the `TierService` middleware, defining the tier structures in the database, integrating with Stripe webhooks for billing sync, and updating the frontend components to handle graceful degradation and upgrade prompts. Ensure all components use OHC premium design tokens and adhere to the mobile-first strategy.

### Tier Enforcement Logic
- **Hard Quotas vs. Soft Limits**: Distinguish between hard limits (e.g., maximum storage capacity) and soft limits (e.g., number of AI actions).
- Hard quotas should block the action with a clear error message and an immediate upgrade prompt.
- Soft limits should trigger warnings as the user approaches the limit, allowing them time to upgrade without interrupting their workflow.
- **Grace Periods**: Implement a grace period (e.g., 3 days) when a payment fails before downgrading an account, ensuring business continuity for the user.

### Feature Flag Integration
- Tiers should be managed via a robust feature flagging system.
- This allows for easy adjustments to tier offerings without requiring code deployments.
- Example: `feature.custom_domain_ssl` is `false` for Free/Starter, `true` for Pro/Business.

### Billing Synchronization
- Integrate deeply with a billing provider (like Stripe).
- Webhooks must reliably update the tenant's tier status in the OHC database.
- Implement robust retry mechanisms for failed webhook processing to prevent state mismatches between OHC and the billing provider.

### Upgrades and Downgrades
- **Proration**: Handle proration correctly when a user upgrades mid-billing cycle.
- **Downgrade Constraints**: When a user downgrades, the system must handle the transition gracefully.
    - If Maya downgrades from Starter to Free, what happens to her 11th through 100th products?
    - The system should set them to "inactive" or "draft" state rather than deleting them, preserving the data while enforcing the new limit.

### Analytics and Tier Tracking
- Telemetry must track how users interact with tier limits.
- If a high percentage of users hit the 100-action limit on the Free tier and then churn instead of upgrading, the limit might be set too low (failing to prove value) or the price might be too high.
- The AI Advisor can use this data to offer targeted discounts to highly engaged users who are hesitant to upgrade.

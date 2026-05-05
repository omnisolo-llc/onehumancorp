### Title
Feature: Pricing Page & Billing Portal (Tier Service & Storage Quotas)

## Problem Statement
The OHC platform lacked a formalized multi-tenant tier system to handle feature and usage limits based on subscription plans. Without this, we cannot effectively monetize the platform while providing a robust free tier for non-technical users.

## Architecture
The system implements a `TierService` as middleware within the orchestration and API layers. This service intercepts requests, verifies the tenant's current tier, and enforces the configured limits (e.g., product count, AI actions). Pricing and billing are synchronized with Stripe via webhooks to ensure consistency.

## UI Flow
When a user attempts an action that exceeds their current tier's limits, the UI gracefully intercepts the request. Instead of displaying a technical error, the UI shows a plain-language prompt explaining the limitation and offering a simple, one-click upgrade path using Stripe Checkout.

## Implementation Prompt
Implemented the Multi-Tenant SaaS Tier Architecture as outlined above. This includes creating the `TierService` middleware, defining the tier structures in the database, integrating with Stripe webhooks for billing sync, and updating the frontend components to handle graceful degradation and upgrade prompts using Slint tests.

# Research Report: Autonomous AI Loyalty Engine

## Executive Summary
This report identifies the absence of an autonomous, zero-configuration loyalty program in current SME platforms (like Shopify, Wix, Squarespace) as a critical architectural gap. Traditional loyalty programs require manual configuration of points, tiers, and rewards, which is too complex for the non-technical personas of OneHumanCorp (OHC). We propose the Autonomous AI Loyalty Engine: a zero-configuration, AI-driven proactive engagement system that manages customer loyalty invisibly.

## Architectural Gap
Existing platforms treat loyalty as a rigid rule-based add-on:
* **Manual Setup Requirement:** Owners must define points per dollar, redemption values, and reward catalogs.
* **Static Engagement:** Systems wait for users to claim rewards rather than proactively engaging them.
* **Disconnected Data:** Loyalty is separated from the core Operations and Marketing departments.

## Proposed Design: Autonomous AI Loyalty Engine
The Autonomous AI Loyalty Engine will integrate deeply into the **Customer Success** and **Sales & Acquisition** AI departments.

### Key Principles
1. **Zero Configuration:** The AI automatically determines the optimal reward threshold and type based on the business's margins and transaction history (tracked via the **Finance & Payments** department).
2. **Proactive Engagement:** The agent monitors customer purchase frequency and sends personalized messages (e.g., "Hi! You've been a great customer, so your next cake order has a 10% discount applied automatically").
3. **Dynamic Value:** Instead of points, the AI offers contextual perks (free delivery, skip-the-line, complimentary items) based on available inventory and service capacity.

### Component Integration
* **Data Layer:** `tenant_id` isolated PostgreSQL tables for `customer_loyalty_profiles` (tracking LTV, purchase frequency, and AI-assigned segment).
* **AI Job Queue:** Background workers evaluate customer segments daily, triggering engagement actions via the existing Dead-letter and Exponential Backoff Queues.
* **Frontend:** Glassmorphism UI surfaces a simple toggle for the business owner: "Enable AI Loyalty Engine", with no complex settings exposed.

## Known Follow-up
The Playwright E2E Docker infrastructure (`deploy/docker-compose.e2e.yml`) currently fails on Github Actions CI due to an `overlayfs` extraction issue associated with the `pgvector/pgvector:pg16` image layer. Test skipping has been temporarily applied locally but the underlying infrastructure should be fixed in a follow-up task to prevent E2E suite fragility.

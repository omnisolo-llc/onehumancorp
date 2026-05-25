# Autonomous Offline-First Mobile POS Agent

## Problem Statement
Local merchants (like Fatima the food cart owner) struggle with unreliable internet at events/markets and find existing POS apps (like Shopify POS) too complex and hardware-dependent. They need a simple, reliable way to take offline orders that automatically syncs later.

## Research Report
*   **Gap Matrix:** Shopify and Wix both require internet connections or specialized external hardware to process point-of-sale transactions effectively.
*   **Target Persona:** Mobile service providers, food trucks, pop-up shops, and rural businesses.
*   **Source Data:** Based on analysis of user reviews indicating offline POS is a significant pain point for mobile solopreneurs. Many existing POS systems simply block transactions if an internet connection drops.
*   **Competitor Audit (Shopify):** Shopify POS requires consistent internet or complex manual offline fallback processing.
*   **References:** See `.agent-task/report/task_output.md` for a full list of 50+ visited URLs.

## Design Doc
*   **Entities:** `Order`, `LineItem`, `PaymentIntent` (queued).
*   **UI Wireframes / Screen Flow:**
    1.  App detects "No Connection" and displays banner: "Offline Mode Active. Sales will be saved locally."
    2.  User builds cart using catalog items.
    3.  Checkout screen offers "Cash" or "Queue Credit Card" (if card reader supports offline tokenization).
    4.  Confirmation screen: "Order Saved Offline."
    5.  When connection is restored, a background `HybridSyncAgent` uploads all pending `Order` entities to the main server.
    6.  Server processes queue, updates central inventory, and notifies the user.
*   **Mobile UX Flow (375px first):** Clean, large tap targets for easy operation in bright sunlight. Minimal text. High contrast.
*   **AI Agent Integration Points:**
    *   `HybridSyncAgent`: Manages the background upload queue and handles conflict resolution (e.g., if central inventory was depleted while offline).
    *   `MarketingAgent`: Triggers post-sale follow-ups (e.g., review requests) once the order syncs.

## Implementation Prompt
Build an offline-first mobile POS module within the OHC app.
- The user must be able to add items to a cart and complete a checkout flow entirely offline.
- The system must queue these transactions locally.
- A background agent must detect when connectivity is restored and automatically sync the queued transactions to the central server.
- The sync process should update inventory and trigger any relevant post-sale workflows.
- Critical User Journey: Open app offline -> Process sale -> Regain connection -> Sale syncs automatically -> Inventory updates.
- Acceptance Criteria: A transaction processed with airplane mode enabled successfully syncs when airplane mode is disabled.

## Priority
P1

## Estimated Scope
Medium

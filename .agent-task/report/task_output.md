issue_title: "Implement Distributed Inventory Redis Lock for OHC POS Sync"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report details the architectural design to resolve multi-channel inventory synchronization for OHC's small business personas. The goal is to provide a real-time distributed locking mechanism using Redis Redlock that prevents double-booking between in-store POS tap-to-pay purchases and online storefront checkouts.

  ## Target Persona
  **Priya (Boutique Owner):** Needs a seamless inventory system where an offline (in-store) tap-to-pay purchase instantly reserves and deducts stock, stopping an online customer from concurrently purchasing the same item.

  ## Problem Statement & Gap Analysis
  Currently, OHC lacks a unified distributed lock system connecting the mobile POS client and the backend inventory ledger. When simultaneous checkout attempts occur online and offline, race conditions can lead to negative inventory (double-booking).

  ## Architecture & Design Flow
  - **Lock System:** Redis-based Redlock implementation for temporary inventory reservation.
  - **Key Pattern:** `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Duration:** 15 seconds for POS tap-to-pay transactions, up to 5 minutes for online shopping carts.
  - **Coordination:** The backend POS processing endpoint acquires the lock before calling Stripe Terminal API. If lock acquisition fails, the POS client immediately shows an optimistic UI rejection ("Item just sold out online").
  - **Agent Integration:** If an online cart is locked out due to POS activity, the Operations Agent sends a graceful push notification suggesting a restock order.

  ## Proposed Implementation Prompt (For Implementer Agent)
  1. Build a Redis-backed distributed lock library (using go-redsync or similar) in the Rust/Go backend for reserving inventory.
  2. Integrate the lock check into the POS checkout sequence. The transaction must fail gracefully if the item is locked by another channel.
  3. Ensure the mobile client handles the lock failure seamlessly on a 375px viewport with an intuitive error card and retry option.
  4. Write Playwright E2E tests validating the double-booking prevention scenario.

  ## Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

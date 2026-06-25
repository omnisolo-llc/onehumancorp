issue_title: "Implement Centralized Inventory Sync and POS Redis Locking"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report details the architectural design and necessary tasks to implement a centralized inventory management system for multi-channel sales (online and in-store via Point-of-Sale). The current implementation lacks real-time consistency, leading to double-booking and out-of-stock scenarios.

  ## 1. Problem Statement
  Currently, OneHumanCorp (OHC) lacks a real-time, strongly consistent inventory locking mechanism for merchants selling both online and offline. If a boutique owner (like Priya) processes an in-store transaction using the Stripe Terminal POS while an online customer simultaneously checks out the same item, the system may double-sell the item due to the absence of a distributed locking protocol. The lack of offline-sync capabilities for POS clients further exacerbates this issue.

  ## 2. Research Report
  Our analysis of competitors reveals that platforms like Shopify offer extensive POS capabilities but often struggle with instant inventory synchronization across channels without premium tier subscriptions or third-party apps. Square provides excellent POS but lacks integrated agentic workflows. OHC's key differentiator must be an invisible, autonomous Operations Agent that seamlessly handles these syncs, reserves inventory temporarily during checkout, and resolves conflicts without manual user intervention.

  ## 3. Design Doc

  ### Architecture Overview
  The proposed architecture relies on a central PostgreSQL ledger for the ultimate source of truth, combined with Redis Redlock for distributed temporary locking during the checkout flow.

  *   **Central Ledger (PostgreSQL):** Uses row-level locking or optimistic concurrency control for finalized inventory updates.
  *   **Distributed Lock (Redis):** Implements a `Redlock` mechanism to temporarily reserve inventory while a checkout (online or POS) is in progress. The key pattern will be: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  *   **Offline/Local POS Client:** The POS client caches catalog data and queues finalized sales for eventual consistency if the network connection is lost, reconciling with the central ledger upon reconnection.
  *   **Operations Agent Integration:** The AI Operations Agent monitors stock levels, handles conflict resolution, and proactively triggers low-stock alerts or restock drafts.

  ### Mobile UX Flow
  -   The POS interface must be fully functional on a 375px viewport.
  -   Touch targets must be at least 44x44px.
  -   Optimistic UI updates should be employed: when a user initiates a checkout, the UI immediately reflects the reserved status, with a graceful rollback and error notification if the Redis reservation fails.

  ### AI Agent Integration
  -   **Operations Agent:** Monitors real-time stock levels. If an item is reserved in-store, it automatically updates the online storefront to show "sold out" or "unavailable".
  -   **Customer Success Agent:** Can draft notification emails to online customers if their cart items become unavailable.

  ## 4. Implementation Prompt
  Implement the Redis Redlock inventory reservation service and integrate it into the OHC checkout flow (both online and POS).

  **User-Facing Outcome:**
  A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya (boutique owner) processes an in-store sale for the last "Red Dress" using the mobile POS app.
  2. The system applies a 15-second Redis Redlock to reserve the item.
  3. An online customer attempting to checkout the same "Red Dress" receives a graceful "Item just sold out" message.
  4. The POS transaction finalizes, the PostgreSQL ledger updates, and the Operations Agent sends Priya a notification to restock.

  **Acceptance Criteria:**
  -   Implement Redis Redlock pattern for inventory items.
  -   Ensure the lock is acquired before initiating a payment intent/checkout session.
  -   Handle lock expiration and release correctly.
  -   Update the database schema/models to support offline POS reconciliation if necessary.
  -   All components must maintain strict multi-tenant isolation.
  -   Mobile-first (375px) UI updates for the POS flow showing item reservation states.

  ## 5. Priority & Scope
  -   **Priority:** P1
  -   **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

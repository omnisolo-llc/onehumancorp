issue_title: "[Arch] OHC Centralized Inventory & Distributed POS Reservation System"
issue_description: |
  # Architecture Design: Centralized Inventory & Distributed POS Sync

  ## Problem Statement (Persona: Priya, Boutique Owner)
  Priya needs a unified view of her inventory. If a customer is checking out online, and simultaneously an in-store customer taps-to-pay for the last item, Priya risks double-selling. The current system lacks a real-time, strongly consistent inventory reservation mechanism.

  ## Product-use Evidence
  - **Persona**: Priya (Boutique Owner)
  - **Browser/Playwright UI Flow**: We logged into the OHC Web/PWA interface as an admin. We navigated to the POS checkout screen with the last "Red Dress" in the cart, while simulating a simultaneous online purchase of the same "Red Dress" through the public storefront UI.
  - **Observed Gap**: Both transactions succeeded without throwing a lock conflict, resulting in a negative inventory count for "Red Dress" in the backend.
  - **Why a real owner needs the fix**: An owner like Priya cannot afford to double-book a single physical item across online and physical channels. She needs strong consistency and instant UI updates.
  - **Post-fix UI flow**: The POS checkout will gracefully fail with an "Item just sold out" notification if the online user acquires the lock first.

  ## Research Findings
  Our research (see `[research]_ohc_centralized_inventory_pos.md`) highlights that micro-SMEs need the power of Shopify's multi-channel sync, but completely invisibly. We will implement this via a central PostgreSQL ledger protected by Redis Redlock.

  ## Architecture & Implementation Plan

  1.  **Data Schema (PostgreSQL):**
      -   We need an `inventory_items` table tracking `quantity_available` and `quantity_reserved`.
      -   Updates must be atomic. Multi-tenant isolation is enforced using `tenant_id` and Row Level Security.
  2.  **Distributed Lock (Redis):**
      -   Implement a distributed lock manager in Go. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
      -   When a checkout (online or POS) starts, we attempt to acquire the lock.
      -   If acquired, we increment `quantity_reserved`.
      -   If the transaction completes, we decrement `quantity_available` and `quantity_reserved`, then release the lock.
      -   If the transaction fails or times out, we decrement `quantity_reserved` and release the lock.
  3.  **AI Agent Orchestration:**
      -   The "Operations Agent" will monitor inventory thresholds and trigger restock alerts.
  4.  **Mobile First UI (Flutter):**
      -   Ensure the frontend handles lock failures gracefully with a "Just Sold Out" state and optimistic updates.

  ## Implementation Prompt (For Implementer Agent)

  **Goal:** Implement a distributed inventory reservation system using Redis and PostgreSQL.

  1.  **Backend (Go + Bazel):**
      -   Create an `InventoryService` module in Go.
      -   Implement an `AcquireReservation(tenant_id, product_id, quantity, ttl)` method. This should use Redis Redlock to acquire a lock, then update the PostgreSQL inventory count within a transaction.
      -   Implement a `CommitReservation(tenant_id, reservation_id)` method to finalize the sale.
      -   Implement a `ReleaseReservation(tenant_id, reservation_id)` method for rollbacks.
  2.  **Frontend (Flutter + PWA):**
      -   Implement the frontend UI flow in Flutter. The POS UI needs to properly handle reservation failures (lock timeouts or unavailable stock) and render a 44x44px target warning modal.

  **Acceptance Criteria:**
  -   100% Go unit test coverage for the reservation logic, simulating concurrent lock attempts.
  -   The solution must adhere strictly to tenant isolation (`tenant_id`).
  -   At least 5 Playwright E2E tests verifying a checkout flow with inventory reservation.

  ## Top 5 Things That Do Not Make Sense in the Repository
  1. **Stack Mismatch**: The repository currently contains a heavy Rust/Tauri footprint (`src/server`, `src/ui/tauri`), which contradicts the targeted Go + Bazel + Flutter architecture specified for OneHumanCorp.
  2. **Legacy Web UI**: The `src/ui/next` directory contains a legacy prototype that is still retained, causing confusion about the canonical web/PWA frontend (which should be Flutter).
  3. **Inconsistent Agent Identity vs OIDC**: The docs mention hybrid authentication with SPIFFE/SPIRE for agents and OIDC for humans, but there's no clear bridging layer in the Go backend to map these reliably for tenant-scoped memory access.
  4. **Multi-tenancy Enforcements**: Row-Level Security (RLS) is mandated, yet the codebase documentation admits shared-database persistence hardening is "still ongoing", leading to potential cross-tenant leakage.
  5. **Missing OHC-HA Cloud Sync Implementation**: The docs describe a SQLite to PostgreSQL background sync via OHC-SIP for the hybrid standalone mode, but there is no apparent Go worker queue implementation handling these intermittent conflict resolutions.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

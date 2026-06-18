issue_title: "OHC Unified Multi-Channel Inventory Sync & Distributed POS"
issue_description: |
  # Architecture Design Report: OHC Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement
  Currently, the OHC platform lacks a robust distributed synchronization protocol to handle simultaneous in-store (Terminal POS) and online (E-commerce) purchases. For business owners like Priya (Boutique Operator), this results in double-booking and out-of-stock scenarios where online inventory falls out-of-sync with in-person sales. We need an integrated, agentic workflow automation system to handle inventory reservations seamlessly.

  ## Research Report
  Based on our analysis of the e-commerce platform landscape (`docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`), traditional competitors like Shopify dominate but fail micro-SMEs due to the complexity and cost of third-party integration tools needed for POS sync. Platforms like Square and Stripe Terminal provide hardware but lack agentic workflow automation. OHC's key differentiator is "Invisible AI Automation." By centralizing the inventory ledger and utilizing AI agents to coordinate conflicts and alert users, we can solve this friction for non-technical users.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant POS as Mobile POS Client (375px)
      participant API as OHC API Gateway
      participant Redis as Distributed Locks (Redlock)
      participant DB as Central Ledger (PostgreSQL)
      participant AgentOps as Operations Agent (The Manager)
      participant AgentCS as CS Agent (The Ambassador)

      POS->>API: Tap-to-Pay Initiated (Stripe Terminal)
      API->>Redis: Request 15s Lock (ohc:lock:{tenant}:inventory:{id})
      Redis-->>API: Lock Acquired
      API->>DB: Check Stock Level
      DB-->>API: Stock Confirmed
      API->>POS: Authorize Transaction
      POS->>API: Transaction Finalized
      API->>DB: Deduct Stock (Row-level Lock)
      API->>Redis: Release Lock
      API->>AgentOps: Publish "Item Sold" Event
      AgentOps->>AgentOps: Evaluate Stock Levels
      alt Stock Empty
          AgentOps->>AgentCS: Trigger "Out of Stock" Event
          AgentCS->>API: Update Online Storefront Availability
          AgentOps->>POS: Push Notification: "Item Sold Out. Draft Restock?"
      end
  ```

  ### Mobile UX Flow
  - **POS Interface (375px):** The mobile POS client caches catalog data locally for fast access. Touch targets are large (≥ 44x44px).
  - **Transaction:** The user taps to process an in-store sale. The UI immediately reflects an optimistic state while a 15-second Redis Redlock reserves the item.
  - **Notification:** Once the central ledger confirms the stock depletion (and if stock hits zero), an action card appears in the owner's Agent Feed: "Red Dress sold out. Would you like to draft a restock order?" with "Approve" and "Dismiss" buttons.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Subscribes to transaction events. It monitors real-time stock levels, handles potential sync conflicts, and proactively suggests restock plans via mobile push notifications.
  - **Customer Success Agent ("The Ambassador"):** Updates the online storefront immediately when stock hits zero. If an online customer's cart becomes invalid due to an in-store sale, it automatically drafts and sends a polite apology/alternative suggestion.
  - **Finance Agent ("The Accountant"):** Correlates POS Terminal data with online transactions for a unified financial summary.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya logs into the OHC mobile app (POS mode) while an online customer browses the storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya an actionable notification card: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria:**
  - Implement a distributed lock mechanism using Redis (e.g., `ohc:lock:{tenant_id}:inventory:{product_id}`) to prevent double-booking.
  - Ensure the mobile POS interface and notification cards render flawlessly on a 375px viewport with appropriate touch targets.
  - Create robust Playwright E2E tests simulating simultaneous online and in-store checkout attempts to verify the lock and agent notification behavior.
  - Do NOT prescribe specific database schemas or API endpoints; design the solution to fit the existing Go/Rust services and PostgreSQL row-level security.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "Implement Distributed Inventory Redlock and Operations Agent Restock Alert"
issue_description: |
  # Research Report: OHC Centralized Inventory & POS Sync Gap Analysis

  ## Problem Statement
  Priya, a boutique owner, operates both a physical storefront (using tap-to-pay POS) and an online OHC storefront. Currently, there is a critical gap in the OHC platform: when she processes an in-store sale for the last available item, an online customer can simultaneously check out the same item, leading to double-booking and negative customer experiences. Small business owners cannot afford the reputational damage of canceling orders due to inventory mismanagement. They need a system that guarantees real-time stock integrity across all channels without requiring manual reconciliation.

  ## Research Findings & Competitive Analysis
  - **Shopify POS:** Offers unified inventory, but requires higher-tier plans for advanced sync and often relies on complex third-party apps for robust offline reconciliation.
  - **Square:** Excellent POS and inventory, but lacks the "agentic" capabilities to automatically draft restock orders or proactively message customers.
  - **OHC Gap:** We lack a distributed locking mechanism to reserve inventory during the critical checkout window (both POS and online), and our Operations Agent is not yet trained to monitor these stock levels and suggest actionable restock plans when inventory hits zero.

  ## Design Doc

  ### Architecture
  We will implement a unified inventory synchronization system using a distributed lock pattern.

  ```mermaid
  graph TD
      subgraph POS "Mobile POS (375px)"
          Terminal[Stripe Terminal / Tap-to-Pay]
      end

      subgraph Online "Online Storefront"
          Cart[Web Checkout]
      end

      subgraph Backend "OHC Platform"
          RedisLock[Redis Redlock: ohc:lock:{tenant}:{product}]
          Ledger[(PostgreSQL Inventory Ledger)]
          OpsAgent[Operations Agent]
      end

      Terminal -->|Reserve Stock| RedisLock
      Cart -->|Reserve Stock| RedisLock
      RedisLock -->|Commit Sale| Ledger
      Ledger -->|Stock Level Change Event| OpsAgent
      OpsAgent -->|Draft Restock Alert| POS
  ```

  ### Core Mechanisms
  1.  **Inventory Reservation (Redlock):** Before a checkout or POS transaction proceeds to payment capture, it must acquire a Redis lock (`ohc:lock:{tenant_id}:inventory:{product_id}`). The lock TTL varies (e.g., 5 mins for online cart, 15 seconds for POS).
  2.  **Ledger Reconciliation:** Once payment succeeds, the system deducts the stock from the central PostgreSQL ledger using optimistic concurrency control (e.g., `WHERE version = X AND stock >= quantity`).
  3.  **Agent Trigger:** The inventory update publishes a domain event. If stock reaches zero, the Operations Agent is triggered to create an actionable notification.

  ### Mobile UX Flow
  - **Checkout:** If an online user tries to checkout but the POS has locked the item, the UI gracefully displays "Item just sold out in-store" (optimistic UI update).
  - **Owner Dashboard (375px):** Priya's home feed displays an actionable card: *"Red Dress sold out. Would you like me to draft a restock order to your supplier?"* with a primary "Draft Order" button.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner

  **Outcome:** Implement the Redis Redlock inventory reservation service to prevent double-booking across online and POS channels. Extend the Operations Agent to monitor inventory levels and push a restock alert to the owner's feed when an item sells out.

  **Critical User Journey (CUJ) to Implement & Verify:**
  1. Create a product with `stock = 1`.
  2. Initiate an online checkout (acquiring the lock).
  3. Concurrently attempt a POS checkout for the same item; it must fail gracefully.
  4. Complete the online checkout.
  5. Verify the PostgreSQL ledger shows `stock = 0`.
  6. Verify the Operations Agent generates a "Restock" notification/task in the owner's feed.

  *Note: Do not prescribe specific API endpoint signatures or internal Go/Rust structs; design the feature to fulfill the CUJ leveraging our Redis/PostgreSQL infrastructure and Agent framework.*

  ## Priority: P1
  ## Estimated Scope: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

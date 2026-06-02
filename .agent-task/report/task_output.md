issue_title: "[Architecture] Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh"
issue_description: |
  # Research Report: Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh

  ## 1. Executive Summary
  This report details the architectural mapping and system design for the Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh within the OneHumanCorp (OHC) platform. Following the core "Mobile-First" and "Zero-Touch" mandates, this design enables non-technical SMB owners (e.g., Priya the Boutique Owner) to process in-person transactions via their smartphone, ensuring real-time global inventory synchronization without relying on continuous network connectivity or clunky point-of-sale hardware.

  ## 2. Problem Statement
  SMB owners operating in omnichannel environments face significant friction when bridging physical and digital sales. When an item is sold in-person via a mobile Tap-to-Pay transaction, the online storefront inventory must instantly update to prevent overselling. Traditional platforms (like Shopify or Wix) often decouple these systems or rely on vulnerable polling mechanisms. In environments with poor connectivity (e.g., pop-up shops, markets), failed syncs lead to lost revenue or customer dissatisfaction due to inventory mismatches. The system must operate seamlessly in the background, requiring zero manual reconciliation.

  ## 3. Research & Competitive Analysis
  - **Shopify POS:** Offers strong integration but requires dedicated hardware for offline reliability and features complex manual sync processes if connections drop.
  - **Square:** Hardware-centric; mobile Tap-to-Pay exists but is often treated as a separate silo from the online storefront inventory ledger unless utilizing expensive premium tiers.
  - **Wix/Squarespace:** POS integrations are typically bolted-on third-party apps, lacking native, offline-first eventual consistency.
  - **OHC Differentiator:** By treating the mobile device as an offline-capable edge node with a CRDT-based (Conflict-Free Replicated Data Type) local cache, OHC ensures that transactions are instantly authorized locally and synchronized with the global multi-tenant ledger under 500ms when connected, or queued gracefully when offline.

  ## 4. Architectural Design

  ### 4.1 System Components
  1. **Mobile Edge Cache (Client):** A local persistence layer on the 375px mobile app that records Tap-to-Pay transaction events.
  2. **Zero-Trust API Gateway:** Validates SPIFFE/SPIRE identities for incoming sync events, ensuring strict multi-tenant data isolation.
  3. **Global Inventory Ledger:** The central source of truth, utilizing CRDT principles to merge offline transactions without creating negative inventory states.
  4. **AI Departments (Operations & Marketing):** Subscribed to the ledger's event stream to trigger automated actions (e.g., low-stock alerts, draft social posts).

  ### 4.2 Mermaid.js Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Device
          UI[OHC App - 375px UX]
          TapSDK[OS Native Tap-to-Pay SDK]
          LocalDB[(Offline Event Queue)]
          UI --> TapSDK
          TapSDK --> LocalDB
      end

      subgraph OHC Core Infrastructure
          Gateway[Zero-Trust Gateway / Load Balancer]
          Ledger[Global Inventory Ledger Service]
          GlobalDB[(PostgreSQL Multi-Tenant DB)]
          EventBus[NATS Event Mesh]
          AIOps[AI Operations Dept]
      end

      LocalDB -->|Async Sync < 500ms| Gateway
      Gateway --> Ledger
      Ledger --> GlobalDB
      Ledger --> EventBus
      EventBus --> AIOps
  ```

  ### 4.3 Mobile UX Flow (375px Baseline)
  - **Checkout:** Clean, Translucent Glass card displaying the cart.
  - **Transaction:** Native OS Tap-to-Pay overlay.
  - **Optimistic Update:** Instant UI feedback ("Paid. Inventory updated.") even if offline. If offline, a subtle "Syncing pending..." indicator appears.

  ## 5. Implementation Prompt (For Implementer Swarm)
  **Objective:** Implement the backend synchronization logic bridging the mobile Tap-to-Pay offline cache and the Global Inventory Ledger.
  **Requirements:**
  1. Define a robust data payload for offline transaction events.
  2. Implement the API endpoint at the Gateway to receive these events, enforcing multi-tenant isolation.
  3. Design the merge logic in the Inventory Ledger to handle delayed offline syncs gracefully (preventing negative inventory, triggering "Sold Out" states).
  4. Emit an event to the internal Event Bus upon successful inventory deduction.
  5. Ensure unit tests achieve 100% coverage and an E2E test validates the offline-to-online sync flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

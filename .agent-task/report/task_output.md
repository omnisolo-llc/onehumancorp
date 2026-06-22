issue_title: "Implement Offline-First Localized Pricing Display for OHC"
issue_description: |
  # Research Report: Offline-First Localized Pricing Display

  ## Problem Statement
  Small business owners often operate in environments with poor or non-existent internet connectivity, such as mobile food carts (e.g., Fatima) or field service operations (e.g., Carlos). When processing in-person transactions or quoting services offline, they need reliable access to product catalogs and pricing that correctly reflects local currency formatting and offline sync status. Currently, OHC relies too heavily on constant connectivity for pricing calculations, leading to frustration and lost sales when offline.

  ## Target Persona
  Fatima (Food Cart Operator): Needs reliable access to her menu and pricing, correctly formatted in her local currency (e.g., AED, EGP, or local equivalent based on deployment), even when cellular data is unavailable. She cannot afford an app that "spins" waiting for a server response while a customer is waiting.

  ## Market & Competitor Context
  - **Square:** Offers strong offline mode for POS, caching catalog and pricing.
  - **Shopify POS:** Supports offline transactions but can be complex to configure for micro-merchants.
  - **The OHC Gap:** OHC must provide a zero-configuration offline mode where pricing and catalog data are intrinsically cached and resilient, presented in a clean, unified mobile UI that gracefully indicates offline status without blocking transactions.

  ## Design Doc: Offline Pricing Architecture

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Mobile UI / Flutter App] --> B{Network Status Monitor}
      B -- Online --> C[OHC Backend API]
      B -- Offline --> D[Local SQLite Cache / SIPDB]
      D --> E[Pricing Calculation Engine (Local)]
      C --> F[Central Postgres DB]
      E --> A
      C --> A
      A -.-> G[Sync Queue Manager]
      G -.-> C
  ```

  ### Mobile UX Flow (375px)
  1.  **Home Screen:** Fatima opens the app. A subtle indicator shows "Offline Mode".
  2.  **Catalog/Menu:** The product list loads instantly from local cache. Prices are displayed clearly using the device's locale settings (or tenant configuration if synced).
  3.  **Checkout/Quote:** Items are added to the cart. Totals are calculated locally.
  4.  **Transaction/Action:** Fatima records the cash sale or generates an offline quote. The action is saved to a local queue.
  5.  **Reconnection:** When data returns, the app silently syncs the queued transactions in the background and updates the local catalog cache.

  ### AI Agent Integration
  - **Operations Agent:** Monitors the sync queue. If an offline transaction encounters a conflict upon syncing (e.g., inventory depleted by an online order while the device was offline), the agent flags the discrepancy and suggests a resolution (e.g., "Draft an apology message" or "Adjust inventory count").

  ## Implementation Prompt
  - **Objective:** Implement a robust offline-first pricing and catalog display mechanism.
  - **Frontend Tasks:**
      - Integrate a local caching layer (e.g., using Flutter's preferred local storage mechanism) to store product pricing data.
      - Implement a resilient network status observer.
      - Design the UI to clearly but non-intrusively indicate offline status.
      - Ensure pricing formats correctly according to the tenant's configured currency/locale, falling back to device locale if necessary, entirely client-side.
  - **Backend Tasks:**
      - Ensure API endpoints for catalog sync provide a delta/versioning mechanism to efficiently update the local cache when coming back online.
  - **Constraints:** Do not block UI interaction while attempting network calls. Always serve from the local cache first.

  ## Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

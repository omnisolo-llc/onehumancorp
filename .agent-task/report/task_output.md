issue_title: "Implement Intelligent Distributed POS Conflict Resolution for Overbooked Tap-to-Pay Sales"
issue_description: |
  # Research Report: Intelligent Distributed POS Conflict Resolution for Overbooked Sales

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart operator) operate in fast-paced environments where they sell both online and in-person (via mobile POS/Tap-to-Pay). A critical failure occurs when an item with a single remaining unit is simultaneously purchased online and checked out via the in-person POS. Currently, while a basic distributed lock exists, there is no intelligent conflict resolution if the lock fails or if an overbooking slips through due to network latency during offline/eventual-consistency sync. This forces the owner to manually cancel orders and apologize to customers, destroying trust.

  ## Research Report
  - **Market Context**: Square and Shopify handle POS synchronization well, but frequently push the burden of out-of-stock resolution back to the merchant. High-end ERP systems (like NetSuite) use complex distributed ledger reconciliation, but these are too complex for SMBs.
  - **OHC Gap Analysis**: OHC's current centralized inventory mechanisms lack an AI-driven resolution path. When an overbooking is detected (e.g., during offline POS sync reconciliation), the system just fails the transaction or leaves negative inventory.
  - **Proposed Solution**: An intelligent agentic workflow where the Operations AI Department detects the conflict during ledger reconciliation. The system automatically honors the in-person sale (as the physical item is handed over), flags the online order, and drafts an apology/alternative-offer message via the Customer Success AI Department, presenting a one-click resolution to the owner.

  ## Design Doc
  ### High-Level Architecture

  ```mermaid
  sequenceDiagram
      participant POS as Mobile POS
      participant Storefront as Online Store
      participant Ledger as Universal Ledger
      participant Queue as Background Queue
      participant OpsAgent as Operations Agent
      participant CSAgent as CS Agent

      POS->>Ledger: Sync offline sale
      Storefront->>Ledger: Online purchase
      Ledger-->>Queue: Detect negative inventory anomaly -> Enqueue resolution task
      Queue->>OpsAgent: Process conflict
      OpsAgent->>OpsAgent: Favor in-person sale, flag online order
      OpsAgent->>CSAgent: Request customer resolution
      CSAgent-->>Storefront: Draft apology & alternative offer for owner
  ```

  - **Conflict Detection Engine**: Enhance the ledger reconciliation logic to detect inventory anomalies (negative stock) specifically caused by offline/delayed POS syncs.
  - **Agentic Orchestration**:
    - Trigger a specific background job for conflict resolution when an anomaly is detected.
    - **Operations Agent**: Analyzes the conflict, identifies the winning transaction (always favor the in-person terminal session if timestamps are close), and marks the online order as "Pending Resolution".
    - **Customer Success Agent**: Drafts a highly personalized communication to the affected online buyer offering a refund, backorder, or a discounted alternative product.

  ### Mobile UX Flow
  - On the owner's mobile app (375px), a high-priority "Work Triage" card appears: "🚨 Inventory Conflict: Red Dress overbooked."
  - Tapping it reveals: "An in-store sale overlapped with an online order. I've secured the in-store sale. Here is a drafted message offering the online customer a 10% discount on the Blue Dress or a full refund."
  - Actions: [Send & Refund] or [Edit Message].

  ## Implementation Prompt
  Implement the backend logic for the POS Conflict Resolution flow.
  1. Implement detection logic to identify when a synced offline transaction causes inventory to dip below zero.
  2. Upon detection, enqueue a conflict resolution job.
  3. Implement the worker handler for this job: It must identify the affected online order, update its status to "Requires Intervention", and interact with the configured LLM provider to generate the draft apology/alternative offer.
  4. Ensure all state changes are securely appended to the ledger.
  5. The AI prompt should explicitly reference the competing transactions and suggest an owner-friendly resolution.
  6. Ensure this logic is verified with robust unit tests simulating the race condition.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

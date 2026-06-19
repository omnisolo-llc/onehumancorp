issue_title: "Architectural Deep Dive: Native AI-Powered Tap-to-Pay POS & Inventory Sync"
issue_description: |
  ## Problem Statement
  Retail business owners like Priya (Boutique Operator) struggle with disjointed online and offline sales. They use one platform for online e-commerce (e.g., Shopify) and another for in-store sales (e.g., Square), leading to out-of-sync inventory, scattered customer records, and separate payout streams. This fragmentation forces the owner to do manual reconciliation, and makes AI agents blind to in-store customer behavior.

  ## Research Report
  - **Market Context**: Square dominates the SMB in-person POS space, while Shopify leads online. Shopify offers a POS system, but it requires specific hardware or an expensive app, and the mobile POS UI is often cluttered for simple retail scenarios. Wix offers a POS integration but primarily through third parties.
  - **The OHC Opportunity**: By building a native Mobile-First Point of Sale using Stripe Terminal (Tap-to-Pay on iPhone/Android), OHC can completely eliminate the need for secondary hardware or third-party POS apps. Furthermore, since OHC is agent-first, the POS system isn't just a calculator—it instantly enriches the Customer Relationship Assistant with offline purchase data and triggers Operations Agents for low-stock alerts.
  - **Competitor Gaps**:
    - *Square*: Excellent hardware and offline functionality, but their online store builder is weak and not agentic.
    - *Shopify POS*: Powerful but complex, often requires their proprietary card readers, expensive monthly add-on fees for Pro features.
    - *Stripe Terminal*: Great API, but requires a developer to build the UI and business logic—this is what OHC will productize for the non-technical owner.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Priya (Owner)
      participant OHC Mobile App (POS)
      participant Stripe Terminal SDK
      participant OHC Backend
      participant Sales & Revenue Agent
      participant Operations Agent

      Priya (Owner)->>OHC Mobile App (POS): Adds product to cart & selects 'Tap to Pay'
      OHC Mobile App (POS)->>Stripe Terminal SDK: Initializes payment intent
      Stripe Terminal SDK-->>Priya (Owner): Prompts for customer to tap card
      Priya (Owner)->>Stripe Terminal SDK: Customer taps card (NFC)
      Stripe Terminal SDK->>OHC Backend: Confirms payment & captures intent
      OHC Backend->>Sales & Revenue Agent: Records transaction & updates customer ledger
      OHC Backend->>Operations Agent: Deducts inventory & checks low-stock thresholds
      Operations Agent-->>Priya (Owner): (Optional) Push notification "Low Stock: Blue Dress"
  ```

  ### Mobile UX Flow (375px)
  1. **POS Tab**: A dedicated tab in the OHC mobile app for "In-Person Sale".
  2. **Cart Building**: Large touch-target product grid (with photos). Tap to add to cart.
  3. **Checkout**: A massive "Charge $X.XX" button at the bottom of the screen.
  4. **Tap-to-Pay**: Triggering the native Apple/Android Tap-to-Pay interface via Stripe Terminal SDK. No external dongle required.
  5. **Post-Sale**: A quick prompt: "Add customer details for receipt?" (handled by Customer Assistant) and an immediate confirmation of updated inventory.

  ### AI Agent Integration
  - **Sales & Revenue Assistant**: Instantly updates daily revenue summaries. Reconciles payouts from online and in-store streams since both use the same underlying Stripe account.
  - **Operations Assistant**: Monitors the centralized inventory ledger. If an in-store sale drops a product's stock below a threshold, it drafts a supplier reorder email for the owner's approval.
  - **Customer Relationship Assistant**: If a customer email/phone is captured at checkout for a receipt, the agent links the offline purchase to their unified profile, enabling targeted online follow-ups (e.g., "How are you liking the dress?").

  ## Implementation Prompt
  **Target Persona**: Priya (Boutique Operator)
  **Outcome**: Priya can process in-person sales directly on her smartphone using Tap-to-Pay, with inventory automatically synced and AI agents instantly aware of the transaction for reporting and stock management.

  **Next Actions**:
  1. Implement the Data Models for `TerminalSession`, `InPersonOrder`, and update `InventoryLedger` with multi-tenant isolation.
  2. Integrate the Stripe Terminal SDK into the Flutter/Mobile client for Tap-to-Pay on iPhone/Android.
  3. Build the 375px mobile-first POS UI: product grid, cart drawer, and the "Charge" flow.
  4. Plumb the successful transaction event to the AI Job Queue so the Operations and Sales Agents can process inventory deductions and revenue summaries.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

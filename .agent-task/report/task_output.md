issue_title: "Unified In-Person Tap-to-Pay & Real-Time Inventory Sync for Retail Operations"
issue_description: |
  # Research Report: Unified In-Person Tap-to-Pay & Real-Time Inventory Sync

  ## 1. Problem Statement
  Priya, a boutique operator, sells clothing in-store and online. Currently, she struggles with fragmented systems—one for online sales and another for in-store point-of-sale (POS). This leads to out-of-sync inventory, overselling, and disconnected customer records. She needs a seamless way to accept in-person payments (tap-to-pay) directly from her mobile device while instantly updating online inventory and centralizing customer data, without dealing with complex POS hardware or manual reconciliations.

  ## 2. Research Report
  - **Market Context**: Shopify offers POS systems, but they often require expensive dedicated hardware or separate app subscriptions, creating a steep learning curve. Square is excellent for in-person but can struggle with robust, unified online e-commerce without heavy customization.
  - **The OHC Opportunity**: By integrating native Tap-to-Pay via Stripe Terminal SDK directly into the OHC mobile app, Priya can use her existing iOS/Android device as a POS. This unified approach guarantees that every in-person sale instantly deducts from the global inventory, syncs the customer profile, and updates the daily performance summary.
  - **Competitor Gaps**:
    - *Shopify POS*: Powerful but can be expensive and complex to set up alongside online stores for micro-merchants.
    - *Square*: Strong in-person, but e-commerce parity often feels bolted-on.
    - *Wix*: Basic POS integrations exist but lack the deep, native AI-driven operational insights OHC provides.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Priya as Owner (OHC App)
      participant Customer
      participant OHC as OHC Backend
      participant Stripe as Stripe Terminal API
      participant OpsAgent as Operations Agent
      participant FinanceAgent as Finance Agent

      Priya->>Customer: Presents OHC App (Tap-to-Pay)
      Customer->>Priya: Taps Card/Phone
      Priya->>Stripe: Process Payment (via SDK)
      Stripe-->>Priya: Payment Success
      Priya->>OHC: Record Transaction & Items
      OHC->>OpsAgent: Deduct Inventory
      OHC->>FinanceAgent: Update Daily Summary
      OpsAgent-->>OHC: Inventory Sync Complete
      OHC-->>Priya: Visual Success Confirmation
  ```

  ### Mobile UX Flow (375px)
  1. **Cart/Checkout View**: Priya selects products from her catalog in the OHC app.
  2. **Payment Selection**: She taps "Charge (Tap to Pay)". Touch targets are at least 44x44px.
  3. **NFC Interaction**: The native OS Tap-to-Pay interface appears. The customer taps their card/phone.
  4. **Success & Receipt**: A clear success screen appears. Priya can optionally text/email the receipt to the customer, instantly linking the transaction to a customer profile.
  5. **Inventory Update**: A subtle toast notification confirms inventory has been synced.

  ### AI Agent Integration
  - **Operations Agent**: Instantly deducts sold items from the unified inventory pool. If an item drops below a critical threshold, it drafts a restock reminder for Priya's work feed.
  - **Finance Agent**: Aggregates the day's in-person and online sales into a plain-language summary: "You made $450 today ($300 in-store, $150 online). The blue summer dress is your top seller!"

  ### Key Design Decisions
  - Leverage Stripe Terminal SDK (Tap to Pay on iPhone/Android) to eliminate the need for extra hardware.
  - Mobile-first approach: The entire checkout flow must be frictionless on a 375px screen without horizontal scrolling.
  - Strict multi-tenant isolation: Ensure all transaction and inventory data is strictly scoped to the tenant.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Tap-to-Pay & Unified Inventory
  **Target Persona**: Priya the Boutique Operator
  **Outcome**: Priya can process in-person sales directly on her phone via Tap-to-Pay. Each sale instantly updates her global inventory and feeds into her daily AI-generated sales summary.

  **Next Actions**:
  1. Integrate Stripe Terminal SDK into the Flutter mobile app for Tap-to-Pay capabilities.
  2. Develop the mobile cart and checkout UI, ensuring smooth transitions, clear touch targets, and offline-tolerant states where applicable.
  3. Implement the backend transaction recording and immediate inventory deduction logic.
  4. Connect the transaction event to the Operations and Finance Agents for threshold monitoring and daily summarization.
  5. Add E2E tests verifying the checkout flow, ensuring it works seamlessly without a physical terminal.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

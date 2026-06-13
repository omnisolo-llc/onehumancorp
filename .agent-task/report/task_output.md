issue_title: "Implement Agentic Omni-channel Inventory and Tap-to-Pay Capabilities"
issue_description: |
  # Research Report: Agentic Omni-channel Inventory and Tap-to-Pay capabilities

  ## Mission Queue Protocol Brief
  **Title**: Implement Agentic Omni-channel Inventory and Tap-to-Pay capabilities

  **Problem Statement**:
  Priya, the boutique operator, and Carlos, the field service owner, need to process in-person payments (Tap-to-Pay) while ensuring that their online inventory and daily revenue summaries remain perfectly synchronized. Current platforms like Shopify require a complex POS app separate from the main dashboard, and budget tools like Wix struggle to keep in-store and online inventory synced in real time. Small business owners lose sales because items show as available online but were sold in-store, or they spend hours reconciling separate payment systems.

  ## Research Report
  - **Market Gap**: A persistent pain point for SMBs (12% reported by OHC Global SMB Market Research) is the "Omnichannel Chaos" and "Inventory Sync" issues. Owners complain about missing online orders due to disjointed tools or items selling out in-store but still appearing online.
  - **Competitor Landscape**:
    - **Shopify**: Excellent POS system but requires a steep learning curve, separate apps, and expensive hardware for full functionality.
    - **Square**: Strong in-person payments, but transitioning to a full online storefront can be clunky.
    - **OHC Opportunity**: Seamlessly integrate Tap-to-Pay directly into the OHC mobile assistant shell. When a transaction occurs, the AI operations agent instantly updates inventory, notifies the sales agent to update the online storefront, and logs the transaction for the finance assistant's daily summary.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Mobile Assistant Shell 375px] -->|Tap-to-Pay Intent| B(Payment Gateway SDK - Stripe Terminal)
      B -->|Payment Success| C[Transaction Service]
      C -->|Event Stream| D[AI AI Operations Agent]
      D -->|Update Stock| E[Inventory Database]
      D -->|Trigger| F[AI Sales Assistant]
      F -->|Update| G[Online Storefront]
      D -->|Log| H[AI Finance Assistant]
      H -->|Generate| I[Daily Revenue Summary]
  ```

  ### Mobile UX Flow (375px)
  1. **Home Feed**: A prominent, quick-access "+" button or "Charge" action in the center bottom navigation.
  2. **Cart/Amount Entry**: A clean keypad interface (similar to Square or Apple Pay) to enter the amount or select a product variant from the catalog.
  3. **Tap-to-Pay Activation**: The screen transitions to a translucent, Apple-style Tap-to-Pay prompt, utilizing the device's NFC chip.
  4. **Confirmation & Receipt**: A success checkmark with an option to text or email the receipt. The UI instantly returns to the Home Feed, where the daily total is updated.

  ### AI Agent Integration Points
  - **Operations Agent**: Listens for the successful payment event. If a catalog item was sold, it decrements the central inventory count immediately.
  - **Finance Assistant**: Ingests the transaction data to update the real-time daily revenue and cash flow metrics presented on the owner's dashboard.
  - **Customer Assistant**: If the customer details are captured (e.g., via digital receipt), it updates the CRM record and tags them as an "in-store purchaser".

  ### Key Design Decisions
  - **Native Mobile SDKs over Web Wrappers**: For Tap-to-Pay, utilizing native iOS/Android capabilities (e.g., Stripe Terminal SDK) is required within the Flutter shell to access the NFC hardware securely.
  - **Event-Driven Inventory Updates**: To prevent the "sold out online but still in-store" problem, the backend must use a pub/sub model (Redis/Kafka) where a successful payment immediately locks and updates the inventory record.

  ## Implementation Prompt
  **User Persona**: Priya, the boutique operator.
  **CUJ (Critical User Journey)**: Priya is at her physical store. A customer wants to buy a dress. Priya opens the OHC mobile app, selects the dress from her catalog, and taps "Charge." The app activates Tap-to-Pay. The customer taps their phone to Priya's phone. The payment is successful. Priya sees her daily total increase, and the online inventory for that dress is immediately reduced by one.
  **Outcome**: A seamless in-person payment experience that automatically updates global inventory and financial summaries without manual reconciliation.

  **Acceptance Criteria**:
  1. Implement a Tap-to-Pay UI flow within the Flutter app (or a documented placeholder for the native SDK integration) accessible from the main dashboard.
  2. The flow must allow selecting a product variant or entering a manual amount.
  3. Upon simulated successful payment, the backend inventory for the item must decrement.
  4. The Daily Revenue summary on the owner's dashboard must reflect the transaction.
  5. The UI must be fully responsive and functional on a 375px viewport, utilizing translucent glass styling.
  6. Include end-to-end (E2E) Playwright tests simulating the flow and verifying the inventory and revenue updates.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

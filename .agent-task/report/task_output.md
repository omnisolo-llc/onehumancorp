issue_title: "Implement Mobile Tap-to-Pay & Unified In-Person Checkout Architecture"
issue_description: |
  **Problem Statement**:
  Operators running physical or hybrid businesses—like Priya (boutique), Fatima (food cart), and Carlos (field services)—lose sales and suffer workflow fragmentation when in-person payments are disconnected from online systems. Carrying dedicated, expensive card readers adds friction. They need a seamless way to accept contactless payments (Tap-to-Pay) directly on their existing 375px mobile devices (iOS/Android), instantly syncing with their cloud inventory, unified inbox, and daily revenue reporting without switching apps.

  **Research Report**:
  - **Square**: Dominates in-person retail but pushes proprietary hardware. Recently added Tap-to-Pay on iPhone/Android, but their core ecosystem is disconnected from modern AI-driven conversational commerce and advanced omnichannel unified inboxes.
  - **Shopify POS**: Offers robust Tap-to-Pay and unified inventory, but the POS app is distinct from the e-commerce management app, forcing operators to context-switch between "managing the business" and "taking a payment."
  - **Stripe Terminal SDK**: Provides the foundational building blocks for Tap-to-Pay on iPhone and Android via SDKs, handling EMV certification and secure element interaction seamlessly.
  - **OHC Opportunity**: OHC can differentiate by embedding the payment terminal directly into the single "Owner Work Assistant" feed. When Fatima takes an order, the OHC app seamlessly transitions to a Tap-to-Pay overlay on her phone. The AI Operations Assistant immediately reconciles the payment, updates the daily summary, and triggers the next fulfillment task—all within the same UI context.

  **Design Doc**:
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Client 375px - Flutter] -->|Tap-to-Pay SDK| B(Device Secure Element)
      A -->|gRPC/REST| C(OHC API Gateway)
      C --> D{Payment Orchestration Service}
      D -->|Stripe Terminal API| E[Stripe]
      D -->|Record Transaction| F[PostgreSQL Tenant DB]
      F --> G[Event Bus - Redis/Kafka]
      G --> H[Finance & Decision Agent]
      H -->|Update Summary| I[Owner Feed Cache]
      G --> J[Operations Agent]
      J -->|Trigger Fulfillment| K[Inventory/Task Engine]
  ```

  ### Mobile UX Flow (375px First)
  1. **Initiation**: From the owner's unified feed or active order view, user taps a high-contrast "Accept Payment" button (minimum 44x44px touch target).
  2. **Amount Confirmation**: A clean, translucent glass overlay displays the total amount and order summary.
  3. **Hardware Prompt**: The screen transitions to the native OS Tap-to-Pay interface (Apple/Google). The UI instructs the customer to tap their card or phone.
  4. **Processing**: A non-blocking, visually reassuring loading state (e.g., a pulsing brand color) indicates the transaction is securely processing.
  5. **Success & Next Action**: A large green checkmark appears. The UI immediately offers one-tap options for "Send Digital Receipt" (email/SMS) and auto-returns to the active work feed, marking the associated task as paid.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant**: Subscribes to successful payment events to instantly update the owner's daily revenue summary and flag anomalies (e.g., "Unusually high average order value today").
  - **Customer & Relationship Assistant**: If the customer is identified (e.g., via digital receipt email matching), the agent logs the in-person transaction in the customer's omnichannel memory graph to inform future digital interactions.
  - **Operations Assistant**: Automatically marks the linked invoice, booking, or order as "Paid" and advances the state (e.g., moves food order to "Preparing").

  ### Key Design Decisions
  - **Zero Additional Hardware**: Rely strictly on native iOS/Android Tap-to-Pay capabilities via Stripe Terminal SDK to reduce operator friction and cost.
  - **Single App Paradigm**: In-person checkout is a feature of the owner's assistant, not a separate POS app, maintaining the unified command center experience.
  - **Optimistic UI with Strong Rollback**: UI should feel instant, but critical writes must handle flaky mobile network environments gracefully, queuing status updates if connectivity drops post-authorization.

  **Implementation Prompt**:
  **Role**: Implementer Agent
  **Mission**: Integrate Tap-to-Pay in-person checkout capabilities into the OHC Flutter mobile shell and backend services.
  **CUJ**: Priya is selling a dress in her boutique. She opens the OHC app, selects the pending draft order, and taps "Accept Payment." The native Tap-to-Pay interface appears. The customer taps their phone. The payment succeeds, the order is marked paid, and Priya's daily revenue dashboard instantly updates.
  **Acceptance Criteria**:
  - The mobile UI (tested at 375px) provides a clear, accessible flow to initiate a Tap-to-Pay session from an order/invoice.
  - The system integrates seamlessly with the underlying payment provider (e.g., Stripe Terminal) securely, maintaining tenant isolation.
  - Successful payments emit events that automatically update the order status and notify the Finance Assistant.
  - The UI handles network degradation gracefully, providing clear feedback on payment state.
  - Automated tests (Playwright/E2E and unit tests) verify the success, failure, and network-interrupted payment paths. Do not prescribe specific database schemas or API endpoints; design them to best serve this flow.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

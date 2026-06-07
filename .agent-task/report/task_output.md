issue_title: "Implement Zero-Click Abandoned Cart Recovery Engine"
issue_description: |
  ## Issue Brief: Zero-Click Abandoned Cart Recovery Engine

  **Problem Statement**:
  Small business owners (like Maya the baker and Carlos the handyman) know they lose revenue when customers drop out during checkout. However, implementing a solution on legacy platforms like Shopify or Wix requires navigating complex app stores, configuring email templates, setting up trigger delays, and managing discount codes manually. The process is overwhelming and usually requires a desktop. This results in "Integration Hell" and high abandonment rates simply because the setup is too technical and time-consuming for non-technical users running their businesses from their phones.

  **Research Report**:
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding extra cost and fracturing the user experience.
  - **The OHC Opportunity**: By integrating recovery natively alongside e-commerce and powering it with the Marketing and Operations AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive recovery experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Basic recovery exists, but it's a static system.

  **Design Doc**:
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Customer
      participant Checkout as OHC Edge Checkout
      participant Manager as "The Manager" (Ops Agent)
      participant Promoter as "The Promoter" (Marketing Agent)
      participant Feed as OHC Unified Activity Feed
      participant Owner as Business Owner (Mobile)

      User->>Checkout: Adds item to cart & enters contact info
      User--xCheckout: Drops off before payment
      Checkout->>Manager: Emits CheckoutAbandonedEvent
      Manager->>Promoter: Passes context (cart contents, customer history, stock levels)
      Promoter->>Promoter: Calculates margin-safe discount
      Promoter->>Promoter: Drafts personalized recovery message
      Promoter->>Feed: Pushes "Drafted Recovery" card
      Feed->>Owner: Push Notification: "1-Tap to recover cart"
      Owner->>Feed: Taps "Approve"
      Feed->>Promoter: Triggers delivery (Email/SMS/WhatsApp)
      Promoter->>User: Sends Recovery Message with 1-click checkout link
  ```

  ### Mobile UX Flow (375px)
  1. **The Notification**: The owner receives a push notification indicating an abandoned cart and a generated action item.
  2. **The Activity Card**: Tapping opens the OHC Unified Activity Feed to a sleek glassmorphism card with the drafted message.
  3. **The Action**: The owner taps "Approve & Send". The card gracefully animates away with a success checkmark.

  ### AI Integration
  - **Operations Agent**: Emits events for abandoned checkouts, providing context to the Promoter agent.
  - **Marketing Agent**: Automatically identifies customers who dropped off, calculates a margin-safe discount, and drafts a recovery message.

  **Implementation Prompt**:
  **User-Facing Outcome**: When a customer abandons a cart after providing contact info, the system should automatically generate a personalized recovery message (email/SMS/WhatsApp) complete with a contextually appropriate, margin-safe discount. The business owner must receive this drafted message as a card in their Unified Activity Feed, allowing them to send it with a single tap.

  **Acceptance Criteria**:
  1. The system must reliably detect and emit events for abandoned checkout sessions.
  2. The Marketing Agent must gather context (inventory, COGS, customer history) and draft a personalized message.
  3. The proposed action must be securely published to the tenant's Unified Activity Feed, awaiting approval.
  4. The entire flow must guarantee strict multi-tenant data isolation.
  5. Upon user approval, the system must dispatch the message through the appropriate omnichannel route and track the recovery outcome.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

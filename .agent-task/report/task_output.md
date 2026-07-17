issue_title: "Mobile Tap-to-Pay & Omnichannel POS Integration"
issue_description: |
  ## Problem Statement
  Small business owners and operators (like Priya the boutique owner and Fatima the food cart operator) need to handle in-person payments effortlessly while maintaining a unified view of inventory and revenue. Existing solutions either require buying expensive, dedicated point-of-sale (POS) hardware (like Square registers) or involve technical setup that splits their online and offline data, creating operational headaches and manual reconciliation. They need a simple, phone-based tap-to-pay capability integrated into their primary work assistant, enabling seamless omnichannel commerce from a single mobile device.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **Square:** The incumbent for in-person payments. Square is hardware-first but offers a Tap to Pay app. However, integrating Square's offline inventory with online storefronts built on other platforms (e.g., Shopify, Wix) is notoriously complex and often requires paid third-party syncing tools.
  - **Shopify POS:** Offers a robust omnichannel solution but pushes users toward purchasing physical hardware. The Tap to Pay on iPhone/Android feature exists but is buried within the broader Shopify ecosystem, which remains complex for non-technical users to set up from scratch.
  - **Stripe Terminal:** Provides the underlying infrastructure for Tap to Pay on compatible mobile devices without extra hardware. It is developer-focused, requiring significant engineering to build a user-facing POS application.

  ### The OHC Differentiator
  OHC will leverage Stripe Terminal to deliver a zero-hardware, Tap to Pay experience directly within the OHC mobile app (Flutter). This feature will be entirely invisible from a setup perspective. When a customer is ready to pay in person, the owner taps "Charge", hands their phone to the customer (or taps the customer's card), and the transaction instantly syncs with the unified OHC ledger and inventory system.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Mobile App - Flutter] -->|Tap to Pay Intent| B(Stripe Terminal SDK)
      B --> C{NFC Hardware on Phone}
      C -->|Card Read| B
      B -->|Process Payment| D[Stripe API]
      D -->|Payment Success Webhook| E[OHC Backend API]
      E --> F[Unified Ledger DB]
      E --> G[Inventory DB]
      E --> H[AI Operations Agent]
      H -->|Generate Receipt & Update UI| A
  ```

  ### Mobile UX Flow (375px First)
  1. **New Sale Screen:** Owner selects items from the visual catalog or enters a custom amount.
  2. **Charge Button:** A prominent "Charge $X.XX" button appears at the bottom.
  3. **Payment Method Modal:** Owner selects "Tap to Pay" (default if supported hardware is detected).
  4. **NFC Prompts:** The native iOS/Android Tap to Pay interface slides up. The screen displays clear instructions: "Hold card or device near the top of phone."
  5. **Success & Receipt:** Upon approval, a success checkmark appears, followed by a prompt to text or email the receipt to the customer, capturing their contact info for the CRM.

  ### AI Agent Integration
  - **The Finance Agent:** Automatically reconciles the tap-to-pay transaction with the daily payout report.
  - **The Operations Agent:** Instantly deducts the sold item from the centralized inventory and flags if stock is low.
  - **The Customer Assistant:** If a receipt is emailed/texted, the agent links the transaction to an existing customer profile or creates a new one, triggering a "Thank You" or review request follow-up after 24 hours.

  ## Implementation Prompt
  Implement the Mobile Tap-to-Pay feature within the OHC Flutter app using the Stripe Terminal SDK. The flow must allow the user to select an item from inventory or enter a custom amount, initiate a Tap to Pay session using the native mobile hardware (iOS/Android), and complete the transaction. Upon success, the backend must be updated to reflect the sale in the unified ledger and decrement inventory accordingly. The UI must be optimized for 375px width, mimicking a premium, simple POS experience. Do not require the user to configure Stripe keys; assume the backend provides the necessary connection tokens. Acceptance criteria include a successful end-to-end mock transaction on a device simulator.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
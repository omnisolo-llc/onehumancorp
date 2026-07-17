issue_title: "Integrate Shippo for Automated Shipping & Label Generation"
issue_description: |
  ### Title
  Integrate Shippo for Automated Shipping & Label Generation

  ### Problem Statement
  Owners like Priya (Boutique Operator) and Maya (Home Baker) sell physical goods directly to customers, but once an order is paid in OHC, fulfillment becomes a disjointed, manual nightmare. They have to copy-paste customer addresses from OHC into a separate shipping portal (like USPS, FedEx, or a standalone app), guess the package dimensions, pay for the label, copy the tracking number back into an email, and manually notify the customer. This breaks the "OneHumanCorp Promise" of keeping all work within one assistant. The owner needs a way to instantly see the cheapest shipping option, print a label directly from OHC, and have the assistant automatically update the customer with tracking details.

  ### Research Report
  **Market Context & Competitors**: E-commerce platforms like Shopify (Shopify Shipping) and Square (Square Online) deeply integrate shipping label generation. Platforms lacking this push users to third-party apps like ShipStation, which are overly complex for small-scale operators.

  **Selected Tool: Shippo**:
  - **Why Shippo?** Shippo abstracts away the complexities of multiple carriers (USPS, UPS, FedEx, DHL) into a single, clean API.
  - **Ease of Use for Owners**: Owners don't need to negotiate carrier rates; Shippo provides discounted rates out-of-the-box (e.g., USPS Commercial Pricing). Non-technical owners just see "USPS Priority - $7.50" inside OHC and click "Buy Label."
  - **Pricing**: Shippo has a "Pay As You Go" tier with no monthly fee ($0.05 per label + postage), making it perfect for low-volume sellers like Maya and scalable for Priya.
  - **Reputation & Reliability**: Shippo is an industry standard for embedded shipping, with robust webhooks for tracking updates and high uptime. It supports both multi-tenant SaaS environments (via OAuth/Connect) and standalone setups via simple API keys.

  ### Design Doc
  - **Trigger**: When an order containing physical goods is marked as "Paid" or "Ready to Fulfill" in the OHC Work Triage feed.
  - **Assistant Action**: The Operations Assistant automatically drafts a shipping fulfillment task for the owner. It pre-fills the package weight/dimensions (if known) and customer address.
  - **User Experience**: The owner taps the task, sees the recommended shipping carrier and rate (e.g., "USPS Ground Advantage - $4.99"), and taps "Purchase Label." The label is displayed as a PDF for easy mobile or desktop printing.
  - **Post-Action**: Once purchased, the Customer Assistant automatically drafts a WhatsApp or Email message to the customer with the tracking link. Shippo's tracking webhooks feed status updates back into OHC, allowing the Operations Assistant to flag delayed shipments.

  ### Implementation Prompt
  **User-Facing Outcome**:
  - The owner can purchase and print shipping labels directly from a customer's order screen without leaving OHC.
  - Tracking numbers are automatically attached to the order and communicated to the customer.
  - The owner can view package tracking statuses (e.g., "In Transit", "Delivered") inside their daily feed.

  **Acceptance Criteria**:
  1. Add a "Fulfill Order" flow to physical product orders that allows the owner to select a shipping rate and buy a label.
  2. Implement an integration with Shippo to fetch rates using the customer's shipping address and owner's origin address.
  3. Generate and display the printable PDF shipping label.
  4. Automatically trigger a customer notification with tracking details when a label is created.
  5. Listen to tracking updates via webhook to update the order status in OHC.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
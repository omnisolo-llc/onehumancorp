issue_title: "Integrate Shippo API for Multi-Carrier Shipping & Label Generation"
issue_description: |
  ## Problem Statement
  Priya (Boutique Operator) and Maya (Home Baker) struggle with manual fulfillment. Currently, creating shipping labels, comparing carrier rates (USPS, UPS, FedEx), and providing tracking numbers requires leaving the OHC platform, entering dimensions manually, and copy-pasting tracking URLs. This friction slows down operations and introduces errors.

  ## Research Report
  - **Tool Evaluated:** Shippo (API & Web App)
  - **Market Position:** Competes with EasyPost and ShipStation. Shippo stands out for its SMB-friendly onboarding and deep carrier integrations without massive upfront fees.
  - **Relevance:** It solves a core "Operations Assistant" problem by bringing fulfillment directly into the work feed.
  - **Pricing:** The API Starter tier is "Pay as you go" with no setup fees. The first 30 labels per month are free, then it’s 7¢ per label. Address validation is 2¢ (US) / 8¢ (Non-US). This variable-cost structure is perfect for SMBs like Maya or Priya.
  - **Usability for Owner:** The owner never has to see "API endpoints." They see a "Create Label" button on a confirmed order in their OHC unified inbox, pre-filled with the customer's address from their previous conversation.
  - **Capabilities:** Rating (rate comparison), Label Generation (PDF/PNG), Tracking webhooks, Address Validation, and Returns. It supports over 40 carriers out of the box (USPS, UPS, FedEx, DHL, etc.).

  ## Design Doc
  - **Trigger:** When a customer completes a payment (via the conversational checkout deposit engine) or the owner creates an order from a chat thread, the "Operations Assistant" detects a fulfillment need.
  - **Actions:**
    1. The Assistant presents a "Draft Shipment" card in the owner's feed.
    2. The card shows the customer's shipping address (auto-validated via Shippo).
    3. The owner inputs package weight/dimensions (or selects a pre-saved box size).
    4. OHC fetches rates via Shippo and displays the top 2-3 cheapest options.
    5. The owner selects a rate and taps "Purchase Label."
    6. OHC securely purchases the label via Shippo, deducts/charges the 7¢ fee (if applicable) + shipping cost, and saves the tracking number to the order record.
  - **User Feedback:** The purchased label PDF is displayed for immediate printing. The Customer Assistant drafts an automatic tracking notification (SMS/Email/WhatsApp) for the owner to 1-tap approve.

  ## Implementation Prompt
  Implement a Shippo API integration within the Operations Assistant domain.
  1. Add a secure way for tenants to link their own Shippo account or utilize a platform-level billing mechanism.
  2. Create a user-facing widget on order details to input package dimensions, fetch live carrier rates, and display them in a clean list.
  3. Allow the owner to purchase a label with a single tap, returning a printable PDF link.
  4. Integrate tracking webhooks so the order status updates automatically when the package is shipped, in transit, and delivered.
  5. Ensure the UI clearly handles address validation errors with easy correction flows.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Scout: Integrate Shippo for Multi-Carrier Shipping & Tracking"
issue_description: |
  ### Title: Integrate Shippo for Automated Multi-Carrier Shipping & Tracking

  **Problem Statement:**
  Small business owners like Maya (Home Baker) and Priya (Boutique Operator) frequently deal with physical product deliveries and shipments. Currently, calculating accurate shipping rates at the time of order and manually generating shipping labels across different carriers (USPS, UPS, DHL, FedEx) is a tedious, error-prone, and time-consuming process. Owners are copying and pasting addresses from order details into carrier websites, paying expensive retail rates, and then manually messaging tracking numbers back to customers. This breaks the seamless "work assistant" flow, causes operational delays, and occasionally leads to lost tracking numbers, which negatively impacts the customer relationship.

  **Research Report:**
  - **Market Discovery**: In competitive ecosystems like the Shopify App Store, Wix App Market, and Square App Marketplace, multi-carrier shipping tools are consistently among the highest-rated and most-installed apps. Shippo stands out due to its developer-friendly API and deep pre-negotiated discounts on USPS and UPS for small volume shippers.
  - **Competitor Analysis**: While platforms like Shopify have built-in shipping networks, a standalone assistant like OHC needs a robust, plug-and-play API to provide parity. Shippo is widely used and battle-tested compared to EasyPost or ShipEngine, specifically excelling in SMB-friendly onboarding and clear pricing.
  - **Usability for Non-Technical Owners**: Shippo abstracts away the complexities of carrier negotiations and API idiosyncrasies. For an owner, they just see "Create Shipping Label" and get the best rate automatically. They do not need to understand shipping zones, dimensional weight APIs, or complex carrier protocols.
  - **Pricing & Viability**: Shippo offers a Pay-As-You-Go tier (just $0.05 per label plus postage, or waived if using Shippo's default carrier accounts). This is highly viable for OHC's multi-tenant cloud environment where owners can either bring their own carrier accounts or use a seamless default. The API is robust, supports webhooks for tracking updates, and operates smoothly in cloud and standalone modes.
  - **Reputation**: Shippo is highly trusted in the SMB space with extensive uptime guarantees, excellent developer documentation, and reliable webhook delivery.

  **Design Doc:**
  - **Triggers**: When an order is confirmed or marked as "Ready to Ship" in the OHC daily feed, the Operations Assistant proactively generates a task card suggesting the creation of a shipping label.
  - **Actions**: The system quietly queries Shippo's API using the customer's delivery address and standard package dimensions to retrieve live rates across available carriers. The owner views a simplified list of rates (e.g., "Standard", "Express") and purchases the label directly via the OHC UI. Shippo returns a printable label (PDF/PNG) and a tracking number. Webhooks listen for transit updates.
  - **User Interface**: Integrated into the OHC feed as a clean, actionable card for "Pending Shipments." Tapping it shows available shipping rates. One tap purchases the label and opens the native mobile or desktop print dialogue. The Customer Assistant then automatically drafts a tracking update message for the owner to approve and send to the customer.

  **Implementation Prompt:**
  - **User-Facing Outcome**: Owners can click "Buy Label" directly from an approved order in their OHC assistant feed. The assistant instantly presents the cheapest and fastest shipping options. After selection, the label is ready to print right from their phone or computer. The AI assistant seamlessly drafts a friendly notification to the customer containing their live tracking link, waiting only for a single tap to send.
  - **Acceptance Criteria**:
    1. A simplified "Shipping" integration option is available in workspace settings.
    2. The OHC assistant feed displays an actionable "Generate Shipping Label" card for paid, unfulfilled physical orders.
    3. The flow presents at least two clear rate options (e.g., Cheapest, Fastest) without exposing complex carrier jargon.
    4. Purchasing a label successfully displays a printable PDF/image directly inside the OHC UI on mobile (375px) and desktop.
    5. The Customer Assistant automatically drafts a customer notification with the tracking URL upon label purchase.
    6. Tracking status changes (via webhook) prompt the AI to summarize transit anomalies to the owner.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

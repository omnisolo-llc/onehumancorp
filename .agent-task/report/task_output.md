issue_title: "Integrate Shippo API for Automated Fulfillment & Label Generation"
issue_description: |
  # Title
  Integrate Shippo API for Automated Fulfillment & Label Generation

  # Problem Statement
  Small business owners and operators like Priya (Boutique Operator) and Maya (Home Baker) struggle with the logistics of physical goods. Currently, fulfilling orders requires manually entering shipping addresses into carrier portals, comparing rates across USPS, UPS, and FedEx, and generating labels one by one. This manual process is error-prone, time-consuming, and takes away from focusing on the core business. A unified fulfillment solution inside OHC is needed to automate shipping label generation, provide real-time rate comparisons, and track shipments directly from the owner's dashboard without needing to log into external shipping sites.

  # Research Report
  - **Tool Evaluated**: Shippo API
  - **Market Position**: Shippo is a leading multi-carrier shipping API that simplifies logistics for e-commerce and SMBs. It abstracts away the complexity of integrating with individual carriers (like USPS, FedEx, UPS, DHL, etc.) into a single RESTful API.
  - **User-First Value Mapping**: For a non-technical owner, Shippo means "one-click shipping." When an order comes in via OHC's storefront, the system can instantly suggest the cheapest shipping rate, generate a printable PDF label, and automatically send tracking information to the customer via OHC's unified omnichannel inbox.
  - **Capabilities & Limits**:
    - Real-time rate generation across 85+ global carriers.
    - Automated shipping label generation (PDF/ZPL).
    - Address validation to prevent failed deliveries.
    - Webhooks for real-time tracking updates.
    - OAuth support for multi-tenant environments (essential for OHC Cloud) and API token support for Standalone users.
  - **SaaS & Open-Source Viability**: Shippo offers a pay-as-you-go pricing model with no monthly fees for basic accounts, which is highly attractive for our SMB personas. It supports both Cloud SaaS (via OAuth/Platform accounts) and Standalone models (where users plug in their own API keys).

  # Design Doc
  - **Integration Strategy**: Introduce a new Rust-based integration module `src/server/integrations/shippo` within the OHC backend.
  - **Triggers**: When an order status in OHC changes to "Ready to Ship," the HR & Logistics Agent invokes the Shippo integration to fetch rates. Once the owner confirms, the label is generated.
  - **Owner View**: The owner sees a "Fulfill Order" button on the order details page. Clicking it shows rate options. After selection, a printable label appears, and the order is marked "Shipped."
  - **Data Flow**:
    1. Order data (sender/recipient address, package weight/dimensions) is sent to Shippo.
    2. Shippo returns rate options.
    3. User selects a rate -> OHC requests a label from Shippo.
    4. Shippo returns the label URL and tracking number.
    5. OHC stores the tracking number and label URL in the Postgres/SQLite database and triggers a notification to the customer.

  # Implementation Prompt
  Implement the Shippo integration to handle automated shipping and fulfillment. Add a new Rust module that provides HTTP clients for Shippo's `/rates`, `/transactions`, and `/tracking` endpoints. The system should allow users to securely store their Shippo API credentials (using the existing SPIFFE/SPIRE identity management or tenant secrets). Create the backend logic to automatically suggest the best shipping rate based on order dimensions and generate a shipping label upon confirmation. Ensure the implementation handles rate-limit backoffs and webhook tracking events. Do not worry about the exact database schema or UI placement; focus on making the integration robust, secure, and ready to be connected to the AI HR & Logistics Agent.

  # Priority
  P1

  # Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

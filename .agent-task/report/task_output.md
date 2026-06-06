issue_title: "[Integration] Localized Shipping & Fulfillment"
issue_description: |
  # Research Report: Localized Shipping & Fulfillment Integration

  ## Problem Statement
  Small business owners often struggle with the complexity of shipping and fulfillment. A non-technical user needs a simple way to calculate shipping costs, generate labels, and track packages without needing to understand the intricacies of different carriers, zones, and pricing tiers.

  ## Research Report
  - **Tool Category**: Shipping & Fulfillment (e.g., Shippo, ShipStation, EasyPost).
  - **Target User**: Small business owners (e.g., Maya the Home Baker, Priya the Boutique Owner).
  - **Key Features**:
    - Automatic shipping cost calculation based on weight, dimensions, and destination.
    - One-click shipping label generation.
    - Real-time package tracking and status updates.
    - Integration with multiple carriers (USPS, UPS, FedEx, DHL) for best rates.
    - Simple pricing model (e.g., pay-as-you-go or low monthly fee).
  - **Competitive Analysis**:
    - Shopify offers built-in shipping but it can be complex for beginners.
    - Wix and Squarespace rely heavily on third-party apps for robust shipping features.
    - OHC needs a seamless, invisible shipping solution that requires minimal setup.

  ## Design Doc
  - **Trigger**: When an order is placed, the Operations AI Agent automatically triggers the shipping calculation.
  - **Action**: The integration communicates with the shipping provider API to get rates, generate a label, and update the order status.
  - **User Interface**: The user sees a simple "Generate Label" button on the order details page. Tracking info is automatically added to the order.

  ## Implementation Prompt
  - Integrate a shipping provider API (e.g., Shippo) to handle rate calculation and label generation.
  - Create a user-friendly UI for generating labels and tracking packages.
  - Ensure the integration works seamlessly across desktop and mobile devices.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

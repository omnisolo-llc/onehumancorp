issue_title: Implement Shippo Multi-Carrier Logistics Integration
issue_description: |
  # Title
  Implement Shippo Multi-Carrier Logistics Integration

  # Problem Statement
  Owners like Priya (Boutique Operator) and Maya (Home Baker) sell physical products that need to be shipped to customers. Managing multiple carrier accounts (USPS, FedEx, UPS), comparing rates, generating shipping labels, and tracking packages are extremely tedious and manual processes. They need a single pane of glass to instantly view the cheapest shipping option, print labels directly from their OmniSolo workspace, and automatically update customers with tracking links without logging into separate portals.

  # Research Report
  - **Tool Evaluated**: Shippo (Multi-carrier Shipping API & Dashboard)
  - **Relevance**: Directly supports the "Logistics, Fulfillment & Shipping" core capability pillar.
  - **Market Position**: Shippo is an industry leader for small-to-medium e-commerce businesses, frequently used as the backend for platforms like Shopify and Wix.
  - **Ease of Use for Owners**: Owners do not need to negotiate individual carrier rates; Shippo provides discounted USPS/UPS rates out of the box. They simply enter package dimensions and weight.
  - **Pricing**: Favorable for small businesses (Pay-as-you-go model with no monthly fees for basic label printing).
  - **Technical Viability**: High. Shippo provides a robust, developer-friendly REST API for rating, shipping, and tracking.
  - **Standalone/Cloud Viability**: Can operate fully in the Cloud for automated fulfillment, and Standalone (local) by communicating via the API.

  # Design Doc
  - **Integration Point**: Extend the `HR & Logistics Agent` and the Order Management visual workflows.
  - **Triggers**:
    - When a physical goods order is placed and marked "ready for fulfillment".
    - When an owner requests a shipping quote during an interactive proposal generation.
  - **Actions**:
    - Request shipping rates across enabled carriers via Shippo API.
    - Purchase and generate a PDF shipping label.
    - Register a tracking webhook to monitor package transit status.
  - **User Experience**: The owner sees a simplified "Ship Order" button on the order details screen. They can select a pre-defined box size, compare 2-3 cheapest rates, and click "Print Label". Tracking details are automatically injected into the unified CRM and customer notification sequence.

  # Implementation Prompt
  Implement the Shippo integration to allow owners to generate multi-carrier shipping labels directly from order screens.
  - The feature must allow the user to input package weight/dimensions and retrieve rates.
  - The feature must allow the user to purchase a label and retrieve the PDF URL for printing.
  - The system must capture the tracking number and associate it with the order.
  - Expose this capability to the `HR & Logistics Agent` so it can draft labels autonomously for review.
  - Use appropriate Rust HTTP clients and ensure the UI uses standard OmniSolo translucent glass elements for the "Ship Order" modal.

  # Priority
  P1

  # Estimated Scope
  Medium
issue_priority: P1
issue_category: Research
issue_type: Feature
issue_label: [research, integration, logistics]
assignees: [scout]

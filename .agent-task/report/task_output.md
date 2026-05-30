issue_title: "Scout: Tool Integration Research - Shippo"
issue_description: |
  # Shippo Integration Research

  ## Problem Statement
  Small business owners who sell physical products (like Maya the Home Baker or Priya the Boutique Owner) need an easy, seamless way to calculate shipping costs, print labels, and track packages. Leaving the app to manage shipments manually in another tool is too complex and breaks the "everything in one place" promise of OHC.

  ## Research Report
  Shippo is a leading multi-carrier shipping platform. It offers an excellent API that connects to USPS, FedEx, UPS, DHL, and local carriers worldwide.
  - **Usability for Non-Tech Owners**: Highly intuitive. The owner doesn't need carrier accounts for most standard shipping; Shippo provides discounted rates out of the box.
  - **Pricing**: They offer a Pay-As-You-Go plan ($0.05 per label) which is perfect for new or low-volume sellers, making it ideal for the OHC free-tier users.
  - **Cloud vs Standalone**: The API is RESTful and uses standard OAuth/API keys, working perfectly in a multi-tenant cloud environment or a local standalone setup.

  ## Design Doc
  The integration fits within the **Operations Department** ("The Manager" AI Agent).
  - **Setup**: The business owner clicks "Enable Shipping" in their OHC settings, linking a Shippo account (or creating one).
  - **Checkout Flow**: The API fetches live rates at checkout based on cart weight and destination.
  - **Fulfillment Flow**: When an order is ready, "The Manager" presents the shipping options in the OHC dashboard. The owner selects a carrier, purchases the label, and prints it. The system automatically sends tracking details to the customer.

  ## Implementation Prompt
  1. Add a Shippo API client to the Rust backend (`src/server/integrations/shippo`).
  2. Create database tables/columns to store Shippo credentials per tenant.
  3. Implement the flow to fetch rates based on order details.
  4. Implement the flow to purchase a label and store the tracking URL.
  5. Add basic UI components in the Tauri dashboard for the owner to view rates and click "Print Label".
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

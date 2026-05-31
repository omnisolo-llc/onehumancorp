issue_title: "[Architecture] Autonomous Multi-Platform Product Syndication Engine"
issue_description: |
  # Research Report: Autonomous Multi-Platform Product Syndication Engine

  ## 1. Executive Summary
  Small business owners such as Priya (Boutique owner) need to meet customers where they are—on platforms like Instagram, TikTok, and Google Shopping. However, managing inventory across multiple sales channels manually leads to overselling, fragmented data, and immense frustration. Current solutions (e.g., Shopify's sales channels) are often complex to set up, requiring manual mapping of product categories and constant monitoring. OneHumanCorp (OHC) needs an invisible syndication engine managed by the AI Marketing and Operations Departments to seamlessly push catalog updates and pull orders across all platforms in real-time.

  ## 2. Findings & Market Gap
  - **The Pain Point**: Manually updating TikTok Shop and Instagram when an item sells out in-store or on the main website is a primary cause of stockouts and canceled orders.
  - **Competitor Landscape**: Shopify offers integrations, but they are treated as separate "apps" with distinct configuration flows. Wix and Squarespace require significant manual intervention.
  - **The OHC Opportunity**: True autonomous syndication. When Priya adds a new dress to her OHC catalog, the AI Marketing Agent automatically formats the product data, optimizes the description for TikTok and Google, and syndicates it via the respective APIs. The AI Operations Agent handles real-time inventory decrementing across all channels simultaneously using the Unified Capacity & Inventory Mesh.

  ## 3. Recommended Architecture
  - **Unified Syndication API**: A microservice that acts as an adapter layer between OHC's `MASTER_CATALOG` and third-party APIs (Meta Graph API, TikTok Shop API, Google Merchant Center).
  - **Event-Driven Updates**: Listen to NATS Event Mesh for `ProductUpdated` and `InventoryChanged` events to trigger async syndication tasks.
  - **AI Data Formatting**: Utilize the AI Marketing Agent to dynamically translate OHC product taxonomy into platform-specific categories (e.g., mapping "Summer Dress" to Google's strict apparel taxonomy).

  ## 4. Next Steps
  1. Build the Syndication Engine adapter framework in Rust.
  2. Integrate Meta Graph API and Google Merchant Center API.
  3. Update the AI Marketing Agent to handle taxonomy mapping.
  4. Create the 1-Tap UI for enabling sales channels on the mobile dashboard.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, backend]
assignees: []

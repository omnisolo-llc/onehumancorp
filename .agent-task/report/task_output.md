issue_title: "Omnichannel Customer 360 & Identity Graph"
issue_description: |
  **Problem Statement:** Small business owners interact with the same customer across multiple fragmented channels (Instagram DMs, SMS, in-person tap-to-pay, online storefront). Currently, there is no unified view. If a customer asks a question on Instagram and later buys in-store, the business owner has no way to connect these interactions. They need a unified "Customer 360" profile that seamlessly tracks order history, communication, and preferences across every touchpoint, powered by a robust Identity Graph.

  **Research Report:**
  *   Current Capabilities: OHC has separate architectures for Tap-to-Pay, AI Inbox, and Storefront, but lacks a centralized Identity Graph to merge customer identities.
  *   Competitor Analysis: Shopify has unified customer profiles, but primarily focuses on e-commerce, struggling with true omnichannel messaging (like Instagram DMs). Square is strong in in-person and online customer directories, but lacks integrated AI-driven social media identity resolution. HubSpot/Salesforce are too complex, expensive, and manual for small business owners.
  *   Gap Identified: A mobile-first, zero-configuration Identity Graph that uses AI to deterministically and probabilistically merge customer identities (e.g., matching a phone number from an SMS with a Tap-to-Pay transaction and an Instagram handle).
  *   Strategic Advantage: By unifying the customer identity, OHC's AI agents can provide highly personalized service, recover abandoned carts via SMS, and offer loyalty perks across all channels, invisibly.

  **Proposed Next Steps:**
  1. Implement the Omnichannel Customer 360 Identity Graph backend services to ingest identity events from various channels (POS, SMS, Social).
  2. Resolve and merge identities into unified CUSTOMER_PROFILE entities probabilistically and deterministically.
  3. Maintain real-time aggregated metrics (like Lifetime Value).
  4. Develop the mobile-first frontend components (Customer 360 Card and Activity Feed) adhering to the macOS-style Translucent Glass aesthetic.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

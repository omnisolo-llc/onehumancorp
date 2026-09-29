issue_title: "[Integration Research] Shippo Multi-Carrier Shipping & Fulfillment Engine"
issue_description: |
  # Mission Queue Protocol: Shippo Integration Brief

  ## Title
  Implement Shippo Multi-Carrier Shipping & Fulfillment Engine for Physical Products

  ## Problem Statement
  Owners who sell physical goods (like Maya/Priya the makers) need a way to fulfill orders, generate shipping labels, and track deliveries. Currently, OmniSolo supports digital services well, but lacks a connected workflow for physical fulfillment. Manually copying addresses to a separate shipping portal, comparing carrier rates, and pasting tracking numbers back to customers is error-prone, wastes time, and breaks the "run my business autonomously" promise.

  ## Research Report
  - **Tool Evaluated**: [Shippo](https://goshippo.com/)
  - **Value Prop**: Single API for 85+ global carriers (USPS, UPS, FedEx, DHL, etc.) with pre-negotiated discounted rates.
  - **Ease of Use for Owners**: Owners just need to connect their Shippo account (or use an OHC-managed default). They don't need to negotiate individual carrier contracts. The AI team can automatically find the cheapest rate for the promised delivery window and buy the label.
  - **Pricing**: Free tier available (pay only for labels + a few cents per label), making it highly viable for SMBs. Transparent API pricing.
  - **Viability**: Shippo is a mature SaaS with robust Webhooks and OAuth. It fits the "Assisted handoffs" and "Fulfill the promise" business capabilities perfectly.

  ## Design Doc
  - **Trigger**: When an order for a physical product is fully paid and enters the "ready for fulfillment" state.
  - **Actions**:
    1. The Delivery Specialist agent retrieves the customer's shipping address and order weight/dimensions.
    2. The agent calls the Shippo integration to fetch available shipping rates.
    3. The agent selects the optimal rate based on the owner's standing policy (e.g., "cheapest rate under 5 days").
    4. The agent purchases the label via Shippo.
    5. The integration receives the tracking number and label URL.
  - **User Experience**: The owner sees a completed task in the feed: "Order #123 fulfilled. Label generated via USPS. Tracking sent to customer." The owner can click to print the label directly from the OHC interface. If the address is invalid, the agent leaves an unresolved exception for the owner to fix.

  ## Implementation Prompt
  Implement the Shippo shipping integration. Create a new fulfillment pathway for physical orders that connects to Shippo's API. The integration must securely store Shippo API credentials (using OMNISOLO_CONNECTION_KEYS), map OHC order addresses to Shippo shipment requests, and handle webhooks for tracking updates. The owner should be able to see the generated shipping label and tracking number on the order details page. Ensure that failure to generate a label (e.g., due to a bad address) results in a clear exception in the owner's feed, not a silent failure.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

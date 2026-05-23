issue_title: "AI-Powered Conversational Quoting & Custom Deposit Engine"
issue_description: |
  # Research Report
  *   **Competitor Analysis**:
      *   *Shopify*: Offers invoicing and drafting orders, but requires manual intervention to adjust line items and request deposits. No native conversational AI quoting in DMs.
      *   *Wix/Squarespace*: Provides static forms for quotes, but lacks real-time conversational negotiation and instant dynamic deposit collection.
      *   *GoDaddy*: Basic appointment booking and quoting, heavily reliant on the merchant manually reading and responding to requests.
  *   **Industry Trends**: Consumers increasingly expect instant, conversational commerce via Instagram, WhatsApp, and SMS. Drop-off rates spike if a merchant takes more than 15 minutes to reply to a DM quote request.
  *   **The Opportunity**: OneHumanCorp can introduce a Zero-Touch Conversational Quoting Engine that securely integrates with our multi-tenant Ledgers and Identity structures, providing an invisible AI layer that handles the entire negotiation and deposit lifecycle.

  # Findings
  Small business owners who provide custom products or services—such as Maya (baker) and Carlos (handyman)—rely heavily on social media DMs and text messages to receive custom requests. Currently, they spend hours each day in a tedious back-and-forth negotiating scope, delivery dates, and pricing. Once a price is agreed upon, they manually generate and send payment links (e.g., Venmo, Square, Stripe) to secure a deposit. This manual process causes delayed responses, lost sales, and significant administrative burden, pulling them away from doing the actual work.

  # Proposed Next Steps
  1. Build the backend state machine that tracks the quote negotiation and deposit lifecycle securely within the multi-tenant architecture.
  2. Develop the AI coordination logic allowing the CS Agent to extract intent and the Finance Agent to generate a quote and a secure payment link.
  3. Construct the mobile-first (375px) UI cards in the OHC unified inbox, allowing the business owner to review, approve, or override AI-generated quotes with zero friction.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "Native Integration with Stripe for Payment Link Generation"
issue_description: |
  # Native Integration with Stripe for Payment Link Generation

  ## Problem Statement
  Small business owners like Priya (Boutique) and Maya (Baker) currently lack a native, automated way to generate and share payment links for their customers within OmniSolo. When closing a sale via Omni-channel Quote-to-Cash, they must manually create a payment link in their Stripe dashboard and paste it back into their chats, which causes friction, errors, and loss of momentum in the sales process.

  ## Research Report
  - **Strategy**: Direct integration with Stripe's Payment Links API.
  - **Target Persona**: Priya (Boutique Owner), Maya (Home Baker).
  - **Advantages**: Streamlines the checkout process natively. Eliminates context-switching for business owners, directly improving conversion speed.
  - **Pricing**: API access is free, but standard Stripe payment processing fees apply.
  - **Ease of Use**: A direct native connection will allow non-technical operators to connect their Stripe accounts and seamlessly send payment links via their 375px mobile dashboard or Chat Interfaces.

  ## Design Doc
  - User goes to Settings -> Connectors and connects their Stripe Account via OAuth.
  - When an invoice is finalized or a customer is ready to checkout in an Omni-channel chat, the agent triggers the generation of a Stripe Payment Link.
  - The generated link is embedded as a 1-tap "Pay with Stripe" button in the customer chat.
  - OmniSolo handles the webhook when payment succeeds and updates the `INVENTORY_LEDGER` or triggers a Fulfillment workflow.
  - **AI Integration**: The `Accountant Agent (Finance)` requests Stripe to generate payment links and updates revenue stats natively on the dashboard.

  ## Implementation Prompt
  Implement a native backend Stripe Payment Links integration. The system should allow the generation of secure session-scoped Stripe Payment Links on demand. These links should be integrated into the Omni-channel conversational interface so users can pay immediately via chat. Ensure a webhook handler is configured to capture `checkout.session.completed` events from Stripe to update the internal order status automatically.

  - **Acceptance Criteria**: Merchants can connect Stripe natively via OAuth. System can generate a valid Stripe payment link for a pending invoice. Webhooks successfully mark the OmniSolo order as 'Paid' when the Stripe payment is completed.
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

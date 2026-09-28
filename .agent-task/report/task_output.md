issue_title: "Add Stripe Issuing connector for autonomous OHC payments"
issue_description: |
  **Title:** Add Stripe Issuing connector for autonomous OHC payments

  **Problem Statement:**
  Currently, owners handle expenses using personal cards or disconnected systems. When the AI team coordinates a physical task (e.g. buying a domain, booking a flight, purchasing supplies for a job), the AI cannot autonomously pay for the required external services without the owner manually intervening to input their credit card, which defeats the purpose of autonomous coordination and adds significant friction. The owner needs a safe, bounded way for the OHC team to handle its own small payments on their behalf.

  **Research Report:**
  We investigated **Stripe Issuing** as a candidate for this integration gap. Stripe Issuing allows platforms to create, manage, and distribute virtual (and physical) cards.
  - **Ease of Use:** Owners who already have a Stripe account can seamlessly opt into Issuing (if eligible/in a supported region like the US or UK). For the end-user (owner), they set a monthly/project budget limit in the OHC UI, and OHC provisions a virtual card under the hood. The owner doesn't deal with the technical API.
  - **Capabilities & Limits:** The Stripe Issuing API allows creating virtual cards, setting strict spending limits per card (e.g., $50 max, specific merchant categories), and listening to webhooks for authorization requests (`issuing_authorization.request`) which can be approved or declined synchronously by the OHC backend based on business rules.
  - **SaaS Viability:** Stripe is already heavily integrated into OHC for incoming payments (Checkout, etc.). Reusing the Stripe account for outgoing payments reduces vendor sprawl. Pricing for virtual cards is typically $0.10 per virtual card (first 500k free in US) + standard interchange fees (which Stripe keeps, though platforms can sometimes share in interchange). It works well in a multi-tenant cloud setup if connected via Stripe Connect.
  - **Competitors:** Marqeta (too enterprise/heavy for small business platforms), Adyen (similar complexity). Stripe Issuing is the most developer-friendly and synergizes with our existing Stripe connection.

  **Design Doc:**
  - **Trigger:** When an owner creates an automated budget in OHC (e.g., "Allow my marketing agent to spend up to $100/mo on ads"), the OHC backend provisions a virtual Stripe Issuing card via the Stripe API.
  - **Action:** The virtual card details (PAN, CVV, expiry) are stored securely and exposed only to the specific OHC autonomous agent executing the task (e.g., via a secure credential vault or MCP server). The agent uses these details at checkout.
  - **Verification:** Stripe sends a synchronous webhook (`issuing_authorization.request`) to OHC when the card is charged. OHC verifies the charge against the agent's intent, the owner's budget, and the merchant, and responds `true` to approve or `false` to decline within Stripe's time limits.
  - **User Experience:** The owner sees a dashboard of "Agent Spend" with itemized receipts and a hard cap, knowing the agent physically cannot overspend because the card itself has a Stripe-enforced limit.

  **Implementation Prompt:**
  Implement a new `StripeIssuing` provider within the existing Stripe integration. It must support creating a virtual card with a fixed spend limit (`stripe.issuing.cards.create`) and updating limits. Add a secure webhook handler for `issuing_authorization.request` that automatically approves or declines transactions based on a stored budget policy. Ensure strict tenant isolation so one owner's funds are never used for another. Expose a safe, read-only interface of the card details to the authorized worker context.

  **Priority:** P2
  **Estimated Scope:** Medium

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

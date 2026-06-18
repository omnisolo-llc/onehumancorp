issue_title: "[research] Build Invisible Agentic Quote & Proposal Engine"
issue_description: |
  # Research Report: Invisible Agentic Quote & Proposal Engine

  ## Problem Statement
  For service-based owners like Carlos (Handyman) and Nora (Agency Principal), the intake-to-quote process is fragmented and slow. Customers inquire via DMs or messy web forms. The owner must manually parse the request, remember their pricing logic, draft a professional proposal document (often in Word or a disconnected SaaS tool), generate a payment link, and follow up. This delays revenue and loses leads to faster competitors. Traditional platforms (Shopify, Wix) treat services like basic products, lacking the capability to autonomously negotiate, quote, and secure custom jobs.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify / Wix / Squarespace:** E-commerce native; treating a custom kitchen remodel or design project like a "product" with variants is incredibly clunky. Native quoting is non-existent without expensive plugins.
  - **HoneyBook / Dubsado:** Excellent CRM and proposal tools, but they are separate platforms requiring manual setup, disconnected from the primary storefront and daily operational feed. They lack proactive AI agents that draft the quote instantly based on a single SMS from a client.
  - **Square Invoices:** Simple, but reactive. The owner still has to do all the typing and math.
  - **OHC Opportunity:** Implement an "Invisible Quote Engine" driven by the Sales & Revenue Assistant. When a lead comes in via any channel (Work Triage), the AI Assistant cross-references the owner's service catalog, past successful quotes, and standard pricing rules to instantly draft a highly professional, interactive proposal (including a deposit payment link). The owner just taps "Approve."

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: DM/Form] -->|Omnichannel Gateway| B(Work Triage Agent)
      B --> C{Sales & Revenue Assistant}
      C -->|Query| D[Service Catalog DB]
      C -->|Query| E[Past Proposals & Memory]
      C --> F[Quote Drafting Engine]
      F --> G[Generated Proposal Record]
      G --> H[Action Card: Mobile Feed 375px]
      H -->|Owner Taps Approve| I[Stripe Deposit Link Generation]
      I --> J[Omnichannel Dispatch: Send to Customer]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Feed View:** A high-priority card: "Carlos, a lead asked for a kitchen sink repair estimate. I drafted a quote for $150 based on your standard rate."
  - **Draft View:** Tapping the card opens a clean, macOS Translucent Glass preview of the proposal. It includes the customer's original message, the drafted line items, and a deposit requirement (e.g., "$50 upfront").
  - **Interaction:** A large, accessible primary button: "Approve & Send". A secondary button: "Edit Items". If editing, a native mobile keyboard allows quick numeric tweaks before sending.
  - **Customer View:** The customer receives a secure, mobile-optimized link showing a beautiful proposal with a 1-tap Apple Pay/Google Pay deposit button.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Uses RAG against the `Service` and `Knowledge` schemas to formulate accurate estimates. If a request is vague ("My sink is broken"), the Assistant drafts a clarifying question instead of a blind quote.
  - **Operations Assistant:** Automatically reserves a tentative block in the scheduling calendar once the quote is sent, converting it to a confirmed booking upon deposit.

  ### Key Design Decisions
  - **Zero Manual Data Entry:** The transition from conversation to structured proposal is entirely handled by the LLM.
  - **Integrated Deposits:** Proposals aren't just PDFs; they are interactive payment intents to secure the work immediately.
  - **Mobile-First Approval:** The owner must be able to send a $5,000 quote from their phone while parked in their truck, in under 10 seconds.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, when a client asks for a custom service via the contact form or DM, I get a push notification. I open the app to see a fully fleshed-out proposal and price estimate already drafted. I tap "Approve," and the client gets a beautiful checkout page to pay the deposit.
  **CUJ & Acceptance Criteria:**
  1. A simulated inquiry for a "custom service" enters the system.
  2. The Sales & Revenue Assistant parses the intent, queries the tenant's pricing memory, and generates a `Proposal` draft.
  3. The `Proposal` appears in the owner's mobile feed (verified in Playwright at 375px).
  4. The owner taps "Approve", which triggers the creation of a Stripe Payment Link/Session for the deposit.
  5. The system records the state change and dispatches the link to the mock customer channel.
  6. Ensure 100% unit test coverage for the Proposal generation logic and E2E Playwright verification of the approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

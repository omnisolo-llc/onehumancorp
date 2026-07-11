issue_title: "AI-Powered Autonomous Quoting & Estimate Generation"
issue_description: |
  ## Title
  AI-Powered Autonomous Quoting & Estimate Generation

  ## Problem Statement
  Service business owners like Carlos (the Handyman) or Nora (Agency Principal) spend hours each week manually drafting quotes and estimates. When a potential customer requests a quote (e.g., via a website form, WhatsApp, or email), the owner must calculate materials, labor, and margins, then format it into a professional document. This delay often leads to lost leads. Existing platforms (Shopify, Wix) are built for static e-commerce products and lack native, intelligent quoting capabilities. Traditional invoicing tools (Quickbooks, Freshbooks) require manual entry and are disconnected from the initial customer inquiry channel.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Geared heavily toward fixed-price products. Workarounds involve complex "draft order" flows that are not intuitive for service businesses. No AI agent to automatically parse customer requirements into a quote.
  - **Jobber / Housecall Pro:** Powerful service-business tools, but highly manual setup for quotes. Lack omnichannel conversational quoting (e.g., generating a quote directly from a WhatsApp DM).
  - **OHC Opportunity:** Leverage our "Operations Agent" and "Sales Agent" to parse natural language service requests, calculate estimated costs based on the owner's predefined pricing rules or past similar jobs, and generate a professional, interactive quote that the customer can approve with one tap.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: Web Form/WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Intent & Entity Extraction LLM]
      C -->|Service Type, Scope, Constraints| D{Quoting Engine}
      D -->|Fetch Pricing Rules| E[Tenant Pricing DB]
      D -->|Generate Draft| F[Sales Agent]
      F --> G[Action Required Queue]
      G --> H[Owner Mobile App Feed 375px]
      H -->|1-Tap Approve & Send| I[Omnichannel Dispatcher]
      I --> J[Customer Interactive Quote Link]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Action Required: Approve Estimate for John's Kitchen Repair".
  - **Interaction:** Tapping the card opens a split view. The top half shows John's original message ("I need my kitchen sink pipe replaced, here are some photos..."). The bottom half shows the AI-generated quote breakdown: Labor (2 hours), Materials ($50), Total ($200).
  - **Action:** A primary button "Approve & Send Quote", secondary buttons "Adjust Pricing" and "Discard".
  - **Customer View:** The customer receives a link (via SMS/WhatsApp) opening a mobile-optimized quote page with a big "Accept & Pay Deposit" button.

  ### AI Agent Integration Points
  - **Sales Agent / Estimator:** Triggered by new inquiries classified as quote requests. Uses RAG against the owner's past accepted quotes and base pricing parameters to generate the estimate.
  - **Operations Agent:** Upon quote acceptance, automatically schedules the work (if integrated with the booking system) and creates the final invoice draft.

  ### Key Design Decisions
  - **Conversational to Transactional:** Bridging the gap between an unstructured DM and a structured financial document invisibly.
  - **Owner Approval Gate:** Estimates are legally binding in many jurisdictions; the AI strictly drafts, but the human owner MUST approve before sending.
  - **Deposit Integration:** The accepted quote instantly transitions into a Stripe payment intent for the deposit.

  ## Implementation Prompt
  **User-Facing Outcome:** When Carlos receives a WhatsApp message asking "How much to fix a leaky pipe?", he gets a notification in his OHC app with a pre-calculated $150 quote draft. He taps "Send", and the customer receives a professional payment link to accept the job.

  **CUJ & Acceptance Criteria:**
  1. An external service request via the Omnichannel Gateway is classified as a "Quote Request".
  2. The Estimator Agent parses the request, looks up the tenant's base labor rate in the database, and creates a `Quote` draft record.
  3. The drafted quote appears in the owner's mobile action feed.
  4. The owner taps "Approve", changing the quote status to "sent" and generating a unique sharing link.
  5. The customer visits the sharing link, clicks "Accept", which creates a linked `Invoice` and `Stripe Payment Intent` for a 50% deposit.
  6. E2E Playwright tests must cover the owner logging in, seeing the drafted quote, approving it, and the simulated customer accepting the quote on the public link.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

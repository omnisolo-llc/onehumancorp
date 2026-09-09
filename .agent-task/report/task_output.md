issue_title: "Implement Instant Quoting & Estimations Engine for Automated Lead Capture"
issue_description: |
  # Problem Statement
  Service owners (e.g., Carlos the handyman) lose up to 30% of leads because they cannot instantly reply and provide quotes while out on a job. Existing solutions either require manual intervention (Shopify) or lack deep vertical integrations for small service businesses. Small business operators need a system that captures demand, quotes instantly, and books autonomously directly from the unified inbox without typing manual responses.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox / Wix Inbox:** Basic unified inboxes. They do not proactively draft contextual quotes or handle complex scheduling logic automatically. They rely on rigid auto-replies.
  - **11x.ai (Alice):** Demonstrates high conversion rates using AI phone/chat handlers, but it is focused on B2B.
  - **OmniSolo Opportunity:** OmniSolo must bridge the gap by enabling the Ambassador Agent to negotiate and quote directly from the omnichannel inbox. The agent should intercept incoming inquiries (e.g., WhatsApp, Instagram DMs), read the customer's intent, query the service catalog and pricing heuristics, generate an estimated quote, and present it as a 1-tap approval card to the owner.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer DM - Instagram/WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C[Unified Inbox Message]
      C --> D{The Ambassador Agent}
      D -->|Query Catalog| E[(Service Catalog)]
      D -->|Query Rules| F[(Pricing Heuristics)]
      D -->|Generate| G[Draft Quote & Line Items]
      G --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Stripe Payment Link]
      J -->|Send Quote| K[Omnichannel Dispatcher]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Customer DMs:** "Need a plumber ASAP for a leaky faucet."
  2. **Owner UI:** A prominent Translucent Glassmorphism card appears in the Assistant-first feed: "1 New Quote Drafted for Carlos (Insta DM)".
  3. **Interaction:** Tapping the card opens a unified view showing the drafted Quote with line items (e.g., "Leaky Faucet Repair", $150, Qty 1) and a pre-written message: "Hi! We can fix that leaky faucet tomorrow at 10 AM. The estimated cost is $150. Here is the link to pay the $50 deposit and confirm the booking."
  4. **Action:** Prominent primary button "Approve & Send Quote", secondary "Edit Quote".
  5. **Visual Design:** Apple/Ubiquiti-style hierarchy, restrained translucent materials (`backdrop-filter: blur(30px)`), readable typography, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **The Ambassador Agent:** Triggered by incoming messages. Uses RAG against the `service_catalog` and `pricing_heuristics` to draft highly personalized quotes (`quote_line_items`).
  - **The Manager Agent:** Coordinates with the Ambassador to verify calendar availability before drafting the response.

  ### Key Design Decisions
  - **Proactive Quoting:** The AI drafts the quote and line items *before* the user opens the app.
  - **1-Tap Approval:** The owner reviews the quote and approves it with a single tap, taking seconds instead of minutes.
  - **Zero Trust & Security:** Strict row-level security (RLS) is enforced on all quoting tables to ensure tenant isolation.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me asking for a service estimate, I open the OmniSolo app to find a pre-written, perfectly accurate quote already drafted with correct line items and pricing. I tap one button to send it, capturing the lead instantly.
  **CUJ & Acceptance Criteria:**
  1. A simulated external message is ingested by the Omnichannel Gateway.
  2. The Ambassador Agent is triggered and successfully queries the `service_catalog` and `pricing_heuristics`.
  3. The Agent generates a Draft Quote and places it in the Action Required Queue for the tenant.
  4. Provide Playwright E2E tests: A user logs in, sees the drafted quote card on the mobile-sized feed, taps "Approve", and the system dispatches the quote back to the mocked external channel.
  5. Ensure 100% unit test coverage for the new quoting logic.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

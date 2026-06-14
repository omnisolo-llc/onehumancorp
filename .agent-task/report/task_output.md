issue_title: "[Research] Architect Automated Omnichannel Review Acquisition & Reputation Management"
issue_description: |
  # Research Report: Automated Omnichannel Review Acquisition & Reputation Management

  ## Problem Statement
  Small business owners (Carlos the handyman, Fatima the food cart operator) rely heavily on word-of-mouth and public reviews (Google Business, Yelp, Trustpilot) to drive new customer acquisition. However, they lack the time and automated tools to systematically ask for reviews at the right moment (e.g., right after a successful service or a 5-star interaction). Traditional review apps (like Loox or Judge.me on Shopify) are often siloed, require manual configuration of trigger rules, and only focus on e-commerce product reviews rather than the broader business reputation across Google or Facebook.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Ecosystem (Loox, Judge.me):** Highly effective for product-specific reviews but require complex logic rules (e.g., "send 3 days after delivery"). They lack intelligent timing and do not natively push to external platforms like Google Business Profiles without additional paid plugins.
  - **Birdeye / Podium:** Specialized reputation management platforms that excel at SMS review generation and Google integration. However, they are enterprise-targeted, expensive ($200+/month), and disconnected from the SMB's core operational system (booking, payments).
  - **Wix / Squarespace:** Basic follow-up emails exist, but they are static and not AI-driven.
  - **OHC Opportunity:** Since OHC operates the entire stack (booking, payment, omnichannel inbox), the "Promoter Agent" can intelligently determine the exact right moment to ask for a review. For example, if Carlos completes a job and the customer pays the invoice promptly without complaints, the agent autonomously sends an SMS: "Thanks for choosing us! Could you take 10 seconds to leave us a Google review?"

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Order/Booking State: Completed] -->|Event| B(Reputation Engine)
      C[Invoice State: Paid] -->|Event| B
      D[Inbox State: Positive Sentiment] -->|Event| B
      B --> E{The Promoter Agent}
      E -->|Determine Timing & Channel| F[Omnichannel Dispatcher]
      F --> G[SMS/Email to Customer]
      G -->|Customer Clicks Link| H[Intelligent Routing]
      H -->|Happy| I[Google Business / Yelp Link]
      H -->|Unhappy| J[Internal Feedback Form to Owner]
      I --> K[Reputation Dashboard]
  ```

  ### Mobile UX Flow (375px First)
  - **Owner Configuration (One-Time):** A simple toggle in the Settings: "Automate Review Requests". Owner pastes their Google Business link.
  - **Customer Flow:** Customer receives a clean, native-looking SMS with a single question: "How did we do? 1-5 Stars."
    - If 4 or 5 stars: "We're thrilled! Could you share that on Google? [Link]"
    - If 1-3 stars: "We're sorry to hear that. What went wrong?" (Captures feedback internally, prevents public bad review).
  - **Owner Dashboard Feed:** "Carlos, you got a new 5-star review from Sarah! The Promoter Agent has drafted a reply."

  ### AI Agent Integration Points
  - **The Promoter Agent (Marketing):** Subscribes to operational events (`order.completed`, `booking.completed`, `invoice.paid`). Uses LLM to assess customer sentiment from recent omnichannel inbox history. If sentiment is positive or neutral, it schedules a review request via the customer's preferred channel.
  - **The Ambassador Agent (Customer Success):** Subscribes to incoming reviews. Drafts context-aware public replies (e.g., "Thanks Sarah, we loved fixing your sink!") for the owner to 1-tap approve.

  ### Key Design Decisions
  - **Event-Driven Timing:** Don't rely on dumb delays (e.g., "send after 3 days"). Trigger based on state changes (e.g., marked as delivered, paid).
  - **Sentiment Gate:** Never ask an angry customer for a public review. The agent must check the unified inbox for recent negative interactions before dispatching the request.
  - **Private Feedback Loop:** Intercept negative reviews privately to give the owner a chance to fix the issue before it hits Google.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner, I flip one switch to enable automated reviews. OHC's AI agents automatically follow up with happy customers after they pay or receive their item, routing good experiences to my Google Business page and intercepting bad experiences privately. I receive mobile notifications to approve AI-drafted replies to public reviews.
  **CUJ & Acceptance Criteria:**
  1. Create a `ReputationProfile` data model linking a tenant to external review sites (e.g., Google Place ID).
  2. Implement an event listener in the Promoter Agent that triggers when an order/booking is marked completed.
  3. The agent must query the customer's recent inbox history for negative sentiment. If negative, abort the review request.
  4. Dispatch an automated SMS/Email via the Omnichannel Gateway asking for a rating.
  5. Provide Playwright E2E tests: A test user configures their Google link, a mock order is completed, the agent dispatches the request, and the simulated customer clicks the link.

  ## Priority
  P2

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

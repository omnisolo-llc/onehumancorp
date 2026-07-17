issue_title: "Agentic Customer Reputation & Review Management"
issue_description: |
  # Research Report: Agentic Customer Reputation & Review Management

  ## 1. Problem Statement
  Small business owners and service operators (e.g., Carlos the Handyman, Maya the Baker) rely heavily on word-of-mouth and online reviews (Google Business, Yelp, Trustpilot) to drive new customer acquisition. However, they lack the time and operational bandwidth to proactively request reviews from satisfied customers after a successful transaction or service completion. Furthermore, when negative reviews occur, they often lack the PR skills or emotional detachment to draft a professional, de-escalating response quickly, leading to long-term brand damage. Existing solutions either require manual intervention or rely on disconnected third-party marketing tools that SMB owners find too complex to configure.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Wix offer basic post-purchase emails, but these are often static templates and do not adapt to the customer's specific experience or sentiment. Third-party tools like Yotpo or Podium exist but add significant overhead (both in cost and cognitive load) and are not natively integrated into the core operational workflow of the business.
  - **The OHC Opportunity**: By embedding an "Agentic Reputation Manager" directly into the OHC platform, we can automate the entire lifecycle of customer feedback. The agent can monitor completed jobs, analyze implicit sentiment (e.g., repeat purchases, lack of complaints), proactively request reviews, and draft professional responses to all public feedback. This turns reputation management from a reactive chore into a proactive, autonomous growth engine.
  - **Competitor Gaps**:
    - *Shopify/Wix*: Require complex third-party app integrations (e.g., Yotpo, Loox) for advanced review management.
    - *Podium/Birdeye*: Powerful but expensive and operate as separate tools outside the core business ledger.
    - *GoDaddy*: Offers basic review widgets but no proactive, AI-driven management.

  ## 3. Design Doc

  ### Data Model (PostgreSQL)
  - `ReviewRequest`: Tracks the status of a review request sent to a customer (linked to a `Booking`, `Order`, or `Customer`).
  - `CustomerFeedback`: Stores internal feedback or public reviews scraped/integrated from external platforms (Google Business, Facebook).
  - `ReputationAction`: Logs actions taken by the Agent (e.g., "Drafted response to 1-star review", "Sent review request for Order #123").

  ### AI Integration
  - **Customer Success Agent ("The Ambassador")**:
    - **Trigger**: Listens for `OrderCompleted` or `BookingFulfilled` events.
    - **Action**: Evaluates customer context. If positive signals exist (e.g., repeat customer, smooth transaction), it drafts and sends a personalized SMS/Email requesting a public review with a direct link.
    - **Monitoring**: Integrates with Google Business Profile API to monitor incoming reviews.
    - **Response Drafting**: Uses an LLM (Gemini) to draft professional, empathetic responses to both positive and negative reviews, surfacing them in the Owner's Agent Feed for 1-tap approval.

  ### Mobile UX Flow (375px)
  1. **Owner View (Agent Feed)**: The owner receives an Action Card: "New 4-star review from John D. Drafted response ready for approval."
  2. **Action Card Details**: The card shows the customer's review and the AI-drafted response (e.g., "Hi John, thanks for the great feedback on the plumbing repair! We're glad everything is working smoothly. - Carlos").
  3. **Interaction**: The owner can tap "Approve & Post", "Edit", or "Dismiss".
  4. **Performance Dashboard**: A simple "Reputation Score" widget on the home dashboard shows recent rating trends without complex analytics.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Agentic Customer Reputation Manager

  **Target Persona**: Carlos the Handyman

  **Outcome**: Carlos automatically requests reviews from happy clients a day after completing a repair job. When a client leaves a review on Google, OHC drafts a professional reply for Carlos to approve with one tap on his phone while he's on the road.

  **Next Actions**:
  1. Implement the Core Data Models (`ReviewRequest`, `CustomerFeedback`) with multi-tenant isolation.
  2. Develop the Customer Success Agent logic to listen for completion events and trigger personalized review request communications via Email/SMS.
  3. Integrate the Google Business Profile API for reading reviews and posting replies.
  4. Build the Mobile-First (375px) Action Card in the Agent Feed to surface drafted responses for owner approval.

  **Key Architecture Decisions**:
  - Use asynchronous background jobs (PostgreSQL `SKIP LOCKED` pattern) for delayed review requests (e.g., 24 hours post-service).
  - Ensure all AI-generated public responses require explicit owner approval via the mobile feed to maintain brand safety.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "AI Unified Review & Reputation Management System"
issue_description: |
  ## Title
  AI Unified Review & Reputation Management System

  ## Problem Statement
  Small business owners (like Carlos the handyman or Priya the boutique owner) live and die by online reviews (Google Business, Yelp, Trustpilot, on-store reviews). However, monitoring multiple platforms, thanking positive reviewers, and mitigating negative feedback requires time they do not have. Ignored negative reviews hurt SEO and conversion. Legacy tools like Yotpo or Podium are expensive and complex.

  ## Research Report
  - **Competitive Landscape**:
    - **Shopify**: Relies on third-party apps like Yotpo, Loox, or Judge.me, which add monthly fees and only handle on-store reviews. They do not naturally sync with Google Business or Yelp without costly Zapier connections.
    - **Podium / Birdeye**: Excellent for local businesses, but they are separate tools (expensive, $200+/mo) that don't deeply integrate with the core operations, catalog, and actual order history.
    - **Wix / Squarespace**: Basic on-site reviews but lack aggressive omnichannel syndication and AI-driven reputation repair.
  - **OHC Opportunity**: Native integration of reputation management where the Customer Success Agent ("The Ambassador") continuously monitors connected channels. It automatically drafts replies to all reviews (matching the brand's tone) and flags negative reviews for the owner's immediate attention via the Agent Feed.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Google Business API] -->|Webhooks/Polling| B(Omnichannel Review Gateway)
      C[Yelp API] -->|Webhooks| B
      D[On-Store Reviews] --> B
      B --> E[Unified Review Graph DB]
      E --> F[Event Mesh]
      F --> G[The Ambassador Agent]
      G -->|Classify Sentiment & Intent| H{Sentiment Analysis}
      H -->|Positive/Neutral| I[Draft Thank You Reply]
      H -->|Negative| J[Draft Mitigation Reply & Flag Urgent]
      I --> K[Action Required Queue]
      J --> K
      K --> L[Mobile App Feed 375px]
      L -->|1-Tap Approve| M[Omnichannel Dispatcher]
      M --> A/C/D
  ```

  ### Mobile UX Flow (375px First)
  1. **Agent Feed (Mobile)**: The owner sees a card: "New 5-star Google Review from Sarah. [Approve Thank You Reply]".
  2. **Negative Review Flow**: If a 2-star review comes in: "Action Required: 2-Star Review on Yelp. [Review & Mitigate]".
  3. **Mitigation Screen**: Tapping the negative review card opens a screen showing the review context, matching it to the actual customer order (e.g., "Sarah's order was delayed by 2 days"). The Agent drafts a reply offering an apology and a 10% discount on the next order.
  4. **Action**: The owner taps "Approve & Send" or "Edit".
  5. **Visual Design**: Uses OHC Premium Tokens (Glassmorphism, high contrast text). The sentiment is clearly color-coded (Green for positive, Red for negative).

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Monitors incoming reviews. Uses RAG against the tenant's order history to understand context (e.g., if a review mentions a broken item, it cross-references the latest order).
  - **The Promoter (Marketing Agent)**: Identifies 5-star reviews and automatically drafts a social media post (e.g., Instagram Story) featuring the review, placing it in the owner's feed for approval.

  ### Key Design Decisions
  - **Unified Identity**: Link third-party reviews (Google, Yelp) to the internal `Customer` record via name/email/phone matching to provide context for the AI.
  - **Zero-Touch Triage**: The AI does the heavy lifting of drafting the reply. The owner just reviews and taps a single button.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer leaves a review on Google or the OHC storefront, the owner receives a drafted, context-aware reply in their mobile feed. They can approve it with one tap, saving time and improving their online reputation.
  **CUJ & Acceptance Criteria:**
  1. A simulated external review (e.g., via a test webhook from Google Business) is ingested by the Omnichannel Review Gateway.
  2. The system correctly identifies the customer and links the review to their past order.
  3. The Ambassador Agent is triggered, analyzes the sentiment, and drafts a contextual reply.
  4. The drafted reply appears in the `ActionRequiredQueue` for the tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the review card on the mobile feed, taps "Approve," and the system dispatches the reply back to the mocked external channel.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

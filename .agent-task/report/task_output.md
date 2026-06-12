issue_title: "Implement Agentic Review & Reputation Management Architecture"
issue_description: |
  # Mission Queue Protocol: Agentic Review & Reputation Management Architecture

  ## Problem Statement
  Small business owners (like Carlos the Handyman and Priya the Boutique Operator) rely heavily on word-of-mouth and public reviews (Google Maps, Yelp, social media) for new customer acquisition. However, collecting reviews is a manual, awkward, and easily forgotten task. Existing tools like Podium or Birdeye are prohibitively expensive ($200+/month) for micro-SMEs, while platforms like Shopify require third-party apps (e.g., Yotpo, Loox) that add to the "App Tax." OHC currently lacks an integrated, agent-driven feedback loop that automatically captures customer sentiment, drives public reviews after a successful transaction, and intercepts negative experiences before they go public.

  ## Research Report (Track 1)
  - **Competitor Landscape**: Pure-play reputation management tools (Podium, Birdeye, Broadly) charge high monthly premiums for automated SMS review requests and inbox aggregation. E-commerce platforms (Shopify, Wix) rely on apps (Judge.me, Loox, Yotpo), resulting in a disjointed experience where the review system is disconnected from the core booking or POS systems.
  - **OHC Opportunity**: By leveraging the Event Mesh and existing AI agents (The Ambassador, The Promoter), OHC can provide a native, zero-configuration reputation management system. This eliminates the "App Tax" and transforms a passive platform into a proactive growth engine for the owner.
  - **The Missing Link**: Currently, when an order or service booking is marked "Complete" in OHC, the journey ends. We need an automated post-service workflow.

  ## Design Doc (Track 2 & Track 3)
  ### High-Level Architecture
  ```mermaid
  graph TD
      A[Event: Order/Booking Completed] -->|Message Bus| B(Operations Agent)
      B -->|Wait 24h| C{Sentiment & Context Filter}
      C -->|Query DB| D[Customer History]
      D --> E(The Ambassador Agent)
      E -->|Draft Personalized SMS/Email| F[Action Required Queue]
      F -->|Push Notification| G[Mobile Owner Feed 375px]
      G -->|1-Tap Approve| H[Omnichannel Dispatcher]
      H -->|Send to Customer| I[Customer Device]
      I -->|Positive Reply/Click| J[Redirect to Google/Yelp]
      I -->|Negative Reply| K[Escalate to Owner Inbox]
      J --> L(The Promoter Agent)
      L -->|Draft Instagram Post of Review| F
  ```

  ### Data Model & System Integrity
  - **`ReviewCampaign`**: Tracks the automated outreach (Customer ID, Order/Booking ID, Status: Scheduled, Drafted, Sent, Responded).
  - **`CustomerFeedback`**: Internal storage of raw feedback before external posting, allowing sentiment classification.
  - **AI Department Coordination**:
    - *Operations Agent*: Triggers the 24-hour post-completion webhook.
    - *The Ambassador (CS Agent)*: Uses RAG (Retrieval-Augmented Generation) to draft a context-aware message (e.g., "Hi Sarah, how was the vegan cake for the birthday party yesterday?").
    - *The Promoter (Marketing Agent)*: Monitors connected Google Business/Yelp APIs. When a 5-star review appears, it drafts a social media post highlighting the review and places it in the Owner's mobile feed for approval.

  ### Mobile UX Flow (375px)
  1. **Owner Feed Card**: A clean, glassmorphism card appears: "3 jobs completed yesterday. Send review requests?"
  2. **Preview State**: Tapping shows the AI-drafted, personalized SMS messages for each customer.
  3. **Interaction**: 44x44px touch targets for "Approve All" or swipe to dismiss/edit individual messages.
  4. **Escalation View**: If a customer replies with negative sentiment ("The cake was dry"), an urgent red-tinted card appears: "Action Required: Sarah was unhappy with her order. [Draft Apology / Offer Refund]".

  ### Technical & Security Constraints
  - Multi-tenant isolation: All `ReviewCampaign` records must enforce PostgreSQL Row-Level Security (`tenant_id`).
  - Opt-out handling: SMS dispatch must automatically respect `do_not_contact` flags in the central `Customer` identity graph.

  ## Implementation Prompt
  **Feature Name**: Agentic Review & Reputation Management
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos finishes repairing a sink and marks the booking complete on his Android phone. 24 hours later, his OHC app suggests sending a drafted SMS to the customer asking for a Google Review. If the customer leaves 5 stars, OHC suggests a celebratory Instagram post.

  **Acceptance Criteria**:
  1. Implement the `ReviewCampaign` PostgreSQL schema and the background event listener for `BookingCompleted`/`OrderCompleted`.
  2. Create a prompt architecture for The Ambassador Agent to generate personalized post-service review requests based on the specific service provided.
  3. Build the Mobile-First (375px) Owner Feed UI card to review and approve the drafted messages.
  4. Playwright E2E Test: Simulate an order completion, trigger the background job, verify the drafted message appears in the owner's feed, and simulate the "Approve" click dispatching the mocked SMS.

  **Top 5 Codebase Optimization Areas Identified During Research (to be fixed later):**
  1. The legacy `src/ui/next` directory still exists and confuses the routing source of truth; needs to be fully decommissioned.
  2. `Cargo.lock` and `pnpm-lock.yaml` contain several outdated dependencies with potential security warnings.
  3. Hardcoded temporary file paths exist in some server test mocks instead of using standard OS temp directories.
  4. Missing Playwright test coverage for edge cases in the Stripe webhook event handler.
  5. The multi-tenant `tenant_id` enforcement in some secondary read-heavy Redis caches lacks strict prefixing rules.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

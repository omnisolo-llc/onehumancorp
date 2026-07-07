issue_title: "Research: Multi-Tenant AI Reputation & Review Management Architecture"
issue_description: |
  # Research Report: Multi-Tenant AI Reputation & Review Management Architecture

  ## Problem Statement
  Small business owners like Carlos (Handyman) and Maya (Baker) survive on word-of-mouth and public reviews (Google, Yelp, Instagram). Currently, legacy platforms (Shopify, Wix, Square) require owners to manually ask for reviews, or they must pay $100-$300/month for disconnected third-party tools like Podium, Yotpo, or Birdeye. OHC lacks a native, event-driven architecture to automatically solicit, intercept, and manage customer reputation directly linked to core business events (e.g., job completion, order fulfillment).

  Without this, our core personas lose crucial SEO and social proof, and negative feedback often ends up public before the owner has a chance to make it right.

  ## Research Report (Track 1 & Competitor Audit)
  - **The Gap in Current Systems:** Tools like Loox or Yotpo on Shopify are strictly e-commerce focused. Podium is service-focused but detached from the core scheduling/invoicing ledger. None of them use LLMs contextually to understand *what* the service was to personalize the ask.
  - **The OHC Opportunity:** Since OHC natively handles `Bookings`, `Projects`, `Invoices`, and `OrderFulfillment`, we can use an event-driven hook to trigger our AI agents. The "Ambassador Agent" can look at the completed work context, draft a highly personalized, natural SMS/Email asking for feedback, and deploy a "Review Interception" strategy (routing 4-5 star reviews to public platforms, and 1-3 star reviews to an internal private Triage Queue).
  - **Persona Journey:** Carlos finishes fixing a sink and marks it "Done" on his phone. He does nothing else. 24 hours later, the AI texts the customer. The customer replies with a 5. The AI replies with Carlos's Google Business link. If the customer replied with a 2, the AI apologizes, asks for details, and puts an "Urgent Triage" card in Carlos's mobile feed.

  ## Design Doc (Track 2 & Track 3)

  ### High-Level Architecture & Data Model (Mermaid.js Concept)
  - **New PostgreSQL Entities (with strict RLS for multi-tenancy):**
    - `reputation_campaigns`: Rules for when to trigger (e.g., 24h post-fulfillment).
    - `feedback_requests`: Tracks the state per customer interaction (scheduled, sent, replied, intercepted, published).
    - `customer_reviews`: The captured rating (1-5), text, and source (Internal, Google, Yelp).
  - **Cross-Agent Coordination:**
    - **Operations Dept:** Fires the `FulfillmentCompleted` or `BookingCompleted` event into the central message bus.
    - **Customer Success (The Ambassador):** Consumes the event, checks the `customer_identities` memory graph (to ensure we aren't spamming frequent buyers), and generates the localized SMS/Email payload.
    - **Triage Dept:** If negative feedback is received, it generates a high-priority Action Card in the unified Owner Feed.

  ### Mobile-First UX Flow (375px Target)
  - **Customer UX:** A frictionless SMS interaction or a fast-loading, single-question mobile web view (Glassmorphism card, huge 1-5 star touch targets >44px).
  - **Owner UX:** The owner does not configure complex campaigns. The Agent Feed simply surfaces a card: *"Carlos, you had 5 jobs this week. I collected 3 new 5-star Google reviews and intercepted 1 complaint about timing. Tap to read the complaint and reply."*

  ### Technical Integrity & Multi-Tenancy Constraints
  - All feedback tables must implement `ENABLE ROW LEVEL SECURITY` with `app.current_tenant` checks.
  - The scheduled jobs must use PostgreSQL `SKIP LOCKED` or the existing OHC job queue to avoid double-sending emails across worker instances.
  - Public URLs for feedback collection must be edge-cached, highly available, and resilient to backend downtime (offline-tolerant read paths).

  ## Implementation Prompt
  **Role:** Backend / Mobile Engineer
  **Task:** Implement the Customer Feedback & Reputation Interception Engine.
  **CUJ (Critical User Journey):**
  1. A backend test simulates completing a `Booking` or `ProjectTask` for a Customer.
  2. A scheduled job (simulated elapsed time) picks up the completion and creates a `feedback_requests` record.
  3. The `Ambassador Agent` generates a personalized text message draft based on the service context.
  4. The Customer submits a 3-star review via an API endpoint.
  5. The system intercepts the review, logs it as `customer_reviews`, and generates a Triage Action Item for the owner instead of prompting for a Google Review.
  6. The mobile owner feed endpoint returns the newly created Triage card.

  **Acceptance Criteria:**
  - Create the necessary database migrations for the reputation entities with strict RLS multi-tenant isolation.
  - Build the event listener / background job that evaluates completed work and schedules the feedback request.
  - Create the API endpoints for the customer to submit feedback and for the owner to view the reputation feed.
  - Ensure 100% unit test coverage for the new logic and at least one E2E Playwright test simulating the customer feedback submission flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

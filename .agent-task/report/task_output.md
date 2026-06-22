issue_title: "AI-Powered Proactive Reputation Management & Review Recovery Agent"
issue_description: |
  # Research Report: AI-Powered Proactive Reputation Management & Review Recovery Agent

  ## 1. Problem Statement
  Small business owners—particularly service providers like Carlos the Handyman and food operators like Fatima—live and die by local word-of-mouth and public reviews (Google Business, Yelp). However, soliciting reviews is a manual, time-consuming process. Worse, when a customer has a poor experience, their only obvious outlet is often a public 1-star review because there is no frictionless private channel to escalate the issue. Owners lack a unified, proactive way to drive positive public reviews while intercepting and resolving negative experiences before they cause reputational damage.

  ## 2. Research Report
  - **Market Context**: Legacy solutions like Podium, Birdeye, or Broadly charge $200-$500/month primarily just to automate SMS review requests and aggregate inbox views. They are prohibitively expensive for micro-SMBs and are disconnected from the core operating system of the business. E-commerce platforms like Shopify or Wix rely on passive post-purchase email plugins (e.g., Yotpo, Loox) that have low conversion rates and lack conversational intelligence.
  - **The OHC Opportunity**: By integrating reputation management natively into the OHC workflow, the "Customer Service Agent" (The Ambassador) can trigger intelligent, conversational follow-ups via SMS, WhatsApp, or email immediately after a service is completed or an order is picked up.
  - **Competitive Advantage**: OHC can use LLM sentiment analysis on the customer's reply. If the sentiment is positive, the Agent provides a direct link to Google/Yelp. If the sentiment is negative or frustrated, the Agent expresses empathy, asks for details, and immediately escalates the interaction to the owner's Agent Feed as an urgent Action Card, effectively intercepting the bad review and turning it into a customer recovery opportunity.

  ## 3. Design Doc

  ### Architecture & AI Integration

  ```mermaid
  sequenceDiagram
    participant O as Operations Agent
    participant J as AI Job Queue
    participant A as Ambassador Agent
    participant C as Customer

    O->>J: Event: Booking/Order Completed (Delay Queue)
    J->>A: Trigger Job (e.g. 24h later)
    A->>C: Send proactive review request (SMS/WhatsApp)
    C-->>A: Customer reply
    alt Positive/Neutral Sentiment
        A->>C: Reply with Google/Yelp Review Link
    else Negative Sentiment
        A->>C: Empathetic response (escalation)
        A->>O: Create High-Priority Action Card in Agent Feed
    end
  ```

  - **Event Trigger**: The Operations Agent marks a `Booking` or `Order` as `Completed`.
  - **Delay Queue (AI Job Queue)**: A job is queued via PostgreSQL `SKIP LOCKED` with a configurable delay (e.g., 2 hours post-food pickup, 1 day post-handyman service).
  - **The Ambassador Agent (LLM)**: Wakes up, retrieves context (what was ordered/serviced, customer name), and drafts a personalized, casual check-in message via the customer's preferred channel (WhatsApp/SMS).
  - **Sentiment Analysis & Routing**:
    - *Positive/Neutral Response*: Agent replies thanking them and includes a one-click review link.
    - *Negative Response*: Agent expresses empathy, asks how to make it right, and creates a high-priority `ActionCard` in the owner's `AgentFeed`.

  ### Data Model (PostgreSQL)
  - `ReviewCampaign`: Tracks the policy for when to ask for reviews (delay, channel preference).
  - `CustomerInteraction`: Logs the conversational flow.
  - `ReputationInterception`: Records instances where a negative sentiment was caught, linking to the corresponding `ActionCard`.

  ### Mobile UX Flow (375px)
  1. **Owner Configuration (Zero-Setup)**: During onboarding, the system defaults to "Auto-ask for reviews 24h after service." The owner sees a single toggle in their Settings: "Proactive Reviews: ON."
  2. **Interception Feed (Owner View)**: If a negative experience is caught, the owner receives a push notification. The app opens to an Action Card: *"Carlos, John was unhappy with the plumbing cleanup. The Ambassador apologized. Would you like to offer a 10% partial refund or call him?"* with one-tap action buttons.
  3. **Customer View**: A simple, friendly SMS. No apps to download.

  ## 4. Implementation Prompt
  **Feature Name**: Proactive Reputation & Interception Engine
  **Target Personas**: Carlos (Handyman) and Fatima (Food Cart)
  **Outcome**: Owners automatically generate 5-star Google reviews from happy customers while catching and privately resolving unhappy customers before they leave public complaints.

  **Next Actions**:
  1. Create the `ReviewCampaign` and `ReputationInterception` schemas in PostgreSQL with row-level security per `tenant_id`.
  2. Implement the background worker (using the existing AI Job Queue) that triggers the Ambassador Agent based on `Order`/`Booking` completion events.
  3. Enhance the Ambassador Agent prompt to handle the two-step conversational review flow (initial check-in -> sentiment analysis -> routing to review link OR owner escalation).
  4. Build the UI for the "Interception Action Card" in the owner's Agent Feed, providing quick resolution options (draft reply, partial refund, call customer).

  **Acceptance Criteria**: E2E test verifying a completed order triggers an SMS draft, a simulated positive reply triggers a review link, and a simulated negative reply generates an Action Card in the owner's feed without sending a review link.

  ## 5. Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

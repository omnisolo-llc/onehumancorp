issue_title: "Agentic Customer Review & Reputation Management System"
issue_description: |
  # Research Report: Agentic Customer Review & Reputation Management System

  ## 1. Problem Statement
  Small business owners and operators (e.g., Carlos the Field Service Owner, Maya the Home Baker) live and die by word-of-mouth and online reviews (Google Business, Yelp, Instagram comments). However, actively soliciting reviews after a successful service, tracking them across platforms, and responding professionally is a time-consuming, manual process. An unresponded negative review can hurt reputation, while missed positive reviews leave growth on the table. Non-technical owners need an invisible assistant to manage this entire lifecycle without requiring a separate SaaS tool like Podium or Birdeye.

  ## 2. Research Report
  - **Market Context**: Platforms like Podium, Birdeye, and Trustpilot offer reputation management, but they are expensive ($200+/month) and add another dashboard for the owner to manage. Shopify and Wix have basic review plugins, but they are passive—they require the customer to proactively return to the site.
  - **The OHC Opportunity**: By deeply integrating reputation management into the core order/booking fulfillment lifecycle, OHC can use its AI agents to automatically trigger review requests at the optimal time (e.g., 2 hours after Carlos marks a repair as "Complete", or 1 day after Maya's cake is delivered).
  - **Competitor Gaps**:
    - *Shopify/Wix*: Passive, email-based review plugins that don't intelligently time requests or auto-draft responses.
    - *Podium*: Expensive, disconnected from the core operational source of truth (the actual service booking or order).

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Carlos (Owner)
      participant System as OHC Ledger
      participant OpsAgent as Operations Agent
      participant Queue as Delay Queue
      participant CustAgent as Ambassador Agent
      participant Customer as Sarah (Customer)

      Owner->>System: Mark Booking "COMPLETED"
      System->>OpsAgent: Emit Event (Booking Completed)
      OpsAgent->>Queue: Schedule Review Request (+2 hours)
      Queue-->>CustAgent: Trigger Job
      CustAgent->>Customer: Send SMS "How did we do?"
      Customer->>System: Submits 5-star Review
      System->>CustAgent: Ingest Review
      CustAgent->>Owner: Push Feed Notification with Drafted Reply
      Owner->>CustAgent: Tap "Approve & Post"
      CustAgent->>System: Publish Reply
  ```

  ### Data Model (ER Diagram)
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
      }
      BOOKING {
          uuid id PK
          uuid tenant_id FK
          string status
      }
      REVIEW_CAMPAIGN {
          uuid id PK
          uuid tenant_id FK
          uuid booking_id FK
          string status "pending, sent, completed"
          timestamp scheduled_for
      }
      REVIEW {
          uuid id PK
          uuid tenant_id FK
          uuid booking_id FK
          int rating
          text content
          string source "sms, google, instagram"
      }
      REVIEW_RESPONSE {
          uuid id PK
          uuid tenant_id FK
          uuid review_id FK
          text drafted_content
          string status "draft, published"
      }

      TENANT ||--o{ BOOKING : has
      TENANT ||--o{ REVIEW_CAMPAIGN : owns
      BOOKING ||--o| REVIEW_CAMPAIGN : triggers
      BOOKING ||--o| REVIEW : receives
      REVIEW ||--o| REVIEW_RESPONSE : has
  ```

  ### AI Agent Coordination
  - **Operations Agent**: Monitors the state machine of an `Order` or `Booking`. When it reaches `COMPLETED`, it queues a delayed job to request a review.
  - **Customer Success Agent ("The Ambassador")**:
    - Drafts and sends personalized review request via SMS/WhatsApp/Email.
    - Ingests incoming reviews via API integrations.
    - Drafts an empathetic, context-aware response to reviews (e.g., apologizing for a delay based on the order history, or thanking them for mentioning a specific cake flavor).
  - **Decision Assistant**: Summarizes sentiment trends in the daily owner feed (e.g., "3 customers mentioned slow delivery this week").

  ### Mobile UX Flow (375px)
  1. **Owner Work Feed**: A new card appears in the daily triage feed: "New 5-star review from Sarah. Drafted reply ready for approval."
  2. **Review Action Card**: The owner taps the card to view the review and the AI-drafted reply. Touch targets are large (44x44px).
  3. **One-Tap Actions**: Buttons for "Approve & Post", "Edit", and "Dismiss".
  4. **Settings (Advanced)**: A simple toggle in the Customer settings: "Auto-request reviews after completed orders."

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Customer Review & Reputation Management
  **Target Persona**: Carlos the Field Service Owner
  **Outcome**: Carlos finishes a repair, marks the job complete on his Android phone, and moves on. Two hours later, the OHC Ambassador Agent texts the customer asking for a review. When the 5-star review comes in, the agent drafts a polite reply thanking them for trusting Carlos with their plumbing, and Carlos approves it with one tap from his feed.

  **Next Actions**:
  1. Implement the core Data Models (`Review`, `ReviewCampaign`) with strict multi-tenant isolation in PostgreSQL.
  2. Create a background job queue worker that schedules review requests based on `Order`/`Booking` completion events.
  3. Develop the Customer Success Agent capability to ingest reviews and draft responses using the LLM provider.
  4. Build the mobile-first (375px) owner feed card for review triage and response approval. Do not implement complex rule builders; stick to the one-tap approval flow.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

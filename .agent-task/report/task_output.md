issue_title: "Feature: Agentic Customer Intake & Autonomous Negotiator for Service Providers"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement**:
  Service business owners like Carlos (Handyman) lose approximately 30% of their inbound leads because they are actively working on jobs and cannot answer calls or messages in real-time. Currently, the OHC platform provides `booking.rs` and `quotes.rs` for calendar and quote management, but it relies on the owner manually reviewing inquiries, drafting quotes, and scheduling. This manual friction prevents owners from capturing immediate demand.

  **Research Report**:
  - **Competitor Analysis**: AI-native competitors like 11x.ai (Alice) and Lindy.ai have proven the demand for AI executive assistants that handle inbound triage and scheduling. However, these tools are often disjointed from the core business ledger. Traditional platforms like Shopify or Square lack robust service-oriented negotiation.
  - **Persona Context (Carlos)**: Carlos needs an agent that intercepts a WhatsApp or web widget inquiry (e.g., "Can you fix my sink?"), understands his standard pricing, checks his calendar availability, proposes times, and secures a deposit autonomously.

  **Design Doc (Architecture & UX)**:
  - **Architecture Diagram (Mermaid)**:
    ```mermaid
    sequenceDiagram
        participant Customer
        participant InboxService
        participant NegotiatorAgent
        participant BookingService
        participant PaymentService

        Customer->>InboxService: "Can you fix a leaky sink tomorrow?"
        InboxService->>NegotiatorAgent: Trigger Intake Event
        NegotiatorAgent->>BookingService: Check Availability
        BookingService-->>NegotiatorAgent: Available: 2 PM, 4 PM
        NegotiatorAgent->>Customer: "Yes! It's $120. I have 2 PM or 4 PM. Which works?"
        Customer->>NegotiatorAgent: "2 PM works."
        NegotiatorAgent->>PaymentService: Generate $50 Deposit Link
        PaymentService-->>Customer: Send Payment Link
        Customer->>PaymentService: Pays Deposit
        PaymentService->>BookingService: Confirm Booking (Redis Lock -> DB)
        BookingService->>InboxService: Add "Confirmed Job" to Owner Feed
    ```
  - **Mobile UX Flow (375px)**: The owner (Carlos) does NOT need to manage the chat. Instead, the OHC Mobile App's home feed displays a UniFi-style translucent glass card: "New Confirmed Booking: Sink Repair tomorrow at 2 PM. $50 deposit secured." The card uses a `#34C759` badge. The owner can tap the card to view the AI conversation history if desired.
  - **AI Agent Integration Notes**:
    - **Customer Success Agent (Negotiator)**: Handles the NLP processing and context management for the conversation.
    - **Operations Agent**: Provides the real-time calendar availability using Redis Redlock (`ohc:lock:{tenant_id}:calendar:{slot}`) to prevent double-booking while the payment is pending.
    - **Finance Agent**: Interfaces with Stripe Checkout Sessions for the deposit and manages the idempotency keys.
  - **Key Design Decisions**: Use PostgreSQL `SKIP LOCKED` for the incoming message queue to allow horizontal scaling of the Negotiator Agent workers. State transitions (e.g., `TentativeBooking` -> `ConfirmedBooking`) are exclusively driven by Stripe Webhooks.

  **Implementation Prompt**:
  - **Objective**: Implement the `NegotiatorAgent` service and its associated background worker queue.
  - **CUJ**: A customer sends a natural language message asking for a service. The agent parses the intent, checks availability, responds with a quote and time options, and generates a payment link. Once paid, the system creates a confirmed booking and displays it on the owner's mobile feed.
  - **Acceptance Criteria**:
    1. A new PostgreSQL table/queue for conversational intake tracking.
    2. The `NegotiatorAgent` can successfully reserve a calendar slot using Redis Redlock for 10 minutes while awaiting payment.
    3. The owner's frontend feed dynamically displays the confirmed booking card without requiring manual intervention.
    4. Provide 100% unit test coverage for the intake parsing and state machine transitions.
    5. A full Playwright E2E test simulating the customer chat and owner feed verification.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

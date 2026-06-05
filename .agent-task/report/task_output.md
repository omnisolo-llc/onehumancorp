issue_title: "[research] Architecting the Proactive Booking Agent Feed"
issue_description: |
  # Research Report: Proactive Booking Agent Feed

  ## Problem Statement
  Solopreneurs like Leo (Music Tutor) and Carlos (Handyman) waste hours manually answering inquiries, negotiating time slots, and sending booking links. They miss leads when they are working or sleeping. They need an automated "Operations Agent" that intercepts inquiries, contextually negotiates time slots, and proactively presents the drafted response as an "Approve & Send" card on their mobile feed.

  ## Research Report
  - **The Gap**: Existing tools (Calendly, Shopify Sidekick) are reactive or require the customer to do the work of finding the link. The "Unified Agent Feed" mobile paradigm requires pushing actions to the business owner for 1-tap approval.
  - **Competitive Landscape**:
    - **Shopify**: Lacks native booking; reliance on 3rd party apps creates friction.
    - **Wix/Squarespace**: Requires complex manual setup. AI is not proactive.
  - **Solution**: A centralized feed where the Operations Agent pushes intent-matched booking inquiries as actionable cards (e.g. "Customer asked for Friday morning. Drafted: Yes, here are 3 slots. [Approve & Send]").

  ## Design Doc
  - **Architecture**:
    - **Event Pipeline**: Webhook ingestion (Instagram DM, Web Chat) -> Redis Pub/Sub -> KAIROS Shared Task List.
    - **Agent Logic**: KAIROS Operations Worker picks up the task, classifies intent via Gemini, queries Cal.com/Google Calendar integration for availability.
    - **UI Delivery**: The draft is saved to the PostgreSQL database with status `PENDING_APPROVAL`. The mobile app (Tauri/Flutter 375px view) polls or receives real-time updates and displays a Premium Glassmorphism card.
  - **UI/UX (Mobile-First 375px)**:
    - Card Title: "Booking Inquiry: @username"
    - Body: Drafted response with 3 available time slots.
    - Actions: Big, thumb-friendly buttons (≥ 44x44px): [Approve & Send] [Edit].

  ## Implementation Prompt
  Implement the "Operations Agent" booking inquiry flow within the Unified Agent Feed.
  1. Set up an event listener for incoming messaging events.
  2. Implement an LLM intent classifier to identify "booking requests".
  3. Generate a database entity (e.g., `AgentActionCard`) representing the drafted response for the 375px mobile feed.
  4. Build the user-facing mobile card UI with an "Approve & Send" interaction.
  Do not prescribe specific database schemas or internal API endpoints. Focus on the user's Critical User Journey (CUJ) of reviewing and approving the drafted response.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

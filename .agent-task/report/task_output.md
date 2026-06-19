issue_title: "Implement Autonomous Voice Receptionist & Ordering Engine"
issue_description: |
  # Mission Queue Protocol: Autonomous Voice Receptionist & Ordering Engine

  ## Problem Statement
  Small business owners like Fatima (Food Cart Operator) and Carlos (Field Service Owner) receive a significant volume of customer inquiries and orders via phone calls. However, they are often too busy preparing food or performing repairs to answer the phone. Missed calls mean lost revenue and frustrated customers. Existing solutions like Google Voice or third-party answering services do not integrate with their OHC inventory, pricing, or booking systems. They need an AI voice receptionist that can answer calls, check real-time inventory/availability, and autonomously process orders or bookings.

  ## Research Report
  - **Market Gap**: Current SMB platforms (Shopify, Wix) focus heavily on web and text commerce. They offer zero native telephony integration. Small businesses resort to disjointed tools that cannot read their live database.
  - **Competitive Analysis**:
    - *Shopify*: No voice agents. Relies on "Shopify Inbox" (text).
    - *Wix*: Lacks voice capabilities.
    - *Specialized Tools*: Services like Ruby Receptionists are expensive and lack deep data integration.
  - **Opportunity**: By integrating WebRTC and modern low-latency Voice AI (e.g., Twilio Voice + OpenAI Realtime API or Gemini), OHC can provide a unique phone number to each tenant. The AI can handle the call, access the exact inventory and availability, and process requests dynamically.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Telephony
          User[Customer Phone] -->|PSTN| Twilio[Twilio Voice]
      end

      subgraph Backend "Rust + Bazel Backend"
          Gateway[OHC API Gateway]
          VoiceMesh[OHC Voice Mesh Service]
          KAIROS[KAIROS Orchestrator]
      end

      subgraph Agent Context
          VoiceLLM[Low-Latency Voice LLM]
          SalesAgent[Sales & Acquisition Agent]
          OpsAgent[Operations Agent]
      end

      subgraph Storage
          DB[(PostgreSQL - Tenant DB)]
      end

      Twilio -->|WebSockets/Media Stream| Gateway
      Gateway --> VoiceMesh
      VoiceMesh -->|STT/TTS Stream| VoiceLLM
      VoiceMesh -->|Context/Function Calls| KAIROS

      KAIROS --> SalesAgent
      KAIROS --> OpsAgent

      SalesAgent --> DB
      OpsAgent --> DB
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **Settings / Voice Agent Tab**: A glassmorphism card on the mobile dashboard.
  2. **Configuration**: User can toggle the "AI Voice Receptionist" on/off. They can select the primary language (e.g., English, Arabic) and provide custom instructions (e.g., "Always mention today's special: Vegan Chocolate Cake").
  3. **Call Log**: A feed showing recorded calls, transcripts, and actions taken (e.g., "Order placed", "Booking scheduled").

  ### Mobile UX Flow
  - User opens the OHC app.
  - Navigates to Settings -> Voice Assistant.
  - Toggles the feature "On".
  - Configures the custom instructions via a simple text area.
  - Taps "Save". The UI updates to show the assigned Twilio phone number.

  ### AI Agent Integration Points
  - **Sales & Acquisition Agent**: Handles the conversation, answers FAQs, and pitches products/services.
  - **Operations Agent**: Checks inventory (e.g., "Do you have any vegan cakes left?") and confirms bookings or orders in the system.

  ### Key Design Decisions
  - **WebSockets for Low Latency**: Use WebSockets between Twilio and the OHC backend to stream audio to the Voice LLM, ensuring near-instantaneous responses.
  - **Zero-Trust Isolation**: The Voice Mesh must strictly enforce tenant isolation (via SPIFFE SVIDs) so the AI only accesses the correct business's data.

  ## Implementation Prompt
  **User-Facing Outcome**: The user can navigate to the "Voice Agent" settings in the OHC mobile app, enable the AI receptionist, set custom instructions, and view a log of transcribed calls and the actions the agent took (like creating an order).

  **Critical User Journey (CUJ)**:
  1. The user logs into the OHC app on a mobile device (375px width).
  2. The user navigates to the "Voice Agent" configuration screen.
  3. The user toggles the feature on and enters custom instructions.
  4. The user saves the configuration.
  5. (Simulated) A call is received, and a new entry appears in the "Call Log" feed showing the transcript and the resulting action.

  **Acceptance Criteria**:
  - The UI must be fully functional and responsive down to 320px width, designed primarily for 375px.
  - Touch targets must be at least 44x44px.
  - The feature must use the OHC Premium Token library (Glassmorphism styling).
  - The backend must provide endpoints to save the configuration and mock/receive the webhook payload.
  - Full E2E Playwright coverage for the configuration flow must be provided, seeding the database via the proper `e2e-seed.sql` mechanism and using no mocked network requests.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

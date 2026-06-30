issue_title: "Design Autonomous Multi-lingual Voice-to-Text Order Interception Architecture for Food and Service Personas"
issue_description: |
  # OHC Owner Work Assistant: Autonomous Multi-lingual Voice-to-Text Order Interception

  ## 1. Problem Statement
  Personas like Fatima (Food Cart Operator) and Carlos (Field Service Owner) face significant friction with inbound phone demand. Fatima struggles with English-speaking customers while cooking; Carlos loses ~30% of leads when he cannot answer the phone on the job. OHC currently lacks an autonomous agent capable of intercepting inbound voice calls, translating/transcribing them, and converting them into actionable Kitchen Display System (KDS) orders or service quotes.

  ## 2. Research Report
  - **Market Context**: Traditional systems (Shopify, Wix) are screen-first and fail the "hands-full" test. AI-native tools like 11x.ai (Alice/Julian) handle inbound calls but are heavily tailored for B2B sales development rather than rapid SMB operational triage (orders/quotes). Intercom Fin focuses on support resolution.
  - **The OHC Opportunity**: Building a Voice Interceptor Agent that seamlessly integrates with our KDS (for Fatima) and Quoting Engine (for Carlos) creates a massive differentiator against screen-bound competitors.
  - **Target Metrics**:
    - < 500ms voice response latency.
    - Graceful fallback to SMS/Text if transcription confidence drops below 80%.
    - Zero technical configuration for the owner (activated via natural language).

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Customer
          PhoneCall[Inbound Phone Call]
      end

      subgraph OHC KAIROS Edge
          Twilio[Twilio/Telco Webhook]
          VoiceAgent[Voice Agent Orchestrator]
          STT[Speech-to-Text Stream]
          LLM[LLM Translation & Intent]
          TTS[Text-to-Speech Response]
      end

      subgraph OHC Core
          OrderService[Order / POS Service]
          QuoteService[Quoting Service]
      end

      subgraph Owner Devices
          TabletKDS[Tablet KDS - Arabic]
          MobileApp[Carlos Mobile - Push]
      end

      PhoneCall <--> Twilio
      Twilio <--> VoiceAgent
      VoiceAgent --> STT
      STT --> LLM
      LLM --> TTS
      VoiceAgent --> TTS

      LLM -- "Fatima: Food Order" --> OrderService
      OrderService --> TabletKDS

      LLM -- "Carlos: Repair Lead" --> QuoteService
      QuoteService --> MobileApp
  ```

  ### Mobile UX Flow (375px First)
  **Fatima's KDS View:**
  1. Fatima's tablet shows an Apple-style Translucent Glass UI.
  2. A new order card drops in with a subtle pulse animation.
  3. The card displays the translated order in Arabic: "2x Chicken Shawarma, 1x Falafel - Customer: John (via AI Agent)".
  4. Large touch targets (44x44px minimum): `[Accept & Print]` | `[Sold Out]`.

  **Carlos's Mobile View:**
  1. Carlos receives a native push notification: "New Lead Captured by Agent: Broken Pipe - $50 Deposit Secured."
  2. Tapping opens a unified feed card showing the call summary, customer intent, and the auto-generated quote ready for dispatch.

  ### AI Agent Integration Points
  - **Trigger**: Twilio webhook initiates the OHC Voice Agent session.
  - **Memory Layer**: The Voice Agent queries `ohc:tenant_memory:{tenant_id}` to know the current menu/inventory (e.g., "Falafel is sold out today").
  - **Handoff**: The LLM output is structured JSON that KAIROS maps to internal gRPC/REST mutations (e.g., `CreateOrder`, `CreateLead`).

  ### Key Design Decisions
  - **Multi-Tenant Isolation**: All voice sessions and resulting data mutations must carry the `tenant_id` for strict PostgreSQL row-level security.
  - **Offline/Low-Data Tolerance**: If Fatima's tablet goes offline, the OHC Core queues the order via Redis. Once reconnected, the KDS syncs instantly.
  - **Language Agnostic KDS**: The Voice Agent translates the customer's English directly to the tenant's configured system language before persisting to the database, ensuring the UI layer doesn't bear the translation burden.

  ## 4. Implementation Prompt
  Implement the foundational backend APIs and the frontend KDS view for the Multi-lingual Voice Order Interceptor.
  - **CUJ**: A customer makes a simulated voice call (mocked via a text/audio-file API endpoint for this phase). The OHC Agent transcribes and translates the order into the owner's language, generating an Order record. The owner opens the KDS view on a 375px screen and sees the translated order appear seamlessly.
  - **Acceptance Criteria**:
    - Build a REST/gRPC endpoint to ingest simulated voice/text orders.
    - Integrate the LLM provider to parse intent and translate to the tenant's default language.
    - Persist the order to PostgreSQL ensuring `tenant_id` isolation.
    - Build a mobile-first (375px) KDS screen using the OHC Premium Token library (Translucent Glass).
    - Ensure zero mock data in the KDS UI—it must render the real database record.
    - Provide 100% unit test coverage and at least one Playwright E2E test verifying the flow from simulated inbound call to KDS display.

  ## 5. Scope & Priority
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large (Requires backend orchestration + mobile-first UI)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

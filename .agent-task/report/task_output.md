issue_title: "[Research] AI Multi-Language Voice Receptionist & Intake Architecture"
issue_description: |
  # Multi-Language Voice Receptionist & AI Intake System

  ## Problem Statement
  Small business owners (Carlos the handyman, Fatima the food cart operator) often miss phone calls while they are working, directly leading to lost revenue. Existing voicemail solutions are static and cannot triage urgent requests, quote estimates, or answer localized, language-specific queries. When they are busy with their hands, they need an agent to answer the phone, capture context, provide pricing when relevant, and summarize it in the Unified Inbox without breaking their operational flow.

  ## Research Report
  - **Competitive Landscape**: Services like Bland AI, Vapi, and Retell AI provide developer-facing voice agent solutions, but they require complex prompt engineering and lack deep integration into small business operations (like inventory or scheduling).
  - **Persona Fit**:
    - **Carlos**: Needs a bilingual voice agent that answers in English or Spanish, captures lead info, references his stored price list, and books a site visit.
    - **Fatima**: Needs an Arabic/English voice bot that can take pre-orders for the food cart when she's too busy cooking.
  - **Gaps in OHC**: We have a robust Work Triage (Unified Inbox) and Customer & Relationship Assistant for text/DMs, but we lack a synchronous, integrated Voice Intake channel. We also lack a standardized AI Action handoff from a Voice Agent transcript directly into our `AI Job Queue`.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer Phone Call] -->|Twilio/WebRTC| B(Voice Provider Gateway - e.g. Vapi/Retell)
      B -->|Streaming Audio/WebSockets| C{OHC Voice Integration Layer}
      C -->|Transcripts & Context| D(OHC Work Triage / Unified Inbox)
      C -->|Tool Calls via WebSocket| E(OHC Agent Job Queue)
      E -->|Fetch Pricing/Availability| F[(Tenant PostgreSQL)]
      E -->|Agent Response| C
      C -->|Synthesized Audio| B
      B -->|Audio| A
      D -->|Owner Notification| G[Mobile / Desktop App]
  ```

  ### Mobile UX Flow (375px first)
  1. **Settings / Voice Assistant**: Owner navigates to "Phone Assistant" under Settings. They see a simple toggle: "Enable AI Receptionist".
  2. **Configuration**: They select a preferred voice, primary/secondary languages, and a core goal (e.g., "Capture Lead", "Answer FAQs", "Take Order"). No prompt engineering required.
  3. **Active Call State**: When a call comes in, the mobile app shows a persistent top banner: "AI Receptionist answering call from +1 555-1234...".
  4. **Post-Call Summary**: Immediately after the call drops, a new Work Triage card appears: "Missed Call: Carlos booked a repair for tomorrow. (Tap to view transcript or edit quote)".

  ### AI Agent Integration Points
  - **Voice Provider Webhook**: OHC needs a standardized ingest route (`/api/v1/voice/inbound`) to receive transcripts and structured data payloads from the external voice provider.
  - **Tool Execution Bridge**: The voice provider must be able to securely invoke OHC's existing multi-tenant tools (e.g., `check_calendar`, `get_inventory_price`) via a secure, fast API bridge so the voice agent can respond to the customer in real-time.

  ### Key Design Decisions
  - **Zero Trust & Multi-Tenancy**: The Voice Gateway API must authenticate requests and bind them strictly to the `tenant_id` associated with the provisioned phone number.
  - **Asynchronous Handoff**: Voice interactions are fast, but complex OHC background jobs (like sending an SMS confirmation) must be decoupled into the PostgreSQL `SKIP LOCKED` job queue after the call context is saved.
  - **No-Code Setup**: The owner configures the voice agent using structured business goals, not raw LLM prompts. OHC internally translates these goals into the provider's required system prompts, injecting the tenant's context.

  ## Implementation Prompt
  **Outcome**: Build the foundational routing and data models for the AI Voice Receptionist integration, enabling seamless ingest of call transcripts into the Unified Inbox.

  **CUJ**:
  1. Carlos navigates to Settings > Voice Assistant and enables it for English/Spanish.
  2. A customer calls Carlos's OHC phone number.
  3. The external voice provider handles the call and POSTs the final transcript and structured outcome to OHC.
  4. Carlos opens the OHC mobile app and sees the call summary in his Unified Inbox, complete with the requested repair context.

  **Acceptance Criteria**:
  - Implement a secure webhook endpoint for receiving call outcomes from an external provider.
  - Create the necessary database tables/migrations to store `voice_call_logs` and map them to the existing unified inbox / messaging system, ensuring strict row-level tenant isolation.
  - Ensure the UI correctly displays the voice interaction in the 375px mobile view, treating it as a first-class citizen alongside text DMs.
  - Write Playwright E2E tests verifying the UI flow and backend tests for the webhook handler. Do not implement the actual real-time streaming audio WebSocket bridge in this phase.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

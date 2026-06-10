issue_title: "Implement Agentic Voice-to-Action Mobile Command Center"
issue_description: |
  **Problem Statement**:
  Small business owners like Carlos (Handyman) and Fatima (Food Cart) operate entirely on mobile devices while using their hands for their actual work. The current OHC mobile UX relies heavily on tapping through menus and forms, which is slow and inaccessible during active service hours. They need a way to orchestrate complex business actions (e.g., creating a quote, marking an item sold out, or rescheduling an appointment) simply by speaking to their assistant.

  **Research Report**:
  - Current OHC design uses "Action Cards" for approval (Agent Feed), but lacks a multimodal input mechanism for the owner to *initiate* complex workflows hands-free.
  - Competitor analysis shows tools like Siri or Google Assistant handle basic consumer tasks, but no platform offers an *integrated business operations voice assistant* capable of deep, multi-tenant state mutation.
  - Allowing owners to push a single "Push-to-Talk" button on a 375px screen to execute commands like "Send a $150 repair quote to the last customer who called" bridges the gap between complex software and real-world physical operations.

  **Design Doc**:
  - **Architecture**:
    1. Mobile App captures audio -> Streams to OHC Backend via WebSocket or gRPC.
    2. Backend routes audio to Whisper (or equivalent LLM audio model) for Speech-to-Text.
    3. The transcribed text is sent to the `Orchestrator Agent`, which uses tools (via MCP or internal definitions) to query state and formulate a mutation plan.
    4. The Orchestrator returns a structured "Action Card" (Proposal) back to the mobile client via WebSocket for final user tap-approval (maintaining safety).
  - **Mobile UX Flow**:
    - A persistent, floating, translucent "Voice Assistant" button (glassmorphism design) at the bottom of the 375px viewport.
    - User holds to speak. A subtle waveform animation indicates listening.
    - Upon release, the assistant replies (text + TTS voice) and drops an actionable Card in the Agent Feed (e.g., "Here is the quote for John. Ready to send? [Send]").
  - **AI Integration**: Orchestrator Agent needs access to `inventory`, `CRM`, and `billing` tools to resolve relative commands ("last caller", "that vegan cake").

  **Implementation Prompt**:
  - Implement a mobile-first (375px) floating Voice Command button using OHC Premium Tokens.
  - Wire the button to stream audio to a new backend endpoint `/api/v1/voice/command`.
  - The endpoint must transcribe the audio, pass it to the Orchestrator Agent with the user's `tenant_id` context, and return a structured proposed action to the frontend.
  - Display the proposed action as an Approval Card in the Agent Feed.
  - Acceptance Criteria: A user can tap the button, speak a command, and receive a correct, actionable Agent Feed card in under 3 seconds without typing.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

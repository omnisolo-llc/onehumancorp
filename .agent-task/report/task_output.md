issue_title: "Implement AI Voice-Driven Field Service Estimator & Dispatcher"
issue_description: |
  ## Title: Implement AI Voice-Driven Field Service Estimator & Dispatcher

  ## Problem Statement
  Field service operators like Carlos (handyman, 42) spend their entire day in transit or on job sites. They operate strictly from an Android phone and cannot stop to type out complex estimates, update CRM records, or manage scheduling grids manually. Legacy solutions like Jobber or Housecall Pro require extensive tapping, typing, and form-filling on mobile devices, which causes friction when wearing gloves or driving. Carlos needs an AI assistant he can talk to that will instantly convert his voice notes into structured estimates, schedule the next job, and automatically draft SMS updates to the customer.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Jobber / Housecall Pro / ServiceTitan:** These are the incumbents for field service management. While they have excellent mobile apps for dispatching and invoicing, they rely on traditional form-based data entry. The "AI" features are mostly limited to backend reporting, not frontline data entry.
  - **Shopify / Wix:** Completely lack native field service dispatching and estimation features; they focus on retail/digital goods.
  - **OHC Opportunity:** Utilize the "Operations Assistant" and a new Voice-to-Action pipeline. Carlos should be able to tap a single microphone button on the OHC mobile app and say: *"I just finished the sink repair at 123 Main St. The customer needs a new garbage disposal. Create an estimate for $250 and text them a link to approve."* The system must parse this, update the job status, generate the structured estimate, and stage an SMS for Carlos to approve with one tap.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Voice Input] -->|Audio Stream| B[Whisper / Voice Gateway]
      B --> C[Speech-to-Text Transcription]
      C --> D[Operations Agent - LLM Intent Parser]
      D -->|Query| E[Tenant CRM & Catalog DB]
      D -->|Update Status| F[Job & Routing Engine]
      D -->|Draft Estimate| G[Quoting Engine]
      D -->|Draft SMS| H[Communications Gateway]
      G --> I[Action Required Queue]
      H --> I
      I --> J[Mobile Feed - 375px]
      J -->|1-Tap Approve| K[Execute External Actions]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Persistent Floating Action Button (FAB):** A large, accessible microphone icon is present on the main mobile feed.
  - **Interaction (Voice Capture):** Tapping the FAB dims the screen with a translucent glass effect and shows an active voice waveform.
  - **Interaction (Processing):** After speaking, the user sees a quick "Processing..." state.
  - **Interaction (Review & Approve):** The mobile feed immediately presents a new Agent Action Card:
    - **Top section:** "Drafted Estimate: $250 for Garbage Disposal Install at 123 Main St."
    - **Middle section:** "Drafted SMS to Customer: 'Hi, here is the estimate for the disposal as discussed...'"
    - **Bottom section:** Large "Approve & Send" (Primary) and "Edit" (Secondary) buttons.

  ### AI Agent Integration Points
  - **Voice Gateway:** Captures and transcodes audio to text.
  - **Operations Agent:** Parses the text for entities (Customer, Address, Service, Price, Action). It must perform RAG against the user's service catalog to match the requested service and verify the customer record.
  - **Customer & Relationship Assistant:** Drafts the outbound SMS based on the parsed intent and the customer's preferred communication channel.

  ### Key Design Decisions
  - **Voice-First Input:** Field workers cannot type effectively on the job. Voice must be treated as a primary input mechanism, not an accessibility afterthought.
  - **Action Staging:** The AI must never send an estimate or text a customer without explicit owner approval. Everything is drafted to the "Action Required" queue first.
  - **Optimistic UI:** The transition from voice recording to the generated Action Card must feel near-instant, leveraging streaming LLM responses if possible.

  ## Implementation Prompt
  **User-Facing Outcome:** As a field service owner, I want to use voice commands on my phone to log job completion, generate new estimates, and draft customer communications, so I don't have to manually type out forms while in my truck.
  **CUJ:**
  1. Open the OHC app (simulated 375px viewport).
  2. Tap the Voice Assistant FAB.
  3. Speak a command involving a job update and an estimate request.
  4. The system transcribes and parses the request.
  5. The mobile feed displays an "Action Required" card containing the drafted estimate and SMS.
  6. Tap "Approve" to finalize the job update and send the communications.
  **Acceptance Criteria:**
  - Introduce an audio capture component or mock interface for the mobile UI.
  - Create the backend pipeline to accept text/audio, parse it via the Operations Agent, and stage an Action Card.
  - The UI must render the Action Card correctly on a 375px screen.
  - Ensure 100% unit test coverage for the parsing logic and E2E Playwright coverage for the approval flow.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Voice-Native AI Customer Intake & Triage Gateway"
issue_description: |
  ## Problem Statement
  Many small business owners in the service and food sectors (e.g., Carlos the handyman, Fatima the food cart operator) rely heavily on inbound phone calls for new business, customer inquiries, and emergency service requests. However, they are often unable to answer the phone while actively working (e.g., carrying tools, cooking). This leads to missed calls, lost revenue, and poor customer experience. Existing voicemail solutions are passive and do not capture structured intent or context.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional Voicemail/Answering Services:** Passive, expensive, and require manual review and callback, disrupting the owner's workflow.
  - **Google Voice / Grasshopper:** Provide basic call routing and voicemail transcription but lack AI-driven triage, context awareness, and integration with the owner's operational workflow.
  - **Twilio / Plivo:** Offer programmable voice APIs but require significant technical expertise to build conversational AI flows, which is entirely inaccessible to our target personas.
  - **OHC Opportunity:** Implement an AI-powered Voice Gateway that answers missed or routed calls, converses naturally with the customer, identifies the intent (e.g., booking request, pricing inquiry, emergency), and translates the unstructured audio into a structured, prioritized "Action Required" card in the owner's mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inbound Call] -->|Twilio/WebRTC| B(Voice Gateway Service)
      B --> C(Speech-to-Text & Intent Classification)
      C -->|LLM Agent| D(The Ambassador / Receptionist)
      D --> E{Action Triage}
      E -->|Booking| F(Operations Database)
      E -->|Query| G(Knowledge RAG)
      F --> H[Owner Mobile Feed - Action Card]
      G --> H
  ```

  ### Mobile UX Flow (375px first)
  1. The AI Receptionist handles the call and captures the customer's intent.
  2. The owner receives a push notification on their mobile device.
  3. Opening the app displays a high-priority, 375px-optimized card at the top of the feed:
     - **Title:** "New Service Request: Leaky Pipe"
     - **Customer:** John Doe (New Caller)
     - **Summary:** Caller needs a plumber for a leaky pipe in the kitchen. Requested time: Tomorrow morning.
     - **Actions:** "Accept & Schedule", "Call Back", "Decline"
  4. Tapping an action seamlessly triggers the Operations Agent to handle the subsequent workflow (e.g., calendar booking, SMS confirmation).

  ### AI Agent Integration
  - **Voice Receptionist Agent:** Handles real-time conversational flow, leveraging low-latency LLMs and TTS/STT pipelines.
  - **Triage Agent:** Analyzes the transcribed conversation to extract structured data (intent, urgency, location, time preferences) and route it to the appropriate internal system (Scheduling, Sales, Support).

  ### Key Design Decisions
  - **Real-time vs. Asynchronous:** Voice requires sub-second latency for natural conversation. We will leverage streaming STT (e.g., Deepgram) and streaming LLM responses (Gemini Pro/Flash) to ensure conversational fluidity.
  - **Seamless Handoff:** The AI must gracefully offer to escalate to the human owner or take a structured message if it cannot resolve the inquiry.
  - **Mobile-First Actionability:** The output of the voice agent must be a structured UI card, not just a raw transcript. The owner must be able to act on the result with a single tap.

  ## Implementation Prompt
  - Integrate a real-time Voice API provider (e.g., Twilio Voice) with the OHC backend.
  - Implement a conversational AI pipeline (STT -> LLM -> TTS) capable of handling basic booking and FAQ intents.
  - Build the backend logic to parse the conversation into a structured task/lead.
  - Develop the mobile-first (375px) action card UI in the owner's feed to present the triaged call summary and 1-tap actions.
  - Ensure the voice interaction feels natural and the mobile UI adheres to the premium OHC translucent glass design tokens.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

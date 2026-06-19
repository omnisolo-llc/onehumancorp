issue_title: "[Research] AI Voice Assistant for Inbound Calls"
issue_description: |
  ## Title
  AI Voice Assistant for Inbound Calls (The Receptionist Agent)

  ## Problem Statement
  Small business owners (like Carlos the handyman or Fatima the food cart owner) miss significant revenue simply because they cannot answer the phone while they are working. A ringing phone during a busy rush or while on a ladder is an interruption, but a missed call is often a lost customer. Traditional voicemail is a dead end for modern consumers, and hiring an answering service or dedicated receptionist is prohibitively expensive for a solopreneur or micro-SME.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional Options:** Voicemail (low conversion, frustrates users), Answering Services (expensive, lack deep integration with business systems).
  - **Emerging AI Voice (e.g., Bland AI, Vapi, Retell, 11x.ai):** Rapidly maturing, capable of near-human latency and natural conversation. However, these are APIs for developers, not turnkey solutions for non-technical owners.
  - **SMB Platform Landscape:**
    - **Shopify/Wix:** Do not natively offer inbound voice reception. They focus purely on digital channels.
    - **Square/GoDaddy:** Basic call routing or auto-attendant ("press 1 for hours"), but no conversational agent capable of taking orders or booking appointments natively.
  - **OHC Opportunity:** Integrate an inbound AI voice agent ("The Receptionist") that natively understands the owner's calendar, inventory, and FAQs. It can answer the phone, answer questions (e.g., "Are you open?", "Do you have vegan cake?"), and most importantly, perform actions like booking an appointment or taking a pre-order directly over the phone, updating the unified OHC system in real-time.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inbound Phone Call] -->|Twilio/SignalWire| B(Voice Gateway)
      B --> C[Speech-to-Text STT]
      C --> D[The Receptionist Agent - LLM]
      D -->|Query Context| E[Unified Graph DB: Calendar/Inventory/FAQs]
      D --> F[Text-to-Speech TTS]
      F --> B
      D -->|Action| G[Booking/Order Engine]
      G --> H[Agent Feed / Unified Inbox]
  ```

  ### Mobile UX Flow (375px First)
  - **Setup:** A simple toggle in the OHC app: "Enable AI Receptionist". The owner claims a local phone number with one tap.
  - **Configuration:** A clean screen to adjust the agent's behavior: "Primary Goal" (e.g., Book Appointment, Take Message, Take Order) and "Tone" (e.g., Professional, Friendly).
  - **Interaction Outcome:** When a call is completed, an Action Card appears in the Agent Feed: "The Receptionist booked an appointment with John for tomorrow at 2 PM. Call summary and audio available."

  ### AI Agent Integration Points
  - **The Receptionist Agent:** Real-time conversational AI. Must have extremely low latency. Connects to the event mesh to perform CRUD operations on bookings/orders based on voice intent.
  - **Handoff:** If the AI encounters a complex issue, it gracefully takes a message, routes it to the unified inbox, and notifies the owner via the Agent Feed.

  ### Key Design Decisions
  - **Latency is King:** STT -> LLM -> TTS pipeline must be optimized for < 800ms response times to feel natural.
  - **Deep Integration:** The agent isn't just a smart FAQ bot; it must have transactional capability (booking, ordering) tightly coupled with the OHC backend.
  - **Transparency:** The system must provide full transcripts and audio recordings of every call in the unified inbox.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (like Carlos), I can turn on my AI Receptionist with one tap. When I am under a sink fixing a pipe and a customer calls, the AI answers, accesses my live calendar, books an estimate for tomorrow morning, and sends me a summary card in my feed. I never miss a lead again.
  **Implementation Goal:** Build the voice gateway and initial agent logic to handle inbound calls, routing audio via Twilio/Vapi (or similar), processing it through the LLM with access to the tenant's context, and generating a voice response, culminating in a synthesized "Call Summary" event in the Agent Feed.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

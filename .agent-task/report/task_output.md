issue_title: "Build the Invisible Multilingual Voice Receptionist Engine"
issue_description: |
  # Architectural Brief: Invisible Multilingual Voice Receptionist Engine

  ## Problem Statement
  For many small business owners like Carlos (a handyman with his hands literally full under a sink) or Fatima (busy cooking halal food in her food cart), answering phone calls immediately is physically impossible. Missed calls mean missed revenue, and playing phone tag creates a poor customer experience. Existing voicemail solutions are passive and frustrating for the caller. They need an intelligent, invisible "receptionist" that speaks multiple languages, never misses a call, understands their business context, can answer common questions ("Do you fix leaky pipes?", "Are you open right now?"), and seamlessly texts the customer a follow-up link (e.g., to book an appointment or view a menu) immediately after the call.

  ## Research Report
  **Market Gap & Competitor Analysis:**
  - **Shopify / Wix / Squarespace:** Highly focused on web storefronts. They have no native voice/telephony integrations. They rely on third-party apps for phone support which are often complex enterprise PBX systems or simple call-forwarding.
  - **GoDaddy:** Offers a "SmartLine" feature which provides a second number and basic voicemail transcription, but no interactive AI agent capable of conversational problem solving or multi-lingual support.
  - **Small Business Needs:** Research shows a significant percentage of high-intent local services and food orders are initiated via phone calls. Voice is the lowest-friction channel for a buyer but the highest-friction channel for a busy sole proprietor.
  - **The Opportunity:** Offering an out-of-the-box, zero-config AI voice receptionist that automatically routes calls to a localized AI, answers queries using the business's data, and converts calls to SMS workflows (bookings, orders) directly positions OneHumanCorp as an operational partner, not just a website builder.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Calling via Phone] -->|PSTN/VoIP| B(Edge Telephony Gateway)
      B --> C{Zero Trust Multi-Tenant Router}
      C --> D[Sub-500ms Streaming Audio Buffer]
      D --> E(Voice AI Engine - STT/TTS)
      E <--> F[Conversational AI Agent]
      F <--> G[(Tenant Knowledge Graph / Memory)]
      F -->|Intent: Booking/Quote| H(SMS / Action Orchestrator)
      H --> I[Customer Phone - SMS Link]
      H --> J[Business Owner Unified AI Inbox]
  ```

  ### UI Wireframes & Screen Flow (Mobile-First 375px)
  - **Settings Screen ("Grandmother Test" Approved):**
    - **Header:** "Phone Receptionist" (macOS-style Translucent Glass header)
    - **Toggle:** A large, friendly toggle switch labeled "Let AI answer my missed calls".
    - **Language Selection:** "What languages do your customers speak?" (Pill-shaped selectors: English, Spanish, Arabic, etc.)
    - **Knowledge Card:** "What should the receptionist know?" with a simple text box that prepopulates from the business profile (e.g., "I'm Carlos, I charge $50/hr, I don't do electrical work").
    - **Hidden Advanced Settings:** A subtle "Advanced" button reveals K8s/WebRTC config, SIP trunking details, and detailed prompt overrides.

  ### Mobile UX Flow
  1. **Activation:** User taps one toggle to turn on the AI Receptionist. The system provisions a local proxy number or sets up call forwarding automatically.
  2. **The Call:** Customer calls, the agent answers in < 1 ring if the owner is busy. Agent speaks naturally, handles the inquiry, and says, "I'll text you a link to book Carlos."
  3. **The Follow-Up:** Customer receives an SMS with an immediate action link.
  4. **The Notification:** The owner receives a push notification on their phone: "AI Receptionist booked a new job with John for Tuesday. [View Details]".

  ### AI Agent Integration Points
  - **Customer Service Department:** The agent must pull from the CS knowledge base (business hours, pricing, services).
  - **Operations Department:** The agent coordinates with the calendar/inventory system to offer real-time availability.
  - **Memory/Context:** Conversation transcripts are summarized and pushed to the unified AI inbox for the owner to review later.

  ### Key Design Decisions
  - **Sub-500ms Latency:** Voice interactions must feel real-time. We must use streaming STT/TTS and edge-deployed inference where possible.
  - **Zero Trust Multi-Tenancy:** Voice data and transcripts must be strictly isolated per tenant using SPIFFE/SPIRE identity propagation from the telephony edge down to the database.
  - **No Implementation Prescriptions:** The specific models (e.g., Whisper, ElevenLabs, or proprietary) and telephony providers (e.g., Twilio, Plivo) are left to the implementer to evaluate and integrate.
  - **SMS Handoff:** Voice is great for context, but poor for complex data entry. The system must bias towards "SMS Handoff" to move the user to a high-converting mobile web view for final checkout/booking.

  ## Implementation Prompt
  **Context:** We need to implement the "Invisible Multilingual Voice Receptionist Engine" for OneHumanCorp.
  **Outcome:** A business owner can toggle on an AI receptionist that answers missed calls, speaks to customers naturally in their preferred language, answers basic questions based on the business profile, and automatically texts the customer a link to book a service or place an order.
  **CUJ (Critical User Journey):**
  1. Maya (baker) turns on the AI receptionist.
  2. A customer calls while Maya is baking and asks for a custom vegan cake.
  3. The AI answers, confirms vegan cakes are available, and texts the customer the custom order form link.
  4. Maya sees the transcribed summary and the completed order in her inbox later.
  **Acceptance Criteria:**
  - The feature must be enabled via a single toggle on a mobile UI (375px optimized).
  - The voice interaction latency must be perceived as natural (target sub-500ms response).
  - The system must support at least English, Spanish, and Arabic natively without manual configuration per language.
  - Transcripts and resulting actions must be strictly isolated to the correct tenant's inbox.
  - Provide a robust testing strategy for voice interactions.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

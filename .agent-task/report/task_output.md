issue_title: "AI Voice Receptionist Engine"
issue_description: |
  # Research Report: AI Voice Receptionist Engine

  ## Problem Statement
  Small business owners—especially service providers like Carlos (handyman, 42) and Leo (music tutor, 22), as well as busy food cart operators like Fatima (50)—miss countless opportunities because they cannot answer the phone while working. Current platforms offer email and chat automation, but completely neglect the most traditional and urgent communication channel: voice. Our users need an invisible assistant that answers the phone 24/7, handles standard inquiries in multiple languages, books appointments, and escalates complex queries via SMS.

  ## Research Report
  Our competitive analysis reveals a significant blind spot in major SMB platforms regarding inbound phone call automation.
  - **Shopify:** Primarily focuses on eCommerce and chat/email support integrations. Voice is non-existent.
  - **Wix:** Offers a robust booking system and basic chat, but no native inbound voice capabilities.
  - **Squarespace:** Completely lacks real-time communication tools, including voice.
  - **GoDaddy:** Provides unified inbox features and virtual phone numbers, but no autonomous AI voice agent.

  *Market Gap:* Small businesses cannot afford $100+/month for human answering services. They need an integrated solution where the AI understands their inventory, pricing, and calendar, and can converse naturally with customers over the phone.

  ## Design Doc
  The AI Voice Receptionist Engine bridges the physical telecommunication network and our KAIROS orchestration engine, translating voice to intent, and intent to action within our existing multi-tenant architecture.

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Caller[Customer Phone] -->|PSTN / SIP| TelephonyGateway[Telephony Ingress Edge];
      TelephonyGateway -->|Audio Stream / WebRTC| AIVoiceAgent[AI Voice Agent Node];
      AIVoiceAgent <-->|Text-to-Speech / Speech-to-Text| STTEngine[Speech & Language Models];

      AIVoiceAgent -->|Intent & Context| KAIROS[KAIROS Orchestration Hub];
      KAIROS -->|Query Catalog/Menu| Inventory[Inventory & Catalog Mesh];
      KAIROS -->|Query Availability| Calendar[Unified Booking Engine];

      KAIROS -->|Action: Book, Quote, Answer| AIVoiceAgent;
      AIVoiceAgent -->|Voice Reply| TelephonyGateway;

      KAIROS -->|Escalation/Summary| UnifiedInbox[Omnichannel AI Inbox];
      UnifiedInbox -->|Push Notification| MobileApp[OHC Mobile App 375px];
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Dashboard Home Card:** A glass-morphic card titled "Your AI Receptionist" showing status ("Active") and today's stats ("3 Calls Handled, 1 Appointment Booked").
  2. **Setup Wizard Flow:**
     - *Screen 1 (Number Selection):* "Choose your business phone number" or "Port your existing number."
     - *Screen 2 (Personality & Language):* Sliders for tone (Professional vs. Casual) and toggles for supported languages (e.g., English, Spanish, Arabic).
     - *Screen 3 (Knowledge Base):* Toggles mapping the agent to business data ("Access my Calendar", "Access my Menu/Pricing", "Access my FAQ").
  3. **Unified Inbox Integration:**
     - When a call finishes, a transcript card appears in the Unified Inbox.
     - A short, plain-language summary is provided: "New call from +1-555-0198. The AI booked them for a plumbing quote on Tuesday at 2 PM. No action needed."
     - If the AI cannot handle the request, it flags as "Needs Your Attention" with an unread indicator and an option to instantly text or call back.

  ### AI Agent Integration Points
  - **KAIROS Orchestrator:** Validates multi-tenant context and routes queries to the correct internal agents.
  - **AutoDream Pipelines:** Consolidates successful call outcomes into the business's long-term memory.
  - **Language Models:** Relies on low-latency STT/TTS pipelines, adhering to the Zero Trust identity model to ensure tenant isolation.

  ### Key Design Decisions
  - **Zero-Touch Knowledge Integration:** The Voice Agent does not require a separate knowledge base setup. It dynamically reads from the existing Catalog, Menu, and Calendar data structures.
  - **Asynchronous Handoff:** All intense audio processing happens on the edge/cloud. The mobile app only receives lightweight JSON summaries.
  - **Fail-Safe Escalation:** If STT confidence is low, the system plays a courteous callback message and terminates the voice session, pushing an urgent notification to the owner's Unified Inbox.

  ## Implementation Prompt
  **Prompt for Implementer Agent:**
  Design and implement the foundational backend services and frontend UI for the "AI Voice Receptionist Engine."

  **User-Facing Outcome:**
  A small business owner can activate an AI phone number from their mobile app with one tap. When customers call this number, a natural-sounding AI answers, answers basic questions based on the business's data, books appointments, or takes orders. The owner receives a text summary of the call in their Unified Inbox and only needs to intervene for complex requests.

  **Core User Journeys (CUJ):**
  1. User activates the voice receptionist and selects supported languages via a clean, glass-morphic card interface on their phone.
  2. A customer calls the number and asks, "Are you open today?" and the AI accurately responds based on store hours.
  3. A customer calls to book a service; the AI checks availability, books the slot, and the owner gets a notification: "New booking secured via phone."

  **Acceptance Criteria:**
  - Create the edge telephony ingress routing mechanism capable of streaming audio to an AI processing node.
  - Integrate the Voice Agent with the KAIROS Orchestrator so it can query availability and catalog data securely within the tenant boundary.
  - Build the 375px mobile-first configuration screens using our established design tokens (macOS glass, modular cards).
  - Ensure all call transcripts and summaries are routed to the Omnichannel AI Inbox.
  - Guarantee strict multi-tenant isolation so one business's Voice Agent cannot access another's data.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
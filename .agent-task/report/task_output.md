issue_title: "AI-Native Multilingual Voice Receptionist"
issue_description: |
  # Problem Statement
  Small business owners like Carlos (handyman) and Fatima (food cart) frequently miss phone calls while they are actively working with their hands. Missing a call often means missing a high-value job or a catering order. Traditional voicemails are frustrating for customers and require the owner to spend hours returning calls at the end of the day. A zero-configuration, multilingual AI voice receptionist is needed to instantly answer calls, answer specific business questions, take messages, book appointments, and process pre-orders natively integrating with OHC's inventory and calendar in real-time.

  # Research Report
  *   **Current Architecture Limits:** OHC currently relies on asynchronous text-based communication (AI Inbox, DMs, SMS). Voice communication requires a completely different real-time streaming architecture (SIP/WebRTC, low-latency STT/TTS).
  *   **Competitor Analysis:**
      *   *Shopify:* Does not offer native voice receptionist capabilities.
      *   *Wix / Squarespace:* Rely on third-party app integrations (like standalone answering services) which do not have real-time, read/write access to the merchant's live inventory or booking calendar.
      *   *Standalone AI Voice APIs (Twilio, Vapi, Bland AI):* Excellent primitives, but require complex developer setup to map to business data. They lack zero-config integration into a multi-tenant business ledger.
  *   **Discovery:** OHC needs a real-time Voice Gateway that connects SIP trunks directly to an AI Voice Agent department. The agent must have instant, edge-cached access to the merchant's Knowledge Base, Calendar, and Menu/Inventory to conduct meaningful conversations (e.g., "Yes, I have availability tomorrow at 2 PM. Shall I book that?").

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  erDiagram
      CUSTOMER-PHONE ||--o{ SIP-TRUNK : "Makes Call"
      SIP-TRUNK ||--o{ OHC-VOICE-GATEWAY : "Streams Audio"
      OHC-VOICE-GATEWAY ||--o{ VOICE-AI-AGENT : "WebSockets (STT/LLM/TTS)"
      VOICE-AI-AGENT ||--o{ EDGE-CACHE : "Reads Calendar/Inventory/KB"
      VOICE-AI-AGENT ||--o{ CORE-LEDGER : "Creates Booking/Order"
      CORE-LEDGER ||--o{ MULTI-TENANT-DB : "Strict Tenant Isolation"
  ```

  ## UI Wireframes & Mobile UX Flow (375px)
  *   **Customer View (Voice Call):** Customer calls the business phone number. The AI answers within 1 second in the business owner's chosen persona/voice and language (e.g., Arabic for Fatima's cart).
  *   **Merchant View (OHC Mobile App - 375px):**
      *   **Unified Inbox Voice Card:** A modular card in the Unifi style showing a missed call or handled call. Includes a beautiful Translucent Glass waveform summary.
      *   **Actionable Transcript:** Opening the card shows a short text summary of the call (e.g., "Booked a plumbing repair for Tuesday 10 AM") and the full transcript.
      *   **Configuration Screen (Grandmother Test Passed):** A simple toggle switch: "AI Voice Receptionist: ON/OFF". Underneath, a single button "Test my AI Receptionist" which initiates a test call to the merchant's own phone. No complex prompt engineering or API keys visible.

  ## Mobile UX Flow
  1. User taps "Enable Voice Receptionist" from the main dashboard.
  2. The app assigns a local virtual number (or provisions call forwarding).
  3. The AI automatically compiles a knowledge base from the existing OHC profile (menu, hours, services, prices).
  4. Any calls handled by the AI generate a native push notification to the merchant: "AI booked a new appointment." with a single tap to view details.

  ## AI Agent Integration Points
  *   **CS Department Agent:** Handles answering FAQs and general inquiries.
  *   **Operations Agent:** Handed off from CS to check real-time inventory or available calendar slots.
  *   **Finance Agent:** Handled implicitly if a deposit is required (triggers an SMS checkout link to the caller's phone number during the call).

  ## Key Design Decisions and Why
  *   **Edge-Cached Knowledge:** To maintain sub-500ms conversational latency, the merchant's business context must be cached close to the voice processing node.
  *   **Zero-Config Prompts:** We completely hide prompt engineering. The system autonomously derives the system prompt from the structured OHC business profile.
  *   **Multilingual by Default:** Fatima needs the AI to handle Arabic and English seamlessly without her explicitly defining language rules.

  # Implementation Prompt
  **Objective:** Implement the real-time AI Voice Gateway and the corresponding mobile UI to allow a merchant to enable the AI Voice Receptionist with a single tap.
  **CUJ:** Carlos (Handyman) enables the receptionist. A customer calls, asks for plumbing rates, and books a time. Carlos receives a push notification confirming the booking, without ever touching his phone during the job.
  **Acceptance Criteria:**
  - The mobile UI uses Translucent Glass aesthetics and matches the 375px modular card design.
  - The Voice Gateway can accept a simulated SIP/WebSocket audio stream, process it via the AI agent using the merchant's specific data context, and return audio.
  - Tenant data isolation is strictly enforced; an AI agent answering for Store A cannot access Store B's calendar.
  - Successful call handling results in a structured event in the Unified Inbox.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

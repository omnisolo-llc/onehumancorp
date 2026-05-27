issue_title: "Autonomous Voice AI Receptionist Engine"
issue_description: |
  # Title: Autonomous Voice AI Receptionist Engine

  ## Problem Statement
  Small business owners, such as Carlos (handyman, 42) and Fatima (food cart, 50), operate in hands-on environments where they cannot always answer the phone. Missed calls result in missed leads and lost revenue. Existing solutions involve setting up complex PBX systems or paying for expensive human answering services. Small business owners need an out-of-the-box, simple solution that can handle incoming calls, answer basic FAQs, take messages, and even schedule appointments autonomously, all configurable from their mobile device with zero technical expertise.

  ## Research Report
  *   **Market Need:** A significant percentage of local service and food businesses rely on inbound phone calls for customer acquisition and order placement. Missed calls are a direct loss of income.
  *   **Competitor Landscape:**
      *   **Shopify:** Focuses primarily on online retail; lacks native voice/telephony integrations.
      *   **Wix/Squarespace:** Offer basic forms and email integrations but no out-of-the-box voice AI capabilities.
      *   **GoDaddy:** Provides simple business numbers but lacks AI-driven autonomous call handling.
  *   **OHC Advantage:** By integrating an Autonomous Voice AI Receptionist natively into the platform, OHC can provide a unique value proposition that directly impacts a business's bottom line. This aligns perfectly with the "invisible teammate" model, where the AI acts as a dedicated receptionist, freeing the owner to focus on their craft.
  *   **Persona Impact:**
      *   **Carlos (Handyman):** The AI can answer calls while he's on a job, capture details about a repair request, and schedule a callback or quote visit.
      *   **Fatima (Food Cart):** The AI can handle simple questions like "Are you open today?" or "What's the special?", allowing her to focus on cooking and serving in-person customers.
      *   **Leo (Music Tutor):** The AI can answer calls from prospective students, provide pricing information, and schedule introductory lessons based on his calendar availability.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Caller[Customer Phone] -->|Inbound Call| TelephonyGateway(Telephony Gateway - Twilio/Plivo)
      TelephonyGateway -->|Audio Stream| STT[Speech-to-Text Engine]
      STT -->|Text Transcript| NLU[NLU / Intent Router]

      NLU -->|FAQ Intent| KBE[Knowledge Base Engine]
      KBE -->|Business Info| TTS[Text-to-Speech Engine]

      NLU -->|Booking Intent| SchedulingAgent[Scheduling Agent]
      SchedulingAgent -->|Check Availability| Calendar[Calendar API]
      Calendar -->|Available Slots| TTS

      NLU -->|Message Intent| MessagingAgent[Messaging Agent]
      MessagingAgent -->|Transcribe & Save| UnifiedInbox[Unified Inbox]

      TTS -->|Audio Stream| TelephonyGateway
      TelephonyGateway -->|Voice Response| Caller

      UnifiedInbox -.->|Push Notification| OHC_App[OHC Mobile App]
  ```

  ### UI Wireframes (375px First)

  **Screen 1: Receptionist Settings (Mobile View)**
  *   **Header:** "AI Receptionist"
  *   **Toggle:** Enable/Disable Receptionist (Switch)
  *   **Card 1: Business Profile:**
      *   "What should the receptionist say when it answers?" (Text Input: e.g., "Hi, thanks for calling Carlos Handyman Services. How can I help you today?")
  *   **Card 2: FAQ Knowledge Base:**
      *   "Add common questions and answers"
      *   List of FAQs (e.g., "What are your hours?", "Do you do free quotes?")
      *   Button: "+ Add New Question"
  *   **Card 3: Call Routing:**
      *   "If it's an emergency, forward call to:" (Input: Mobile Number)
      *   "Otherwise, take a message."

  **Screen 2: Unified Inbox - Call Log**
  *   **Header:** "Inbox"
  *   **List Item:**
      *   Icon: Phone Call (Missed/Handled)
      *   Contact Name or Number
      *   Time
      *   Snippet: "Hi, I need a leaky pipe fixed..."
  *   **Action Sheet (on tap):**
      *   Play Audio Recording
      *   Read Full Transcript
      *   Button: "Call Back"
      *   Button: "Mark as Resolved"

  ### Mobile UX Flow
  1.  **Onboarding:** Upon activating the AI Receptionist feature, the user is guided through a conversational setup asking for their business name, basic hours, and a few common questions customers ask.
  2.  **Activation:** The system provisions a dedicated business phone number (or allows porting an existing one) and instantly activates the AI agent.
  3.  **Operation:** When a call comes in, the AI answers, interacts with the caller using natural language, and determines the intent (FAQ, booking, message).
  4.  **Notification:** If a message is taken or an appointment is booked, the owner receives an instant push notification on the OHC app.
  5.  **Review:** The owner can review call transcripts and recordings in the Unified Inbox at their convenience.

  ### AI Agent Integration Points
  *   **Telephony Gateway:** Integration with services like Twilio or Plivo for SIP trunking and number provisioning.
  *   **STT/TTS Engines:** Utilizing low-latency Speech-to-Text and Text-to-Speech models (e.g., Deepgram, ElevenLabs, or native provider APIs) for natural conversational flow.
  *   **NLU Routing:** A lightweight LLM acts as the router to classify intent and direct the flow to the appropriate specialized agent (FAQ, Booking, Messaging).
  *   **Unified Inbox Synchronization:** Ensuring seamless integration with the existing OHC messaging infrastructure for centralized communication management.

  ### Key Design Decisions
  *   **Simplicity over Customization:** The focus is on a functional default configuration that works out-of-the-box, minimizing the need for complex PBX-style routing rules.
  *   **Natural Language Interfaces:** Configuration is driven by conversational prompts rather than technical forms, making it accessible to non-technical users.
  *   **Mobile-First Management:** All aspects of the receptionist (setup, review, disabling) must be easily accessible from the primary OHC mobile application.
  *   **Fail-Safe Routing:** Always providing a fallback option (e.g., taking a message or forwarding to a human) if the AI cannot handle the caller's request confidently.

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the foundational architecture for the Autonomous Voice AI Receptionist Engine. The goal is to create a system that can handle inbound phone calls, determine caller intent, provide basic FAQ responses based on the business profile, and record messages, seamlessly integrating with the OHC Unified Inbox.

  **Acceptance Criteria:**
  1.  **Provisioning:** The system must be able to provision a virtual phone number associated with a specific OHC tenant.
  2.  **Call Handling Pipeline:** Implement a reliable pipeline to receive an inbound call stream, convert it to text (STT), process intent via an NLU router, and generate a voice response (TTS).
  3.  **FAQ Intent:** The system should successfully answer basic questions (e.g., business hours, location) by referencing a configured knowledge base.
  4.  **Message Intent:** If the AI cannot resolve the query or the user wants to leave a message, it should record the audio, generate a transcript, and store it as a new thread in the tenant's Unified Inbox.
  5.  **Mobile Settings Mockup:** Create the data structures necessary to support the mobile UI wireframes (e.g., endpoints to update the greeting, manage FAQs, and set forwarding numbers).
  6.  **Performance:** The STT -> NLU -> TTS roundtrip latency must be minimized to ensure a natural conversational experience.

  ## Priority
  P1 (High - Critical differentiator for service-based businesses)

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

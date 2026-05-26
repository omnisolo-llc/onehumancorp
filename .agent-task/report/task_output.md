issue_title: "[Architecture] Autonomous Voice Receptionist and Dispatch Engine"
issue_description: |
  # [Architecture] Autonomous Voice Receptionist and Dispatch Engine

  ## Title
  Build the Autonomous Voice Receptionist and Dispatch Engine

  ## Problem Statement
  Service-based solopreneurs (like Carlos the handyman) and high-volume food vendors (like Fatima the food cart owner) face a constant dilemma: answering the phone interrupts their work, but missing a call means losing a sale. Traditional voicemail is a "black hole" where leads go to die. Small business owners cannot afford a human receptionist, and existing IVR (Interactive Voice Response) systems ("Press 1 for hours") are impersonal, rigid, and too technical to set up. They need an intelligent, invisible employee who can answer calls 24/7, converse naturally in multiple languages, answer FAQs, take orders, and book appointments—all without requiring any code or manual configuration.

  ## Research Report
  *   **The Voice Gap:** Despite the shift to digital, a significant portion of SMB revenue (especially in local services and food) still originates from voice calls.
  *   **Competitor Analysis:**
      *   **Shopify:** No native inbound voice capabilities. Focuses purely on digital e-commerce channels.
      *   **Wix & Squarespace:** No native voice functionality. Rely on third-party app integrations that require complex setup and separate billing.
      *   **GoDaddy:** Offers basic virtual number routing and standard voicemail, but lacks conversational AI or deep business integration.
  *   **OHC Opportunity:** By integrating real-time voice capabilities directly into the OneHumanCorp (OHC) platform, we can completely eliminate the "missed call" problem. The AI Receptionist acts as a seamless extension of the business, possessing full context of the owner's schedule, inventory, and FAQs.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Caller
      participant VoiceGateway as Voice/PSTN Gateway
      participant KAIROS as KAIROS Orchestrator
      participant CSAgent as Customer Success Agent
      participant OpsAgent as Operations/Sales Agent
      participant OHCInbox as OHC Unified Inbox

      Caller->>VoiceGateway: Inbound Call
      VoiceGateway->>KAIROS: WebRTC / Audio Stream Initiation
      KAIROS->>CSAgent: Forward Audio Stream & Business Context
      CSAgent->>Caller: Conversational Greeting (TTS)
      Caller->>CSAgent: Speech Input (STT)
      CSAgent->>OpsAgent: Query Availability/Inventory
      OpsAgent-->>CSAgent: Real-time status (e.g., "Available at 2 PM")
      CSAgent->>Caller: Propose action (e.g., "Would you like me to book that?")
      Caller->>CSAgent: Confirmation
      CSAgent->>OpsAgent: Execute Transaction (Book/Order)
      CSAgent->>VoiceGateway: End Call
      KAIROS->>OHCInbox: Log Transcript, Summary & Action
      OHCInbox-->>Owner (Mobile): Push Notification ("New Appointment Booked via Call")
  ```

  ### Mobile UX Flow (375px First)
  1.  **Activation:** The owner navigates to the "Communications" tab on their OHC mobile app.
  2.  **Toggle:** A simple, premium "macOS-style" translucent card displays: "AI Receptionist: Off". The owner taps to toggle it "On".
  3.  **Knowledge Base Injection:** A sub-card expands asking, "What should I know?" The owner can type plain-text instructions: *"If they ask about emergency plumbing, say it costs $150 minimum and I can be there in 2 hours."*
  4.  **The Result (Unified Inbox):** When a call is completed, the owner receives a push notification. Tapping it opens the Unified Inbox, displaying the contact's name (extracted from the call), a brief summary (*"Customer needs a leaky pipe fixed today"*), the full transcript, and the action taken (*"Scheduled for 3:00 PM"*).

  ### Key Design Decisions
  *   **Multi-tenant Isolation:** Voice streams and processed transcripts must be strictly isolated per tenant using SPIFFE/SPIRE identity propagation.
  *   **Sub-second Latency:** The conversational engine must prioritize ultra-low latency for natural interaction, leveraging edge-deployed models where possible.
  *   **Agent Handoff:** The KAIROS Orchestrator manages the handoff between the Customer Success Agent (handling conversation) and the Operations/Sales Agent (checking inventory/calendar) seamlessly.
  *   **Graceful Degradation:** If the AI encounters a scenario it cannot handle confidently, it will politely take a message and route the transcript to the owner's inbox as a high-priority task.
  *   **Visual Excellence:** All configuration must pass the "Grandmother Test." No developer terminology (like "SIP," "WebRTC," or "STT/TTS") will be exposed in the UI.

  ## Implementation Prompt
  Design and implement the Autonomous Voice Receptionist engine for OneHumanCorp. Provide a seamless experience where an owner can activate an AI-driven virtual phone number with a single tap.

  **Core User Journey (CUJ):**
  1. The business owner toggles the "AI Receptionist" feature on in their mobile app.
  2. A customer calls the provided business number.
  3. The AI answers, converses naturally, and fulfills the customer's intent (e.g., answering a question about business hours, or booking an available time slot).
  4. Upon call completion, a structured summary and transcript are injected into the OHC Unified Inbox, notifying the owner of any actions taken.

  **Acceptance Criteria:**
  *   The feature must be manageable entirely from a mobile viewport (375px) without complex configuration screens.
  *   The underlying architecture must handle real-time audio streams with low conversational latency.
  *   The engine must integrate deeply with existing OHC data models (availability, inventory, business context) to perform actions on behalf of the owner.
  *   Ensure strict multi-tenant data boundaries for audio processing and transcripts.
  *   Do not prescribe specific STT/TTS vendors or database schemas; focus on the robust orchestration and seamless user-facing outcome.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

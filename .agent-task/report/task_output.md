issue_title: "[Architecture] Autonomous AI Voice Receptionist & Multilingual Dispatch Engine"
issue_description: |
  ## Title
  Autonomous AI Voice Receptionist & Multilingual Dispatch Engine

  ## Problem Statement
  Small business owners (SMBs) miss out on 30-50% of potential revenue simply because they cannot answer the phone while working.
  - **Carlos (handyman, 42)** is up on a ladder or under a sink. When a potential customer calls for an emergency repair, it goes to voicemail, and the customer immediately calls the next handyman on Google.
  - **Fatima (food cart, 50)** is dealing with a rush of in-person customers. She cannot take phone calls for pre-orders simultaneously, especially given her limited English fluency.
  - **Maya (baker, 28)** has her hands covered in dough and cannot answer inquiries about custom cake availability.

  These businesses need a system that ensures **zero missed revenue** from phone calls. They do not need a traditional PBX or a complex call routing system. They need a human-sounding AI receptionist that can instantly answer calls in multiple languages, check real-time availability/inventory, take pre-orders or book appointments, and immediately dispatch a confirmation/payment link via SMS.

  ## Research Report
  **Market Deficiencies:**
  - **Traditional SaaS (RingCentral, Dialpad)**: Geared towards enterprise call centers, incredibly complex to configure (IVR trees, SIP routing), and disconnected from the SMB's core commerce engine.
  - **Horizontal AI Voice Tools (Bland, Vapi)**: Extremely powerful but require developer knowledge to set up webhooks, configure LLM prompts, and integrate with booking systems or payment gateways.
  - **SMB Platforms (Shopify, Wix)**: Completely lack native voice capabilities. They rely entirely on web interfaces and text.

  **The OneHumanCorp (OHC) Opportunity:**
  By deeply integrating a low-latency Voice AI engine (e.g., Daily/Vapi/Twilio) directly into OHC’s multi-tenant architecture, we can offer an "Instant AI Receptionist" that activates with one tap. Because OHC already controls the Universal Capacity & Inventory Ledger and the AI Quoting Engine, the voice agent has instant, Zero-Trust access to the business's real-time truth.
  - **Latency Target**: Sub-500ms voice-to-voice response time to feel completely natural.
  - **Multilingual Support**: Real-time conversational ability in English, Spanish, Arabic, etc., lowering language barriers for owners like Fatima.
  - **Omnichannel Dispatch**: The agent answers the call, completes the intent, and instantly dispatches an SMS via Twilio to the caller with a deep link to complete the deposit or view the quote.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      CALLER ||--o{ INBOUND_CALL : places
      INBOUND_CALL ||--|| VOICE_SESSION : creates

      VOICE_SESSION {
          string session_id
          string tenant_id
          string caller_phone
          datetime started_at
          string active_language
          string session_status
      }

      VOICE_SESSION ||--|| KAIROS_ORCHESTRATOR : streams_intent

      KAIROS_ORCHESTRATOR ||--o{ TENANT_LEDGER : queries
      KAIROS_ORCHESTRATOR ||--o{ SMS_DISPATCHER : triggers

      TENANT_LEDGER {
          string resource_id
          string type
          int available_capacity
      }

      SMS_DISPATCHER {
          string message_id
          string recipient
          string payment_link
      }
  ```

  ### UI Wireframes (375px Mobile-First)
  **Screen: AI Receptionist Dashboard (macOS-style Translucent Glass + UniFi layout)**
  ```text
  [  < Back              Receptionist       (i)  ]
  [----------------------------------------------]
  [                                              ]
  [  [ Toggle: Enable AI Voice ] (ON - Green)    ]
  [                                              ]
  [  +-- Voice Settings -----------------------+ ]
  [  | Persona: "Friendly & Professional"  [>] | ]
  [  | Voice: Maya (Female)                [>] | ]
  [  | Primary Lang: English & Spanish     [>] | ]
  [  +-----------------------------------------+ ]
  [                                              ]
  [  +-- Agent Capabilities -------------------+ ]
  [  | [x] Book Appointments (Calendar Sync)   | ]
  [  | [x] Take Food Pre-Orders (Menu Sync)    | ]
  [  | [x] Answer FAQs & Business Hours        | ]
  [  | [ ] Negotiate Custom Quotes             | ]
  [  +-----------------------------------------+ ]
  [                                              ]
  [  +-- Recent Calls -------------------------+ ]
  [  | 10:42 AM - (555) 123-4567               | ]
  [  | -> Booked "Plumbing Repair" via AI      | ]
  [  | -> Deposit SMS Sent                     | ]
  [  |                                         | ]
  [  | 09:15 AM - (555) 987-6543               | ]
  [  | -> Answered "Are you open today?"       | ]
  [  +-----------------------------------------+ ]
  [                                              ]
  ```

  ### Mobile UX Flow
  1. **Activation**: Carlos toggles "Enable AI Voice" from his OHC app home screen. OHC automatically provisions a local Twilio phone number linked to his OHC tenant profile.
  2. **Configuration**: Carlos selects a pre-configured persona ("Friendly Handyman") and links it to his OHC Calendar. No code or prompt engineering required.
  3. **The Call Journey**:
     - A customer calls the provisioned number.
     - The call is routed to the Voice Session engine.
     - The AI greets the customer: "Hi, you've reached Carlos' Repairs. Carlos is on a job right now, but I can help you book an appointment or give you a quote. What do you need?"
     - The AI securely queries the `TENANT_LEDGER` to find available times.
     - The AI confirms the slot and triggers the `SMS_DISPATCHER`.
     - The customer receives a text: "Click here to pay your $50 deposit and confirm your 2 PM slot with Carlos: [Link]"
  4. **Post-Call**: Carlos receives an instant push notification on his Android phone: "New Booking at 2 PM. Deposit paid."

  ### AI Agent Integration Points
  - **Customer Service (CS) Agent**: Handles the inbound WebRTC/SIP audio stream, performing rapid STT (Speech-to-Text), contextual LLM generation, and TTS (Text-to-Speech).
  - **Operations Agent**: Monitors the real-time intent stream from the CS Agent. If the customer asks for a time slot, Operations checks the Unified Calendar Ledger and places a temporary hold on the slot.
  - **Finance Agent**: Triggered immediately upon call completion if a deposit is required, generating the Zero-Touch invoice and dispatching the SMS link.

  ### Key Design Decisions
  1. **Edge-Terminated Audio**: Audio streams should terminate at edge nodes close to the caller to minimize latency. The backend only handles text/intent streams.
  2. **Strict Data Boundary**: The Voice AI has zero access to other tenants' data. All ledger queries are injected with a verified `organization_id` claim derived from the active phone number mapping.
  3. **Graceful Human Handoff**: If the caller is frustrated or asks for a human, the agent instantly rings Carlos's actual device via a VoIP push notification, bridging the call.

  ## Implementation Prompt
  Design and implement the `AutonomousVoiceReceptionist` service. The implementation must map a provisioned inbound phone number to an OHC `organization_id`. Create the event loop that handles rapid STT/TTS streams while concurrently allowing the agent to query the business's availability ledger and trigger SMS actions. Ensure the UI implementation in the Tauri mobile app provides a one-tap activation toggle and a simple configuration card (no prompt engineering exposed to the user). Focus on multi-tenant security and sub-second response latency.

  **Acceptance Criteria:**
  - The feature is fully functional and configurable from the mobile UI (375px width).
  - Toggling "Enable AI Voice" provisions a phone number and maps it correctly to the tenant's `organization_id`.
  - The voice agent can successfully query the `TENANT_LEDGER` and trigger the `SMS_DISPATCHER`.
  - Voice-to-voice response latency remains under 500ms.
  - Tenant data is strictly isolated; queries enforce the `organization_id` claim.
  - Inbound calls can seamlessly transition to a human handoff via VoIP push notification.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement Multilingual Autonomous Voice Receptionist"
issue_description: |
  **Research Report & Architecture Design**

  ## Problem Statement
  Small business owners, especially those running solo operations like food carts (Fatima) or handyman services (Carlos), constantly miss phone calls while they are actively working (cooking, driving, repairing). Every missed call is a lost customer, a lost order, or a delayed quote. Existing voicemail systems are static, require manual checking, and do not integrate with modern, dynamic business workflows or CRM tools. Non-English speaking owners also face barriers when customers speak a different language, and many automated systems do not natively handle multi-lingual real-time transcription, translation, and action execution.

  ## Research Report
  The problem is well documented across the SMB space:
  - 60% of calls to small businesses go to voicemail.
  - 80% of callers sent to voicemail do not leave a message.
  - Traditional answering services are prohibitively expensive for solopreneurs (often >$150/mo).
  - Platforms like Twilio, Bland AI, and Vapi offer powerful programmable voice APIs, while speech-to-text engines like Whisper and deepgram have achieved near human parity across dozens of languages.
  - Competitors (Shopify, Wix) do not offer native, fully integrated inbound voice agents that can read the real-time calendar, check inventory, and take deposits over the phone via SMS links. This represents a significant capability gap and a core feature needed to close the offline-to-online loop for OneHumanCorp.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      INBOUND_CALL ||--o{ VOICE_SESSION : creates
      VOICE_SESSION ||--o{ AGENT_TRANSCRIPT : records
      VOICE_SESSION ||--o{ INTENT_ACTION : triggers
      INTENT_ACTION ||--o| CALENDAR_BOOKING : can_be
      INTENT_ACTION ||--o| PREORDER_TICKET : can_be
      INTENT_ACTION ||--o| SMS_DISPATCH : sends

      INBOUND_CALL {
          string caller_id
          string to_number
          datetime timestamp
      }
      VOICE_SESSION {
          string session_id
          string detected_language
          string status
          duration length
      }
      AGENT_TRANSCRIPT {
          string transcript_id
          string role
          string content
      }
      INTENT_ACTION {
          string action_id
          string type
          string status
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Caller
      participant Twilio as Telephony Edge (Twilio)
      participant Vapi as Voice AI Engine
      participant OHC as OHC Router
      participant CoreAgent as Core AI Agent
      participant DB as Shared Tenant DB

      Caller->>Twilio: Dials OHC Provided Number
      Twilio->>Vapi: Websocket Audio Stream
      Vapi->>OHC: Inbound Call Webhook (Caller ID)
      OHC->>DB: Lookup Tenant & Customer Profile
      DB-->>OHC: Tenant Context (Inventory, Calendar)
      OHC-->>Vapi: Initialize Agent Context & Prompt
      Vapi-->>Caller: "Hello! This is Fatima's Halal Cart..."
      Caller->>Vapi: "Can I order 2 chicken over rice for pickup?"
      Vapi->>OHC: Tool Call: CheckInventory(Chicken Over Rice, 2)
      OHC->>DB: Query Inventory
      DB-->>OHC: Available
      OHC-->>Vapi: Inventory Confirmed
      Vapi->>OHC: Tool Call: CreatePreorder(Chicken Over Rice, 2)
      OHC->>DB: Insert Preorder Ticket
      OHC->>Twilio: Dispatch SMS with Payment Link
      Vapi-->>Caller: "Got it! I sent you a text with a payment link."
  ```

  ### Mobile UX Flow (375px)
  1. **Dashboard Card:** A prominent card on the main dashboard showing "Missed Calls Handled" today, and active "AI Receptionist" status (Toggle On/Off).
  2. **Call Log View:** Tapping the card opens the Inbox. Voice calls are shown in the same feed as Instagram DMs and emails.
  3. **Call Details:** Tapping a specific call shows a beautiful Translucent Glass card containing:
     - Caller Name (resolved via CRM if known)
     - Audio playback scrubber
     - Full translated transcript (English + Original language)
     - Extracted Action Chips (e.g., "Order placed", "Appointment booked", "Quote requested")
  4. **Configuration Screen:** Simple settings to set the "Voice Style" (Professional, Casual, Friendly) and update the "Knowledge Base" (e.g., inputting today's specials or temporary closures).

  ### Key Design Decisions
  - **Unified Inbox Integration:** Voice calls are not a separate silo; they are integrated into the main omnichannel inbox to provide a single view of the customer.
  - **Multilingual Support:** The AI automatically detects the caller's language and responds in the same language, while translating the summary into the business owner's native language for review.
  - **Action-Oriented:** The agent goes beyond just taking messages. It is equipped with tools to execute actions (bookings, orders) based on real-time business data.
  - **Zero-Trust & Tenant Isolation:** Voice webhook endpoints must strictly validate tenant context and prevent cross-tenant data leakage during tool execution.

  ## Implementation Prompt
  **Task:** Implement the backend routing and frontend UI for the Multilingual Autonomous Voice Receptionist.
  **User Journey:** Maya the baker is covered in flour and cannot answer her phone. A customer calls to ask if she does vegan cakes. The AI answers, confirms she does, quotes a starting price from her catalog, and texts the caller a link to submit a custom order form. Maya sees this interaction summarized in her OHC Inbox as a successful lead.
  **Acceptance Criteria:**
  - Integrate a telephony provider (e.g., Twilio) with a conversational AI engine.
  - Create an inbound webhook handler that authenticates the tenant based on the dialed number.
  - Expose basic tools to the voice agent (Check Catalog, Check Availability, Send SMS).
  - Render voice interactions within the main Inbox UI, including audio playback and text transcripts.
  - Ensure all mobile UI components adhere to the 375px Translucent Glass design standards.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

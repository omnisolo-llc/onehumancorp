issue_title: "[Communications] AI Voice Receptionist Integration via Twilio Voice"
issue_description: |
  ## [Communications] Twilio Voice AI Receptionist Integration

  **Title**: Enable Real-time AI Voice Receptionist for Missed Calls

  **Problem Statement**:
  Small business owners like Carlos (Field Service Owner) and Fatima (Food Cart Operator) frequently miss phone calls because their hands are full, they are driving, or they are serving a customer. A missed call often means a lost lead or a frustrated customer asking a simple question like "Are you open?" or "Where are you located?". They need a receptionist that never sleeps and can answer calls, provide basic information, and capture leads seamlessly.

  **Research Report**:
  - **Market Demand:** Voice remains a critical channel for local SMBs (e.g., plumbers, food carts). Many leads still prefer to call rather than text or email.
  - **Competitor Landscape:** While platforms like Square have basic voicemail or automated text-back, native AI voice agents that converse naturally are a massive differentiator.
  - **Tool Capabilities (Twilio Voice + LLMs):** Twilio Programmable Voice allows receiving calls. When combined with Twilio Media Streams and a real-time LLM (like OpenAI Realtime API or Gemini Live), OHC can build a conversational voice agent. Twilio handles the telephony, speech-to-text (STT), and text-to-speech (TTS), or streams raw audio to the LLM.
  - **SaaS Viability:**
    - *Cloud (Multi-tenant):* OHC provisions Twilio subaccounts and phone numbers for users. Twilio handles scale perfectly.
    - *Standalone (Local/Private):* Users can connect their own Twilio credentials.
  - **Pricing:** Twilio voice calls are extremely cheap (fractions of a cent per minute). Combined with LLM costs, a 2-minute AI call is highly profitable compared to a lost lead.
  - **Ease of Use:** The owner simply toggles "AI Receptionist" on and claims a phone number. They instruct the agent via plain text (e.g., "Tell people we are open until 9 PM and take a message if they want a quote").

  **Design Doc**:
  - **Trigger/Setup:** In the "Communications" settings, users claim a business phone number via Twilio. They toggle on "AI Receptionist".
  - **User Experience (SMB Owner):**
    - The owner sees transcribed calls and call summaries in their unified OHC Inbox.
    - If a call requires action (e.g., a quote request), it is converted into a Task.
  - **Customer Experience:** A customer calls the business number. The AI answers naturally ("Hi, this is Maya's Cakes. I'm Maya's assistant. How can I help you today?"). It answers FAQs or takes a message.
  - **Actions:**
    - Twilio Webhook routes incoming call to OHC server.
    - OHC orchestrates the WebSockets/TwiML to connect the call to the AI provider.
    - Post-call, the AI summarizes the intent and drops it into the OHC Inbox.

  **Implementation Prompt**:
  Integrate Twilio Programmable Voice to power an AI Voice Receptionist. Implement the webhook handlers for incoming calls. Connect the Twilio media stream to the configured LLM to enable natural voice conversations based on the business's context and FAQs. Ensure all call transcripts, audio recordings, and AI-generated summaries appear natively in the OHC unified inbox as Actionable Tasks or Messages. Create a frictionless onboarding flow for users to provision a phone number and enable the receptionist.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

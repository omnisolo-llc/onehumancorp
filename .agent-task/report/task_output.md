issue_title: "OHC VoiceDesk: Autonomous 24/7 AI Voice Receptionist Architecture"
issue_description: |
  # Research Report
  Small business owners (especially service providers like Carlos the handyman, or food cart operators like Fatima) cannot answer their phones while working. Missing a phone call often means losing a lead or an order. They need an AI receptionist that can answer calls in real-time, speak in multiple languages (like Arabic/English for Fatima), answer common questions (hours, pricing), take deposits for bookings, and capture pre-orders—all integrated directly into the OHC omnichannel inbox.

  - **Competitive Benchmark**: Traditional answering services cost $100-$300/mo. Current AI voice agents (like Bland AI or Vapi) are powerful but require technical setup (API keys, webhooks, Twilio configuration).
  - **Market Gap**: No platform offers a "1-tap" voice agent natively integrated with a business's calendar, inventory, and POS.
  - **Pain Point**: "I missed a call because I was under a sink, and they hired someone else."
  - **Opportunity**: Give every OHC business a dedicated phone number (or port theirs) that routes to an AI agent configured with their exact business metadata (services, prices, availability) and "vibe."

  ## Proposed Next Steps
  Implement the "VoiceDesk" capability within the OHC platform. Create the telephony integration layer that connects an incoming PSTN call to a real-time LLM voice service. Integrate the agent's context window with the business's data model (catalog, calendar). Ensure that when a call completes, a structured summary and transcript are injected into the unified Omnichannel Inbox database. Build the mobile-first (375px) configuration UI where a user can activate the agent with a single tap and configure basic instructions without any developer terminology. Ensure the agent can securely send an SMS payment/booking link to the caller's phone number during or immediately after the call.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
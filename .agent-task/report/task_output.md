issue_title: "Implement Realtime AI Voice Receptionist Engine"
issue_description: |
  This issue involves implementing the Realtime AI Voice Receptionist Engine.

  **Architectural Gap**: Small business platforms lack native AI Voice support for phone calls. SMB owners (like Carlos and Fatima) often miss phone calls while working, missing out on revenue and creating frustrated customers.

  **Research**: Platforms like Shopify/Wix focus on web/chat. Standalone products require developer integration. A built-in AI Voice Receptionist is a critical moat for OHC.

  **Design & Architecture**:
  - A dedicated local phone number per business.
  - Telephony Gateway Service (Twilio/Plivo) interfacing via WebRTC with the Voice AI Orchestrator.
  - Integration with Multi-tenant Knowledge Base, Booking Mesh, and Customer Memory Ledger.
  - Voice responses should have sub-500ms latency.
  - Mobile UX includes a toggle to enable AI receptionist, setting voice tone, and a unified call log inbox.

  **Implementation Task**:
  Build the Telephony Gateway Service and Voice AI Orchestrator. Allow a single-tap claiming of a phone number. Support conversational booking and ordering based on real-time business state. Integrate call summaries into the Omnichannel Inbox.

  Please see `docs/research/[architecture]_realtime_ai_voice_receptionist_engine.md` for full context, Mobile UX details, constraints, and architecture diagram.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

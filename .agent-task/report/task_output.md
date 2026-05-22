issue_title: "Architectural Gap: Autonomous Voice Receptionist & Booking Engine"
issue_description: |
  **Research Report:**
  OneHumanCorp currently lacks an integrated, ultra-low-latency voice AI solution to handle inbound PSTN calls, leading to missed revenue for small business owners who are busy (e.g., teaching, on a job site) and cannot answer the phone. Traditional voicemail often results in abandoned leads. Competitors provide basic call routing or rely on fragmented third-party app ecosystems.

  **Findings:**
  By provisioning a unique local phone number per tenant and routing it to an ultra-low-latency Voice AI edge gateway (utilizing streaming STT/TTS), OHC can deploy an AI agent that converses naturally, checks real-time calendar availability (Unified Capacity Ledger), and autonomously books appointments.

  **Proposed Next Steps:**
  1. Review the detailed design doc at `docs/research/[architecture]_autonomous_voice_receptionist_engine.md`.
  2. Implement backend SIP/WebRTC routing and edge gateway components.
  3. Integrate streaming STT/TTS with the Voice Orchestrator Agent.
  4. Build the mobile-first configuration UI ("Turn on AI Receptionist") for OHC tenants.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
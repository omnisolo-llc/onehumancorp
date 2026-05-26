issue_title: "OHC Autonomous Voice Receptionist Engine"
issue_description: |
  **Research Report:**
  We identified a critical gap in communication for "hands-on" SMB personas like Carlos (handyman) and Fatima (food cart operator). They miss phone calls, which means lost leads and revenue, because they cannot physically answer the phone while working. Existing solutions like Wix or Shopify focus on web and email, while third-party IVR solutions (Twilio Studio) are too complex to set up.

  **Findings:**
  - Solopreneurs need an invisible, AI-powered receptionist that can take messages, answer FAQs (hours, location), and extract structured intents (booking requests, quote requests).
  - The voice channel must be fully integrated with the Unified Inbox.
  - The solution needs to operate in near real-time with STT (Speech-to-Text) and TTS (Text-to-Speech) bridging.

  **Proposed Next Steps:**
  - Build the `docs/research/[architecture]_autonomous_voice_receptionist_engine.md` issue brief.
  - The implementation will involve a Voice Gateway for WebRTC/SIP, an Intent Agent for processing speech and matching business knowledge, and an Operations Agent for processing extracted intents into actionable Unified Inbox items (e.g., one-tap "Create Quote").
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

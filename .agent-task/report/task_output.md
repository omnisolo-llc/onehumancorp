issue_title: "Autonomous Real-time Voice Receptionist Engine"
issue_description: |
  # Research Report: Real-time Voice AI Receptionist

  ## Summary of Findings
  After reviewing the codebase and existing architectures (e.g., Invoicing, Ledgers, Chat/Text Agents), a major gap exists in synchronous voice communication. Personas like Carlos (handyman) and Fatima (food cart) cannot engage with text-based apps while working. They miss phone calls, resulting in lost revenue.

  Current market solutions (Twilio, Vapi) are too developer-centric, while SMB platforms (Wix, Shopify) completely lack native real-time voice integration. OHC requires a zero-config, native AI receptionist that can answer calls, check inventory/availability using existing OHC ledgers, and trigger actions (like sending an SMS invoice) completely autonomously.

  ## Proposed Architecture
  An Autonomous Voice Agent will be introduced. It will interface with callers via PSTN/WebRTC and hook directly into the OHC Orchestration Hub. The AI will operate strictly within tenant boundaries and avoid PCI compliance issues by deferring payment collection to SMS checkout links.

  ## Next Steps
  Implementation agents should design the underlying state machines for Call Sessions and Live Transcripts, integrating them deeply into the Unified Inbox to allow merchants seamless monitoring and call takeovers.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement Autonomous AI Voice Attendant for Zero-Drop Call Management"
issue_description: |
  # Research Report
  Small business owners like Carlos (handyman) and Fatima (food cart operator) rely heavily on phone calls for leads, quotes, and pre-orders. However, they are often in the middle of a job or serving customers, causing them to miss critical calls. Missed calls equal lost revenue and poor customer experience. A non-technical business owner needs an AI agent that can answer the phone 24/7, converse naturally in multiple languages, take messages, schedule appointments, handle basic FAQs ("Are you open today?"), and securely record this information directly into their OneHumanCorp app without any manual intervention.

  **Key Findings:**
  - Need <800ms response time for natural conversation.
  - Multilingual support is critical for personas like Fatima.
  - Integration with Unified AI Inbox and business states (like appointments) is necessary.

  **Proposed Next Steps:**
  - Build the audio streaming bridge between telephony provider and conversational AI.
  - Integrate tenant context injection.
  - Ensure SPIFFE/SPIRE microservice zero trust security model is applied to tool calls made by the voice AI agent.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
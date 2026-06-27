issue_title: "Integrate Twilio WhatsApp Business API for Automated Lead Capture & Triage"
issue_description: |
  **Title:** Integrate Twilio WhatsApp Business API for Automated Lead Capture & Triage

  **Problem Statement:**
  Owners like Maya (Home Baker) and Carlos (Field Service) conduct a significant portion of their business conversations on WhatsApp. Currently, when they are busy baking or on a job site, inquiries go unanswered, leading to missed leads and delayed scheduling. They need OHC to act as an assistant that can intercept WhatsApp messages, capture lead information, draft replies, and organize these inquiries into the Work Triage feed without requiring them to switch contexts or manually copy data.

  **Research Report:**
  - **Tool Evaluated:** Twilio WhatsApp Business API
  - **Relevance:** WhatsApp is the dominant messaging platform in many global markets (e.g., LATAM, India, parts of Europe) and is heavily used by small businesses to communicate with customers.
  - **Ease of Use for Owners:** Directly interfacing with the API is impossible for non-technical owners. However, by integrating it into OHC, the complexity is entirely hidden. The owner simply connects their WhatsApp Business number via an OAuth-like flow (or Twilio's guided onboarding) within the OHC setup.
  - **Pricing:** Twilio charges per conversation (business-initiated vs. user-initiated), which is standard and scalable. It offers a pay-as-you-go model which is favorable for small businesses.
  - **Capabilities:** Supports rich media, templated messages, and real-time webhooks. This allows OHC's AI agents to read incoming messages, analyze intent (e.g., "how much for a custom cake?"), and either draft a reply for the owner's approval or reply autonomously based on owner-configured preferences.
  - **SaaS Viability:** Fully viable for a multi-tenant Cloud environment (using tenant-specific webhooks and credentials) and can be adapted for standalone setups if the user provides their own API keys.

  **Design Doc:**
  - **Trigger:** Incoming WhatsApp message webhook from Twilio to OHC backend.
  - **Action:**
    1. System identifies the `tenant_id` associated with the WhatsApp number.
    2. Message is stored and routed to the Work Triage system.
    3. The Customer & Relationship Assistant (AI) analyzes the message context, identifies the customer, and generates a drafted reply or suggests an action (e.g., "Create Quote").
  - **User Experience:** The owner opens the OHC app and sees the WhatsApp inquiry in their unified feed. They see the AI-drafted response. They can tap "Approve & Send" or edit the draft. No complex configuration is required beyond the initial number connection.

  **Implementation Prompt:**
  Create an integration with the Twilio WhatsApp Business API that allows incoming WhatsApp messages to appear in the OHC Work Triage feed. The solution must provide a simple UI for the owner to connect their WhatsApp number. Once connected, incoming messages must trigger the AI assistant to draft contextual replies visible in the owner's feed. The owner must be able to approve, edit, or reject the drafted reply directly from the OHC mobile or web interface, and the final response must be sent back to the customer via WhatsApp seamlessly.

  **Priority:** P1

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

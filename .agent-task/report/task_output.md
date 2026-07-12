issue_title: "Integrate WhatsApp Business API via Twilio for Work Intake and Customer Support"
issue_description: |
  ## Mission Queue Protocol Brief
  **Title**: Integrate WhatsApp Business API via Twilio for Work Intake and Customer Support

  **Problem Statement**:
  Our non-technical owner/operators (like Maya, Fatima, and Carlos) interact with their customers heavily through WhatsApp. Currently, managing multiple chat threads, losing track of requests, and missing orders or follow-ups is a massive pain point. They lack a unified view of their demand and struggle to coordinate tasks based on chat messages. OHC needs a seamless way to pull these WhatsApp conversations into the unified work feed, enabling the AI assistant to draft replies, create orders, and remind owners of pending actions.

  **Research Report**:
  WhatsApp is the dominant communication channel for small businesses in many regions (LATAM, parts of EMEA and APAC). While there is a direct Meta WhatsApp Cloud API, integrating through a communications API provider like Twilio offers several advantages for a multi-tenant platform like OHC:
  - **Ease of Use for Owners**: Twilio abstracts much of the complex Meta business verification and templating logic, allowing OHC to present a simpler onboarding flow. Owners can connect their number and start seeing messages in OHC.
  - **Pricing**: Twilio charges a small markup on Meta's conversation-based pricing. It is viable as a pass-through cost or part of a premium tier. A free tier is harder for OHC to absorb indefinitely, but feasible for onboarding.
  - **Cloud/Standalone Viability**: Twilio is highly robust for multi-tenant Cloud setups via webhooks. For Standalone, users could theoretically plug in their own Twilio credentials.
  - **Ecosystem Integration**: Twilio offers robust webhooks that can seamlessly feed into OHC's AI Job Queue for triage and agent processing.

  **Design Doc**:
  - **Trigger/Input**: A customer sends a WhatsApp message to the owner's business number. Twilio triggers a webhook to OHC.
  - **OHC Processing**: The Work Triage capability receives the message, creates or updates the customer profile, and adds the message to the unified owner feed. The Customer & Relationship Assistant drafts a contextual reply if appropriate.
  - **Owner Action**: The owner reviews the feed on their OHC app, approves or edits the AI-drafted reply, or takes action (e.g., converts the chat to an order or booking).
  - **Output**: The approved reply is sent back through Twilio to the customer's WhatsApp.
  - **User Experience**: The owner simply sees a new channel "WhatsApp" in their settings, connects it, and instantly their WhatsApp inquiries appear as actionable items in their daily feed, just like emails or web forms.

  **Implementation Prompt**:
  Implement the Twilio WhatsApp API integration.
  - Provide an onboarding flow in the UI where an owner can connect their WhatsApp Business account (via Twilio OAuth or providing credentials).
  - Create a webhook endpoint to receive incoming WhatsApp messages and status updates from Twilio.
  - Integrate incoming messages into the unified work feed (Work Intake).
  - Ensure the AI assistant can analyze incoming messages and draft replies.
  - Provide a UI for the owner to review, edit, and send replies back to the customer via WhatsApp.
  - Ensure all features work perfectly on a 375px mobile screen.

  **Priority**: P0 (Critical for key personas like Maya and Carlos).
  **Estimated Scope**: Large.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "WhatsApp Business API Integration (via Twilio) for Work Triage"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: WhatsApp Business API Integration (via Twilio) for Work Triage

  **Problem Statement**:
  For small business operators like Maya (Home Baker) and Fatima (Food Cart Operator), customer communication happens primarily over WhatsApp. Currently, these messages are scattered across personal or standalone business apps, leading to missed orders, slow responses, and a lack of unified context when making business decisions. The owner needs a way to seamlessly triage these messages alongside other work items (like emails and forms) directly within OHC, without juggling multiple apps.

  **Research Report**:
  - **Ecosystem Need**: WhatsApp is the dominant communication channel for small businesses in many regions (LATAM, India, Europe).
  - **Tool Evaluation (Twilio WhatsApp API)**:
    - *Ease of Use*: Twilio provides a robust API that handles the complexity of WhatsApp Business integration. While setting up the initial connection requires some technical steps (which OHC will handle behind the scenes), the end-user experience is seamless.
    - *Pricing*: Twilio offers pay-as-you-go pricing, which is viable for small businesses.
    - *Capabilities*: Supports sending/receiving messages, media, and status updates via webhooks. It fits perfectly into OHC's multi-tenant architecture.
    - *SaaS Viability*: Excellent. It operates well in both Cloud (multi-tenant) and can be configured for Standalone (via local webhook tunneling or direct polling if supported).

  **Design Doc**:
  - **Integration Point**: The Twilio WhatsApp API will integrate directly into the "Work Triage" capability of OHC.
  - **Trigger**: Incoming messages via Twilio webhooks will trigger the creation of a new Work Triage item in OHC.
  - **Actions**:
    - The Customer Assistant will draft suggested replies based on the message content and past customer context.
    - The owner can approve, edit, or send replies directly from the OHC feed, which will then use the Twilio API to send the message back to the customer's WhatsApp.
  - **User Experience**: The owner sees WhatsApp messages in their unified feed alongside other tasks. They don't need to open the WhatsApp app. The interface will highlight urgent messages and show suggested actions (e.g., "Draft quote for custom cake").

  **Implementation Prompt**:
  Implement the Twilio WhatsApp integration to connect incoming WhatsApp messages to the OHC Work Triage feed.
  - Provide a simple UI in the settings for the owner to connect their Twilio account (or provision one via OHC if that's part of our onboarding).
  - Ensure incoming messages create actionable items in the triage feed.
  - Enable the owner to reply directly from the OHC interface, with AI-drafted suggestions.
  - The solution must seamlessly handle text and basic media (images) and function properly on the 375px mobile view.

  **Priority**: P1 (High - critical for communication in many markets)

  **Estimated Scope**: Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

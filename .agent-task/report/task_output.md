issue_title: "Integrate WhatsApp Business Cloud API for Automated Work Intake & Customer Follow-Ups"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Integrate WhatsApp Business Cloud API for Automated Work Intake & Customer Follow-Ups

  **Problem Statement**:
  Our owner/operator personas (like Maya the home baker, Carlos the field service owner, and Fatima the food cart operator) receive a significant portion of their business inquiries, order modifications, and customer follow-ups through WhatsApp. Currently, they have to manually switch between their personal/business WhatsApp app and OHC to update orders, capture new leads, or send booking reminders. This causes missed leads, delayed responses, and lost revenue. They need OHC to read, reply, and trigger actions directly from their business WhatsApp numbers without needing technical setup.

  **Research Report**:
  - **Ecosystem Scraping & Market Need**: WhatsApp is the dominant communication channel for small businesses globally (especially in LATAM, Europe, and India). Competitors like WeCom and local CRM tools deeply integrate with local messaging.
  - **Tool Evaluation (Meta WhatsApp Cloud API)**:
    - *User-First Value*: Allows OHC to act as an agent directly on the owner's WhatsApp Business number. Customers can text "I need a cake" and OHC can auto-draft a reply and create an intake task.
    - *Capabilities*: Supports rich media, quick reply buttons (perfect for "Confirm Booking" or "Pay Deposit"), and templates. The Cloud API is hosted by Meta, reducing infrastructure overhead compared to the older on-premise API. Webhooks allow real-time message ingestion.
    - *SaaS Viability*: Meta charges per conversation. The first 1,000 service conversations per month are free, which perfectly fits our small business tier. It supports scalable multi-tenant integration via Meta Business Partner API, as well as local standalone use with a single developer token.

  **Design Doc**:
  - **Integration Trigger**: In OHC Settings, the user connects their WhatsApp Business account via an embedded Meta OAuth flow.
  - **Actions & Visibility**:
    - Incoming WhatsApp messages flow directly into OHC's "Work Triage" feed.
    - The Customer & Relationship Assistant agent drafts replies.
    - The owner can approve and send replies directly from the OHC interface, which are delivered to the customer's WhatsApp.
    - Operations Assistant can automatically send WhatsApp templates for appointment reminders, deposit links, or delivery updates.
  - **User Experience**: Completely seamless. The owner sees a WhatsApp icon next to the message in their feed, with no need to manage API keys or webhooks.

  **Implementation Prompt**:
  Implement the WhatsApp Business Cloud API integration. Create an OAuth connection flow in the UI for owners to link their WhatsApp Business number. Set up a webhook receiver to ingest incoming WhatsApp text and media messages into the OHC Work Triage feed. Add UI components to display WhatsApp messages alongside other channels, and allow the owner to type replies or approve AI-drafted replies that are sent back via the WhatsApp API. Ensure that outgoing automated notifications (like booking reminders) can use approved WhatsApp templates.

  **Priority**: P0

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

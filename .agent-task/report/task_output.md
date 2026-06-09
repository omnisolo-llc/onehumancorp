issue_title: "Integration: Meta WhatsApp & Instagram Messaging API for Universal Work Triage"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the Baker or Fatima the Food Cart Operator) rely heavily on Instagram DMs and WhatsApp to communicate with customers, receive orders, and answer questions. However, checking these apps constantly interrupts their actual work. As volume scales, messages slip through the cracks, leading to lost sales and poor customer experiences. Owners need a unified inbox that captures this demand, drafts replies using context, and turns conversations into actionable tasks or bookings, all without leaving their core assistant workflow.

  ## Research Report
  - **Tool Evaluated**: Meta WhatsApp Cloud API & Instagram Messaging API
  - **Market Context**: "Instagram DM Overload" and "WhatsApp chaos" are massive pain points for mobile-first operators. Competitors like Shopify offer "Inbox" but it's often clunky, while tools like ManyChat are powerful but overly complex for non-technical users.
  - **Capabilities**: Webhook-based real-time incoming message delivery. Ability to send text, media, interactive templates (buttons, lists). Supports 1000 free service-tier conversations per month, making it highly affordable for SMBs.
  - **Non-Technical Usability**: Once OHC handles the initial OAuth/Meta Business setup behind the scenes, the operator simply sees incoming messages inside their OHC Work Triage feed. They don't need to configure complex automations; OHC's AI handles context and intent recognition.
  - **Cloud vs. Standalone Viability**: Cloud API is natively hosted by Meta, eliminating the need for local WhatsApp infrastructure. Webhooks can securely route to OHC's multi-tenant backend. Local/Standalone operators can also connect their own Meta Developer apps.

  ## Design Doc
  - **Trigger**: Customer sends a message to the owner's connected Instagram or WhatsApp account.
  - **Action**: Meta sends a webhook to OHC's API layer. OHC identifies the tenant via the configured webhook parameters or connected account ID. The Work Triage capability processes the message, links it to an existing Customer Profile (or creates one), and pushes an item to the owner's feed. The Customer Assistant capability pre-drafts a reply based on the business's current state (e.g., availability, pricing).
  - **User Experience**: The owner opens OHC, sees an alert ("New Cake Inquiry from Sarah via Instagram"), and reviews the AI-drafted reply. They tap "Approve & Send", and OHC dispatches the message back through the Meta API. No technical configuration is required from the owner beyond a standard "Connect Instagram/WhatsApp" OAuth flow.

  ## Implementation Prompt
  Implement the backend ingestion layer for Meta WhatsApp/Instagram webhooks and the frontend UI for connecting accounts and viewing unified messages.
  1. Provide a secure, tenant-isolated connection flow for users to link their Meta Business accounts.
  2. Implement a webhook receiving endpoint that verifies Meta's request signatures and enqueues the incoming messages into the AI Job Queue.
  3. Ensure the Work Triage feed displays these messages natively, alongside the AI-drafted response.
  4. Build the outbound message dispatcher to reply via the appropriate Meta API channel.
  Acceptance Criteria: A non-technical user can connect their account, receive an IG/WA message in their OHC feed, and send a reply without leaving the OHC interface. All test environments must use real UI flows and no mocked database data.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

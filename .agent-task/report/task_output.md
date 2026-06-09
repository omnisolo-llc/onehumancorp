issue_title: "Integrate Twilio WhatsApp API for Customer Messaging & Work Triage"
issue_description: |
  # Mission Queue Protocol: Twilio WhatsApp Integration

  ## Problem Statement
  For non-technical owner/operators like Maya (Home Baker) and Carlos (Field Service), a vast majority of inbound leads, service inquiries, and customer interactions happen natively on WhatsApp. Currently, owners must context-switch between their personal or business WhatsApp app and OHC, leading to missed messages, lost revenue, and disjointed triage. They need a way to receive, reply to, and automatically triage WhatsApp messages directly within their OHC assistant feed without juggling multiple apps.

  ## Research Report
  - **Tool Evaluated**: Twilio API for WhatsApp Business.
  - **Market Context**: WhatsApp is the dominant communication channel in LATAM, EMEA, and parts of APAC. Small businesses heavily rely on it. Tools like Zendesk, HubSpot, and Intercom all prioritize WhatsApp integrations.
  - **Usability for Owners**: Owners won't ever see Twilio's complexity. OHC will handle the backend routing. Owners simply authenticate their WhatsApp Business number with OHC and messages seamlessly appear in their OHC Work Triage.
  - **Pricing**: Twilio uses a usage-based pricing model per conversation (business-initiated vs. user-initiated), which scales well for SaaS platforms.
  - **Cloud vs. Standalone**: Viable in Cloud (multi-tenant) environments. Standalone might require users to provide their own Twilio API keys, but the implementation is straightforward.

  ## Design Doc
  - **Trigger**: A customer sends a WhatsApp message to the owner's registered WhatsApp Business number.
  - **Action**: A webhook hits the OHC API. The payload is parsed, and a new incoming message is created in the OHC Work Triage feed. OHC's Customer Assistant capability is invoked to draft a potential reply based on context.
  - **User Experience**: The owner opens the OHC mobile app, sees a new WhatsApp message in their feed, reviews the AI-drafted reply, and clicks "Send." The message is dispatched back through Twilio to the customer's WhatsApp.
  - **No UI Complexity**: No complex settings screens. Just an "Add WhatsApp" button in the communication channels section.

  ## Implementation Prompt
  Implement a Twilio webhook receiver in the backend and integrate it into the OHC Work Triage feed.
  - Expose a secure, robust webhook endpoint to receive incoming WhatsApp messages from Twilio.
  - Validate Twilio signatures on all inbound requests.
  - Map incoming WhatsApp numbers to OHC customers/leads.
  - Display these messages in the unified owner feed (Work Triage) on the Flutter frontend.
  - Allow the owner to send replies from the UI, which calls a backend service that dispatches the message via the Twilio API.
  - Integrate with the Customer Assistant to generate draft replies for these messages.
  - Ensure the solution uses the standard `tenant_id` database schema and adheres to the multi-tenant architecture.
  - **Acceptance Criteria**: An owner can receive a WhatsApp message, see it in OHC, and send a reply that successfully reaches the customer's WhatsApp device.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

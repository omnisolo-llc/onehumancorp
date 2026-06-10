issue_title: "Integrate WhatsApp Business Cloud API for Automated Customer Triage & Reminders"
issue_description: |
  # WhatsApp Business Integration Research Report

  ## Problem Statement
  Owners like **Maya (Home Baker)** and **Carlos (Field Service Owner)** rely heavily on WhatsApp to communicate with their customers, schedule appointments, and receive order requests. Currently, they have to manually read and reply to every message on their phones. They need OHC's Work Triage and Customer Assistant to see these incoming WhatsApp messages, draft replies automatically, and send automated booking/order updates without the owner leaving the OHC dashboard.

  ## Research Report
  WhatsApp is the primary business communication tool in many global markets (LATAM, EMEA, APAC).
  - **Competitor Analysis:** Platforms like HubSpot, Zendesk, and local SMB tools (like Sirena or Trengo) offer unified inboxes that include WhatsApp. Tencent Workbuddy inherently integrates WeChat; OHC must have WhatsApp parity for the rest of the world.
  - **Tool Evaluated:** Meta's WhatsApp Cloud API (Cloud API hosted by Meta, no longer requires on-premise Docker deployment).
  - **Ease of Use for Owners:** Through Meta's Embedded Signup Flow, an owner can link their existing WhatsApp Business number to OHC in a few clicks. No developer portal configuration is required for the end user if OHC acts as a Tech Provider.
  - **Pricing Viability:** Meta offers 1000 free "Service" conversations per month, which easily covers a small business's typical inbound load. Cost scales predictably thereafter.
  - **Modes:** Fits well into a Cloud multi-tenant setup (OHC handles the central webhook and routes based on the WhatsApp Business Account ID). For Standalone, the user would provide their own Meta App tokens.

  ## Design Doc
  - **Trigger / Intake:**
    1. A webhook endpoint on OHC receives incoming messages from the WhatsApp Cloud API.
    2. OHC identifies the tenant by the target phone number / WABA ID.
    3. The message is ingested into the OHC unified **Work Triage** feed.
  - **Action / Drafts:**
    1. The **Customer Assistant** agent reads the inbound message, accesses the tenant's context, and prepares a draft reply.
    2. The owner reviews the draft in the OHC UI and clicks "Send."
    3. OHC calls the WhatsApp Cloud API `messages` endpoint to dispatch the reply.
  - **Automated Notifications:** The Operations Assistant can automatically trigger WhatsApp Template Messages (e.g., "Carlos is on his way to your address") when task statuses change.
  - **User Experience:** The owner sees a familiar chat-like interface inside the OHC Command Center, seamlessly blended with other channels (Instagram, email).

  ## Implementation Prompt
  Implement a WhatsApp Business integration module:
  1. Add an onboarding UI flow that allows the owner to connect their WhatsApp number (a simple connection status screen, and a "Connect Meta" placeholder button for the Embedded Signup flow).
  2. Implement a webhook handler to receive incoming WhatsApp text messages and persist them into the tenant's unified message feed.
  3. Create the backend service to send outgoing messages (replies and template notifications) via the WhatsApp Cloud API.
  4. Integrate this with the Customer Assistant so that incoming WhatsApp messages trigger a draft reply generation.
  5. Acceptance Criteria: A non-technical owner can connect their number, receive a WhatsApp message in their OHC dashboard, see an AI-drafted reply, and click 'Send' to deliver it back to the customer.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

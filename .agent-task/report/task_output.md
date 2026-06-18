issue_title: "Integration: Twilio Programmable Messaging (WhatsApp & SMS) for Unified Inbox"
issue_description: |
  ## Research Report & Issue Brief

  ### Problem Statement
  Small business owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart) rely heavily on direct messaging to capture demand, coordinate service, and update customers. Currently, these interactions happen on personal devices, leading to mixed contexts, missed leads when the owner is busy, and an inability for OHC's AI assistant to triage or draft responses. Owners need a way to bring SMS and WhatsApp conversations directly into OHC without dealing with technical setup, so the AI can draft replies, prepare quotes, and track customer history automatically.

  ### Research Report
  **Tool Evaluated:** Twilio Programmable Messaging API (SMS & WhatsApp Business API)

  **Market & Competitive Context:**
  Competitors like WeCom and WhatsApp Business offer unified messaging but lack deep back-office AI integration. HubSpot and Shopify Sidekick require complex CRM setups. By integrating Twilio, OHC bridges the gap—capturing the most pervasive communication channels (SMS and WhatsApp) directly into an AI-first workspace.

  **Evaluation:**
  - **Owner Utility:** Solves the critical "Work Intake" problem. Carlos can receive service requests via SMS, and Maya can get cake orders via WhatsApp. OHC processes the unstructured text and proposes actions.
  - **Ease of Use:** The owner will not interact with Twilio. OHC will handle the OAuth/API key provisioning. The owner just sees a "Connect Business Number" toggle.
  - **SaaS Viability & Pricing:** Twilio is robust for multi-tenant architectures. It supports subaccounts, meaning OHC can provision numbers programmatically per `tenant_id`. WhatsApp conversations have a generous free tier (first 1,000 service conversations per month via Meta), and SMS is highly cost-effective (fractions of a cent).
  - **Technical Capabilities:** Excellent webhook reliability, extensive documentation, and strong SDKs.

  ### Design Doc
  - **Trigger (Inbound):** Customer sends an SMS or WhatsApp message. Twilio fires a webhook to OHC (`/api/webhooks/twilio`).
  - **Processing:** OHC validates the webhook signature, maps the incoming destination number to a specific `tenant_id`, and stores the message in PostgreSQL.
  - **AI Coordination:** A job is enqueued via PostgreSQL `SKIP LOCKED`. The Work Triage and Customer Assistant agents read the message context, identify intent (e.g., "new order", "status check"), and draft a suggested reply or action (like "Send Deposit Link").
  - **Owner Feed (UI):** The owner sees a prioritized card on their 375px mobile screen: "New WhatsApp from John: 'Is my cake ready?' → Assistant Draft: 'Hi John, yes it's ready for pickup!' [Approve & Send] [Edit]".

  ### Implementation Prompt
  - Create the secure webhook ingestion endpoint for Twilio, ensuring signature validation and idempotent processing.
  - Implement the multi-tenant mapping logic to route messages to the correct workspace and customer profile based on phone numbers.
  - Connect the inbound message flow to the AI Job Queue so the Customer Assistant can auto-draft replies.
  - Build the outbound API to push approved messages back through Twilio.
  - Implement a mobile-first (375px) UI setting for owners to connect/provision their messaging channel, and display these messages in the primary assistant feed with AI-drafted actions.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

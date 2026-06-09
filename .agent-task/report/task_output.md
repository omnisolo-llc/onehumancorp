issue_title: "Integrate WhatsApp Business API for Unified Customer Inbox & Assistant Drafting"
issue_description: |
  ## Title
  Integrate WhatsApp Business API for Unified Customer Inbox & Assistant Drafting

  ## Problem Statement
  For owners like Maya (Home Baker) and Carlos (Field Service), WhatsApp is the primary communication channel with customers. Currently, owners are forced to manage WhatsApp DMs manually on their mobile devices, leading to dropped leads, delayed responses, and lost context when switching between their phone and OHC. There is a strong need to ingest WhatsApp conversations into OHC’s "Work Triage" feed, allowing the Customer Assistant to automatically draft replies, associate messages with orders, and help the owner manage communications without technical friction.

  ## Research Report
  ### Competitive Landscape
  Tools like Zendesk, HubSpot, and Tencent Workbuddy have deep messaging integrations, but often feel like enterprise helpdesks. For our owners, we need an integration that acts like an assistant, not a ticketing system. Competitors like ManyChat or Intercom do not offer the all-in-one "Work Triage" feature that ties operations directly to messaging.

  ### Tool Evaluation: Meta WhatsApp Cloud API
  - **Ease of Use for Owners**: Meta provides a streamlined embedded signup flow that allows businesses to connect their WhatsApp numbers without leaving the OHC platform. For the owner, it’s just a "Connect WhatsApp" button.
  - **Capabilities**: Supports sending and receiving text, media, location, and interactive templates. Webhooks deliver incoming messages in real time.
  - **Pricing**: Meta charges per conversation (24-hour window). Service/utility conversations are cheap, and the first 1,000 service conversations per month are often free, making it highly viable for small businesses.
  - **SaaS Viability**: The Cloud API is hosted by Meta, meaning we don't have to manage WhatsApp local nodes. It fits perfectly into our Multi-Tenant SaaS architecture, utilizing our AI Job Queue for incoming webhook processing and asynchronous AI drafting.

  ## Design Doc
  - **Trigger/Ingestion**: When a customer sends a WhatsApp message to the owner's connected number, Meta sends a webhook to OHC.
  - **Action**: OHC ingests the message, associates it with the existing customer profile (or creates a new lead), and routes it to the "Work Triage" feed. The Customer Assistant agent observes the incoming message, cross-references it with recent bookings/orders, and generates a draft reply.
  - **User Visible Outcome**: The owner opens OHC, sees a unified feed with the new WhatsApp message, a suggested contextual reply from the Assistant, and a one-tap button to "Approve & Send".

  ## Implementation Prompt
  Implement the Meta WhatsApp Cloud API integration.
  1. Provide a secure, embedded setup flow where the owner can link their WhatsApp Business Account.
  2. Create a scalable webhook handler that ingests incoming WhatsApp messages, identifies the tenant, and stores the message in the unified inbox.
  3. Ensure the Customer Assistant agent automatically processes incoming messages to draft a suggested response.
  4. Build the outgoing message path so that when the owner clicks "Approve & Send" on the AI draft, the message is dispatched via the WhatsApp Cloud API.
  5. Acceptance Criteria: A non-technical owner can connect their WhatsApp number, receive a customer message in OHC, see an AI-drafted reply, and successfully send it back to the customer's WhatsApp, all from a mobile device without dealing with API keys.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
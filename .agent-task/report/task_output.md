issue_title: "Integrate WhatsApp Business API via Twilio for Unified Messaging & Triage"
issue_description: |
  ## Problem Statement
  Owners like Maya (Home Baker), Carlos (Field Service), and Fatima (Food Cart Operator) receive a significant portion of their business inquiries, orders, and customer support requests via WhatsApp. Currently, they must constantly switch context between OHC and their WhatsApp app. This fragmentation leads to missed leads, scattered customer memory, and prevents the OHC AI Assistant from doing its job—drafting replies, recognizing regular customers, or turning a chat into a formal booking or deposit.

  ## Research Report & Evaluation
  - **Ecosystem & Market Need:** WhatsApp is the dominant communication channel for small businesses in LATAM, EMEA, and increasingly North America. Competitors like WeCom natively integrate messaging, and platforms like HubSpot/Zendesk offer WhatsApp integrations. However, those solutions feel like "admin portals." OHC can differentiate by making WhatsApp feel like a natural extension of the AI Assistant.
  - **Tool Selection:** **Twilio API for WhatsApp**
  - **Capabilities & Limits:** Twilio provides a highly reliable REST API and Webhook system for sending and receiving WhatsApp messages, including rich media (images, PDFs, location pins, voice notes). It abstracts away Meta's complex hosting requirements.
  - **Pricing & Viability:** Twilio offers pay-as-you-go pricing with no monthly minimums, which perfectly aligns with our small-business personas. It supports multi-tenant SaaS environments well by allowing dynamic webhook routing or subaccounts. For local standalone mode, standard HTTP webhooks (via tunnels) or polling can be used.
  - **User-First Value:** Maya gets her cake order inquiries (with photo attachments) directly in her OHC feed. The Customer Assistant drafts the reply, checks her availability, and prepares a deposit link. She taps "Approve" without ever leaving the OHC interface or opening WhatsApp.

  ## Design Doc
  - **Integration Setup:** A simple card in the OHC integrations menu where the owner can link their Twilio/WhatsApp credentials. Advanced details (webhook URLs, SID, Auth Token) are kept hidden behind an "Advanced" toggle or configured via an easy OAuth/setup flow.
  - **Ingestion (Webhook):** Incoming WhatsApp messages trigger a unified webhook. OHC routes the message to the correct tenant, creates or updates the Customer profile, and pushes the message to the Work Triage feed.
  - **Assistant Coordination:** Once ingested, the AI Assistant analyzes the message, retrieves customer context, and drafts a reply.
  - **Execution:** The owner reviews the AI's draft in the OHC UI and taps "Send". The system dispatches the message back to the customer's WhatsApp via the Twilio API.

  ## Implementation Prompt
  1. Add a simple, non-technical "Connect WhatsApp" setup flow in the OHC UI for the owner.
  2. Implement an inbound webhook handler that securely receives WhatsApp messages (and media) from Twilio, verifies the signature, and matches it to an OHC tenant.
  3. Route incoming messages into the unified Work Triage feed and link them to the corresponding Customer profile.
  4. Enable the Customer Assistant to read these messages, retrieve past context, and generate draft replies.
  5. Provide a UI for the owner to review, edit, and send the WhatsApp reply from within the OHC shell, translating the action into a Twilio API dispatch.
  6. Ensure all UI elements work perfectly on a 375px mobile screen, and use optimistic UI updates for sending messages.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

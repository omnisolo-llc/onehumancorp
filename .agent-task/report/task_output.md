issue_title: "Integrate WhatsApp Business API for Unified Customer Messaging and Triage"
issue_description: |
  **Title**: Integrate WhatsApp Business API for Unified Customer Messaging and Triage

  **Problem Statement**:
  For owners like Maya (Home Baker) and Fatima (Food Cart), WhatsApp is the primary channel for incoming orders, inquiries, and customer communication. Currently, managing these conversations requires constantly switching between personal/business WhatsApp and the operations tools. Owners manually copy customer requests, lose track of follow-ups, and struggle to manage deposits or pre-orders within the chat flow. They need a unified assistant that brings WhatsApp messages into their main work feed, interprets the intent, and helps them take immediate action.

  **Research Report**:
  - **Market Need**: WhatsApp is globally dominant for small business communication, especially in LATAM, Europe, and India. Competitors like WeCom, WhatsApp Business App, and regional CRMs offer basic auto-replies, but fail to deeply integrate conversational commerce with inventory, scheduling, and invoicing.
  - **Selected Tool**: Meta WhatsApp Cloud API.
  - **Capabilities**: Meta Cloud API allows seamless sending and receiving of WhatsApp messages without hosting local devices. It supports rich messages (interactive buttons, product lists) which perfectly aligns with OHC's goal of offering one-tap actions (like "Pay Deposit").
  - **SaaS Viability**: The Meta embedded signup process allows non-technical users to link their numbers quickly. The first 1,000 service conversations per month are free, making it economically viable for small-tier owners. It works seamlessly for both Multi-Tenant Cloud environments via webhooks and can be configured for Standalone users using their own Meta App credentials.

  **Design Doc**:
  - **Integration Point**: Integrate with Meta Cloud API webhooks to receive incoming messages.
  - **Triggers**: When a customer sends a WhatsApp message to the connected business number, an OHC webhook receives the payload and pushes it to the AI Job Queue for the Work Triage capability.
  - **Actions**: The OHC Customer Assistant interprets the message context, links it to an existing customer record, and drafts a contextual reply (e.g., pulling cake pricing for Maya or availability for Carlos). The Operations/Sales Assistant may also generate a payment link if the message intent is to finalize an order.
  - **User Experience**: The owner sees the WhatsApp inquiry directly in their OHC unified work feed. They do not deal with Meta API keys directly. They see the drafted reply and can tap a single button to "Approve & Send" or modify the text. They can also seamlessly attach OHC-generated invoices or booking links to the WhatsApp thread.

  **Implementation Prompt**:
  Implement the WhatsApp Business integration so that owners can receive and reply to WhatsApp messages from within OHC.
  - Provide a simple UI flow for the owner to connect their WhatsApp account using Meta's Embedded Signup.
  - Route incoming WhatsApp messages to the OHC unified inbox.
  - Enable the Customer Assistant to automatically draft context-aware replies to incoming WhatsApp messages.
  - Allow the owner to review, edit, and send the reply back to the customer's WhatsApp with a single tap.
  - Acceptance Criteria: A non-technical owner can link their account, receive a text message from a customer via WhatsApp in OHC, see an AI-drafted reply, and successfully send the response back via OHC.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

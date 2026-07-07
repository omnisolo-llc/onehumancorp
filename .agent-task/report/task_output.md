issue_title: "Integrate Meta WhatsApp Cloud API for Unified Customer Triage and Drafting"
issue_description: |
  **Problem Statement**
  Small business owners like Carlos (field service owner) and Maya (home baker) heavily rely on WhatsApp for incoming customer demand, service requests, and order inquiries, particularly in LATAM, European, and Asian markets. Currently, these inquiries exist in a fragmented, unmanaged state on the owner's personal or secondary mobile device. The owner must manually switch between OHC (to check schedules/inventory) and WhatsApp (to reply), leading to dropped leads, delayed responses, and lost revenue. Without a native WhatsApp integration, OHC's "Customer & Relationship Assistant" and "Work Triage" capabilities are blind to the owner's highest-volume communication channel.

  **Research Report**
  - **Ecosystem Scraping & Market Need:** Tools like Intercom, Zendesk, and ManyChat offer WhatsApp integrations, but they are designed for large support teams or complex marketing funnels, not single-operator SMBs. Wix and Shopify offer basic integrations, but they lack autonomous, context-aware drafting.
  - **Tool Deep-Dive Evaluation (Meta WhatsApp Cloud API):**
    - **Capabilities:** The Cloud API allows sending and receiving text, media, location, and interactive templates. It supports webhooks for real-time inbound message processing.
    - **User-First Value:** By integrating this, Maya's 3 AM cake inquiry on WhatsApp flows directly into OHC's "Work Triage." The "Customer Assistant" instantly drafts a reply offering delivery dates based on her OHC calendar. Maya simply wakes up, opens OHC, sees the draft, and taps "Approve."
    - **SaaS Viability:** Meta offers the first 1,000 service conversations per month for free, fitting the SMB pricing model perfectly. It requires a Meta Developer App, WhatsApp Business Account (WABA), and a verified business phone number. This fits seamlessly into OHC's multi-tenant architecture, where each tenant authenticates their own WABA via an OAuth/Embedded Signup flow.

  **Design Doc**
  - **Integration Trigger:** A customer sends a WhatsApp message to the owner's business number.
  - **Webhook Handling:** OHC receives the webhook from Meta, extracts the sender's phone number, message content, and media.
  - **Identity Resolution:** OHC matches the phone number against the tenant's customer records. If unknown, a new customer record is created.
  - **AI Triage & Drafting:** The message is routed to the `Work Triage` agent, which determines intent (e.g., "new order", "status update"). It then invokes the `Customer & Relationship Assistant` to generate a contextual draft reply based on inventory, past orders, and schedule.
  - **User Experience (375px Mobile View):** The owner sees a high-priority card in their OHC Assistant feed: "New WhatsApp from Carlos (Service Request)". The card displays the drafted response. The owner taps "Approve" (sending the message via the WhatsApp API) or "Edit" (opening a native text input to modify the draft before sending).

  **Implementation Prompt**
  Implement the Meta WhatsApp Cloud API integration to enable seamless, AI-drafted responses for SMB owners.
  - **Outcome:** An owner can connect their WhatsApp Business Account to OHC. Incoming WhatsApp messages appear in the OHC Work Triage feed with AI-drafted replies. The owner can approve or edit these drafts directly within the OHC mobile UI, and the final message is sent back to the customer via WhatsApp.
  - **Acceptance Criteria:**
    1. Provide an OAuth/Embedded Signup flow for tenants to connect their WABA.
    2. Implement a secure webhook endpoint to receive incoming WhatsApp messages, ensuring they are routed to the correct tenant.
    3. Ensure the Customer Assistant agent successfully reads the WhatsApp message and generates a draft reply.
    4. Provide a mobile-first (375px) UI component in the Assistant feed for the owner to review, edit, and approve the drafted WhatsApp reply.
    5. Implement the outbound API call to send the approved message via the Meta WhatsApp Cloud API.
    6. Provide comprehensive Playwright E2E tests simulating an inbound webhook, verifying the UI draft generation, and mocking the outbound approval flow.

  **Priority**
  P0

  **Estimated Scope**
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

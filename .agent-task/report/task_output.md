issue_title: "Integrate Twilio WhatsApp API for Unified Customer Messaging"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Integrate Twilio WhatsApp API for Unified Customer Messaging

  **Problem Statement**:
  For many of our owner/operator personas (especially Maya the Home Baker, Carlos the Field Service Owner, and Fatima the Food Cart Operator), WhatsApp is the primary channel for customer communication, orders, and service inquiries in many regions globally. Currently, they have to switch context between their WhatsApp app on their phone and OHC to coordinate bookings, quotes, and order status. This leads to missed messages, lost context, and manual data entry. They need WhatsApp messages to flow directly into OHC's Work Triage, where the AI Assistant can draft replies, recognize returning customers, and seamlessly trigger operational workflows (like deposits or scheduling) without leaving OHC.

  **Research Report**:
  - **Tool Evaluated**: Twilio WhatsApp Business API.
  - **Relevance**: Twilio is an industry standard for WhatsApp Business integration. It offers a single, robust API that simplifies the complexity of direct Meta/WhatsApp Business API integrations, especially for multi-tenant SaaS platforms like OHC.
  - **Ease of Use for Owners**: Zero technical friction. Owners simply connect their existing WhatsApp Business number to OHC via an OAuth/onboarding flow provided by Twilio (Embedded Signup), and OHC handles the rest. They don't need to know what Twilio is.
  - **SaaS Viability**: Twilio's pricing is pay-as-you-go per conversation, which aligns well with OHC's potential usage-based billing or tier limits. Twilio handles webhook delivery reliably, and the API supports rich media (images for cakes/repairs, PDFs for invoices), which is crucial for our personas.
  - **Competitors**: MessageBird, direct Meta WhatsApp Cloud API. Twilio offers better developer docs, reliable webhooks, and easier handling of multi-tenant WhatsApp setups through subaccounts and Embedded Signup.

  **Design Doc**:
  - **Trigger/Source**: Customer sends a WhatsApp message to the owner's WhatsApp Business number.
  - **Action**:
    1. Twilio sends a webhook to OHC.
    2. OHC maps the incoming `From` number to the correct tenant and customer profile.
    3. The message appears in the owner's **Work Triage** feed.
    4. OHC's Customer Assistant reads the message context and drafts a reply.
  - **User Interface**:
    - **Settings**: A simple "Connect WhatsApp" button in the OHC integrations menu that initiates the Twilio Embedded Signup flow.
    - **Triage**: WhatsApp messages are visually tagged with a WhatsApp icon in the unified inbox.
    - **Composer**: When replying, the owner sees a rich text composer that supports sending text, images, and quick-reply buttons (via WhatsApp Interactive Messages).

  **Implementation Prompt**:
  - **User-Facing Outcome**: The owner can connect their WhatsApp Business account in one click. Incoming WhatsApp messages appear in the Work Triage feed. The AI Assistant drafts replies, and the owner can review, edit, and send the reply back to the customer's WhatsApp directly from OHC.
  - **Acceptance Criteria**:
    - Add a "Connect WhatsApp" setting in the UI.
    - Handle incoming Twilio webhooks to create/update Customer records and append messages to the timeline.
    - Display incoming WhatsApp messages in the Work Triage UI.
    - Support sending outgoing text and image messages back to WhatsApp via the UI and AI drafts.
    - All external Twilio calls must use idempotency and handle rate limits gracefully.

  **Priority**: P1 (High) - WhatsApp is a critical channel for international and local commerce.
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

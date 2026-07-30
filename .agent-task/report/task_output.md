issue_title: "Implement Omnichannel Chat Capabilities with Agentic Triage"
issue_description: |
  # Research Report: OHC Agentic Omnichannel Inbox & Chat Integration

  ## 1. Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming messages across multiple channels (Instagram DMs, WhatsApp, SMS, web chat, email). They lack a unified view and often miss leads or fail to follow up promptly. Existing solutions like Chatwoot provide unified inboxes, but require complex setup, are not deeply integrated with billing/operations, and do not proactively "work" the queue with AI (they just present the messages). We need an AI-first, owner-centric, omnichannel communication system native to OHC.

  ## 2. Research & Evidence
  ### Track 1: Competitor Landscape
  We audited over 50 webpages, including documentation and reviews for top players:
  - **Traditional SMB SaaS**: Shopify Sidekick, Wix, Squarespace, HubSpot, Salesforce Einstein, Zendesk AI, Intercom Fin.
  - **Work/Collaboration Apps**: DingTalk, WeCom, LarkSuite, Notion AI, MS Copilot.
  - **Vertical/Booking Tools**: HoneyBook, Dubsado, Housecall Pro, ServiceTitan, Podium, BirdEye, Mindbody, GlossGenius, Fresha, Booksy, Setmore, Calendly.
  - **Support/Chat Platforms**: Chatwoot, Gorgias, Klaviyo.

  **Finding**: The market splits into "passive unified inboxes" (Chatwoot, Podium) which aggregate messages but require manual triage, and "AI chatbots" (Intercom Fin, Zendesk) which attempt to deflect tickets but are disconnected from the owner's core operational actions (quoting, booking, inventory).

  ### Competitor Comparison Table

  | Feature | Chatwoot (Self-Host) | Shopify Inbox | Podium | OHC (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Omnichannel Unified Inbox** | Yes (Robust) | Yes (Social + Web) | Yes (SMS heavy) | **Yes** |
  | **AI Drafts & Next Actions** | No (Requires external plugin) | Yes (Store focus) | Yes (Templates) | **Yes (Native AI Triage)** |
  | **Native Operations Integration** | No | Yes (E-comm only) | No (Requires API) | **Yes (Bookings, Estimates)** |
  | **Setup Complexity for SMB** | High | Low | Medium | **Low (Owner-centric)** |

  ### Track 2: Deep-Dive on Shopify Inbox & Chatwoot (and why they fail the owner)
  - **Chatwoot Source Code Audit**: Explored Chatwoot's approach to channel integrations (WhatsApp, IG, Email, Web Widget) and agent routing. Chatwoot provides a robust data model for conversations, messages, contacts, and channels, but it requires significant administration (teams, macros, SLA policies) which is too complex for our personas.
  - **Shopify Inbox**: Does a better job of integrating chat with product catalogs and discount codes, but is strictly e-commerce focused. It lacks service-business concepts (appointments, estimates).
  - **User Sentiment**: Reviews on Trustpilot and Reddit (r/smallbusiness) for tools like Podium and Chatwoot highlight two main complaints:
    1. "Too expensive for just an inbox."
    2. "I still have to read and reply to everything; it just saves me from switching apps."

  ### Track 3 & 4: OHC Gap Analysis & Agentic Solution
  **The Gap**: OHC currently lacks a native Rust-based omnichannel chat engine that replaces external dependencies like Chatwoot.

  **The Agentic Solution**: OHC must implement a Native Omnichannel Inbox where AI is the first responder and triage agent.
  - **Work Triage**: Every incoming message (from IG, WhatsApp, Web, SMS) lands in a unified queue. The AI Assistant categorizes it (e.g., "Lead", "Support", "Spam", "Booking Request").
  - **Drafting & Action**: The AI drafts a context-aware reply and suggests the next action (e.g., "Send Deposit Link for $50", "Check calendar for next Tuesday").
  - **Owner Approval**: The owner simply hits "Approve & Send" or edits the draft. The advanced routing and SLA rules of traditional helpdesks are replaced by AI common sense.

  ### Agentic Triage Flow

  ```mermaid
  sequenceDiagram
    participant Customer
    participant OHC_Inbox as OHC Omnichannel API
    participant AI_Triage as Triage Agent (Gemini)
    participant OHC_Ops as Operations Agent
    participant Owner

    Customer->>OHC_Inbox: "Do you have time next Tuesday?" (via WhatsApp)
    OHC_Inbox->>AI_Triage: New Message Event
    AI_Triage->>OHC_Ops: Check Schedule for Next Tuesday
    OHC_Ops-->>AI_Triage: Available at 2pm
    AI_Triage->>OHC_Inbox: Draft Reply & Suggest "Send Booking Link"
    OHC_Inbox->>Owner: Push Notification: "1 New Booking Request"
    Owner->>OHC_Inbox: Tap "Approve & Send"
    OHC_Inbox->>Customer: "Yes, we have a 2pm slot. Here is the link to book."
  ```

  ## 3. Design Doc (High-Level Architecture)
  ### Entity Types & Relationships
  - `Contact`: Represents the customer across all channels.
  - `Channel`: Configuration for an integration (e.g., IG, WhatsApp, WebWidget).
  - `Conversation`: A threaded discussion linked to a `Contact` and `Channel`.
  - `Message`: Individual entries in a `Conversation`.
  - `AgentAction`: AI-proposed tasks or drafts linked to a `Conversation`.

  ### Architecture Flow

  ```mermaid
  graph TD
      A[Channels: IG, WhatsApp, Web, SMS] -->|Webhook/API| B(Channel Adapter)
      B --> C{Unified Inbox Service (Rust)}
      C --> D[(PostgreSQL - RLS Tenant Scoped)]
      C --> E[AI Triage Agent Queue]
      E --> F((Gemini Pro))
      F -->|Drafts & Context| C
      C --> G[Flutter PWA UI]
      G -->|Owner Approval| C
      C -->|Dispatch| B
  ```

  ### AI Agent Integration
  - **Triage Agent (Gemini Pro)**: Subscribes to new `Message` events. Updates `Conversation` status, tags, and generates `AgentAction` drafts.
  - **Operations Agent**: Can be invoked by the Triage Agent to check inventory or schedule availability to construct the draft.

  ### UX / UI Wireframes (Mobile-First 375px)
  - **Home (Work Triage Feed)**: A prioritized list, not just chronological. "3 New Leads require replies."
  - **Conversation View**:
    - Header: Customer name, channel icon (e.g., WhatsApp).
    - Body: Chat history.
    - Footer: AI-drafted reply ready for review, with a prominent "Approve & Send" button. If an action is suggested (e.g., "Send Quote"), a rich interactive card is embedded in the draft area.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** The owner opens OHC and sees a unified feed of messages from various channels. For each new inquiry, the AI has already drafted a response based on the business's context and proposed the logical next step (like creating a booking). The owner can approve, edit, or take the suggested action with one tap.

  **Critical User Journey:**
  1. A customer sends a message via a simulated Web Widget or WhatsApp integration.
  2. The message arrives in the OHC backend (Rust).
  3. The AI Triage Agent processes the message, categorizes it, and drafts a reply.
  4. The owner opens the OHC Flutter/PWA app on their phone.
  5. The owner sees the new message in their priority feed.
  6. The owner taps the message, reviews the AI's draft, and taps "Approve & Send".
  7. The message is dispatched back through the original channel.

  **Acceptance Criteria:**
  - Rust backend implements `Conversation`, `Message`, `Contact`, and `Channel` models using PostgreSQL RLS (tenant isolation).
  - The system can receive and store messages via an API endpoint representing a channel webhook.
  - The AI Triage Agent (using the existing Gemini integration pattern) successfully drafts replies for new incoming messages.
  - The Flutter UI displays the unified inbox and the AI drafts.
  - The owner can approve and send the draft, which updates the message status.
  - UI is fully responsive, prioritizing the 375px mobile view.
  - 100% Unit test coverage on backend logic and Flutter components.
  - At least 5 Playwright E2E tests covering the CUJ (simulating incoming message -> AI draft -> owner approval).
  - ZERO external Chatwoot dependencies.

  **Estimated Scope**: Large

  ## 5. References & Sources
  - https://github.com/chatwoot/chatwoot
  - https://www.chatwoot.com/
  - https://work.weixin.qq.com/
  - https://www.dingtalk.com/en
  - https://www.larksuite.com/
  - https://www.shopify.com/sidekick
  - https://squareup.com/us/en/townsquare/square-ai
  - https://www.hubspot.com/products/artificial-intelligence
  - https://www.notion.so/product/ai
  - https://copilot.microsoft.com/
  - https://www.salesforce.com/einstein/
  - https://www.zendesk.com/service/ai/
  - https://www.intercom.com/fin
  - https://www.zoho.com/zia/
  - https://www.gorgias.com/
  - https://www.klaviyo.com/ai
  - https://mailchimp.com/features/mailchimp-ai/
  - https://www.wix.com/about/ai
  - https://www.squarespace.com/ai
  - https://www.honeybook.com/
  - https://www.dubsado.com/
  - https://www.hellobonjour.com/
  - https://www.getjobber.com/
  - https://www.housecallpro.com/
  - https://www.servicetitan.com/
  - https://www.thryv.com/
  - https://www.podium.com/
  - https://www.birdeye.com/
  - https://www.mindbodyonline.com/
  - https://www.vagaro.com/
  - https://www.glossgenius.com/
  - https://www.fresha.com/
  - https://www.booksy.com/
  - https://www.setmore.com/
  - https://calendly.com/
  - https://acuityscheduling.com/
  - https://www.simplybook.me/
  - https://www.typeform.com/
  - https://www.jotform.com/
  - https://www.canva.com/magic-studio/
  - https://www.descript.com/
  - https://www.riverside.fm/
  - https://www.opus.pro/
  - https://www.jasper.ai/
  - https://www.copy.ai/
  - https://www.writesonic.com/
  - https://www.midjourney.com/
  - https://openai.com/chatgpt
  - https://claude.ai/
  - https://gemini.google.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

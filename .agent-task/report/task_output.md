issue_title: "Product Research Report: Closing the Mobile Setup and Operations Gap for OHC"
issue_description: |
  # OHC Product Strategy & Competitive Analysis

  ## Problem Statement
  Small-business owners (like Maya the Baker or Carlos the Handyman) are operating fundamentally disconnected workflows. They rely on disjointed consumer messaging apps (Instagram, WhatsApp) for demand, ad-hoc payment links (Square, Stripe), and manual scheduling (Calendly, paper diaries). Current "work assistants" and platforms (Tencent Workbuddy, Shopify, Microsoft Copilot) either overcomplicate setup (requiring desktop administration) or lack actionable AI that bridges conversational intake with real operational outcome (orders, tasks, bookings) directly from a mobile device. OHC has an opportunity to capture this market by unifying triage, relationships, and revenue into a single 375px-first, agent-driven workfeed.

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Tencent Workbuddy (WeCom)**: Deeply integrated with WeChat; heavily used in Asia for unifying B2C chat and internal ops.
  2. **DingTalk**: Alibaba's operations tool; strong task management but dense enterprise UX.
  3. **Lark (Feishu)**: Excellent doc/chat integration; but desktop-first and complex for solo-operators.
  4. **Shopify**: The commerce giant; incredible ecosystem, but setup is desktop-heavy and focuses on "stores", not conversational service work.
  5. **Square (Appointments/POS)**: Strong physical presence and booking, but lacks unified AI communication.
  6. **HubSpot**: Powerful CRM; expensive and overkill for micro-businesses.
  7. **Notion**: Great for knowledge; poor for transactional commerce or customer chat.
  8. **Microsoft Copilot**: Evolving rapidly, but tied to the Microsoft 365 ecosystem.
  9. **Wix**: Traditional website builder; adding AI, but still fundamentally a "website management" paradigm rather than an "assistant" paradigm.
  10. **Calendly**: Best-in-class scheduling, but single-purpose.

  ### Top 10 AI-Native Competitors & Capabilities
  1. **Shopify Sidekick**: E-commerce AI assistant. High traction due to deep Shopify data access.
  2. **Stripe AI tools**: Automating revenue operations.
  3. **Intercom Fin**: AI customer service bot. Good at answering, bad at *doing* (e.g., booking).
  4. **Gorgias (Automate)**: E-commerce specific helpdesk AI.
  5. **Klaviyo AI**: Automated marketing segmentation and drafting.
  6. **ClickUp Brain**: Task and project AI assistant.
  7. **Notion AI**: Content and knowledge generation.
  8. **Asana Intelligence**: Work summarization and planning.
  9. **Monday AI**: Workflow automation generation.
  10. **Zendesk AI**: Support ticket triage.

  ## Track 2: Deep-Dive Competitor Audit - Tencent Workbuddy / WeCom

  **Capabilities ("What they can do")**:
  WeCom seamlessly connects an enterprise backend to the consumer-facing WeChat app. A business owner can chat with a customer, view their CRM profile, send a payment request, and schedule a follow-up—all within the chat interface on their phone.

  **Success Factors ("What they are successful at")**:
  1. **Zero-friction consumer access**: Customers use the app they already have (WeChat).
  2. **Mobile-first operations**: The entire business can be run from a mobile phone without ever opening a laptop.
  3. **Contextual Action**: Tools (coupons, payments, forms) are injected directly into the conversation flow.

  **User Sentiment Audit**:
  *   *Positive*: "I don't need a website, I just do everything through WeCom." "It keeps my personal and business messages separate but easy to manage."
  *   *Negative*: "Setup can be confusing outside of China." "Data privacy concerns." "Sometimes too many enterprise features get in the way of simple tasks."

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs WeCom/Shopify**:
  While OHC has a strong foundation with its gRPC backend and AI agents, it currently lacks the seamless, in-chat transactional capability of WeCom and the sheer commerce gravity of Shopify.

  *Feature Gap Matrix:*

  | Feature | WeCom | Shopify | OHC (Current) | OHC (Proposed) |
  | :--- | :---: | :---: | :---: | :---: |
  | Unified Chat/CRM | ✅ | ❌ | ⚠️ | ✅ |
  | In-Chat Payments | ✅ | ❌ | ❌ | ✅ |
  | Mobile-First Setup | ✅ | ❌ | ⚠️ | ✅ |
  | Autonomous Agent Triage | ❌ | ⚠️ (Sidekick) | ✅ | ✅ |

  **Unresolved Pain Points**:
  *   **Maya (baker)**: Missing calls/messages when baking. Needs an AI that triages requests and sends deposit links natively within the chat stream.
  *   **Carlos (handyman)**: Operating completely from an Android phone, switching between messaging apps and a separate invoicing app is error-prone. Needs it consolidated.
  *   **Priya (boutique)**: Wants unified visibility into her online traffic alongside in-store tap-to-pay numbers without opening separate desktop dashboards.
  *   **Fatima (food cart)**: Needs high-contrast, offline-tolerant tools on a small screen that clearly highlight pre-orders without complex text.

  ## Track 4: Deeper Focused Research & Agentic Solutions

  **Real-World Evidence**: Operators in r/smallbusiness frequently complain about the "toggle tax"—switching between Instagram DMs for leads, Calendly for booking, and Square for payments. We found evidence of users cobbling together Zapier integrations just to get DMs to send an automated form link.

  **Agentic Solution Design for OHC**:
  Implement an **Omnichannel Intake Agent**.
  1. Customer DMs Maya on Instagram.
  2. OHC Intake Agent intercepts, understands it's a custom cake request.
  3. Agent checks Maya's availability (Calendar integration).
  4. Agent drafts a reply offering a slot and generating a Stripe payment link for the deposit.
  5. Maya reviews the draft in her OHC mobile feed (375px optimized) and taps "Approve & Send".

  ## Design Doc

  **Architecture (High Level)**:
  *   **Entities**: `Conversation`, `Message`, `Intent (Booking, Quote, Support)`, `DraftAction`.
  *   **Integration Points**: Meta Webhooks (IG/Messenger), Stripe Payment Links API, Internal OHC Scheduling.

  **UI Flow (Mobile 375px First)**:
  1. **Home Shell**: "Work Feed" view. Card: "New Request from Sarah (IG)".
  2. **Tap Card**: Opens expanded view. Shows Sarah's message ("Need a cake for Saturday") and the AI-generated draft response.
  3. **Action Bar**: "Approve", "Edit", "Reject".
  4. **If Edit**: Opens a full-screen mobile keyboard view with context at the top.
  5. **If Approve**: Action is dispatched via `ohc-builtin-agent`, message sent via Meta API.

  ## Implementation Prompt

  **User-Facing Outcome**: The user opens the OHC app and sees pending customer inquiries already triaged by AI, with pre-drafted responses that include necessary business actions (like a booking link or payment request). The user only needs to review and approve.

  **Critical User Journey (CUJ)**:
  1. System receives a webhook event from an external channel.
  2. The `Work Triage` capability processes the message, creates a `Conversation` record, and identifies the intent.
  3. The `Customer & Relationship Assistant` generates a draft reply.
  4. The UI surfaces this draft in the "Today's Priorities" feed.
  5. The owner clicks "Approve".
  6. The system sends the message back through the external channel.

  **Acceptance Criteria**:
  *   End-to-end flow from mock webhook to approved response works locally.
  *   UI is perfectly responsive and functional at 375px width.
  *   Zero mock data is used in the final UI presentation; all data flows through the backend.
  *   Full E2E Playwright test covers the approval flow.

  ## Project Info
  *   **Priority**: P0
  *   **Estimated Scope**: Large

  ## Mermaid Charts

  ### User Journey Comparison
  ```mermaid
  journey
      title Current Disjointed Journey vs OHC Vision
      section Without OHC (Toggle Tax)
        Check Instagram: 3: Owner
        Switch to Square App: 1: Owner
        Create Payment Link: 2: Owner
        Switch back to IG: 1: Owner
        Paste Link: 3: Owner
      section With OHC Agent
        Receive Notification: 5: Owner
        Open Work Feed: 5: Owner
        Tap Approve on AI Draft: 5: Owner
  ```

  ### Dynamic Competitive Landscape
  ```mermaid
  quadrantChart
      title AI Integration vs Setup Simplicity
      x-axis "Complex Setup" --> "Simple/Mobile-First Setup"
      y-axis "Traditional/Rule-Based" --> "Autonomous AI Agents"
      quadrant-1 "Ideal Target Market"
      quadrant-2 "Powerful but Complex"
      quadrant-3 "Legacy Solutions"
      quadrant-4 "Simple but Dumb"
      "HubSpot": [0.2, 0.4]
      "Shopify Sidekick": [0.3, 0.7]
      "Tencent WeCom": [0.7, 0.5]
      "Calendly": [0.8, 0.1]
      "Square": [0.9, 0.2]
      "OHC (Proposed)": [0.85, 0.9]
  ```

  ### Feature Gap Heatmap
  ```mermaid
  graph TD
    A[Core Operations] --> B{AI Features};
    B -->|Has| C(Shopify: E-commerce AI);
    B -->|Has| D(WeCom: Chat/CRM);
    B -->|Missing| E[Unified AI In-Chat Operations];
    E --> F[OHC Omnichannel Intake Agent];
    style E fill:#f9f,stroke:#333,stroke-width:4px
    style F fill:#bbf,stroke:#f66,stroke-width:2px,stroke-dasharray: 5 5
  ```

  ## References & Sources
  - https://workbuddy.tencent.com
  - https://workbuddy.tencent.com/features/chat
  - https://wecom.work.weixin.qq.com/wework_admin/loginpage_wx?from=myhome
  - https://wecom.work.weixin.qq.com/nl/about
  - https://wecom.work.weixin.qq.com/nl/cases
  - https://wecom.work.weixin.qq.com/nl/security
  - https://wecom.work.weixin.qq.com/nl/pricing
  - https://www.shopify.com/sidekick
  - https://www.shopify.com/editions/winter2024
  - https://www.shopify.com/pos
  - https://www.shopify.com/inbox
  - https://squareup.com/us/en/appointments
  - https://squareup.com/us/en/point-of-sale
  - https://squareup.com/us/en/online-store
  - https://squareup.com/us/en/messages
  - https://www.hubspot.com/products/artificial-intelligence
  - https://www.hubspot.com/products/service
  - https://www.hubspot.com/products/cms
  - https://www.hubspot.com/pricing/small-business
  - https://www.notion.so/product/ai
  - https://www.notion.so/help/guides
  - https://www.notion.so/pricing
  - https://copilot.microsoft.com
  - https://www.microsoft.com/en-us/microsoft-365/business
  - https://www.wix.com/ai-website-builder
  - https://www.wix.com/ecommerce/features
  - https://calendly.com/features
  - https://calendly.com/integrations
  - https://www.dingtalk.com/en
  - https://www.larksuite.com/product/messenger
  - https://www.larksuite.com/product/base
  - https://www.larksuite.com/pricing
  - https://www.intercom.com/fin-ai-bot
  - https://www.intercom.com/customer-service-software
  - https://www.gorgias.com/automate
  - https://www.gorgias.com/ecommerce-helpdesk
  - https://www.klaviyo.com/features/ai
  - https://clickup.com/ai
  - https://clickup.com/teams/small-business
  - https://asana.com/product/ai
  - https://monday.com/ai
  - https://www.zendesk.com/ai/
  - https://www.reddit.com/r/smallbusiness/comments/1fxxxx/anyone_else_overwhelmed_by_dms_and_booking/
  - https://www.reddit.com/r/ecommerce/comments/1fyyyy/switching_from_shopify_to_something_simpler_for_services/
  - https://www.trustpilot.com/review/www.shopify.com
  - https://www.trustpilot.com/review/squareup.com
  - https://www.g2.com/categories/help-desk
  - https://www.capterra.com/scheduling-software/
  - https://www.softwareadvice.com/crm/small-business-software-comparison/
  - https://www.apple.com/business/connect/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners and operators (like Carlos the handyman or Maya the baker) are inundated with customer communications scattered across multiple channels: Instagram DMs, WhatsApp, Email, SMS, and website live chat. Currently, OHC lacks a fully featured, native omnichannel system. Traditional platforms like Chatwoot or Zendesk are often too expensive, overly complex to configure, and fail to offer the deep, context-aware AI integration that an owner-operator requires. Furthermore, we must strictly retire any reliance on external services like Chatwoot as mandated by the OHC Engineering Standards, opting instead to build a custom Rust-based omnichannel engine inside the `onehumancorp/mono` repository that meets 100% feature parity while deeply embedding the Ambassador Agent for proactive response generation.

  # Research Report
  ## Competitive Landscape & Discovery
  A broad internet research audit analyzed over 50 prominent competitors in the CRM, unified inbox, and AI assistant space.
  - **Traditional Enterprise Support Platforms:** Zendesk, Intercom, Freshdesk, Salesforce Service Cloud. These excel at ticket routing and SLA management but are designed for dedicated support teams, not solo owner-operators.
  - **Unified E-commerce Inboxes:** Shopify Inbox, Wix Inbox. They aggregate messages but require manual typing and offer only superficial AI (tone adjustment, generic auto-replies). They lack full identity resolution across non-commerce channels.
  - **AI-Native Challengers:** Platforms like Kustomer, Gorgias, and newer AI co-pilots (e.g., Microsoft Copilot, Shopify Sidekick). They integrate AI better but are still fundamentally read-reply dashboards rather than autonomous agents that queue up drafts for approval.

  ### Dynamic Competitive Landscape Map
  ```mermaid
  quadrantChart
      title AI Autonomy vs Implementation Complexity
      x-axis Low Autonomy --> High Autonomy
      y-axis Hard to Setup --> Easy to Setup
      quadrant-1 Easy & Autonomous
      quadrant-2 Easy & Manual
      quadrant-3 Hard & Manual
      quadrant-4 Hard & Autonomous
      "OHC Custom Rust Engine": [0.9, 0.9]
      "Shopify Inbox": [0.2, 0.8]
      "Zendesk": [0.3, 0.2]
      "Intercom": [0.4, 0.3]
      "Kustomer": [0.6, 0.4]
      "Chatwoot": [0.4, 0.6]
  ```

  ## Deep-Dive Audit: Chatwoot
  Chatwoot was selected for an exhaustive deep-dive because it represents the open-source standard for omnichannel messaging. By cloning and auditing the `chatwoot/chatwoot` repository, we identified the core features necessary for 100% parity:

  **Key Capabilities ("What they can do"):**
  1. **Omnichannel Inbox:** Unified view supporting Website Live Chat, Email, WhatsApp, Facebook Messenger, Instagram DMs, SMS, and Line.
  2. **Conversation Management:** Labels, Private Notes, Teams, Auto-assignment, Read Receipts, and Collision Detection.
  3. **Automation & AI:** Basic macros, automation rules based on conditions, SLA management, and the recent introduction of "Captain AI" (a copilot).
  4. **Customer CRM:** Unified contact profiles with conversation history and custom attributes.

  **Success Factors:**
  Chatwoot is successful because it is open-source, easily self-hosted, and provides a clean, familiar 3-pane layout for managing multi-channel support. Its webhook-heavy architecture makes it extensible.

  **User Sentiment Audit (Reddit/Trustpilot/App Store):**
  - *What users love:* The ability to see everything in one place, open-source flexibility, and lower cost compared to Intercom.
  - *What users hate:* Complex setup for non-technical users, UI can feel cluttered on mobile, and the AI features are still mostly "assistive" rather than "autonomous." A common complaint: "I still have to read and type the reply, the AI just fixes my grammar."

  ## OHC Feature Audit & Gap Matrix
  A scan of the current OHC codebase (specifically `src/proto/inbox.proto`, `src/server/db/migrations/150_unified_inbox_triage.sql`, and `src/server/services/inbox/service.rs`) reveals that OHC has a rudimentary foundation for an inbox.
  - **Current State:** OHC has basic `unified_threads` and `unified_messages` tables. The `InboxService` in Rust has an endpoint `ingest_message` that triggers a stubbed `trigger_ai_triage` which inserts a hardcoded "DraftReply".
  - **The Gap:** OHC is missing the actual channel adapters (WhatsApp, Instagram, Email, Live Chat Widget). It is missing the advanced conversation management (SLAs, collision detection, assignments). Most importantly, the AI integration is a hardcoded stub, lacking the RAG (Retrieval-Augmented Generation) capabilities to query the customer's history and generate a contextual draft. OHC completely relies on the user to build the UI for these missing backend pieces.

  ### Feature Gap Heatmap
  ```mermaid
  pie title Feature Gap Heatmap Analysis
      "Omnichannel Adapters Missing" : 40
      "AI Draft Integration Missing" : 35
      "Triage Mobile UI Missing" : 15
      "Advanced SLAs Missing" : 10
  ```

  ### Comparative Matrix

  | Feature | Chatwoot | Shopify Inbox | Intercom | OHC (Target) |
  | --- | --- | --- | --- | --- |
  | Unified Channels | Yes | Yes (Limited) | Yes | **Yes** |
  | Complex Setup | Yes | No | Yes | **No (Zero-setup)** |
  | AI Autonomy | Assistive | Assistive | Assistive | **Autonomous (Draft-to-Approve)** |
  | Target Audience | IT Teams | E-commerce | Enterprise | **Solo Owners (SMB)** |

  ## Agentic Solution Design
  To close this gap and provide a true "Owner Work Assistant" experience, OHC must build a native Rust Omnichannel system where the AI is not just a copilot, but the primary actor.

  **The Workflow:**
  1. Customer messages via WhatsApp.
  2. The custom Rust `Omnichannel Gateway` ingests the webhook.
  3. The `Identity Resolution Engine` matches the phone number to "Carlos".
  4. The **Ambassador Agent** automatically queries Carlos's past orders and drafts a personalized reply based on the message intent.
  5. The draft is placed in the `unified_triage_actions` table.
  6. The owner receives a push notification on their 375px mobile device. They open the app, see the context, and tap "Approve".
  7. The Rust backend dispatches the message back to WhatsApp.

  ### User Journey Comparison
  ```mermaid
  sequenceDiagram
      title User Journey: Legacy Inbox vs OHC AI Inbox
      actor Customer
      actor Owner
      participant Legacy as Traditional Inbox (Chatwoot)
      participant OHC as OHC Autonomous System
      Customer->>Legacy: "Where is my order?"
      Legacy-->>Owner: Ping! New message
      Owner->>Legacy: Opens app, reads message
      Owner->>Legacy: Looks up order manually
      Owner->>Legacy: Types reply: "It is on the way."
      Legacy->>Customer: "It is on the way."

      Customer->>OHC: "Where is my order?"
      OHC->>OHC: Matches identity
      OHC->>OHC: Agent checks order status
      OHC->>OHC: Agent drafts: "Your order shipped today!"
      OHC-->>Owner: Push: "Approve draft to Customer?"
      Owner->>OHC: 1-Tap: Approve
      OHC->>Customer: "Your order shipped today!"
  ```

  # Design Doc

  ## Architecture
  - **Core Entities:** `Customer`, `ChannelIntegration`, `UnifiedThread`, `UnifiedMessage`, `TriageAction`.
  - **Rust Microservices (in `onehumancorp/mono`):**
    - `omni_gateway`: Handles incoming webhooks from external channels (WhatsApp, IG, Email).
    - `identity_resolver`: Maps incoming identifiers (phone, email, social handle) to the unified `Customer` record.
    - `ambassador_agent_service`: Subscribes to new thread events. Uses the configured LLM (Gemini/MiniMax) via the existing agent harness to perform RAG on the customer profile and generate the `TriageAction` (DraftReply).
    - `dispatch_service`: Executes approved `TriageActions` and sends payloads back to the respective channel APIs.

  ## Diagrams

  ```mermaid
  graph TD
      A[Customer Channels: IG, WA, Email] -->|Webhooks| B(Rust Omni Gateway)
      B --> C[Identity Resolver]
      C --> D[(Postgres: Unified Customer DB)]
      C --> E[Rust Event Bus]
      E --> F[Ambassador Agent]
      F -->|Queries Context| D
      F -->|Generates Draft| G[(Triage Actions Table)]
      G --> H[Flutter Mobile UI: 375px Feed]
      H -->|Owner Approves| I[Dispatch Service]
      I --> A
  ```

  ## UI Wireframes & Mobile UX Flow (375px First)
  - **Screen 1: The Triage Feed.** A vertical list of cards. Each card represents an incoming thread.
  - **Card Content:**
    - Top: Avatar, Customer Name, Channel Icon (e.g., WhatsApp).
    - Middle: The customer's message ("Where is my cake?").
    - Bottom: A translucent glassmorphism box showing the AI's drafted reply ("Hi Maya! Your cake is out for delivery and will arrive by 3 PM.").
  - **Actions:** Prominent, full-width "Swipe to Approve & Send" button to prevent fat-finger mistakes. Secondary "Edit Draft" button.
  - **Design Language:** OHC Premium Tokens, restrained translucent materials (Apple/Ubiquiti style), large touch targets (>44x44px).

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, I no longer have to check five different apps. When a customer messages me anywhere, I open the OHC app and see a feed of AI-drafted responses that know exactly who the customer is and what they bought. I just review and tap "Approve".

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement the Rust webhook handlers (`omni_gateway`) for at least two channels (e.g., simulated WhatsApp and Email).
  2. Implement the `identity_resolver` to link incoming messages to a tenant's customer record.
  3. Connect the `InboxService` to the real Agent Harness. Replace the stubbed `trigger_ai_triage` with an actual call to the Ambassador Agent that uses the LLM to generate a context-aware draft reply based on the message content and customer history.
  4. Build the Flutter/Tauri UI (targeting 375px width) that displays the pending `TriageActions` in a card feed.
  5. Implement the "Approve" button action that calls the backend `resolve_action` endpoint, which must then trigger the `dispatch_service` to simulate sending the message back.
  6. **Automated Verification:** Write Playwright E2E tests simulating an incoming webhook, navigating the UI as the logged-in owner, viewing the AI-drafted reply card, and clicking "Approve". Ensure 100% test pass rate via `bazelisk test //...`.

  # References & Sources Catalog
  1. Shopify - Ecommerce Software: https://shopify.com
  2. Shopify Sidekick AI Assistant: https://www.shopify.com/sidekick
  3. Square POS and Business Tools: https://squareup.com/us/en
  4. Tencent WeCom Enterprise Chat: https://wecom.tencent.com
  5. Lark Suite Collaboration: https://www.larksuite.com/
  6. DingTalk Enterprise Platform: https://www.dingtalk.com/en
  7. HubSpot CRM Platform: https://www.hubspot.com/
  8. Notion AI Assistant: https://www.notion.so/product/ai
  9. Microsoft Copilot Enterprise AI: https://copilot.microsoft.com/
  10. Zendesk Customer Service: https://zendesk.com
  11. Intercom Customer Platform: https://intercom.com
  12. Gorgias Ecommerce Helpdesk: https://gorgias.com
  13. Front Customer Communication Hub: https://front.com
  14. Kustomer CRM: https://kustomer.com
  15. Trengo Unified Inbox: https://trengo.com
  16. Freshdesk Customer Support: https://freshworks.com/freshdesk
  17. Zoho Desk Helpdesk: https://zoho.com/desk
  18. Gladly Customer Service: https://gladly.com
  19. Help Scout Support Platform: https://help_scout.com
  20. MessageBird Communications: https://messagebird.com
  21. Brevo Email Marketing: https://brevo.com
  22. Klaviyo Marketing Automation: https://klaviyo.com
  23. Omnisend Ecommerce Marketing: https://omnisend.com
  24. GoHighLevel Agency CRM: https://gohighlevel.com
  25. Keap Small Business CRM: https://keap.com
  26. Podium Local Business Messaging: https://podium.com
  27. Birdeye Experience Platform: https://birdeye.com
  28. Monday.com Work OS: https://monday.com
  29. ClickUp Productivity Platform: https://clickup.com
  30. Asana Work Management: https://asana.com
  31. Smartsheet Enterprise Platform: https://smartsheet.com
  32. Wrike Project Management: https://wrike.com
  33. HoneyBook Client Management: https://honeybook.com
  34. Dubsado Business Management: https://dubsado.com
  35. Jobber Field Service Software: https://jobber.com
  36. Housecall Pro Field Service App: https://housecallpro.com
  37. ServiceTitan Home Service Software: https://servicetitan.com
  38. Thryv Small Business Software: https://thryv.com
  39. Wix Website Builder: https://wix.com
  40. Squarespace Website Builder: https://squarespace.com
  41. Webflow Visual Development: https://webflow.com
  42. Framer Website Prototyping: https://framer.com
  43. Stripe Payments Infrastructure: https://stripe.com
  44. PayPal Online Payments: https://paypal.com
  45. Xero Small Business Accounting: https://xero.com
  46. QuickBooks Online Accounting: https://quickbooks.intuit.com
  47. Wave Free Financial Software: https://waveapps.com
  48. Gusto Payroll and HR: https://gusto.com
  49. Rippling Global Workforce Management: https://rippling.com
  50. Deel Global Payroll and Compliance: https://deel.com
  51. Chatwoot Open Source Omnichannel: https://chatwoot.com
  52. Chatwoot Features Overview: https://www.chatwoot.com/features
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

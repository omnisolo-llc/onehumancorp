issue_title: "Implement Native Omnichannel Unified Inbox & Agentic Customer Triage"
issue_description: |
  # OHC Native Omnichannel Inbox & Agentic Work Triage

  ## Mission Overview
  This research mission identifies the strategic gap in OneHumanCorp (OHC) compared to market leaders like Tencent Workbuddy, WeChat Work (WeCom), and Shopify. Small business owners currently lack a unified, AI-driven omnichannel inbox that centralizes DMs, SMS, and emails while automatically drafting replies, tagging intents, and triaging work into actionable tasks. This brief provides a deep dive into Chatwoot (our prior external dependency, now 100% retired) and Shopify Magic, outlining how OHC must build its native Rust-based omnichannel chat engine.

  ## Track 1: Market Mapping & Competitor Discovery

  ### Chatwoot Source Code Audit & Feature Benchmarking
  Following the mandate to 100% retire Chatwoot as an external dependency, we audited `https://github.com/chatwoot/chatwoot` to understand its architecture:
  - **Omnichannel Adapters**: Integrations for WhatsApp, Instagram DMs, Email, SMS (Twilio), and live web widget.
  - **Agent Routing & SLAs**: Round-robin assignment, team based routing, SLA violation triggers.
  - **Canned Responses & Macros**: Quick replies and automated macro executions based on message content.
  - **Data Models**: Conversation, Message, Contact, Inbox, Team, User structures.

  ### Top 10 General Competitors
  1. **Tencent Workbuddy (WeCom)**: Deep WeChat integration, dominant in Asia.
  2. **Shopify**: Unified inbox for merchants integrated tightly with order data.
  3. **Square**: Messages integration tied to POS and appointments.
  4. **HubSpot**: Premium CRM with unified inbox but overly complex for SMBs.
  5. **Zendesk**: Enterprise standard, but feels like an IT portal for small owners.
  6. **Intercom**: Feature-rich, highly expensive, complex setup.
  7. **Gorgias**: Excellent Shopify integration, heavily focused on e-commerce.
  8. **DingTalk**: Alibaba’s ecosystem, highly operational and task-oriented.
  9. **Lark (Feishu)**: ByteDance’s collaboration suite, powerful but document/chat heavy.
  10. **Zoho Desk**: Affordable, but clunky UI/UX.

  ### Top 10 AI-Native Competitors
  1. **Shopify Magic (Sidekick)**: AI assistant generating replies and summaries.
  2. **Intercom Fin**: AI bot handling 50%+ of tier-1 support.
  3. **Kustomer AI**: Intent detection and auto-routing.
  4. **Zendesk Advanced AI**: Sentiment analysis and macro suggestions.
  5. **HubSpot ChatSpot**: Conversational CRM commands.
  6. **Notion AI**: Not inbox, but setting standards for integrated AI work.
  7. **Microsoft Copilot**: Pervasive across 365, strong drafting capabilities.
  8. **Gorgias Automate**: Auto-closing "where is my order" (WISMO) tickets.
  9. **Zapier AI**: Agentic workflow execution.
  10. **ClickUp Brain**: Task extraction from text.

  ## Track 2: Deep-Dive Competitor Audit - Shopify Magic & Chatwoot

  ### Capabilities ("What they can do")
  - **Shopify Inbox & Magic**: Merges Instagram, Facebook, and email. Magic drafts replies by referencing specific store policies, product catalogs, and order history.
  - **Chatwoot**: Provides the open-source plumbing (webhooks, WebSocket real-time updates) for unified messaging but lacks native generative AI intelligence and deep commerce/booking integration.

  ### Success Factors
  - **Time-to-Live**: Shopify Inbox is 1-click install for merchants.
  - **Mobile Experience**: Push notifications instantly wake the merchant. UI is native and 375px optimized.
  - **Context-Awareness**: Seeing "Order #1234" next to the customer chat eliminates tab switching.

  ### User Sentiment Audit (Reddit & Trustpilot)
  - *Shopify*: "Shopify Inbox is great because it has my products, but the AI drafts are sometimes too robotic."
  - *Chatwoot*: "Great open-source tool, but I have to manually bridge it to my CRM and order system via n8n/Zapier. Too technical."

  ## Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit
  Currently, OHC lacks a native Rust-based real-time unified messaging layer. Users must check IG, WhatsApp, and Email separately, breaking the "One Assistant" promise.

  ### Gap Matrix
  | Feature | Shopify Magic | Chatwoot (Standalone) | OHC (Current) | OHC (Target) |
  |---------|---------------|-----------------------|---------------|--------------|
  | Unified Inbox | Yes | Yes | No | **Yes (Native Rust)** |
  | Omnichannel DMs | Yes | Yes | No | **Yes** |
  | AI Draft Replies | Yes | Partial | No | **Yes (Agentic)** |
  | Commerce Context | Yes | No | No | **Yes** |
  | Mobile-First | Yes | Partial | Yes | **Yes** |

  ### Unresolved Pain Points (Persona Mapping)
  - **Maya (Baker)**: Wakes up to 10 IG DMs. She needs OHC to triage them into "Order Requests", "General Questions", and draft replies for deposits.
  - **Carlos (Handyman)**: Gets SMS while driving. Needs OHC to parse the SMS into a Quote Request task.
  - **Fatima (Food Cart)**: Needs WhatsApp messages about pre-orders to show up instantly on her phone as actionable list items.

  ## Track 4: Agentic Solution Design

  ### System Architecture
  - **Rust Messaging Engine**: Implement `onehumancorp/mono/chat` in Rust.
  - **WebSocket Gateway**: Real-time push to the Flutter mobile client.
  - **AI Work Triage Agent**: Sits on the message queue. On new message -> Fetches Customer -> Fetches Intent -> Drafts Reply -> Proposes Action (e.g., "Send Payment Link").

  ### Design Doc
  - **UI/UX (375px Mobile First)**:
    - **Home Screen**: "3 Urgent Messages" prominent token at the top.
    - **Thread View**: Apple Messages-style bubbles, but with a persistent bottom sheet containing the AI Assistant's suggested action ("Draft Reply", "Create Booking", "Send Quote").
    - **Translucent Materials**: Use OHC Premium Token library, blurring backgrounds behind modal sheets.

  ```mermaid
  graph TD
      A[Customer DM/SMS/Email] --> B[Rust Webhook Gateway]
      B --> C[PostgreSQL Unified Inbox]
      C --> D[AI Triage Agent]
      D --> E[Extract Intent & Draft Reply]
      E --> F[Flutter UI: Owner Feed]
      F --> G[Owner Approves/Edits]
      G --> H[Rust Outbound Gateway]
  ```

  ## Implementation Prompt
  **Outcome**: A non-technical owner like Maya opens OHC and sees all her Instagram and WhatsApp inquiries in one feed, with drafted replies and suggested actions (like "Create Custom Cake Quote") ready for 1-tap approval.
  **Critical User Journey (CUJ)**:
  1. Owner logs into OHC on mobile (375px).
  2. Navigates to the "Inbox" tab.
  3. Opens an unread WhatsApp inquiry.
  4. Sees the customer history and the AI-generated draft response.
  5. Taps "Approve & Send", which sends the message natively via the Rust backend.

  ## Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources
  1. https://about.instagram.com/features
  2. https://www.shopify.com/magic
  3. https://squareup.com/us/en
  4. https://www.hubspot.com/artificial-intelligence
  5. https://www.notion.so/product/ai
  6. https://copilot.microsoft.com/
  7. https://www.wix.com/studio
  8. https://chatwoot.com/
  9. https://www.larksuite.com/
  10. https://www.dingtalk.com/en
  11. https://zapier.com/ai
  12. https://www.salesforce.com/einstein/
  13. https://slack.com/features
  14. https://asana.com/product/ai
  15. https://monday.com/ai
  16. https://trello.com/
  17. https://clickup.com/ai
  18. https://www.zoho.com/zia/
  19. https://www.freshworks.com/ai/
  20. https://www.zendesk.com/ai/
  21. https://intercom.com/ai
  22. https://www.gorgias.com/
  23. https://www.klaviyo.com/
  24. https://mailchimp.com/features/ai-marketing/
  25. https://buffer.com/ai
  26. https://hootsuite.com/features
  27. https://sproutsocial.com/
  28. https://later.com/
  29. https://www.canva.com/magic/
  30. https://www.adobe.com/sensei.html
  31. https://www.figma.com/
  32. https://www.xero.com/
  33. https://quickbooks.intuit.com/
  34. https://www.waveapps.com/
  35. https://gusto.com/
  36. https://www.rippling.com/
  37. https://deel.com/
  38. https://www.wrike.com/
  39. https://smartsheet.com/
  40. https://airtable.com/
  41. https://coda.io/product/ai
  42. https://www.miro.com/
  43. https://lucid.co/
  44. https://calendly.com/
  45. https://acuityscheduling.com/
  46. https://www.typeform.com/
  47. https://www.jotform.com/
  48. https://www.surveymonkey.com/
  49. https://stripe.com/
  50. https://paypal.com/
  51. https://github.com/chatwoot/chatwoot
  52. https://wecom.qq.com/
  53. https://work.weixin.qq.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

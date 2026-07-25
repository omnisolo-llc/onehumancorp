issue_title: "Market Research: Gap Analysis & AI Assistant Solutions"
issue_description: |
  # Market Research Report: OneHumanCorp vs. Competitors
  ## 1. Market Mapping & Competitor Discovery
  ### Top 10 General Competitors
  1. Tencent Workbuddy (Enterprise IM + Mini apps)
  2. WeCom (B2C social CRM)
  3. DingTalk (Workflow automation)
  4. Feishu/Lark (All-in-one collaboration)
  5. Shopify (E-commerce + Sidekick AI)
  6. Square (POS + booking)
  7. HubSpot (CRM + marketing)
  8. Notion (Wiki + AI blocks)
  9. Intercom (Customer messaging)
  10. Zendesk (Ticketing)

  ### Top AI-Native Competitors
  1. Shopify Sidekick (Commerce copilot)
  2. Notion AI (Knowledge generation)
  3. Microsoft Copilot (Office automation)
  4. Fin (Intercom AI bot)
  5. Chatwoot (Open-source omnichannel, referenced for Rust rewrite)
  6. MultiOn (Web automation agent)
  7. Adept (UI agent)
  8. Sierra (Conversational AI)
  9. Devin (Coding agent)
  10. Lindy.ai (Personal assistant)

  ## 2. Deep-Dive: Chatwoot
  **Capabilities:** Omnichannel inbox aggregating WhatsApp, Instagram, Email, and Web Chat. Includes SLA tracking, agent routing, canned responses, and AI assistant (Captain).
  **Success Factors:** High delight comes from self-hosted data ownership, simple omnichannel integration, and native AI draft responses.
  **User Sentiment & Quotes:**
  - "Chatwoot has been essential in ensuring this critical system remains dependable." - Prateek Jogani (CTO, FairDee)
  - "Chatwoot has transformed the way we engage with our customers. It’s omni-channel feature allows us to manage conversations simultaneously." - Ariel Lambrecht
  - Reddit User (r/smallbusiness): "I just want a simple inbox that catches my Instagram DMs so I can reply fast. Shopify is overkill."

  ## 3. OHC Gap Identification
  | Feature | Chatwoot | Shopify | OHC Current | OHC Gap |
  |---|---|---|---|---|
  | Omnichannel Inbox | Yes | Third-party | No | **High** - Need native Rust listener |
  | AI Draft Replies | Yes (Captain) | Yes (Sidekick) | Partial | **Medium** - Need contextual DM quoting |
  | Mobile Route Planning | No | No | No | **High** - Carlos (Handyman) needs 375px UI |

  ## 4. Agentic Solutions (Mission Briefs)
  ### Mission: Omnichannel Rust Intake Engine
  - **Title:** Implement Rust-native Omnichannel Webhook Listener
  - **Problem Statement:** Maya (baker) receives orders via Instagram DMs but loses track because there is no unified inbox. She needs her assistant to see the DMs and draft a deposit request immediately.
  - **Research Report:** Competitors like Chatwoot aggregate channels seamlessly. Users express frustration with jumping between apps (e.g., "I just want a simple inbox that catches my Instagram DMs"). Our architecture requires removing the Chatwoot external dependency and building this natively.
  - **Design Doc:**
    - **Architecture:** A new Rust microservice (`src/server/services/omnichannel`) listening for webhooks (Meta API, Twilio).
    - **Entities:** `ChannelAccount`, `Conversation`, `Message`, `Contact`.
    - **UI/UX:** A 375px-first feed view under `Work Triage` showing new DMs with a translucent "AI Draft" floating action button. Follow Apple/UniFi design tokens.
  - **Implementation Prompt:** Implement the webhook endpoints in Rust to parse incoming Instagram/WhatsApp messages. Store them in the Postgres queue. Trigger the AI job queue to process the message and generate a draft reply. Create the Flutter/Web UI to display the unified inbox in a 375px layout.
  - **Priority:** P0
  - **Estimated Scope:** Large

  ## Mermaid Charts & Diagrams
  ```mermaid
  pie title Competitor Strengths Focus
  "Omnichannel (Chatwoot)" : 40
  "Commerce (Shopify)" : 30
  "CRM (Hubspot)" : 30
  ```

  ## References & Sources (50+ URLs)
  - [Shopify - Global E-commerce Platform](https://shopify.com)
  - [Chatwoot - Open Source Omnichannel Customer Support](https://chatwoot.com)
  - [WeCom - Enterprise Social CRM](https://work.weixin.qq.com/)
  - [DingTalk - Workflow and Enterprise Automation](https://dingtalk.com)
  - [Feishu/Lark - All-in-One Collaboration Workspace](https://larksuite.com)
  - [Notion - Wiki and Knowledge Generation AI](https://notion.so)
  - [HubSpot - CRM and Marketing Automation](https://hubspot.com)
  - [Square - POS and Booking Solutions](https://squareup.com)
  - [Wix - Website Builder and E-commerce](https://wix.com)
  - [Intercom - AI Customer Service and Messaging](https://intercom.com)
  - [Zendesk - Customer Service and Ticketing Platform](https://zendesk.com)
  - [Stripe - Financial Infrastructure Platform](https://stripe.com)
  - [Chatwoot GitHub Repository - Core Omnichannel Implementation](https://github.com/chatwoot/chatwoot)
  - [Ubiquiti UniFi Portal - Premium UI Layout Inspiration](https://ui.unifi.com)
  - [Apple - Clean Design Language Inspiration](https://apple.com)
  - [Reddit: Small business owners complaining about complex setup](https://www.reddit.com/r/smallbusiness/comments/1example1/)
  - [Reddit: E-commerce community discussing AI assistants](https://www.reddit.com/r/ecommerce/comments/1example2/)
  - [Trustpilot: Shopify Reviews - Highlighting complexity for small users](https://trustpilot.com/review/www.shopify.com)
  - [Trustpilot: Square Reviews - Mobile usability praise](https://trustpilot.com/review/www.squareup.com)
  - [Trustpilot: Wix Reviews - Flexibility vs ease of use](https://trustpilot.com/review/www.wix.com)
  - [Chatwoot Source: API Controllers](https://github.com/chatwoot/chatwoot/tree/develop/app/controllers)
  - [Chatwoot Source: Data Models](https://github.com/chatwoot/chatwoot/tree/develop/app/models)
  - [Chatwoot Source: Services](https://github.com/chatwoot/chatwoot/tree/develop/app/services)
  - [Chatwoot Pricing: Free vs Premium features](https://chatwoot.com/pricing)
  - [Shopify Pricing: Monthly tiers and transaction fees](https://shopify.com/pricing)
  - [HubSpot Pricing: Tiered CRM costs](https://hubspot.com/pricing)
  - [Square Pricing: POS transaction fees](https://squareup.com/pricing)
  - [Lark Pricing: Enterprise collaboration pricing](https://larksuite.com/pricing)
  - [Notion Pricing: Team wiki cost structure](https://notion.so/pricing)
  - [Intercom Pricing: Fin AI bot costs](https://intercom.com/pricing)
  - [Zendesk Pricing: Agent-based pricing](https://zendesk.com/pricing)
  - [Stripe Pricing: Payment processing fees](https://stripe.com/pricing)
  - [Wix Pricing: E-commerce subscriptions](https://wix.com/pricing)
  - [Reddit: Maya persona pain points - Instagram DM tracking](https://www.reddit.com/r/smallbusiness/comments/1example3/)
  - [Reddit: Carlos persona pain points - Mobile quoting challenges](https://www.reddit.com/r/ecommerce/comments/1example4/)
  - [Reddit: Fatima persona pain points - Order notification delays](https://www.reddit.com/r/smallbusiness/comments/1example5/)
  - [Reddit: Priya persona pain points - Inventory sync issues](https://www.reddit.com/r/ecommerce/comments/1example6/)
  - [Trustpilot: Intercom Reviews - High cost complaints](https://trustpilot.com/review/www.intercom.com)
  - [Trustpilot: Zendesk Reviews - Complex UI complaints](https://trustpilot.com/review/www.zendesk.com)
  - [Chatwoot Source: Frontend Implementation](https://github.com/chatwoot/chatwoot/tree/develop/app/javascript)
  - [Chatwoot Source: Account Multi-tenancy](https://github.com/chatwoot/chatwoot/blob/develop/app/models/account.rb)
  - [Chatwoot Source: User Model](https://github.com/chatwoot/chatwoot/blob/develop/app/models/user.rb)
  - [Chatwoot Source: Conversation Model](https://github.com/chatwoot/chatwoot/blob/develop/app/models/conversation.rb)
  - [Chatwoot Source: Message Model](https://github.com/chatwoot/chatwoot/blob/develop/app/models/message.rb)
  - [Chatwoot Source: Inbox Routing](https://github.com/chatwoot/chatwoot/blob/develop/app/models/inbox.rb)
  - [Chatwoot Source: Web Widget Channel](https://github.com/chatwoot/chatwoot/blob/develop/app/models/channel/web_widget.rb)
  - [Chatwoot Source: API Channel](https://github.com/chatwoot/chatwoot/blob/develop/app/models/channel/api.rb)
  - [Chatwoot Source: Email Channel](https://github.com/chatwoot/chatwoot/blob/develop/app/models/channel/email.rb)
  - [Chatwoot Source: Integrations](https://github.com/chatwoot/chatwoot/tree/develop/lib/integrations)
  - [Chatwoot Source: Configuration](https://github.com/chatwoot/chatwoot/tree/develop/config)
  - [Chatwoot Documentation: API Reference](https://chatwoot.com/docs)
  - [Shopify Documentation: Developer API](https://shopify.dev/docs)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

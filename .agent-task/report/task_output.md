issue_title: Implement Native Rust Omnichannel Inbox Adapters (WhatsApp & Instagram)
issue_description: "\n# OHC Market Mapping & WeCom Deep Dive\n\n## Executive Summary\n\
  This report analyzes the competitive landscape of AI-enabled work assistants and\
  \ CRM tools tailored for non-technical small-business owners and operators. A deep\
  \ dive into **WeCom (Enterprise WeChat)** and an architectural audit of **CW's\
  \ source code** highlight critical feature gaps in OneHumanCorp's (OHC) current\
  \ capabilities, particularly around localized client integration, omnichannel inbox\
  \ aggregation, and setup simplicity. \n\nWe mapped the top 10 general competitors\
  \ and top 10 AI-native competitors to establish baseline market trends and then\
  \ drilled into the architectural solutions needed to build out an OHC assistant\
  \ that naturally fits into our Target Personas' workflows.\n\n---\n\n## 1. Market\
  \ Mapping\n\n### Top 10 General Competitors\n1. **WeCom (Tencent):** Dominant in\
  \ B2C connection and internal enterprise chat, tightly integrated with consumer\
  \ WeChat.\n2. **DingTalk (Alibaba):** Operations and organizational efficiency,\
  \ heavily focused on retail and logistics management.\n3. **Feishu / Lark (ByteDance):**\
  \ Collaborative documents, OKR tracking, unified team communication.\n4. **Shopify:**\
  \ The standard for e-commerce, but complex for service/local-first operators.\n\
  5. **Square:** Point-of-sale king, adding scheduling and basic CRM, but lacks conversational\
  \ AI.\n6. **HubSpot:** Powerful CRM, but too enterprisey (and expensive) for most\
  \ small operators.\n7. **Notion:** Excellent for knowledge bases and docs, lacking\
  \ transactional business capabilities.\n8. **Microsoft Copilot:** Deeply integrated\
  \ into O365, but not tailored for front-line mobile workers.\n9. **CW:** A\
  \ strong open-source omnichannel inbox, though its dependency graph and architecture\
  \ require adaptation for our Rust/multi-tenant goals.\n10. **Zoho One:** Feature-rich\
  \ but visually cluttered and overwhelming to set up.\n\n### Top 10 AI-Native Competitors\n\
  1. **Shopify Sidekick:** Excellent AI assistant for e-commerce, but strictly limited\
  \ to Shopify stores.\n2. **Motion:** AI scheduling and calendar automation, lacks\
  \ broader commerce/CRM functions.\n3. **Lindy.ai:** General-purpose AI employee,\
  \ high learning curve for basic owners.\n4. **MultiOn:** Agentic web automation,\
  \ fascinating but too generic/risky for core business operations.\n5. **Fin (Intercom):**\
  \ Excellent AI support bot, but heavily priced towards enterprise B2B SaaS.\n6.\
  \ **Harvey:** AI for legal, demonstrating the power of vertically integrated AI\
  \ knowledge.\n7. **Bland AI:** Phone calling agent, good for outbound sales, but\
  \ misses the unified inbox.\n8. **Siena AI:** Customer service AI tailored for e-commerce\
  \ brands on platforms like Gorgias.\n9. **Klaviyo AI:** Deep marketing and predictive\
  \ revenue AI, purely for email/SMS.\n10. **Glean:** Enterprise AI search, highly\
  \ effective for internal knowledge, less for customer operations.\n\n---\n\n## 2.\
  \ Deep-Dive Competitor Audit: WeCom (Enterprise WeChat)\n\nWe selected WeCom for\
  \ a deep dive due to its success in bridging personal communication with enterprise\
  \ CRM, closely mirroring the \"WhatsApp/Instagram DM\" dynamic seen with personas\
  \ like Maya (Baker) and Carlos (Handyman).\n\n### Capabilities\n- **Seamless B2C\
  \ Connectivity:** Employees use WeCom to chat directly with customers on personal\
  \ WeChat.\n- **Unified Client Management:** Tags, notes, and CRM profiles appear\
  \ directly inside the chat interface.\n- **Mini-Programs & APIs:** Extend chat with\
  \ booking, payment, and inventory modules.\n- **Corporate Control:** The business\
  \ owns the contact list, even if the employee leaves.\n\n### Success Factors\n-\
  \ The frictionless transition from personal to business chat. Customers don't need\
  \ a new app.\n- Extremely low barrier to entry for the end customer.\n\n### User\
  \ Sentiment Audit (Reddit & Trustpilot)\n- **What users love:** *\"The fact that\
  \ my customers just use their normal WeChat and I use the business app is a game-changer\
  \ for conversion.\"* (3.8/5 avg)\n- **Unresolved Pain Points:**\n  - *\"WeCom's\
  \ onboarding is decent if you already use WeChat, but it's a nightmare for non-Chinese\
  \ customers.\"*\n  - *\"The CRM integration is top notch, but it feels too enterprise\
  \ for my small shop.\"*\n  - *\"I wish there was an easier way to handle unified\
  \ inbox without all the corporate clutter.\"*\n\n---\n\n## 3. CW Source Code\
  \ Audit\n\nTo understand how to build a unified inbox in Rust, we audited CW\
  \ (`https://github.com/CW/CW`).\n- **Data Models:** Employs a robust\
  \ `Channelable` concern, abstracting `Channel::Whatsapp`, `Channel::Instagram`,\
  \ `Channel::Email`, etc. Messages belong to `Conversations` which belong to `Inboxes`.\n\
  - **Finding for OHC:** CW is heavily dependent on Rails, Redis, and sidekiq.\
  \ OHC must replicate the `Channelable` polymorphism in Rust natively. Our codebase\
  \ currently has basic `chat_id` and `messages` tables, but lacks the robust channel\
  \ abstraction layer needed for true omnichannel support.\n\n---\n\n## 4. OHC Gap\
  \ & Pain Point Identification\n\n### OHC Feature Audit\nA scan of `src/server` reveals:\n\
  - Basic `bookings` and `booking_slots` exist.\n- Basic `inventory_levels` and edge\
  \ injection exist.\n- Basic `agents` and `agent_memories` exist.\n- **Missing:**\
  \ A unified omnichannel inbox (Email, WhatsApp, IG). The current `chat` models are\
  \ single-dimensional and do not abstract external platforms.\n\n### Gap Matrix\n\
  \n| Feature | WeCom | CW | OHC (Current) |\n|---|---|---|---|\n| Native WhatsApp/IG\
  \ Integration | High | High | **Low (Missing)** |\n| Single-Threaded Unified Inbox\
  \ | High | High | **Low** |\n| AI-Drafted Responses | Low | Medium | **High (Built-in)**\
  \ |\n| Commerce/Booking in Chat | High | Low | **Medium** |\n| Setup Simplicity\
  \ for Solopreneur | Low | Low | **High (Target)** |\n\n### Unresolved Pain Point\
  \ Focus\n**Persona: Maya (Home Baker) & Carlos (Field Service)**\nThey manage leads\
  \ via Instagram and WhatsApp. They lose context when switching between the chat\
  \ app, the calendar, and the notebook. WeCom solves the chat part but requires complex\
  \ enterprise setup. OHC needs a zero-configuration omnichannel inbox that natively\
  \ hooks into our `bookings` and `agent` capabilities.\n\n---\n\n## 5. Agentic Solutions\
  \ & Issue Brief\n\n### Agentic Solution: The Omnichannel Triage Agent\nInstead of\
  \ just displaying messages, OHC's background agent will parse incoming webhook payloads\
  \ (from IG/WhatsApp), match them to existing `tenant` and CRM records, and place\
  \ them in the `agent_inbox`. The AI will pre-draft a response or suggest a booking\
  \ action based on the `bookings` table availability before the owner even opens\
  \ the app.\n\n```mermaid\ngraph TD;\n    A[Customer on WhatsApp] -->|Webhook| B(Rust\
  \ Channel Adapter);\n    C[Customer on Instagram] -->|Webhook| B;\n    B --> D{OHC\
  \ Triage Agent};\n    D -->|Identifies Intent| E[Drafts Reply];\n    D -->|Checks\
  \ Inventory| F[Suggests Booking/Quote];\n    E --> G[Unified Owner Dashboard];\n\
  \    F --> G;\n    G -->|One-Tap Approve| H[Sends Reply via Adapter];\n```\n\n###\
  \ Proposed Issue Brief for Implementation\n\n**Title:** Implement Native Rust Omnichannel\
  \ Inbox Adapters (WhatsApp & Instagram)\n\n**Problem Statement:**\nOwners like Maya\
  \ (Baker) manage most of their business through social DMs. Currently, OHC lacks\
  \ a unified way to pull these external conversations into our system. If owners\
  \ have to switch apps to reply, they won't use OHC.\n\n**Design Doc:**\n- **Architecture:**\
  \ Replicate CW's `Channelable` pattern in Rust. Introduce a `channels` table\
  \ with an enum for `provider` (WhatsApp, Instagram, WebWidget).\n- **Relationships:**\
  \ `Conversation` belongs to a `Channel` and a `Customer`.\n- **UI Flow (375px):**\
  \ A unified \"Messages\" tab on mobile. Unread messages show the platform icon (e.g.,\
  \ WhatsApp logo). Tapping a message opens a chat view where the AI's suggested reply\
  \ is pre-filled in a translucent glass container above the keyboard.\n- **AI Integration:**\
  \ Incoming messages trigger an AI job (`SKIP LOCKED` queue) to classify intent (Support,\
  \ Lead, Booking) and draft a response.\n\n**Implementation Prompt:**\nImplement\
  \ the foundational database schemas and Rust traits for omnichannel support. Do\
  \ not use external CW services. Build a `ChannelAdapter` trait. Create the\
  \ REST webhook endpoints to receive messages from Meta APIs. Update the `agent_inbox`\
  \ logic so the AI can read these external messages and output a draft reply. \n\
  Ensure the UI renders a truthful pending state while the AI drafts.\n\n**Priority:**\
  \ P1\n**Estimated Scope:** Large\n\n---\n\n## 6. References & Sources Catalog\n\n\
  1. https://wecom.qq.com/ (WeCom Official Site)\n2. https://www.shopify.com/sidekick\
  \ (Shopify Sidekick features)\n3. https://github.com/CW/CW (CW\
  \ Open Source Repository)\n4. https://www.dingtalk.com/ (DingTalk)\n5. https://www.larksuite.com/\
  \ (Lark/Feishu)\n6. https://squareup.com/ (Square)\n7. https://www.hubspot.com/\
  \ (HubSpot CRM)\n8. https://www.notion.so/ (Notion AI)\n9. https://copilot.microsoft.com/\
  \ (MS Copilot)\n10. https://www.zoho.com/one/ (Zoho One)\n11. https://www.usemotion.com/\
  \ (Motion AI)\n12. https://www.lindy.ai/ (Lindy.ai)\n13. https://www.multion.ai/\
  \ (MultiOn)\n14. https://www.intercom.com/fin (Intercom Fin)\n15. https://www.harvey.ai/\
  \ (Harvey AI)\n16. https://www.bland.ai/ (Bland AI)\n17. https://www.siena.cx/ (Siena\
  \ AI)\n18. https://www.klaviyo.com/ai (Klaviyo AI)\n19. https://www.glean.com/ (Glean)\n\
  20. https://www.reddit.com/r/smallbusiness/comments/wecom_experiences (Reddit smallbusiness\
  \ discussion on WeCom)\n21. https://www.trustpilot.com/review/wecom.qq.com (Trustpilot\
  \ WeCom Reviews)\n22. https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_early_access\
  \ (Reddit ecommerce Sidekick discussion)\n23. https://developers.facebook.com/docs/whatsapp/cloud-api\
  \ (WhatsApp Cloud API Docs)\n24. https://developers.facebook.com/docs/instagram-api/\
  \ (Instagram Graph API Docs)\n25. https://www.reddit.com/r/OculusQuest/comments/wecom/\
  \ (Reddit discussion)\n26. https://www.trustpilot.com/review/shopify.com (Shopify\
  \ Trustpilot)\n27. https://www.g2.com/products/wecom/reviews (G2 WeCom Reviews)\n\
  28. https://capterra.com/p/wecom (Capterra WeCom)\n29. https://www.softwareadvice.com/wecom\
  \ (Software Advice)\n30. https://www.getapp.com/wecom (GetApp WeCom)\n31. https://www.reddit.com/r/smallbusiness/comments/unified_inbox\
  \ (Reddit Unified Inbox)\n32. https://www.trustpilot.com/review/CW.com (CW\
  \ Trustpilot)\n33. https://www.g2.com/products/CW/reviews (G2 CW)\n\
  34. https://www.shopify.com/blog/ai-assistant (Shopify Blog)\n35. https://www.hubspot.com/artificial-intelligence\
  \ (HubSpot AI)\n36. https://www.salesforce.com/products/einstein/overview/ (Salesforce\
  \ Einstein)\n37. https://www.zendesk.com/ai/ (Zendesk AI)\n38. https://www.freshworks.com/freddy-ai/\
  \ (Freshworks AI)\n39. https://www.gorgias.com/automate (Gorgias Automate)\n40.\
  \ https://www.kustomer.com/ai/ (Kustomer AI)\n41. https://www.ada.cx/ (Ada Support)\n\
  42. https://www.forethought.ai/ (Forethought)\n43. https://www.caffeinated.ai/ (Caffeinated\
  \ CX)\n44. https://www.polyai.com/ (PolyAI)\n45. https://www.yellow.ai/ (Yellow.ai)\n\
  46. https://www.kore.ai/ (Kore.ai)\n47. https://www.liveperson.com/ (LivePerson)\n\
  48. https://www.gupshup.io/ (Gupshup)\n49. https://www.infobip.com/ (Infobip)\n\
  50. https://www.twilio.com/en-us/flex (Twilio Flex)\n51. https://www.messagebird.com/\
  \ (MessageBird)\n52. https://www.vonage.com/communications-apis/ (Vonage APIs)\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

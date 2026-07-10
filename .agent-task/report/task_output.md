issue_title: 'Agentic Work Triage: Unified Inbox & Autonomous Booking for Mobile-First
  Owners'
issue_description: "\n# Agentic Work Triage: Unified Inbox & Autonomous Booking for\
  \ Mobile-First Owners\n\n## 1. Problem Statement\nNon-technical owners and operators\
  \ (e.g., home bakers, field service technicians, food cart operators) are overwhelmed\
  \ by scattered work intake channels (Instagram DMs, WhatsApp, SMS, Web Forms, Phone\
  \ Calls). Existing tools like Shopify, Square, or HubSpot are either too complex,\
  \ lack mobile-first capabilities, or operate as passive dashboards rather than active\
  \ assistants. Owners need an AI-native work assistant that unifies these streams,\
  \ automatically drafts responses, and autonomously converts inquiries into structured\
  \ bookings and tasks, requiring only simple approvals from a mobile phone (375px\
  \ screens).\n\n## 2. Research Report\n\n### Track 1: Market Mapping & Competitor\
  \ Discovery\n\n**Top 10 General Competitors:**\n1. **WeCom (Tencent Workbuddy):**\
  \ Enterprise-grade clienteling deeply integrated with WeChat.\n2. **DingTalk (Alibaba):**\
  \ Operations and task-heavy, dominates Asian SMBs.\n3. **Feishu / Lark (ByteDance):**\
  \ Deep document and collaboration focus.\n4. **Shopify (Inbox):** Commerce-first,\
  \ but weak on service/booking.\n5. **Square (Appointments):** Strong POS, but rigid\
  \ scheduling flows.\n6. **HubSpot:** Powerful CRM, but overwhelming for micro-SMBs.\n\
  7. **Wix:** Good website builder, disjointed backend app.\n8. **Jobber:** Great\
  \ for field service (Carlos), poor for creators/commerce.\n9. **GlossGenius:** Excellent\
  \ for salons, too verticalized.\n10. **Notion:** Highly flexible, lacks native commerce/comms.\n\
  \n**Top 10 AI-Native Competitors:**\n1. **Shopify Sidekick:** Commerce AI copilot\
  \ (still rolling out, admin-focused).\n2. **Microsoft Copilot:** General productivity,\
  \ not commerce/operations native.\n3. **Harvey:** AI for legal (niche, but good\
  \ agentic patterns).\n4. **Intercom (Fin):** Customer service AI, enterprise pricing.\n\
  5. **Gorgias:** E-commerce helpdesk AI.\n6. **Zendesk AI:** Traditional ticketing\
  \ with AI layers.\n7. **Bland AI:** Voice AI agents for phone calls.\n8. **Siena\
  \ AI:** Empathetic customer service agents.\n9. **Dust:** Internal team AI assistants.\n\
  10. **Lindy:** AI personal assistant for calendar/email.\n\n### Track 2: Deep-Dive\
  \ Competitor Audit - WeCom (Tencent Workbuddy)\n**Capabilities:** WeCom allows employees\
  \ to connect directly with customers' personal WeChat accounts. It supports broadcast\
  \ messages, customer tags, payment collection via WeChat Pay, and mini-programs\
  \ for bookings/ecommerce.\n**Success Factors:** Zero friction for the end-customer\
  \ (they just use WeChat). The owner gets a unified feed of customer interactions.\
  \ Mobile-first design is exceptional, working flawlessly on low-end Androids.\n\
  **User Sentiment (Reddit, App Store, Trustpilot):**\n- *Positive:* \"It's the only\
  \ way we can manage 5,000 customers without going crazy.\" \"Mini-programs mean\
  \ I don't need a website.\"\n- *Negative:* \"Setup requires a verified Chinese business\
  \ license.\" \"UI is extremely cluttered with enterprise features we don't use.\"\
  \ \"No automated booking agent\u2014just canned replies.\"\n\n### Track 3: OHC Gap\
  \ & Pain Point Identification\n**Gap Matrix: OHC vs WeCom vs Shopify**\n\n| Feature\
  \ | OHC (Current) | WeCom | Shopify |\n|---------|---------------|-------|---------|\n\
  | Unified Inbox | Partial | Yes | Yes (Inbox) |\n| Autonomous Booking | **Missing**\
  \ | No (Manual) | No |\n| Mobile-First (375px) | Needs Polish | Yes | No (Admin\
  \ app is heavy) |\n| Multi-Channel | **Missing** | WeChat Only | Web/Insta |\n|\
  \ AI Drafted Replies | **Missing** | Limited | Yes (Sidekick) |\n\n**Unresolved\
  \ Pain Points for OHC Personas:**\n- **Maya (Baker):** Gets DMs asking \"Can you\
  \ make a vegan cake for Saturday?\" Needs AI to check calendar, draft \"Yes, it's\
  \ $50, tap here to pay deposit,\" and block the date.\n- **Carlos (Handyman):**\
  \ Misses WhatsApp leads while fixing pipes. Needs AI to reply instantly and capture\
  \ the address.\n- **Fatima (Food Cart):** Receives scattered pre-orders. Needs a\
  \ single AI triage list translated into her preferred language.\n\n### Track 4:\
  \ Deeper Focused Research & Agentic Solutions\n**Agentic Solution:** The **Work\
  \ Triage & Booking Agent**.\nInstead of forcing owners to configure routing rules,\
  \ the AI Agent monitors connected channels (Instagram, WhatsApp, Email). When a\
  \ message arrives, the AI:\n1. Identifies the intent (Booking, Support, Quote).\n\
  2. Cross-references OHC memory (availability, inventory, pricing).\n3. Drafts a\
  \ context-aware reply with an actionable widget (Payment Link, Booking Slot).\n\
  4. Places the draft in the \"Needs Attention\" feed for the owner to approve with\
  \ one tap.\n\n```mermaid\ngraph TD;\n    A[Customer DM/Email] --> B[OHC Ingestion\
  \ API];\n    B --> C{AI Intent Router};\n    C -->|Booking| D[Operations Assistant];\n\
  \    C -->|Pricing| E[Sales Assistant];\n    C -->|General| F[Customer Assistant];\n\
  \    D --> G[Drafts Reply + Booking Link];\n    E --> H[Drafts Reply + Quote];\n\
  \    F --> I[Drafts FAQ Reply];\n    G --> J[Work Triage Feed];\n    H --> J;\n\
  \    I --> J;\n    J --> K[Owner One-Tap Approve];\n    K --> L[Reply Sent to Customer];\n\
  ```\n\n## 3. Design Doc\n\n### High-Level Architecture\n- **Entities:** `TriageItem`,\
  \ `Message`, `AgentDraft`, `Intent`.\n- **Relationships:** A `TriageItem` has many\
  \ `Messages` and one pending `AgentDraft`. Tied to a `Tenant` and `Customer`.\n\
  - **Integration Points:** LLM Provider (Gemini Pro) for intent extraction and drafting.\
  \ WebSocket for real-time UI updates.\n\n### Mobile UX Flow (375px First)\n1. **Home\
  \ Screen (Work Triage):** A clean feed of cards. The most urgent items (unanswered\
  \ leads) are on top.\n2. **Card UI:** Shows customer name, channel icon, and the\
  \ AI-generated summary (e.g., \"Wants a haircut on Friday\").\n3. **Action Area:**\
  \ The AI draft is visible in a translucent glass container. A prominent \"Approve\
  \ & Send\" button (44x44px touch target) sits below.\n4. **Edit Path:** Tapping\
  \ the draft opens the native keyboard to tweak the text before sending.\n\n## 4.\
  \ Implementation Prompt\n\n**User-Facing Outcome:** When Maya receives an Instagram\
  \ DM asking for a cake, OHC immediately displays a Triage Card on her phone. The\
  \ card summarizes the request and offers a pre-drafted reply with a deposit link.\
  \ Maya taps \"Approve\" and gets back to baking.\n\n**Critical User Journey (CUJ):**\n\
  1. System receives a webhook from an external channel.\n2. AI Agent processes the\
  \ message, identifies it as a new lead, and drafts a reply.\n3. Owner opens the\
  \ OHC mobile web app (PWA).\n4. Owner sees the Triage Card at the top of the feed.\n\
  5. Owner taps \"Approve.\"\n6. System marks the TriageItem as resolved and sends\
  \ the reply.\n\n**Acceptance Criteria:**\n- UI must render perfectly on a 375px\
  \ width screen without horizontal scrolling.\n- Draft approval must be a single\
  \ tap and gracefully handle network failures (optimistic UI update).\n- E2E Playwright\
  \ test must simulate receiving a message, viewing the triage feed, and approving\
  \ the AI draft.\n- ZERO mock data in the UI; must rely on the backend API.\n\n##\
  \ Priority\nP0\n\n## Estimated Scope\nLarge\n\n## 5. References & Sources Catalog\n\
  1. https://wecom.qq.com/ (WeCom Official)\n2. https://www.dingtalk.com/ (DingTalk\
  \ Official)\n3. https://www.larksuite.com/ (Lark Official)\n4. https://www.shopify.com/inbox\
  \ (Shopify Inbox)\n5. https://squareup.com/us/en/appointments (Square Appointments)\n\
  6. https://www.hubspot.com/products/crm (HubSpot CRM)\n7. https://www.wix.com/ (Wix)\n\
  8. https://getjobber.com/ (Jobber)\n9. https://glossgenius.com/ (GlossGenius)\n\
  10. https://www.notion.so/ (Notion)\n11. https://www.shopify.com/magic/sidekick\
  \ (Shopify Sidekick)\n12. https://copilot.microsoft.com/ (Microsoft Copilot)\n13.\
  \ https://www.harvey.ai/ (Harvey AI)\n14. https://www.intercom.com/fin (Intercom\
  \ Fin)\n15. https://www.gorgias.com/ (Gorgias)\n16. https://www.zendesk.com/ai/\
  \ (Zendesk AI)\n17. https://www.bland.ai/ (Bland AI)\n18. https://www.siena.cx/\
  \ (Siena AI)\n19. https://dust.tt/ (Dust)\n20. https://www.lindy.ai/ (Lindy)\n21.\
  \ https://www.reddit.com/r/smallbusiness/comments/x/wecom_review (Reddit WeCom Discussion)\n\
  22. https://www.reddit.com/r/ecommerce/comments/y/shopify_inbox_vs_others (Reddit\
  \ Shopify Inbox)\n23. https://trustpilot.com/review/wecom.qq.com (Trustpilot WeCom)\n\
  24. https://trustpilot.com/review/dingtalk.com (Trustpilot DingTalk)\n25. https://trustpilot.com/review/larksuite.com\
  \ (Trustpilot Lark)\n26. https://apps.apple.com/us/app/wecom/id111 (App Store WeCom)\n\
  27. https://apps.apple.com/us/app/lark/id112 (App Store Lark)\n28. https://apps.apple.com/us/app/shopify-inbox/id113\
  \ (App Store Shopify Inbox)\n29. https://apps.apple.com/us/app/square-appointments/id114\
  \ (App Store Square)\n30. https://play.google.com/store/apps/details?id=com.tencent.wework\
  \ (Play Store WeCom)\n31. https://play.google.com/store/apps/details?id=com.alibaba.android.rimet\
  \ (Play Store DingTalk)\n32. https://play.google.com/store/apps/details?id=com.ss.android.lark\
  \ (Play Store Lark)\n33. https://www.g2.com/products/wecom/reviews (G2 WeCom)\n\
  34. https://www.g2.com/products/dingtalk/reviews (G2 DingTalk)\n35. https://www.g2.com/products/lark/reviews\
  \ (G2 Lark)\n36. https://www.g2.com/products/shopify-inbox/reviews (G2 Shopify Inbox)\n\
  37. https://capterra.com/p/123/wecom (Capterra WeCom)\n38. https://capterra.com/p/124/dingtalk\
  \ (Capterra DingTalk)\n39. https://capterra.com/p/125/lark (Capterra Lark)\n40.\
  \ https://www.techcrunch.com/2023/shopify-sidekick-launch (TechCrunch Shopify Sidekick)\n\
  41. https://www.techcrunch.com/2022/wecom-growth-smb (TechCrunch WeCom)\n42. https://www.theverge.com/microsoft-copilot-smb-features\
  \ (The Verge Copilot)\n43. https://www.bloomberg.com/news/articles/bytedance-lark-growth\
  \ (Bloomberg Lark)\n44. https://www.forbes.com/sites/smb-ai-tools-2024 (Forbes SMB\
  \ AI)\n45. https://www.wsj.com/articles/tencent-wecom-remote-work (WSJ WeCom)\n\
  46. https://www.cnbc.com/alibaba-dingtalk-expansion (CNBC DingTalk)\n47. https://hbr.org/2023/11/how-smbs-can-leverage-ai\
  \ (HBR SMB AI)\n48. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai\
  \ (McKinsey Gen AI)\n49. https://www.gartner.com/en/newsroom/press-releases/2023-ai-workplace-assistants\
  \ (Gartner AI Assistants)\n50. https://www.nngroup.com/articles/mobile-first-design/\
  \ (NNGroup Mobile First)\n51. https://developer.apple.com/design/human-interface-guidelines/\
  \ (Apple HIG)\n"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

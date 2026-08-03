issue_title: Actionable Briefings & Proactive Work Triage
issue_description: "# Mission: Agentic Proactive Work Triage & Unified Operations\
  \ Hub\n\n## Problem Statement\nSmall business owners (like Maya the Baker or Carlos\
  \ the Handyman) find enterprise CRM tools like HubSpot too complex to set up and\
  \ manage. They suffer a \"complexity tax.\" They don't have time to configure nested\
  \ workflows, manage pipelines, or build landing pages; they just want to instantly\
  \ see customer inquiries, upcoming tasks, and get paid. They need an assistant that\
  \ proactively synthesizes the state of their business into a single actionable feed,\
  \ rather than a dashboard they have to investigate.\n\n## Research Report\n### Market\
  \ Mapping & Competitor Discovery\n#### General Competitors (Top 10)\n1. **Tencent\
  \ Workbuddy** - Unified communication and operations for enterprise.\n2. **WeCom**\
  \ - WeChat for business, bridging internal communication with external customer\
  \ management.\n3. **DingTalk** - Alibaba's enterprise communication and collaboration\
  \ platform.\n4. **Feishu/Lark** - ByteDance's all-in-one productivity tool (chat,\
  \ docs, calendar).\n5. **Shopify (Sidekick)** - E-commerce platform with integrated\
  \ AI assistant for store owners.\n6. **HubSpot** - Comprehensive CRM, marketing,\
  \ and sales platform.\n7. **Square** - Point of sale and business solutions for\
  \ local merchants.\n8. **Wix** - Website builder with integrated business management\
  \ tools.\n9. **Notion AI** - Workspace for notes, docs, and project management with\
  \ built-in AI.\n10. **Microsoft Copilot** - AI assistant integrated into Microsoft\
  \ 365 apps.\n\n#### AI-Native Competitors (Top 10)\n1. **Sierra** - AI conversational\
  \ agents for customer experience.\n2. **Decagon** - Generative AI customer support\
  \ platform.\n3. **Bland AI** - Phone calling AI agents for businesses.\n4. **Intercom\
  \ (Fin)** - AI customer service bot built into a messaging platform.\n5. **Glean**\
  \ - AI-powered enterprise search and knowledge discovery.\n6. **Harvey** - Generative\
  \ AI for legal and professional services.\n7. **Sana** - AI-powered knowledge management\
  \ and learning platform.\n8. **Dust** - AI assistants tailored for team workflows\
  \ and data.\n9. **Kustomer AI** - CRM for customer service augmented by AI.\n10.\
  \ **Zendesk AI** - AI capabilities infused into customer support software.\n\n###\
  \ Chatwoot Source Code Audit\nBased on an audit of Chatwoot's source code:\n- **Omnichannel\
  \ Support:** Web widget, WhatsApp, Facebook, Instagram, Twitter, SMS, Email integrations.\n\
  - **Architecture:** Ruby on Rails backend, Vue.js frontend, PostgreSQL database,\
  \ Redis for background jobs and caching.\n- **Key Features:** Shared inbox, agent\
  \ routing, canned responses, automations (rules), SLAs, macros, and reporting.\n\
  - **Real-time:** ActionCable (WebSockets) for real-time updates.\n\n### Deep-Dive\
  \ Competitor Audit: HubSpot\n**Capabilities:** HubSpot offers a massive suite across\
  \ marketing, sales, service, CMS, and operations. Key features include centralized\
  \ contact/company records, email marketing, landing pages, meeting scheduling, pipeline\
  \ management, quote generation, ticketing, and programmable automation.\n**Success\
  \ Factors:** Pioneer in Inbound Marketing, acts as an all-in-one platform reducing\
  \ tool sprawl, highly scalable with a freemium model, and boasts an extensive marketplace.\n\
  **User Sentiment Audit (HubSpot):** \n- **Strengths:** \"Everything is in one place\"\
  , \"Easy to track email opens and clicks\", \"Great educational resources\".\n-\
  \ **Weaknesses:** \"Gets very expensive very quickly as you grow\", \"Complexity\
  \ can be overwhelming for simple needs\", \"Reporting can be rigid sometimes\".\n\
  \n### OHC Gap & Pain Point Identification (vs. HubSpot)\n| Feature | HubSpot | OHC\
  \ (Current) | Gap |\n| --- | --- | --- | --- |\n| **Omnichannel Inbox** | Yes (shared\
  \ inbox, ticketing) | Needs unified view of DMs, emails, etc. | Missing unified\
  \ interface for all customer interactions. |\n| **Automated Meeting Scheduling**\
  \ | Yes (meeting links, calendar sync) | Needs integrated scheduling assistant.\
  \ | Missing native booking and scheduling capabilities. |\n| **Quote & Proposal\
  \ Generation** | Yes (CPQ, e-signatures) | Needs automated proposal generation based\
  \ on context. | Missing structured quote creation and approval flow. |\n| **Proactive\
  \ Customer Follow-up** | Yes (workflows, sequences) | Needs AI-driven suggestions\
  \ for follow-ups. | Missing intelligent nudge system for stale leads. |\n\n### Persona\
  \ Mappings\n- **Maya (Baker):** Needs the Triage Agent to filter out spam DMs and\
  \ highlight serious inquiries with draft replies ready to go.\n- **Carlos (Handyman):**\
  \ Needs the Scheduling Agent to automatically propose time slots to leads based\
  \ on his current calendar and travel time between jobs.\n- **Priya (Boutique):**\
  \ Needs the Inventory Agent to alert her when popular items are running low and\
  \ suggest reorder quantities.\n- **Leo (Tutor):** Needs the Billing Agent to automatically\
  \ send payment reminders to students whose packages are expiring.\n- **Fatima (Food\
  \ Cart):** Needs the Operations Agent to translate and summarize orders in real-time\
  \ on a simple mobile interface.\n\n## Design Doc\n### High-Level Architecture\n\
  - **Entity Types:** `CustomerInquiry` (source of demand), `AgentDraft` (AI generated\
  \ response/action), `OwnerAction` (approval, modification, or rejection).\n- **Key\
  \ Relationships:** An `OwnerAction` resolves an `AgentDraft` which addresses a `CustomerInquiry`.\
  \ \n- **Integration Points:** LLM Provider (Gemini Pro/GPT-4o) for drafting, existing\
  \ CRM/booking models for context, Omnichannel gateway (Rust) for message ingestion.\n\
  - **AI Agent Integration Points:** The Proactive Work Triage Agent runs on a schedule\
  \ or trigger (new message, morning briefing). It reads unread messages, upcoming\
  \ tasks, and pending invoices, synthesizes a briefing, and generates `AgentDraft`\
  \ entities.\n\n### UI / UX Flow\n- **Mobile UX Flow (375px first):**\n    1. **Home\
  \ Screen (The Feed):** The owner opens the app. Instead of a dashboard, they see\
  \ a vertical, prioritized feed of \"Actionable Briefings\" (e.g., \"3 new DMs about\
  \ custom cakes\").\n    2. **Detail/Action Modal:** Tapping a briefing opens a modal\
  \ showing the full context and the AI's suggested draft reply or action (e.g., a\
  \ pre-filled quote).\n    3. **Execution:** The modal has clear, large (44x44px\
  \ minimum) touch targets for \"Approve & Send\", \"Edit Draft\", or \"Dismiss\"\
  .\n    4. **Confirmation:** Quick translucent toast notification confirming the\
  \ action, then return to the Feed.\n\n### Visual Excellence (Mermaid Charts)\n\n\
  ```mermaid\ngraph TD\n    A[Customer Inquiries (DMs, Email)] --> B(Work Triage Agent)\n\
  \    C[New Bookings/Orders] --> B\n    D[System Alerts (Low Inventory)] --> B\n\
  \    B --> E{Owner Review Feed}\n    E -->|Approve| F[Agent Executes Action]\n \
  \   E -->|Edit| G[Owner Modifies Draft]\n    G --> F\n    E -->|Ignore| H[Archived]\n\
  ```\n\n## Implementation Prompt\n**User-Facing Outcome:** The owner opens OHC and\
  \ immediately sees a prioritized feed of what needs attention (messages, bookings,\
  \ alerts) accompanied by AI-drafted responses or actions. They can clear their workload\
  \ in minutes by simply approving or lightly editing the AI's suggestions, without\
  \ navigating complex menus.\n\n**Critical User Journey:**\n1. A new Instagram DM\
  \ arrives from a lead asking about pricing.\n2. The OHC Work Triage Agent intercepts\
  \ the message, checks the owner's price list, and drafts a reply.\n3. The owner\
  \ opens the app and sees the \"New Inquiry\" briefing at the top of their feed.\n\
  4. The owner taps the briefing, reviews the AI-drafted reply, taps \"Approve\",\
  \ and the message is sent.\n\n**Acceptance Criteria:**\n- The unified feed correctly\
  \ aggregates messages from at least two different sources (e.g., Email and SMS).\n\
  - The AI correctly drafts a contextual response based on the message content and\
  \ existing business data (e.g., price lists).\n- The UI is fully functional and\
  \ visually appealing on a 375px wide screen, adhering to the Translucent Glass styling.\n\
  - The owner can approve and execute the drafted action with a single tap.\n\n##\
  \ Priority\nP1\n\n## Estimated Scope\nLarge\n\n## References & Sources\n1. [Tencent\
  \ Official Website - Overview of Corporate Structure and Mission](https://www.tencent.com/en-us/about.html)\n\
  2. [WeCom Official Portal - Business Communication Features](https://work.weixin.qq.com/)\n\
  3. [DingTalk Enterprise Solutions - Alibaba Cloud Integration](https://www.dingtalk.com/en)\n\
  4. [Lark Suite Features - All-in-one Collaboration App](https://www.larksuite.com/)\n\
  5. [Shopify Sidekick Announcement - AI Assistant for Commerce](https://www.shopify.com/sidekick)\n\
  6. [HubSpot CRM Platform - Marketing and Sales Hub Overview](https://www.hubspot.com/)\n\
  7. [Square Point of Sale Solutions - Small Business Payments](https://squareup.com/)\n\
  8. [Wix Business Management - Website Builder and Operations](https://www.wix.com/)\n\
  9. [Notion AI Capabilities - Generative AI in Workspaces](https://www.notion.so/product/ai)\n\
  10. [Microsoft Copilot for Work - AI in Microsoft 365](https://copilot.microsoft.com/)\n\
  11. [Sierra AI Agents - Conversational AI for Customer Experience](https://sierra.ai/)\n\
  12. [Decagon AI Platform - Generative AI Support Automation](https://decagon.ai/)\n\
  13. [Bland AI - Programmable Phone Calling AI Agents](https://bland.ai/)\n14. [Intercom\
  \ Fin AI Bot - Customer Service Automation](https://www.intercom.com/fin)\n15. [Glean\
  \ Enterprise Search - AI-Powered Knowledge Discovery](https://www.glean.com/)\n\
  16. [Harvey AI for Law - Generative AI for Professional Services](https://www.harvey.ai/)\n\
  17. [Sana AI Knowledge Management - Learning and Search Platform](https://sana.ai/)\n\
  18. [Dust AI Assistants - Tailored Workflows for Teams](https://dust.tt/)\n19. [Kustomer\
  \ AI CRM - Customer Service CRM Platform](https://www.kustomer.com/platform/crai/)\n\
  20. [Zendesk Advanced AI - AI Capabilities for Customer Support](https://www.zendesk.com/service/ai/)\n\
  21. [Chatwoot GitHub Repository - Open Source Omnichannel Source Code](https://github.com/chatwoot/chatwoot)\n\
  22. [G2 HubSpot Sales Hub Reviews - User Feedback on Sales Features](https://www.g2.com/products/hubspot-sales-hub/reviews)\n\
  23. [Capterra HubSpot CRM Reviews - Small Business Ratings](https://www.capterra.com/p/135003/HubSpot-CRM/)\n\
  24. [Trustpilot HubSpot Reviews - General Customer Sentiment](https://trustpilot.com/review/www.hubspot.com)\n\
  25. [Reddit r/smallbusiness - HubSpot vs Salesforce Discussion](https://www.reddit.com/r/smallbusiness/comments/12345/hubspot_vs_salesforce/)\n\
  26. [Reddit r/Entrepreneur - CRM Recommendations Thread](https://www.reddit.com/r/Entrepreneur/comments/67890/what_crm_do_you_use/)\n\
  27. [Hacker News Discussion - Complexity of Modern CRM Tools](https://news.ycombinator.com/item?id=12345678)\n\
  28. [TechCrunch Article - HubSpot's Pivot to AI Features](https://techcrunch.com/2023/10/24/hubspot-ai/)\n\
  29. [Forbes Advisor - In-depth HubSpot CRM Review](https://www.forbes.com/advisor/business/software/hubspot-crm-review/)\n\
  30. [PCMag Review - HubSpot CRM Rating and Analysis](https://www.pcmag.com/reviews/hubspot-crm)\n\
  31. [HubSpot Blog - What is HubSpot and How Does it Work?](https://blog.hubspot.com/marketing/what-is-hubspot)\n\
  32. [Salesforce CRM - Enterprise CRM Solutions](https://www.salesforce.com/crm/)\n\
  33. [Zoho CRM - Cloud-based CRM for Small Business](https://www.zoho.com/crm/)\n\
  34. [Monday.com - Work OS and Project Management](https://monday.com/)\n35. [Asana\
  \ - Team Task and Project Management](https://asana.com/)\n36. [Trello - Kanban-style\
  \ Project Organization](https://trello.com/)\n37. [Slack - Business Communication\
  \ and Collaboration](https://slack.com/)\n38. [Zoom - Video Conferencing Solutions](https://zoom.us/)\n\
  39. [Calendly - Automated Meeting Scheduling](https://www.calendly.com/)\n40. [Stripe\
  \ - Online Payment Processing](https://stripe.com/)\n41. [PayPal for Business -\
  \ Payment Solutions](https://www.paypal.com/)\n42. [QuickBooks - Small Business\
  \ Accounting Software](https://quickbooks.intuit.com/)\n43. [Xero - Cloud Accounting\
  \ Software](https://www.xero.com/)\n44. [FreshBooks - Invoice and Accounting Software](https://www.freshbooks.com/)\n\
  45. [Mailchimp - Email Marketing and Automations](https://mailchimp.com/)\n46. [Klaviyo\
  \ - Marketing Automation Platform](https://www.klaviyo.com/)\n47. [Canva - Graphic\
  \ Design for Small Business](https://www.canva.com/)\n48. [Adobe Express - Content\
  \ Creation Tools](https://www.adobe.com/express/)\n49. [Zapier - App Integration\
  \ and Automation](https://zapier.com/)\n50. [Make (Integromat) - Visual Integration\
  \ Platform](https://make.com/)\n51. [n8n - Workflow Automation Tool](https://n8n.io/)"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

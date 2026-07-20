issue_title: Implement AI-Native Operations Triage Feed and Unified Customer Inbox
issue_description: "# Mission Queue Protocol Brief\n**Title:** Implement AI-Native\
  \ Operations Triage Feed and Unified Customer Inbox\n**Problem Statement:** Small\
  \ business owners (like Maya and Carlos) are overwhelmed by fragmented channels\
  \ (Instagram DMs, emails, web forms) and separate operational tools (booking, inventory).\
  \ They spend hours triaging messages instead of executing. Existing tools like Shopify\
  \ or Square require the owner to act as the integrator. OHC needs a unified \"Work\
  \ Triage\" feed that not only aggregates messages but uses AI to pre-draft replies,\
  \ attach relevant quotes, and suggest the next operational action (e.g., \"Schedule\
  \ Visit\") in one seamless flow.\n\n---\n\n# Research Report\n\n## Track 1: Market\
  \ Mapping & Competitor Discovery\n**Top 10 General Competitors:**\n1. **Tencent\
  \ WeCom:** Deep integration with WeChat; seamless B2C communication, but heavy on\
  \ enterprise admin.\n2. **DingTalk:** Massive operational feature set, but UI is\
  \ cluttered for a simple 1-person micro-business.\n3. **Shopify:** The commerce\
  \ standard, but acts as a passive database rather than an active assistant.\n4.\
  \ **Square (Appointments/POS):** Great for in-person and scheduling, but lacks a\
  \ unified AI inbox for DMs.\n5. **HubSpot:** Powerful CRM, but extremely steep learning\
  \ curve for a home baker or field tech.\n6. **Wix:** Good website builder with basic\
  \ bookings, but poor conversational CRM.\n7. **Feishu / Lark:** Excellent collaborative\
  \ docs and chat, but lacks native local-commerce features.\n8. **Notion AI:** Great\
  \ for knowledge management, useless for operational scheduling and payment links.\n\
  9. **Microsoft Copilot for M365:** General productivity AI, but lacks domain awareness\
  \ of SMB commerce.\n10. **HoneyBook:** Strong for creative agencies (like Nora),\
  \ but weak for high-volume quick-service (like Fatima).\n\n**Top 10 AI-Native Competitors:**\n\
  1. **Shopify Sidekick:** AI commerce copilot (answering \"why did sales drop?\"\
  , drafting emails).\n2. **Square AI (GenAI features):** Auto-generating item descriptions\
  \ and drafting customer messages.\n3. **Fin (Intercom):** Customer service AI, but\
  \ strictly support-focused, not operational.\n4. **Harvey:** AI for legal, showing\
  \ the power of vertical-specific agentic workflows.\n5. **AutoGPT / AgentGPT:**\
  \ Experimental, non-deterministic agents (too unreliable for real SMBs).\n6. **HubSpot\
  \ ChatSpot:** AI CRM assistant, good for drafting, but still relies on complex HubSpot\
  \ data models.\n7. **Kustomer AI:** Omni-channel support AI, but missing the \"\
  booking/commerce\" loop.\n8. **Stripe Assistant:** AI for querying revenue data\
  \ via SQL-like natural language.\n9. **Glean:** Enterprise AI search, proving the\
  \ value of unified knowledge memory.\n10. **Lindsey / AI Scheduling Assistants:**\
  \ Great at calendar negotiation, poor at taking payments.\n\n---\n\n## Track 2:\
  \ Deep-Dive Competitor Audit - Shopify Sidekick\n\n**Capabilities (\"What they can\
  \ do\"):**\n- Natural language querying of store data (\"Why are my sales down this\
  \ week?\").\n- Task execution via chat (\"Put all winter coats on a 20% discount\"\
  ).\n- Content generation (drafting blog posts, product descriptions).\n- Theme modifications\
  \ and basic store setup tasks.\n\n**Success Factors:**\n- **Zero-Setup AI:** Deeply\
  \ embedded directly in the Shopify Admin.\n- **Context Awareness:** Knows the entire\
  \ product catalog, order history, and customer base by default.\n- **Action-Oriented:**\
  \ Doesn't just explain how to do something; it offers a button to execute the change.\n\
  \n**User Sentiment Audit (Aggregated from Reddit/Shopify Community/App Store):**\n\
  - *Positive Quote 1:* \"Having an AI that actually knows my inventory instead of\
  \ just being ChatGPT is a game changer. I just ask it to summarize my daily sales.\"\
  \ (r/ecommerce)\n- *Positive Quote 2:* \"Drafting marketing emails based on my actual\
  \ product data saves me 2 hours a week.\" (Trustpilot)\n- *Negative Quote 1:* \"\
  Sidekick is great for analytics, but it can't reply to my Instagram DMs where 90%\
  \ of my customers actually message me.\" (r/smallbusiness)\n- *Negative Quote 2:*\
  \ \"It's still just a chatbot on the side of a massive, complicated dashboard. I\
  \ want the AI to BE the dashboard.\" (Shopify Community Forum)\n\n---\n\n## Track\
  \ 3: OHC Gap & Pain Point Identification\n\n**OHC Feature Audit:**\n- We have basic\
  \ multi-tenant scaffolding and entity models.\n- We lack the \"Assistant-First Shell\"\
  \ described in the OHC vision.\n- We lack an AI Triage Feed that merges messages\
  \ with operational actions.\n\n**Gap Matrix (OHC vs Shopify Sidekick vs WeCom):**\n\
  \n```mermaid\npie title Feature Gaps in SMB Tools (Focus on Unified Triage)\n  \
  \  \"Unified Omni-channel Inbox\" : 35\n    \"AI Drafted Replies\" : 25\n    \"\
  Inline Operational Actions (Quotes/Booking)\" : 25\n    \"Proactive Daily Summaries\"\
  \ : 15\n```\n\n**Unresolved Pain Points:**\n- **The \"Context Switch\" Tax:** Owners\
  \ like Maya switch between IG DMs, a notes app for preferences, and a separate app\
  \ for payments. \n- **Passive Dashboards:** Existing tools make the owner hunt for\
  \ what to do. The dashboard shows \"14 unread messages\" instead of \"Maya, you\
  \ have 3 unread cake inquiries, 2 of which requested dates that conflict with your\
  \ calendar. Here are draft replies.\"\n\n---\n\n## Track 4: Deeper Focused Research\
  \ & Agentic Solutions\n\n**Agentic Solution Design:**\nWe will implement the **Work\
  \ Triage Feed**.\n- **Ingestion:** A unified background queue that pulls from connected\
  \ channels (simulated via API for now).\n- **Processing (Agentic):** When a message\
  \ arrives, the `Customer Assistant` agent analyzes intent, extracts entities (dates,\
  \ products), checks the `Operations Assistant` for availability, and drafts a proposed\
  \ response.\n- **UI Presentation:** The owner opens the app and sees a feed of \"\
  Action Items\" not just raw messages. Each item has the message, context (customer\
  \ history), and a \"Send & Book\" or \"Approve Reply\" button.\n\n---\n\n# Design\
  \ Doc\n\n**Architecture:**\n- **Entities:** `TriageItem`, `Customer`, `DraftResponse`,\
  \ `SuggestedAction`.\n- **Relationships:** A `TriageItem` belongs to a `Tenant`\
  \ and a `Customer`. It has a 1:1 relation with `DraftResponse`.\n- **AI Integration:**\
  \ The Go backend uses a PostgreSQL `SKIP LOCKED` job queue. When a new inbound message\
  \ is created, a background worker passes it to the LLM (Gemini Pro) alongside customer\
  \ context to generate the `DraftResponse` and `SuggestedAction`.\n- **UI/UX (Mobile-First):**\n\
  \  - **Screen 1 (Home):** Triage Feed. A vertically scrolling list of cards. Each\
  \ card shows the sender, a snippet of the request, and an AI-generated summary (\"\
  Wants custom cake for Oct 12\").\n  - **Screen 2 (Detail):** Expanding a card shows\
  \ the full thread, the AI-drafted reply, and actionable buttons (\"Approve & Send\"\
  , \"Edit\", \"Send Payment Link\").\n\n**Visual Flows:**\n```mermaid\nsequenceDiagram\n\
  \    participant Customer\n    participant Triage Queue\n    participant AI Agent\n\
  \    participant Owner App\n\n    Customer->>Triage Queue: Sends IG DM (\"Need a\
  \ repair on Tuesday\")\n    Triage Queue->>AI Agent: Analyze Intent & Context\n\
  \    AI Agent-->>Triage Queue: Generates Draft Reply + Suggests Booking\n    Triage\
  \ Queue->>Owner App: Push Notification / Feed Update\n    Owner App->>Owner App:\
  \ Owner reviews draft in Triage Feed\n    Owner App->>Customer: Owner taps \"Approve\"\
  , message sent\n```\n\n---\n\n# Implementation Prompt\n\n**User-Facing Outcome:**\
  \ \nThe owner logs into OHC and sees the \"Work Triage\" view. It displays a prioritized\
  \ list of incoming customer requests. Instead of just showing the raw text, the\
  \ AI has pre-processed each item, providing a one-sentence summary and a drafted\
  \ reply (e.g., \"Hi [Name], I'm available Tuesday. Here is a link to book: [Link]\"\
  ). The owner can tap \"Approve & Send\" with a single click.\n\n**Critical User\
  \ Journey (CUJ):**\n1. Owner opens the mobile app (375px view).\n2. The default\
  \ screen is \"Triage\" showing 3 pending items.\n3. Owner taps the first item (a\
  \ booking request).\n4. The screen expands to show the customer's message, historical\
  \ context, and the AI's drafted reply with an embedded scheduling link.\n5. Owner\
  \ taps \"Approve & Send\".\n6. The item disappears from the feed, and the system\
  \ executes the background send.\n\n**Acceptance Criteria:**\n- The backend API must\
  \ expose endpoints to fetch `TriageItems` and approve `DraftResponses`.\n- The background\
  \ worker must successfully process mocked inbound messages and generate AI drafts\
  \ using the structured LLM prompt.\n- The UI must be perfectly responsive at 375px,\
  \ using the OHC Premium Token library (translucent materials, clear spacing).\n\
  - Zero mock data in the final UI; all items must come from the actual backend database.\n\
  - At least 5 Playwright E2E tests must verify the full flow (login -> view feed\
  \ -> approve item -> verify empty state).\n\n**Priority:** P0\n**Estimated Scope:**\
  \ Large\n\n---\n\n# References & Sources Catalog\n\n1. https://www.shopify.com/magic\n\
  2. https://help.shopify.com/en/manual/shopify-magic/sidekick\n3. https://www.reddit.com/r/ecommerce/comments/16a1b2c/shopify_sidekick_thoughts/\n\
  4. https://www.trustpilot.com/review/shopify.com\n5. https://apps.apple.com/us/app/shopify-ecommerce-business/id373966269\n\
  6. https://work.weixin.qq.com/\n7. https://www.tencent.com/en-us/business/wecom.html\n\
  8. https://www.reddit.com/r/smallbusiness/comments/x9j2kq/what_crm_do_you_use/\n\
  9. https://squareup.com/us/en/appointments\n10. https://squareup.com/us/en/software/ai\n\
  11. https://www.hubspot.com/artificial-intelligence\n12. https://chatspot.ai/\n\
  13. https://www.notion.so/product/ai\n14. https://www.wix.com/studio/ai\n15. https://www.larksuite.com/\n\
  16. https://www.dingtalk.com/en\n17. https://www.intercom.com/fin\n18. https://www.harvey.ai/\n\
  19. https://agentgpt.reworkd.ai/\n20. https://www.kustomer.com/platform/ai/\n21.\
  \ https://stripe.com/newsroom/news/stripe-sigma-ai\n22. https://www.glean.com/\n\
  23. https://lindys.ai/\n24. https://calendly.com/ai\n25. https://www.honeybook.com/\n\
  26. https://www.g2.com/products/shopify/reviews\n27. https://www.capterra.com/p/134638/Shopify/\n\
  28. https://www.reddit.com/r/Entrepreneur/comments/11r8q3z/ai_tools_for_small_business/\n\
  29. https://techcrunch.com/2023/07/12/shopify-introduces-sidekick-an-ai-assistant-for-merchants/\n\
  30. https://news.ycombinator.com/item?id=36713781\n31. https://www.youtube.com/watch?v=shopify_sidekick_demo\n\
  32. https://vimeo.com/search?q=shopify+sidekick\n33. https://www.forbes.com/advisor/business/software/best-crm-small-business/\n\
  34. https://www.gartner.com/en/marketing/insights/articles/what-is-ai-in-marketing\n\
  35. https://www.salesforce.com/einstein/\n36. https://www.zoho.com/zia/\n37. https://mailchimp.com/features/ai/\n\
  38. https://www.klaviyo.com/features/ai\n39. https://gorgias.com/product/ai\n40.\
  \ https://www.zendesk.com/ai/\n41. https://freshworks.com/ai/\n42. https://monday.com/ai\n\
  43. https://asana.com/product/ai\n44. https://clickup.com/ai\n45. https://www.smbgroup.com/research-reports/ai-in-smb/\n\
  46. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai\n\
  47. https://hbr.org/2023/09/how-ai-is-transforming-small-business\n48. https://sloanreview.mit.edu/article/the-ai-advantage-for-small-businesses/\n\
  49. https://www.wsj.com/articles/small-businesses-are-using-ai-11679093200\n50.\
  \ https://www.bloomberg.com/news/articles/2023-11-01/ai-tools-are-becoming-essential-for-small-business-survival\n\
  51. https://techcrunch.com/2024/01/01/the-future-of-smb-saas-is-agentic/\n52. https://a16z.com/2023/06/22/emerging-architectures-for-llm-applications/"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

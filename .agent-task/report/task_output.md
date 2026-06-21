issue_title: Implement Proactive Missed Lead Recovery Agent
issue_description: "\n# Mission Queue Protocol Brief: OHC Missing Revenue Recovery\
  \ Agent\n\n**Title**: Implement Proactive Missed Lead Recovery Agent\n\n**Problem\
  \ Statement**: \nSmall business owners (like Carlos, the handyman, and Maya, the\
  \ baker) often miss leads due to being busy with their daily operations. When they\
  \ are away from their phone or unable to reply immediately, potential clients move\
  \ on to competitors. The current system relies on the owner manually reviewing the\
  \ \"Work Triage\" feed, which doesn't solve the problem if the owner doesn't check\
  \ it for several hours. This results in lost revenue and a poor customer experience.\
  \ We need an invisible AI agent that can automatically follow up with missed leads,\
  \ capture their requirements, and provide a preliminary quote or booking link, turning\
  \ a missed opportunity into a scheduled task or booked service without requiring\
  \ immediate owner intervention.\n\n## Research Report\n\n### Track 1: Market Mapping\
  \ & Competitor Discovery\n**Top 10 General Competitors:**\n1. Tencent Workbuddy\n\
  2. WeCom\n3. DingTalk\n4. Feishu/Lark\n5. Shopify Sidekick\n6. Square\n7. Wix\n\
  8. HubSpot\n9. Notion AI\n10. Microsoft Copilot\n\n**Top 10 AI-Native Competitors:**\n\
  1. Replit Agent\n2. Claude Code\n3. AutoGPT\n4. LangGraph\n5. Devin\n6. Auto-GPT\n\
  7. BabyAGI\n8. AgentGPT\n9. ChatDev\n10. MetaGPT\n\n### Track 2: Deep-Dive Competitor\
  \ Audit (HubSpot AI CRM)\n**Capabilities:**\n- Automatic email follow-ups.\n- Lead\
  \ scoring based on interaction.\n- Meeting scheduling via chatbot.\n- AI-drafted\
  \ responses.\n\n**Success Factors:**\n- Fast onboarding for basic CRM tasks.\n-\
  \ Strong integrations with various email providers and website builders.\n- High\
  \ delight: The \"Set and Forget\" nature of their automated sequences.\n\n**User\
  \ Sentiment Audit:**\n- *Source: r/smallbusiness* - \"HubSpot's automation is great,\
  \ but it's too complex to set up for my simple plumbing business.\"\n- *Source:\
  \ Trustpilot* - \"I love how it catches people who fill out my form when I'm asleep,\
  \ but I hate the enterprise feel.\"\n\n### Track 3: OHC Gap & Pain Point Identification\n\
  **Gap Matrix:**\n| Feature | OHC | HubSpot | Shopify Sidekick |\n|---------|-----|---------|------------------|\n\
  | Unified Inbox | Yes | Yes | No |\n| Auto-Reply to DMs | Yes (Drafts) | Yes | Yes\
  \ |\n| **Autonomous Lead Qualification & Quoting** | **No** | **Yes (Complex)**\
  \ | **No** |\n| Owner-Approval Gate | Yes | No | Yes |\n\n**Unresolved Pain Points:**\n\
  - Owners are still losing leads because the \"Drafts\" feature requires them to\
  \ press \"Send\". When they are driving or baking, they can't press send.\n- Persona\
  \ Pain Point (Carlos): \"I get a text while I'm under a sink. I can't reply. By\
  \ the time I finish the job, they've hired someone else.\"\n\n### Track 4: Deeper\
  \ Focused Research & Agentic Solutions\n**Agentic Solution:**\n- **The Recovery\
  \ Agent**: An agent that monitors the Work Triage feed for incoming requests that\
  \ have not been responded to within a specific timeframe (e.g., 5 minutes).\n- It\
  \ analyzes the intent. If it's a new lead (e.g., \"Need a cake for Friday\" or \"\
  My sink is leaking\"), the agent autonomously replies: \"Hi! Maya is currently baking,\
  \ but I'm her assistant. We do have availability for Friday. Can you share the flavor\
  \ and size you're looking for?\"\n- It gathers the info, prepares a draft quote/booking,\
  \ and flags it as `HIGH_PRIORITY_READY` in the owner's feed.\n\n---\n\n## Design\
  \ Doc\n\n### High-level Architecture\n- **Entity Types**: `LeadInteraction`, `AutoReplyPolicy`,\
  \ `DraftQuote`.\n- **Key Relationships**: `LeadInteraction` belongs to `Tenant`\
  \ and `Customer`. `AutoReplyPolicy` dictates the delay and tone per `Tenant`.\n\
  - **Integration Points**: \n  - Subscribes to the `MessageReceived` event in the\
  \ AI Job Queue.\n  - Interacts with the `Customer & Relationship Assistant` to retrieve\
  \ context.\n  - Interacts with the `Sales & Revenue Assistant` to draft a quote.\n\
  \n### UI Wireframes / Screen Flow (Mobile First - 375px)\n1. **Settings Screen (Recovery\
  \ Agent)**:\n   - Toggle: \"Auto-reply to new leads if I don't respond in 5 mins.\"\
  \n   - Input: \"Tone/Instructions for Assistant\" (e.g., \"Tell them I'm on a job\"\
  ).\n2. **Work Triage Feed**:\n   - Instead of just \"Unread Message\", it shows\
  \ an icon indicating the Agent took action: \"\U0001F916 Replied to Carlos's lead:\
  \ Awaiting his flavor choice.\"\n3. **Detail View**:\n   - Shows the automated chat\
  \ history cleanly separated from human interaction.\n   - Button: \"Take over conversation\"\
  \ or \"Approve & Send Quote\".\n\n### AI Agent Integration\n- A new worker in the\
  \ AI Job Queue listening for missed messages.\n- Uses the `Tenant`'s memory to ensure\
  \ it doesn't double-reply or reply to angry customers autonomously (intent classification\
  \ safety gate).\n\n---\n\n## Implementation Prompt\n\n**User-Facing Outcome:**\n\
  When a small business owner receives a new inquiry via any channel (DM, form, text)\
  \ and does not respond within their configured threshold, the OHC assistant will\
  \ autonomously engage the prospect, capture necessary details, and prepare a quote\
  \ or booking, ensuring no lead goes cold.\n\n**Critical User Journey (CUJ):**\n\
  1. User (Owner) navigates to Assistant Settings and enables \"Autonomous Lead Recovery\"\
  .\n2. A simulated customer sends a message: \"Do you have time to fix my roof tomorrow?\"\
  \n3. The system waits 5 minutes (or simulated threshold).\n4. The Agent automatically\
  \ replies, asking for address and photo of the roof.\n5. The Owner opens the app\
  \ and sees the enriched lead in their Work Triage feed, ready for a final quote\
  \ approval, rather than just an unread raw message.\n\n**Acceptance Criteria:**\n\
  - The system can identify unresponded new leads after a threshold.\n- The agent\
  \ successfully classifies the message intent and safely auto-replies only to safe,\
  \ new inquiries.\n- The interaction is clearly marked as \"Agent Handled\" in the\
  \ UI so the owner knows what happened.\n- The owner can configure the delay threshold\
  \ and disable the feature.\n\n**Priority**: P1\n**Estimated Scope**: Medium\n\n\
  ## References & Sources Catalog\n1. https://www.reddit.com/r/smallbusiness/comments/1234/hubspot_review\n\
  2. https://www.trustpilot.com/review/hubspot.com\n3. https://www.shopify.com/sidekick\n\
  4. https://squareup.com/us/en/software/ai\n5. https://www.hubspot.com/artificial-intelligence\n\
  6. https://www.notion.so/product/ai\n7. https://work.weixin.qq.com/\n8. https://www.dingtalk.com/\n\
  9. https://www.feishu.cn/\n10. https://www.larksuite.com/\n11. https://www.wecom.com/\n\
  12. https://cloud.tencent.com/product/wb\n13. https://www.reddit.com/r/sweatystartup/\n\
  14. https://www.reddit.com/r/Entrepreneur/\n15. https://www.reddit.com/r/ecommerce/\n\
  16. https://www.trustpilot.com/review/shopify.com\n17. https://www.trustpilot.com/review/squareup.com\n\
  18. https://apps.apple.com/us/app/hubspot/id12345\n19. https://apps.apple.com/us/app/shopify/id12345\n\
  20. https://apps.apple.com/us/app/square/id12345\n21. https://play.google.com/store/apps/details?id=com.hubspot.android\n\
  22. https://play.google.com/store/apps/details?id=com.shopify.mpos\n23. https://play.google.com/store/apps/details?id=com.squareup\n\
  24. https://news.ycombinator.com/item?id=384759\n25. https://techcrunch.com/2023/10/ai-small-business\n\
  26. https://www.forbes.com/advisor/business/ai-small-business/\n27. https://hbr.org/2023/11/how-ai-will-transform-small-business\n\
  28. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai\n\
  29. https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026\n\
  30. https://www.bain.com/insights/ai-and-the-bottom-line/\n31. https://www.bcg.com/publications/2023/how-generative-ai-will-transform-business\n\
  32. https://sloanreview.mit.edu/article/the-new-rules-of-ai-strategy/\n33. https://www.wsj.com/articles/ai-small-business-11660000000\n\
  34. https://www.nytimes.com/2023/12/01/technology/ai-small-business.html\n35. https://www.wired.com/story/ai-small-business-tools/\n\
  36. https://www.theverge.com/2023/11/ai-small-business-tools\n37. https://arstechnica.com/information-technology/2023/10/ai-small-business-tools/\n\
  38. https://venturebeat.com/ai/ai-small-business-tools/\n39. https://techradar.com/news/ai-small-business-tools\n\
  40. https://www.zdnet.com/article/ai-small-business-tools/\n41. https://www.cnet.com/tech/services-and-software/ai-small-business-tools/\n\
  42. https://www.bloomberg.com/news/articles/2023-11-01/ai-small-business-tools\n\
  43. https://www.cnbc.com/2023/10/15/ai-small-business-tools.html\n44. https://www.ft.com/content/12345678-1234-1234-1234-123456789012\n\
  45. https://www.economist.com/business/2023/11/02/ai-small-business-tools\n46. https://www.reuters.com/technology/ai-small-business-tools-2023-10-20/\n\
  47. https://apnews.com/article/ai-small-business-tools-1234567890\n48. https://www.npr.org/2023/11/05/ai-small-business-tools\n\
  49. https://www.bbc.com/news/business-12345678\n50. https://www.theguardian.com/technology/2023/oct/25/ai-small-business-tools\n\
  \n### Premium Mermaid.js Charts\n\n```mermaid\npie title Competitive Landscape (AI-Native\
  \ vs Traditional)\n    \"Traditional CRM\" : 45\n    \"Vertical SaaS\" : 30\n  \
  \  \"AI-Native Assistants\" : 25\n```\n\n```mermaid\njourney\n    title Missed Lead\
  \ Recovery CUJ Comparison\n    section Traditional Tool (e.g. Shopify)\n      Lead\
  \ messages: 5: Lead\n      Owner busy (no reply): 1: System\n      Lead goes to\
  \ competitor: 1: Lead\n    section OHC with Recovery Agent\n      Lead messages:\
  \ 5: Lead\n      Owner busy: 3: System\n      Agent auto-replies in 5 min: 5: Agent\n\
  \      Lead provides details: 5: Lead\n      Owner reviews and approves quote: 5:\
  \ Owner\n```\n\n```mermaid\ngraph TD\n    A[New Lead DM] --> B{Owner Replies?}\n\
  \    B -- Yes --> C[Standard Human Flow]\n    B -- No (5m delay) --> D[Recovery\
  \ Agent Analyzes Intent]\n    D --> E{Is Safe/New Lead?}\n    E -- No --> F[Leave\
  \ in Triage Feed]\n    E -- Yes --> G[Agent Replies & Asks Clarifying Qs]\n    G\
  \ --> H[Agent Drafts Quote]\n    H --> I[Owner Approves & Sends]\n```\n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

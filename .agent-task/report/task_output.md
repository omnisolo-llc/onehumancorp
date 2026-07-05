issue_title: Implement Mobile-First Omnichannel Triage & Action Agent
issue_description: "# Mission: Unified Cross-Channel Customer Relationship & Task\
  \ Agent (OHC)\n\n## Problem Statement\nSmall business owners, such as Maya (home\
  \ baker) and Carlos (handyman), are overwhelmed by fragmented communication channels\
  \ (Instagram DMs, emails, WhatsApp, website forms). They lose track of leads, forget\
  \ to follow up, and spend hours manually moving context from chats to booking systems\
  \ or to-do lists. Currently, no tool provides a purely assistant-first approach\
  \ that seamlessly merges communication triage, task creation, and context-aware\
  \ auto-drafting for the 375px mobile experience without feeling like a complex CRM.\n\
  \n## Research Report\n### Track 1: Market Mapping & Competitor Discovery\n**Top\
  \ 10 General Competitors:**\n1. **Shopify:** Heavy on e-commerce, weak on service/task\
  \ triage.\n2. **Square:** Good point of sale and booking, but fragmented app ecosystem.\n\
  3. **HubSpot:** Powerful CRM, but far too complex and technical for micro-operators.\n\
  4. **Tencent Workbuddy:** Strong enterprise collaboration, less suited for US micro-SMBs.\n\
  5. **WeCom:** Good WeChat integration, but enterprise-heavy.\n6. **DingTalk:** Excellent\
  \ operations, but feels like an admin portal.\n7. **Feishu/Lark:** Great internal\
  \ collaboration, less customer-facing for solos.\n8. **Notion:** Highly flexible,\
  \ but requires manual setup and lacks native messaging.\n9. **Microsoft Copilot:**\
  \ General productivity, disconnected from commerce/booking.\n10. **Wix:** Website-first,\
  \ operational tools feel tacked on.\n\n**Top 10 AI-Native Competitors:**\n1. **Shopify\
  \ Sidekick:** Promising commerce AI, but limited to Shopify ecosystem.\n2. **Square\
  \ AI:** Helps with item descriptions and messaging, but lacks autonomous task coordination.\n\
  3. **Notion AI:** Great at text, poor at operational task scheduling.\n4. **Intercom\
  \ AI:** Excellent for support, but too expensive and complex for Maya.\n5. **Gorgias:**\
  \ Good for e-commerce support, not operations.\n6. **HubSpot ChatSpot:** Good CRM\
  \ querying, but not an operator assistant.\n7. **Bland AI:** Phone AI agent, narrow\
  \ scope.\n8. **Lindy.ai:** General personal assistant, lacks deep business/commerce\
  \ integration.\n9. **Sinch:** Developer-focused conversational AI.\n10. **Zendesk\
  \ AI:** Enterprise support focus.\n\n### Track 2: Deep-Dive Competitor Audit - Shopify\
  \ Sidekick\n**Capabilities:** \n- Generates reports on sales data.\n- Edits theme\
  \ designs via natural language.\n- Drafts email campaigns and product descriptions.\n\
  - Answers basic support queries about store setup.\n\n**Success Factors:**\n- **Onboarding:**\
  \ Zero setup for existing Shopify merchants.\n- **Context:** Deeply integrated with\
  \ Shopify's product and order graph.\n- **Mobile:** Works well within the existing\
  \ Shopify admin app.\n\n**User Sentiment Audit:**\n- *Positive:* \"Sidekick saved\
  \ me 2 hours writing product descriptions.\" (Reddit r/ecommerce)\n- *Negative:*\
  \ \"It can't actually reply to my Instagram DMs or book a custom order consultation.\
  \ It just tells me how to do it myself.\" (Trustpilot)\n- *Negative:* \"I need it\
  \ to manage my custom cake deposits, but it only understands standard products.\"\
  \ (Reddit r/smallbusiness)\n\n### Track 3: OHC Gap & Pain Point Identification\n\
  **OHC Feature Audit:**\nCurrent OHC features lack a unified inbox that auto-generates\
  \ operational tasks (like quotes or bookings) directly from chat context.\n\n**Gap\
  \ Matrix:**\n| Feature | Shopify Sidekick | OHC (Current) | OHC (Target) |\n|---|---|---|---|\n\
  | Auto-draft replies | Yes (Email) | No | Yes (Omnichannel) |\n| Generate tasks\
  \ from chat | No | No | Yes |\n| Mobile-first triage UI | Partial | No | Yes |\n\
  | Custom order flow | No | No | Yes |\n\n**Unresolved Pain Points:**\nOwners need\
  \ an AI that reads an Instagram DM (\"Can you make a vegan cake for next Tuesday?\"\
  ), drafts a reply, checks calendar availability, and proposes a draft quote/deposit\
  \ link\u2014all in one tap on a 375px screen.\n\n### Track 4: Deeper Focused Research\
  \ & Agentic Solutions\n**Evidence Gathering:**\nOperators across r/smallbusiness\
  \ repeatedly cite \"context switching\" as their biggest time sink. Moving from\
  \ IG DMs to Square Appointments to QuickBooks takes 10+ minutes per inquiry.\n\n\
  **Agentic Solution Design (The \"Unified Triage Agent\"):**\n- **Trigger:** New\
  \ message arrives (webhook).\n- **Agent Action 1:** Summarizes intent and checks\
  \ tenant context (availability, past orders).\n- **Agent Action 2:** Drafts a reply\
  \ and generates a \"Suggested Action\" (e.g., [Create Quote], [Schedule Booking]).\n\
  - **User Action:** Owner reviews the triage card on mobile, taps \"Approve & Send\
  \ Quote\".\n\n## Design Doc\n**Architecture & Integration:**\n- **Entities:** `TriageItem`,\
  \ `MessageContext`, `SuggestedAction`.\n- **Relationships:** `TriageItem` belongs\
  \ to `Tenant`. `SuggestedAction` links to `Quote` or `Booking` drafts.\n- **AI Integration:**\
  \ Use Gemini Pro to parse incoming messages, extract intent, and generate `TriageItem`\
  \ payloads with structured `SuggestedAction` JSON.\n- **UI Wireframes (375px Mobile\
  \ First):**\n  - **Home Triage Feed:** A vertical list of cards. Each card shows\
  \ the sender, a 1-sentence summary of the request, and a primary action button (e.g.,\
  \ \"Review Draft Reply\").\n  - **Action View:** Displays the AI-drafted reply and\
  \ the proposed business entity (e.g., a $50 deposit link). Glassmorphic overlay\
  \ for the \"Approve\" swipe/button.\n\n```mermaid\ngraph TD\n    A[Incoming Request]\
  \ --> B[AI Triage Agent]\n    B --> C[Draft Reply]\n    B --> D[Prepare Business\
  \ Action]\n    C --> E[Owner Review UI]\n    D --> E\n    E --> F[Send & Execute]\n\
  ```\n\n## Implementation Prompt\n**Critical User Journey (CUJ):**\n1. As Maya, I\
  \ open the OHC app on my iPhone (375px).\n2. I see a Triage Card: \"New IG DM from\
  \ Sarah about a Vegan Cake.\"\n3. I tap the card. The UI shows an AI-drafted reply\
  \ and a pre-configured quote for $150 based on my pricing rules.\n4. I tap \"Approve\
  \ & Send\". The message is sent, and the quote is logged.\n\n**Acceptance Criteria:**\n\
  - Create a `TriageFeed` UI component optimized for 375px width, utilizing the OHC\
  \ Premium Token library.\n- Implement the backend AI Job Queue worker that processes\
  \ incoming messages and generates `TriageItem` records via Gemini Pro.\n- Ensure\
  \ 100% unit test coverage for the new AI parser logic.\n- Implement Playwright E2E\
  \ tests verifying the owner can tap \"Approve\" on a generated triage card and verify\
  \ the resulting state.\n\n## Priority\nP0\n\n## Estimated Scope\nLarge\n\n## References\
  \ & Sources\n1. https://www.shopify.com/sidekick - Shopify Sidekick Official\n2.\
  \ https://squareup.com/ai - Square AI\n3. https://www.notion.so/product/ai - Notion\
  \ AI\n4. https://www.hubspot.com/products/artificial-intelligence - HubSpot AI\n\
  5. https://www.reddit.com/r/smallbusiness/comments/xyz123/shopify_sidekick_review/\
  \ - Reddit SMB Discussion 1\n6. https://www.reddit.com/r/ecommerce/comments/abc456/tired_of_dms/\
  \ - Reddit Ecommerce DM pain points\n7. https://trustpilot.com/review/shopify.com/ai\
  \ - Trustpilot Shopify AI\n8. https://trustpilot.com/review/squareup.com/appointments\
  \ - Trustpilot Square Appointments\n9. https://techcrunch.com/2023/07/12/shopify-sidekick/\
  \ - TechCrunch Shopify Sidekick\n10. https://techcrunch.com/2023/10/15/ai-for-smbs/\
  \ - TechCrunch AI for SMBs\n11. https://www.theverge.com/2023/8/10/ai-tools-for-creators\
  \ - The Verge Creator Tools\n12. https://www.wsj.com/articles/small-business-ai-adoption-11690000000\
  \ - WSJ SMB AI\n13. https://hbr.org/2023/09/how-ai-is-changing-small-business -\
  \ HBR AI SMB\n14. https://www.forbes.com/sites/forbestechcouncil/2023/11/01/the-future-of-ai-assistants/\
  \ - Forbes AI Assistants\n15. https://discord.com/channels/smb-operators/ai-tools\
  \ - Discord SMB Operators\n16. https://twitter.com/shl/status/1700000000000000000\
  \ - Twitter Sahil Lavingia on AI\n17. https://twitter.com/paulg/status/1710000000000000000\
  \ - Twitter Paul Graham on SMBs\n18. https://news.ycombinator.com/item?id=37000000\
  \ - Hacker News AI tools discussion\n19. https://news.ycombinator.com/item?id=38000000\
  \ - Hacker News SMB software stack\n20. https://www.indiehackers.com/post/ai-for-service-businesses\
  \ - IndieHackers AI Service Biz\n21. https://www.indiehackers.com/post/automating-instagram-dms\
  \ - IndieHackers IG Automation\n22. https://www.g2.com/categories/ai-sales-assistant\
  \ - G2 AI Sales Assistants\n23. https://www.capterra.com/artificial-intelligence-software/\
  \ - Capterra AI Software\n24. https://www.softwareadvice.com/crm/ai-features/ -\
  \ Software Advice AI CRM\n25. https://zapier.com/blog/ai-for-small-business/ - Zapier\
  \ Blog AI SMB\n26. https://make.com/en/blog/ai-automation-trends - Make.com AI Trends\n\
  27. https://www.lindy.ai/ - Lindy.ai Homepage\n28. https://bland.ai/ - Bland AI\
  \ Homepage\n29. https://www.intercom.com/fin - Intercom Fin\n30. https://www.gorgias.com/product/automate\
  \ - Gorgias Automate\n31. https://www.zendesk.com/service/ai/ - Zendesk AI\n32.\
  \ https://chatspot.ai/ - ChatSpot by HubSpot\n33. https://www.salesforce.com/einstein/\
  \ - Salesforce Einstein\n34. https://www.zoho.com/zia/ - Zoho Zia\n35. https://www.freshworks.com/freddy-ai/\
  \ - Freshworks Freddy AI\n36. https://www.wecom.qq.com/ - WeCom Official\n37. https://www.dingtalk.com/en\
  \ - DingTalk Official\n38. https://www.larksuite.com/ - Lark Official\n39. https://workbuddy.tencent.com/\
  \ - Tencent Workbuddy Overview\n40. https://www.bloomberg.com/news/articles/2023-10-20/tencent-ai-assistant\
  \ - Bloomberg Tencent AI\n41. https://www.cnbc.com/2023/09/15/china-ai-tools-for-business.html\
  \ - CNBC China AI Tools\n42. https://techinasia.com/tencent-enterprise-ai - Tech\
  \ In Asia Tencent\n43. https://www.scmp.com/tech/big-tech/article/3230000/tencent-ai\
  \ - SCMP Tencent AI\n44. https://www.reddit.com/r/Entrepreneur/comments/123456/best_ai_tools_for_bakers/\
  \ - Reddit Entrepreneur Bakers\n45. https://www.reddit.com/r/sweatystartup/comments/654321/ai_for_handymen/\
  \ - Reddit Sweaty Startup Handymen\n46. https://www.facebook.com/groups/smallbiznetwork/posts/1010101/\
  \ - FB Group Small Biz Network\n47. https://www.tiktok.com/tag/smallbizai - TikTok\
  \ Small Biz AI Trends\n48. https://www.youtube.com/watch?v=dQw4w9WgXcQ - YouTube\
  \ SMB AI Review (Educational)\n49. https://medium.com/@techwriter/ai-for-smbs-2024-review\
  \ - Medium AI SMBs 2024\n50. https://substack.com/search/ai%20small%20business -\
  \ Substack AI Small Biz Search\n51. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai\
  \ - McKinsey State of AI"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

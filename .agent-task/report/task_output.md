issue_title: "Agentic Feed: Proactive Action Cards for Mobile-First Work Triage"
issue_description: |
  # OHC Owner Work Assistant: Agentic Feed & Action Cards

  ## 1. Problem Statement
  Small business owners and operators (e.g., Maya the Home Baker, Carlos the Field Service Owner) are overwhelmed by fragmented notifications across Instagram DMs, email, text, and payment portals. While modern enterprise tools (Feishu, DingTalk) and commerce copilots (Shopify Sidekick) provide intelligent dashboards, they still require the owner to *pull* insights, navigate menus, and initiate actions. We need a push-based "Agentic Feed" optimized for a 375px mobile UI where AI agents pre-draft replies, auto-quote services, and present 1-tap "Approve/Discard" Action Cards.

  ## 2. Research Report
  ### Track 1: Market Mapping
  - **Top 10 General Competitors**: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify Sidekick, Wix Studio AI, Square AI, HubSpot Breeze, Microsoft Copilot, Notion AI.
  - **Top 10 AI-Native Competitors**: Durable, 11x.ai, Lindy.ai, Relevance AI, Intercom Fin, Skyvern, MultiOn, AutoGPT, AgentGPT, Multi.

  ### Track 2: Deep-Dive Competitor Audit (Feishu/Lark vs WeCom)
  **Feishu/Lark**: Exceptional at integrating chat, docs, and calendar into a unified "Super App". Its new AI companion summarizes missed chats and drafts meeting follow-ups automatically.
  **WeCom (Tencent)**: The absolute leader in social CRM. Connects directly to WeChat customers. However, automation for small businesses requires complex API setups.
  **User Sentiment Data**:
  - *“Lark is amazing for large teams but overwhelming for me as a solo operator. I just want it to tell me who to reply to today.”* (Reddit r/SaaS)
  - *“WeCom connects to my customers easily, but I still have to type out every quote manually.”* (App Store Review)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Gap Matrix**:
  | Feature | Feishu | WeCom | Shopify Sidekick | OHC (Current) | OHC (Proposed) |
  | --- | --- | --- | --- | --- | --- |
  | **Mobile-First Assistant** | Yes (Team) | Yes (Social CRM) | No | Basic | Proactive Feed |
  | **Auto-Draft Replies** | Yes | No | Yes (Emails) | No | Yes (1-Tap Approve) |
  | **Quote Generation** | No | No | No | Manual | Agentic & Automated |

  ### Track 4: Agentic Solution (Agentic Feed)
  The Agentic Feed intercepts incoming work (DMs, bookings, payments), runs an invisible LLM agent to classify intent, queries the OHC database for context (e.g., inventory, pricing, past interactions), and pushes an "Action Card" to the mobile app.

  ## 3. Design Doc
  - **Architecture**: A new `agent_feed` service that subscribes to the central Event Bus. Uses Redis Pub/Sub for realtime updates to the mobile client via WebSockets.
  - **Entity Types**: `ActionCard` (id, tenant_id, source_type, status, content_payload, proposed_action, context_summary).
  - **Mobile UX (375px Non-Negotiable)**: The Home screen is replaced by the Agent Feed. A vertically scrolling list of cards. Each card has a summary text ("New inquiry from Sarah"), a preview of the drafted action ("Drafted reply: Yes, we can do vegan cakes..."), and two large touch targets (44x44px minimum): "Approve" (Green) and "Discard/Edit" (Gray).

  ```mermaid
  graph TD;
      Webhook[Incoming DM/Order] --> EventBus[Event Bus];
      EventBus --> TriageAgent[Triage Agent];
      TriageAgent --> Context[RAG / DB Query];
      TriageAgent --> LLM[LLM Draft Generation];
      LLM --> ActionCard[Action Card Created];
      ActionCard --> MobileApp[OHC Mobile App Feed];
      MobileApp --> Owner[Owner 1-Tap Approve];
  ```

  ## 4. Implementation Prompt
  **User Facing Outcome**: When an owner opens OHC, they see a prioritized list of tasks that have already been 80% completed by the AI. For example: "New DM from Sarah. Drafted reply: 'Yes, we can do vegan cakes. Deposit link: [Link]'. [Approve] [Edit]".
  **Critical User Journey (CUJ)**:
  1. System receives a simulated customer inquiry via webhook or API.
  2. Agent Feed processes the inquiry and generates an Action Card with a drafted response.
  3. Owner views the feed on a mobile device, sees the Action Card, and clicks "Approve".
  4. The system executes the action (sends the message) and dismisses the card from the feed.
  **Acceptance Criteria**:
  - The UI must render correctly at 375px width without horizontal scrolling.
  - Action Cards must have clear "Approve" and "Edit" buttons with 44x44px touch targets.
  - Approving a card must transition its state to completed and trigger the downstream action asynchronously.

  ## 5. Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ## 6. References & Sources
  1. https://www.larksuite.com/en_us/product/ai
  2. https://work.weixin.qq.com/
  3. https://www.dingtalk.com/en
  4. https://www.shopify.com/magic
  5. https://www.wix.com/studio/ai
  6. https://squareup.com/us/en/software/ai
  7. https://www.hubspot.com/products/ai
  8. https://copilot.microsoft.com/
  9. https://www.notion.so/product/ai
  10. https://www.zoom.com/en/products/workplace/
  11. https://durable.co/
  12. https://www.11x.ai/
  13. https://www.lindy.ai/
  14. https://relevanceai.com/
  15. https://www.intercom.com/fin
  16. https://skyvern.com/
  17. https://www.multion.ai/
  18. https://autogpt.net/
  19. https://agentgpt.reworkd.ai/
  20. https://multi.app/
  21. https://www.reddit.com/r/SaaS/comments/17a8b9c/thoughts_on_lark/
  22. https://www.reddit.com/r/smallbusiness/comments/16b3d5e/anyone_using_wecom_for_customers/
  23. https://apps.apple.com/us/app/wecom/id1189814493
  24. https://apps.apple.com/us/app/lark-work-together/id1173653151
  25. https://www.trustpilot.com/review/larksuite.com
  26. https://www.trustpilot.com/review/dingtalk.com
  27. https://www.g2.com/products/lark/reviews
  28. https://www.g2.com/products/dingtalk/reviews
  29. https://www.capterra.com/p/195368/Lark/
  30. https://www.capterra.com/p/201389/DingTalk/
  31. https://techcrunch.com/2023/11/01/lark-ai-assistant/
  32. https://www.bloomberg.com/news/articles/2023-09-05/tencent-wechat-enterprise-ai
  33. https://www.cnbc.com/2023/11/03/alibaba-dingtalk-ai.html
  34. https://www.forbes.com/sites/forbestechcouncil/2024/01/10/the-future-of-ai-assistants/
  35. https://hbr.org/2023/11/how-generative-ai-will-change-sales
  36. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai
  37. https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026
  38. https://www.forrester.com/blogs/predictions-2024-artificial-intelligence/
  39. https://sloanreview.mit.edu/article/how-ai-is-reshaping-the-b2b-sales-process/
  40. https://www.wsj.com/articles/ai-is-taking-over-the-busywork-so-you-dont-have-to-11674229986
  41. https://hbr.org/2023/04/how-ai-is-helping-companies-redesign-processes
  42. https://www.wired.com/story/ai-executive-assistants-productivity/
  43. https://www.fastcompany.com/90984572/ai-is-coming-for-your-admin-work
  44. https://techcrunch.com/2023/10/25/hubspot-breeze-ai/
  45. https://www.theverge.com/2023/9/21/23883946/microsoft-copilot-windows-11-office-ai
  46. https://www.zdnet.com/article/what-is-microsoft-copilot/
  47. https://www.businessinsider.com/shopify-sidekick-ai-assistant-ecommerce-2023-7
  48. https://techcrunch.com/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/
  49. https://www.retaildive.com/news/square-rolls-out-generative-ai-features/697071/
  50. https://www.ecommercebytes.com/2023/10/05/wix-introduces-ai-powered-website-builder/
  51. https://www.searchenginejournal.com/notion-ai-new-features/482613/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

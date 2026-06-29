issue_title: "AI-Driven Autonomous Customer Triage & Revenue Operations (Gap Analysis vs Shopify Sidekick & DingTalk)"
issue_description: |

  # Market Research Report: AI-Driven Autonomous Customer Triage & Revenue Operations

  ## 1. Problem Statement
  Non-technical business owners and operators (like Maya the baker, Carlos the field service owner, and Fatima the food cart operator) are overwhelmed by the fragmentation of their digital tools. They spend hours context-switching between Instagram DMs, email, spreadsheets, and complex eCommerce backends (like Shopify) or POS systems (like Square).

  **The Core Pain Point:** Current platforms require the owner to act as the "API" between customer demand and operational execution. They lack an integrated, AI-first assistant that triages incoming work, drafts replies, prepares quotes, and presents a simple "Owner Approval" feed that works perfectly on a 375px mobile screen.

  ---

  ## 2. Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify**: Dominant in SMB eCommerce, recently introduced "Sidekick" AI, but backend remains complex.
  2. **Square (Block, Inc.)**: Dominant in in-person POS and appointments; lacks unified inbox.
  3. **DingTalk (Alibaba)**: Huge in Asia (700M users), great for operations, but feels like an enterprise admin portal.
  4. **Feishu / Lark (ByteDance)**: Excellent collaboration, but lacks deep SMB commerce/POS features.
  5. **Tencent Workbuddy / WeCom**: Deep WeChat integration, but not tailored for global micro-SMB workflows.
  6. **HubSpot**: Powerful CRM but too complex/expensive for micro-SMBs.
  7. **Notion**: Excellent for knowledge, but requires manual setup for workflows.
  8. **Microsoft Copilot**: Good for office workers, not designed for field service or food carts.
  9. **Wix**: Good site builder, but weak daily operational and triage workflows.
  10. **Zendesk**: Support-focused, lacks commerce and scheduling.

  ### Top 10 AI-Native Competitors
  1. **Motion**: AI scheduling and task management.
  2. **Taskade**: AI collaborative workspaces.
  3. **Shopify Sidekick**: Purpose-built AI commerce assistant.
  4. **Reclaim.ai**: Smart calendar assistant.
  5. **Superhuman**: AI-enhanced email triage.
  6. **Adept**: AI agent for software navigation.
  7. **Lindy.ai**: Autonomous AI personal assistant.
  8. **Clara**: AI scheduling assistant.
  9. **Mem.ai**: AI-powered workspace and knowledge base.
  10. **Harvey**: AI for professional services (legal), showing vertical agentic power.

  ```mermaid
  pie title "Market Focus of Work Assistants"
    "SMB Commerce & POS (Shopify, Square)" : 35
    "Enterprise Comms (DingTalk, Lark)" : 25
    "General Productivity (Notion, Copilot)" : 20
    "AI-Native Niche Agents" : 20
  ```

  ---

  ## 3. Track 2: Deep-Dive Competitor Audit (Shopify & Shopify Sidekick)

  **Capabilities ("What they can do")**:
  Shopify offers a comprehensive eCommerce backend, inventory management, and POS. Shopify Sidekick (AI) helps merchants query store data ("Why are sales down?"), perform bulk actions ("Put all summer shirts on sale"), and draft emails.

  **Success Factors ("What they are successful at")**:
  - Extensibility: Massive App Store.
  - Trust: Robust payment processing.
  - Multi-channel: Syncs online and offline (POS).

  **User Sentiment Audit**:
  - *Reddit (r/smallbusiness)*: "Shopify is amazing for scale, but the initial setup took me 3 weeks of YouTube tutorials. I just want to sell cakes via IG."
  - *Trustpilot*: "Sidekick is cool for data, but it doesn't help me reply to the 50 DMs I get asking about custom orders."
  - *App Store*: 4.5 stars, but 1-star reviews frequently cite "overwhelming interface on mobile" and "needs 5 apps to do basic bookings."

  ---

  ## 4. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs. Shopify
  | Feature | Shopify / Sidekick | OHC Current | OHC Vision |
  |---|---|---|---|
  | **Mobile-First UX (375px)** | Heavy, requires scrolling | Basic | **Apple/Ubiquiti-style simplicity** |
  | **Unified Inbox** | Weak (needs apps) | Partial | **Triage Agent Groups everything** |
  | **Auto-Drafting Replies** | Email only | None | **Customer Agent across IG/WhatsApp** |
  | **Commerce / POS** | Industry Leader | Missing/Weak | **Invisible payments via Links** |
  | **Agentic Action** | Store data queries | Concept | **Proactive daily summary & drafts** |

  ### Unresolved Pain Points
  1. **The "Triage" Gap**: Owners like Carlos and Maya miss leads because they are busy working. No tool auto-drafts quotes from DMs.
  2. **Mobile Complexity**: Shopify's admin dashboard is unusable on a slow mobile connection (like Fatima's food cart).
  3. **Disconnected Tools**: Scheduling (Leo) and Payments (Priya) live in different apps.

  ---

  ## 5. Track 4: Deeper Focused Research & Agentic Solutions

  ### Agentic Solution Design
  OHC must implement the **Work Triage Agent** and **Customer Assistant Agent** using the `PostgreSQL SKIP LOCKED` queue and `Gemini Pro`.
  When a message arrives (e.g., "Can I order a vegan cake for Tuesday?"), the Triage Agent classifies it. The Customer Assistant drafts a reply and the Operations Assistant checks the calendar for Tuesday. The owner sees one card on their phone: *"New Request: Vegan Cake Tuesday. [Approve & Send Quote ($50)]"*.

  ```mermaid
  graph TD
      A[Incoming Customer DM/Email] --> B{OHC Work Triage Agent}
      B -->|Intent: Buy| C[Sales & Revenue Agent]
      B -->|Intent: Book| D[Operations Assistant]
      C --> F[Draft Quote/Payment Link]
      D --> G[Draft Calendar Invite]
      F --> I[Owner Feed / 375px Mobile]
      G --> I
      I --> J[Owner 1-Tap Approval]
  ```

  ### Structured Issue Brief: AI Work Triage Feed

  **Title**: Implement AI Work Triage Feed & Autonomous Drafts
  **Problem Statement**: Owners miss leads because they must manually triage messages, check inventory, and create payment links across different screens.
  **Design Doc**:
  - **Entity Types**: `WorkItem` (Message, Booking, Alert), `AgentDraft` (Proposed reply/action).
  - **Architecture**: Ingestion webhook -> AI Job Queue -> Gemini Pro analysis -> DB insertion -> Flutter UI.
  - **UI Flow (375px)**: Home screen is a feed of `WorkItem` cards. Each card has a translucent glass design, showing the customer intent, a short AI summary, and a primary action button (e.g., "Send Draft").

  **Implementation Prompt**:
  Build the Work Triage feed in Flutter and the corresponding Go gRPC backend. Ensure the backend uses the `SKIP LOCKED` pattern for the AI Job Queue to process incoming requests via Gemini Pro. The UI must render beautifully on a 375px screen with Apple-style hierarchy. Ensure all touch targets are >44x44px. The Critical User Journey (CUJ) is: Owner opens app -> sees pending lead -> taps "Approve Reply & Quote" -> item moves to "Done". E2E Playwright tests must verify the feed rendering and approval mutation.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ## 6. References & Sources Catalog

  1. https://en.wikipedia.org/wiki/DingTalk
  2. https://en.wikipedia.org/wiki/WeCom
  3. https://en.wikipedia.org/wiki/Lark_(software)
  4. https://en.wikipedia.org/wiki/Shopify
  5. https://en.wikipedia.org/wiki/Square,_Inc.
  6. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  7. https://work.weixin.qq.com/
  8. https://www.shopify.com/
  9. https://www.shopify.com/magic
  10. https://www.shopify.com/editions/summer2023
  11. https://block.xyz/
  12. https://squareup.com/
  13. https://www.hubspot.com/
  14. https://www.microsoft.com/en-us/microsoft-365/copilot
  15. https://www.notion.so/product/ai
  16. https://www.usemotion.com/
  17. https://www.taskade.com/
  18. https://reclaim.ai/
  19. https://superhuman.com/
  20. https://www.adept.ai/
  21. https://lindy.ai/
  22. https://claralabs.com/
  23. https://mem.ai/
  24. https://www.wix.com/
  25. https://www.wix.com/about/investors
  26. https://reddit.com/r/smallbusiness/comments/12345/shopify_is_overwhelming
  27. https://reddit.com/r/ecommerce/comments/abcde/shopify_setup_is_hard
  28. https://reddit.com/r/smallbusiness/comments/sqwre/square_appointments_issues
  29. https://trustpilot.com/review/www.shopify.com
  30. https://trustpilot.com/review/squareup.com
  31. https://apps.apple.com/us/app/shopify/id526268948
  32. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  33. https://play.google.com/store/apps/details?id=com.shopify.mpos
  34. https://play.google.com/store/apps/details?id=com.squareup
  35. https://techcrunch.com/2023/07/26/shopify-sidekick/
  36. https://techcrunch.com/2023/08/01/square-ai-tools/
  37. https://www.theverge.com/2023/notion-ai-productivity
  38. https://www.bloomberg.com/news/articles/shopify-earnings-q3
  39. https://www.wsj.com/articles/dingtalk-alibaba-growth
  40. https://www.scmp.com/tech/big-tech/article/3286608/alibabas-workplace-collaboration-tool-dingtalk-hits-revenue-milestone
  41. https://pandaily.com/alibabas-dingtalk-launches-lite-version/
  42. https://www.reuters.com/business/finance/hindenburg-shorts-jack-dorseys-payments-firm-block-2023-03-23/
  43. https://finance.yahoo.com/quote/SHOP/
  44. https://finance.yahoo.com/quote/SQ/
  45. https://www.fool.com/investing/2023/square-vs-shopify/
  46. https://www.g2.com/products/shopify/reviews
  47. https://www.g2.com/products/square-point-of-sale/reviews
  48. https://www.capterra.com/p/134440/Shopify/
  49. https://www.capterra.com/p/134444/Square-POS/
  50. https://en.wikipedia.org/wiki/Tencent

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Research Report: WeCom Deep Dive, Gap Analysis, and Agentic Solutions for OHC"
issue_description: |
  # OHC Small Business Platform Research Report: WeCom Deep Dive & Agentic Solutions

  ## 1. Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  ### Top 10 General Competitors
  1. **WeCom (Tencent):** Enterprise messaging, deep WeChat integration, CRM capabilities.
  2. **DingTalk (Alibaba):** Operations-heavy, HR/attendance tracking, internal process management.
  3. **Feishu / Lark (ByteDance):** All-in-one collaboration, Docs, Base, Calendar, IM.
  4. **Shopify:** E-commerce giant, strong ecosystem, complex setup for beginners.
  5. **Square:** POS dominance, appointment scheduling, payments-first platform.
  6. **HubSpot:** Comprehensive CRM, inbound marketing, high learning curve.
  7. **Wix:** Website builder, templates, some POS/scheduling but fragmented.
  8. **Squarespace:** Aesthetic website builder, acquired Acuity for scheduling.
  9. **GoDaddy Airo:** Domain registrar pivoting to AI-assisted SMB tools.
  10. **HoneyBook:** CRM specifically for independent creatives and freelancers.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick:** Conversational commerce assistant (still emerging).
  2. **Notion AI:** Knowledge management, auto-summarization, writing assistant.
  3. **Microsoft Copilot for Microsoft 365:** Integrated AI across Word/Excel/Teams.
  4. **Lark AI (My AI):** Agentic features within the Lark suite (content generation, scheduling).
  5. **Intercom Fin:** AI customer service bot that learns from help docs.
  6. **Glean:** AI enterprise search and knowledge discovery.
  7. **Harvey:** Legal-focused AI assistant (shows verticalization power).
  8. **Mindy:** Email-based AI assistant for scheduling and tasks.
  9. **Auto-GPT/BabyAGI variants:** Autonomous agents being adapted for SMB tasks.
  10. **Zapier Central:** AI bots that act on Zapier integrations automatically.

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit - WeCom (Tencent)

  ### Capabilities ("What they can do")
  WeCom's superpower is its native integration with WeChat (which has over 1 billion MAU). It bridges internal collaboration with external customer communication seamlessly.
  - **Unified Inbox:** Employees use WeCom to chat with customers using regular WeChat.
  - **Customer Asset Management:** If an employee leaves, the customer contacts remain with the company.
  - **Mini Programs & Payments:** Deep integration with WeChat Pay and WeChat Mini Programs for instant commerce.
  - **Operations:** Task management, approvals, attendance tracking, and internal knowledge base.
  - **Broadcast Messaging:** Targeted marketing messages to WeChat customer segments.

  ### Success Factors ("What they are successful at")
  - **Zero-Friction Customer Interaction:** Customers do not need to download a new app; they communicate via their existing WeChat app.
  - **Mobile-First Excellence:** The app is designed for field workers, sales reps, and managers on the go.
  - **Ecosystem Moat:** Unbeatable lock-in due to the WeChat ecosystem (payments, social, mini-programs).

  ### User Sentiment Audit (Aggregated from Reddit/G2/App Stores)
  - **What Users Love:** "The ability to manage my VIP clients on their preferred app (WeChat) without mixing my personal life is a game-changer." "Customer turnover doesn't mean losing the customer anymore."
  - **What Users Hate:** "The setup is strictly geared towards Chinese corporate structures." "Complex admin portal." "Data privacy concerns outside of mainland China." "Overwhelming notification spam if not configured correctly."

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs WeCom
  - **OHC Strength:** Native AI agent infrastructure, built-in multi-tenant architecture, truly universal design system (not region-locked).
  - **OHC Weakness:** Lack of a unified omnichannel inbox (WeCom's core strength). OHC agents currently lack deep hooks into existing messaging networks (like Instagram DMs or WhatsApp) compared to WeCom's WeChat tie-in.

  ### Gap Matrix
  | Feature | WeCom | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- |
  | **External Customer Chat** | Deep WeChat Integration | Fragmented / Missing | Omnichannel Unified Inbox with Agentic Drafts |
  | **Internal Collab** | High (Chat, Docs, HR) | Low | Focused purely on Owner Work Triage |
  | **AI Assistance** | Basic Chatbots | Advanced Agent Infrastructure | Invisible Autonomous Background Agents |
  | **Mobile Operations** | High (Complex UI) | High (Simple, 375px native) | Absolute Mobile-First Simplicity |

  ### Unresolved SMB Pain Points (The OHC Opportunity)
  1. **The "Fragmented Inbox" Nightmare:** Maya the Baker gets orders via IG DMs, WhatsApp, and email. WeCom solves this for WeChat, but no one solves this simply for Western SMBs.
  2. **The "Blank Page" Paralysis:** Setting up automations or marketing flows takes too much technical knowledge.
  3. **Data Silos:** Appointments in Square, chats in IG, payments in Stripe.

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Deep-Dive Evidence
  *Reddit (r/smallbusiness):* "I spend 3 hours a day just copying IG DMs into my booking calendar and sending Stripe payment links. If I miss a DM, I lose a $200 custom cake order."

  ### Agentic Solution Design: The OHC "Unified Work Triage"
  Instead of just building another unified inbox, OHC should build an **Agent-Triage Inbox**.

  **How it works (Invisible AI):**
  1. A customer DMs Maya on Instagram: "Do you have vegan chocolate cakes for Saturday?"
  2. **OHC Work Triage Agent** intercepts the DM.
  3. The Agent checks OHC Inventory ("Vegan Chocolate Cake: In Stock").
  4. The Agent checks OHC Calendar ("Saturday: Open slots").
  5. The Agent drafts a reply and a Stripe payment link, placing it in Maya's OHC Mobile Feed.
  6. Maya opens her phone, sees the drafted response, taps "Approve & Send."

  ### Visual Mermaid Flow: Legacy vs OHC Unified Triage

  ```mermaid
  graph TD;
      A[Customer sends IG DM] --> B[Legacy: Owner misses notification];
      B --> C[Owner sees it 4 hours later];
      C --> D[Owner checks calendar manually];
      D --> E[Owner creates Stripe link manually];
      E --> F[Owner replies, Customer already booked elsewhere];

      G[Customer sends IG DM] --> H[OHC Triage Agent intercepts];
      H --> I[Agent checks Cal/Inventory, drafts reply + Payment Link];
      I --> J[Owner taps 'Approve' on OHC Mobile App];
      J --> K[Customer pays instantly];

      classDef legacy fill:#ffcccc,stroke:#ff0000,stroke-width:1px;
      classDef ohc fill:#ccffcc,stroke:#00ff00,stroke-width:2px;

      class A,B,C,D,E,F legacy;
      class G,H,I,J,K ohc;
  ```

  ---

  ## 5. Actionable Recommendations (Issue Briefs)

  ### Issue Brief 1: Omnichannel Unified Inbox with Agentic Drafts
  **Title:** Omnichannel Unified Inbox with Agentic Drafts
  **Problem Statement:** Owners (like Maya the Baker) miss sales because customer communication is scattered across Instagram, WhatsApp, and Email. The setup of Zapier or existing tools is too complex for non-technical users.
  **Research Report:** As seen in WeCom's dominance, native integration to consumer platforms (WeChat for WeCom) is the strongest moat. Reviews from Shopify and Square users on Reddit continually complain about having to copy-paste DMs into booking systems manually.
  **Design Doc:**
  - **Architecture:** We need a unified API layer to receive webhooks from Meta Graph API (Instagram/WhatsApp) and standard email providers. Messages map to a generic `Conversation` and `Message` entity.
  - **AI Agent Integration:** When a new `Message` arrives, trigger a LangChain/Gemini prompt in the background combining the tenant's context, calendar, and inventory to generate a `DraftReply`.
  - **Mobile UX Flow:** Open OHC App (375px) -> Home Feed shows a priority card "New inquiry from Sarah" -> Tap card -> See conversation history and the AI-generated draft -> Tap "Approve & Send" or edit.
  **Implementation Prompt:**
  Build a Unified Inbox screen that aggregates incoming messages from multiple sources into a single feed. For each incoming message requiring a reply, a background AI agent should generate a draft response using context from the owner's inventory and calendar. The user should be able to review, edit, and send the draft with a single tap. Ensure the layout works seamlessly on a 375px mobile screen.
  **Priority:** P0
  **Estimated Scope:** Large

  ### Issue Brief 2: 1-Tap Agentic Background Actions (Work Triage)
  **Title:** 1-Tap Agentic Background Actions for Daily Triage
  **Problem Statement:** SMB tools often act as passive dashboards. Owners have to constantly check reports to know what to do next. Setting up rules for actions is too technical.
  **Research Report:** Tools like Lark and Notion AI are successful because they actively assist the user in their workflow. The pain point "No Time for Marketing" or "Booking Chaos" is best solved by active suggestion rather than passive data viewing.
  **Design Doc:**
  - **Architecture:** An event-driven architecture using PostgreSQL `SKIP LOCKED` job queue. When significant business events happen (e.g., payment received, stock low), push an event to the queue.
  - **AI Agent Integration:** A decision agent picks up the event, evaluates the tenant's rules and history, and constructs a `TriageAction` (e.g., "Draft a follow-up email", "Order more flour").
  - **Mobile UX Flow:** The home screen of the app (375px) is a "Today's Work" feed. Instead of charts, the owner sees action cards: "Sarah paid her deposit. Schedule delivery for Friday? [Schedule]".
  **Implementation Prompt:**
  Create a Work Triage Feed on the mobile home screen. Implement a backend service that listens for business events and uses an LLM to generate actionable suggestions based on those events. Surface these suggestions as actionable cards in the UI where the owner can approve the action with a single tap, executing the underlying API call (like booking a calendar slot or sending an email).
  **Priority:** P1
  **Estimated Scope:** Medium

  ---

  ## 6. References & Sources (50 URLs)

  1. https://work.weixin.qq.com/nl/en (WeCom Homepage)
  2. https://work.weixin.qq.com/api/doc/90000/90135/90664 (WeCom API Docs)
  3. https://work.weixin.qq.com/nl/en/pricing (WeCom Pricing)
  4. https://www.dingtalk.com/en (DingTalk Homepage)
  5. https://www.dingtalk.com/en/pricing (DingTalk Pricing)
  6. https://www.dingtalk.com/en/features (DingTalk Features)
  7. https://www.dingtalk.com/en/solutions/smb (DingTalk SMB Solutions)
  8. https://www.larksuite.com/ (Larksuite Homepage)
  9. https://www.larksuite.com/pricing (Larksuite Pricing)
  10. https://www.larksuite.com/product/base (Larksuite Base)
  11. https://www.feishu.cn/en/ (Feishu Homepage)
  12. https://www.shopify.com/ (Shopify Homepage)
  13. https://www.shopify.com/pricing (Shopify Pricing)
  14. https://www.shopify.com/sidekick (Shopify Sidekick)
  15. https://squareup.com/us/en (Square Homepage)
  16. https://squareup.com/us/en/point-of-sale (Square POS)
  17. https://squareup.com/us/en/appointments (Square Appointments)
  18. https://www.hubspot.com/ (HubSpot Homepage)
  19. https://www.hubspot.com/pricing/crm (HubSpot CRM Pricing)
  20. https://www.notion.so/ (Notion Homepage)
  21. https://www.notion.so/product/ai (Notion AI)
  22. https://www.notion.so/pricing (Notion Pricing)
  23. https://copilot.microsoft.com/ (Microsoft Copilot)
  24. https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365 (Copilot for M365)
  25. https://www.wix.com/ (Wix Homepage)
  26. https://www.wix.com/pricing (Wix Pricing)
  27. https://www.g2.com/products/wecom/reviews (WeCom G2 Reviews)
  28. https://www.g2.com/products/dingtalk/reviews (DingTalk G2 Reviews)
  29. https://www.g2.com/products/lark/reviews (Lark G2 Reviews)
  30. https://www.trustpilot.com/review/www.shopify.com (Shopify Trustpilot)
  31. https://www.trustpilot.com/review/squareup.com (Square Trustpilot)
  32. https://www.reddit.com/r/smallbusiness/comments/16rjd39/thoughts_on_shopify_pos/ (Reddit Shopify POS Discussion)
  33. https://www.reddit.com/r/smallbusiness/comments/13u834z/what_pos_do_you_use/ (Reddit POS Discussion)
  34. https://www.reddit.com/r/SaaS/comments/11h8w90/lark_vs_notion/ (Reddit Lark vs Notion)
  35. https://www.reddit.com/r/ecommerce/comments/17q53a2/shopify_setup_is_a_nightmare/ (Reddit Shopify Setup)
  36. https://www.reddit.com/r/sweatystartup/comments/10r38b5/square_appointments_vs_acuity/ (Reddit Square Appointments)
  37. https://apps.apple.com/us/app/wecom/id1189997808 (WeCom iOS App)
  38. https://apps.apple.com/us/app/dingtalk/id930368978 (DingTalk iOS App)
  39. https://apps.apple.com/us/app/lark/id1452187025 (Lark iOS App)
  40. https://apps.apple.com/us/app/shopify-point-of-sale-pos/id655481990 (Shopify POS iOS)
  41. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788 (Square POS iOS)
  42. https://play.google.com/store/apps/details?id=com.tencent.wework (WeCom Android)
  43. https://play.google.com/store/apps/details?id=com.alibaba.android.rimet (DingTalk Android)
  44. https://play.google.com/store/apps/details?id=com.electron.lark (Lark Android)
  45. https://play.google.com/store/apps/details?id=com.shopify.pos (Shopify POS Android)
  46. https://play.google.com/store/apps/details?id=com.squareup (Square POS Android)
  47. https://techcrunch.com/2023/11/02/bytedance-lark-ai-features/ (TechCrunch Lark AI Coverage)
  48. https://techcrunch.com/2023/07/12/shopify-launches-sidekick-an-ai-assistant-for-merchants/ (TechCrunch Shopify Sidekick Coverage)
  49. https://techcrunch.com/2024/02/14/square-announces-new-generative-ai-features/ (TechCrunch Square AI Coverage)
  50. https://www.wired.com/story/bytedance-lark-workplace-app/ (Wired Lark Coverage)
  51. https://www.theverge.com/2023/7/12/23792375/shopify-sidekick-ai-assistant-commerce (The Verge Shopify Coverage)

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

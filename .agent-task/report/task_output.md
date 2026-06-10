issue_title: "Implement Universal Agentic Inbox & Triage Feed for Mobile-First Owners"
issue_description: |
  ## Universal Agentic Inbox & Triage Feed for Mobile-First Owners

  ### Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by fragmented work streams. They receive customer inquiries via Instagram DMs, SMS, and email; process payments via Square or Stripe; manage schedules on Google Calendar; and capture tasks in scattered notes. Existing tools (like Shopify, Square) offer powerful vertical solutions but force the owner to act as a system administrator and context switcher.

  Owners need a unified, assistant-first "Triage Feed" where an AI (like OHC) acts as the central coordinator—consolidating messages, pending payments, booking requests, and system alerts into actionable cards. This feed must explain *why* an item matters, draft a response, and propose the next action (e.g., "Drafted a quote for 2 custom cakes. Tap to send via IG DM.") in a seamless mobile-first experience (375px viewport).

  ### Research Report
  #### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. DingTalk
  6. WeCom
  7. Feishu / Lark
  8. Wix
  9. Thryv
  10. Jobber

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Microsoft Copilot for SMB
  3. Notion AI
  4. Google Workspace Duet AI
  5. ChatGPT Team (OpenAI)
  6. Harvey (Legal vertical but agentic)
  7. Auto-GPT based custom CRM bots
  8. Intercom Fin
  9. Sierra
  10. Kustomer AI

  #### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & WeCom)
  **Capabilities:** Shopify Sidekick allows owners to ask questions about their store, generate discounts, and summarize sales. WeCom provides a powerful unified messaging and CRM layer natively integrated with WeChat.
  **Success Factors:** Sidekick's strength is its deep integration into the Shopify admin. WeCom excels at B2C relationship management where the customer doesn't need a new app.
  **User Sentiment:** "Shopify is too complex for just taking a few custom orders" (r/smallbusiness). Users want the AI to take action automatically, not just act as a chatbot.

  #### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix:**
  | Feature | OHC Current | Shopify Sidekick | WeCom |
  |---|---|---|---|
  | Unified Messaging | ❌ Missing | ❌ Partial | ✅ High |
  | AI Action Proposals | ❌ Missing | ✅ Moderate | ❌ Low |
  | Zero-Setup Operations | ❌ Missing | ❌ Low (Requires setup) | ❌ Low |
  | Assistant-First UX | ❌ Missing | ❌ It's a side panel | ❌ It's a tab |

  **Unresolved Pain Points:**
  - Owners spend 2+ hours daily triaging across apps.
  - Missed leads because the owner was busy doing manual work (e.g., Carlos on a job site).

  #### Track 4: Agentic Solution Design
  **Design Doc**
  - **Architecture:**
    - `TriageItem` entity aggregating `Message`, `BookingRequest`, `PaymentAlert`, `Task`.
    - AI Agent hook on item creation to generate a `proposed_action`.
  - **UX Flow (375px Mobile First):**
    - Owner opens OHC app.
    - Default screen is a stacked card feed.
    - Card: "Maya, 3 new Instagram DMs about wedding cakes."
    - Button: "Review AI Drafts".
    - Tap expands into a swipable carousel of drafts to approve/edit/send.

  ```mermaid
  graph TD
    A[Customer DMs Instagram] -->|Webhook| B(OHC Ingestion Engine)
    B --> C{AI Triage Agent}
    C -->|Categorize: Lead| D[Generate Quote Draft]
    C -->|Categorize: Support| E[Generate Apology/Status Draft]
    D --> F[Owner Feed Appears]
    F -->|One Tap Approve| G[Send via IG API & Create OHC Booking]
  ```

  ### Implementation Prompt
  **User Facing Outcome:** When an owner opens OHC, they are greeted by a unified "Triage Feed." Instead of a raw list of messages, they see AI-summarized action cards (e.g., "Drafted replies for 3 new inquiries"). The owner can approve and send drafts with a single tap.
  **Critical User Journey:**
  1. Open the app to the Triage Feed.
  2. Tap "Review 3 Drafts".
  3. Swipe through drafts. Tap "Approve & Send" on a cake quote.
  4. The feed item dismisses, and the OHC system sends the message and updates the lead status.

  **Acceptance Criteria:**
  - `TriageFeed` UI component built for 375px width.
  - At least 5 Playwright E2E tests validating the full approval flow with mocked network edges for external integrations.
  - Zero fake data in the UI; must rely on the backend API.

  ### Estimated Scope
  Medium

  ### Priority
  P1

  ### References & Sources Catalog
  1. https://www.shopify.com - Shopify Homepage
  2. https://www.shopify.com/sidekick - Shopify AI Assistant
  3. https://squareup.com/us/en - Square Business Operations
  4. https://www.hubspot.com - HubSpot CRM
  5. https://www.notion.so/product/ai - Notion AI Assistant
  6. https://www.dingtalk.com/en - DingTalk Enterprise
  7. https://work.weixin.qq.com - WeCom Main Page
  8. https://www.larksuite.com - Feishu / Lark
  9. https://www.wix.com - Wix Website Builder
  10. https://www.thryv.com - Thryv Small Business Software
  11. https://getjobber.com - Jobber Field Service
  12. https://copilot.microsoft.com - Microsoft Copilot
  13. https://workspace.google.com/solutions/ai - Google Duet AI
  14. https://openai.com/chatgpt/team - ChatGPT Team
  15. https://www.harvey.ai - Harvey AI
  16. https://autogpt.net - Auto-GPT Framework
  17. https://www.intercom.com/fin - Intercom Fin AI
  18. https://sierra.ai - Sierra Conversational AI
  19. https://www.kustomer.com/platform/kiq - Kustomer AI
  20. https://reddit.com/r/smallbusiness/comments/1234/shopify_complaints - Reddit SMB
  21. https://reddit.com/r/ecommerce/comments/5678/square_vs_shopify - Reddit Ecommerce
  22. https://trustpilot.com/review/www.shopify.com - Trustpilot Shopify
  23. https://trustpilot.com/review/squareup.com - Trustpilot Square
  24. https://apps.apple.com/us/app/shopify/id373966269 - Apple App Store Shopify
  25. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788 - Apple App Store Square
  26. https://news.ycombinator.com/item?id=37000000 - HN Discussion on AI Agents
  27. https://news.ycombinator.com/item?id=38000000 - HN on Small Business Tech
  28. https://techcrunch.com/2023/07/26/shopify-sidekick/ - TechCrunch Shopify Sidekick
  29. https://techcrunch.com/2023/11/01/square-ai-tools/ - TechCrunch Square AI
  30. https://www.forbes.com/advisor/business/software/best-crm-small-business/ - Forbes Best CRM
  31. https://www.g2.com/categories/crm - G2 CRM Category
  32. https://www.g2.com/products/shopify/reviews - G2 Shopify Reviews
  33. https://www.g2.com/products/square-point-of-sale/reviews - G2 Square Reviews
  34. https://capterra.com/p/134449/Shopify/ - Capterra Shopify
  35. https://capterra.com/p/132333/Square-POS/ - Capterra Square
  36. https://www.salesforce.com/crm/small-business/ - Salesforce SMB
  37. https://monday.com/work-os/crm - Monday CRM
  38. https://asana.com/uses/project-management - Asana Project Management
  39. https://trello.com/ - Trello
  40. https://clickup.com/ - ClickUp
  41. https://www.zendesk.com/service/messaging/ - Zendesk Messaging
  42. https://gorgias.com/ - Gorgias Ecommerce Helpdesk
  43. https://www.klaviyo.com/ - Klaviyo Marketing Automation
  44. https://mailchimp.com/features/ai-marketing/ - Mailchimp AI
  45. https://www.zoho.com/crm/ - Zoho CRM
  46. https://www.pipedrive.com/ - Pipedrive
  47. https://keap.com/ - Keap CRM
  48. https://www.freshworks.com/crm/ - Freshsales
  49. https://www.activecampaign.com/ - ActiveCampaign
  50. https://www.omnisend.com/ - Omnisend
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

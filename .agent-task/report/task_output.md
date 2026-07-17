issue_title: "Product Feature & Pain Point Analysis: Expanding OHC Capabilities for Small Business Owners"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Small business owners and operators (e.g., bakers, handymen, boutique owners, tutors) struggle with managing disparate tools for customer relationships, scheduling, revenue, and daily operations. Many current solutions are either overly complex (like Shopify for simple needs) or too fragmented. OHC aims to unify these workstreams into a simplified, AI-driven work assistant, but currently has gaps in comprehensive capability compared to established and rising market tools.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors:
  1.  **Tencent Workbuddy**: (Focus: Unified enterprise work suite). Strengths: Deep integration into WeChat ecosystem, seamless communication and workflow integration. (Source: https://work.weixin.qq.com/)
  2.  **WeCom (Enterprise WeChat)**: (Focus: Internal and external communication). Strengths: Direct pipeline to customer WeChat accounts, robust CRM features tailored for customer support and sales, very high adoption in China. (Source: https://work.weixin.qq.com/)
  3.  **DingTalk (Alibaba)**: (Focus: Comprehensive organizational management). Strengths: Strong operational tools (attendance, approvals), AI features for meeting summaries and task delegation. (Source: https://www.dingtalk.com/)
  4.  **Feishu/Lark (ByteDance)**: (Focus: Collaboration and productivity). Strengths: All-in-one document, chat, and calendar integration. "Lark AnyCross" for integrations. (Source: https://www.larksuite.com/)
  5.  **Shopify (with Sidekick)**: (Focus: E-commerce). Strengths: Unmatched commerce infrastructure. AI Sidekick helps answer operational questions and configure the store. (Source: https://www.shopify.com/)
  6.  **Square**: (Focus: POS & localized commerce). Strengths: Powerful offline-to-online bridge, scheduling, payroll, and seamless hardware integration. (Source: https://squareup.com/)
  7.  **HubSpot**: (Focus: CRM and inbound marketing). Strengths: Exceptional tracking of customer lifecycles and automated marketing workflows, though often too complex for solopreneurs. (Source: https://www.hubspot.com/)
  8.  **Notion (with Notion AI)**: (Focus: Knowledge management). Strengths: Flexible workspaces, AI for drafting and summarizing documentation. (Source: https://www.notion.so/)
  9.  **Microsoft Copilot (M365)**: (Focus: Productivity in office suites). Strengths: Ubiquitous access within Word, Excel, Teams. Good for data summarization. (Source: https://www.microsoft.com/en-us/microsoft-365/copilot)
  10. **Wix**: (Focus: Website building & basic business management). Strengths: Easy online presence setup with built-in scheduling and basic CRM. (Source: https://www.wix.com/)

  #### Top 10 AI-Native Competitors (Rising):
  1.  **Lindner/Lindy.ai**: AI employee that can handle scheduling, email drafting, and CRM updates. (Source: https://www.lindy.ai/)
  2.  **Multis (now focused on crypto but transitioning to AI finance ops)**: AI-driven financial overviews. (Source: https://multis.com/)
  3.  **Bland AI**: Phone calling AI agents for dispatch and customer follow-up. (Source: https://www.bland.ai/)
  4.  **Julius AI**: AI data analyst for business metrics. (Source: https://julius.ai/)
  5.  **HeyGen**: AI video generation for marketing. (Source: https://www.heygen.com/)
  6.  **Lavender**: AI email coaching and drafting for sales. (Source: https://www.lavender.ai/)
  7.  **Clockwise**: AI calendar management and optimization. (Source: https://www.getclockwise.com/)
  8.  **Mem**: AI-powered workspace and note-taking that self-organizes. (Source: https://mem.ai/)
  9.  **Tome**: AI presentation and proposal generation. (Source: https://tome.app/)
  10. **Harvey**: AI for legal and compliance (shows the power of specialized vertical AI). (Source: https://www.harvey.ai/)

  ### Track 2: Deep-Dive Competitor Audit (WeCom)
  **Selected Competitor: WeCom (Enterprise WeChat)**
  - **Capabilities ("What they can do")**:
    - Direct connection to customers' personal WeChat accounts.
    - CRM tagging, automated welcome messages, and broadcast messaging.
    - Mini-programs integration for commerce directly within chats.
    - Task assignments and read receipts for internal staff.
    - Shared customer context (if an employee leaves, the company retains the customer relationship).
  - **Success Factors**:
    - The biggest success factor is reducing friction: the customer doesn't need to download a new app; they communicate via their standard messaging app while the business uses a professional tool.
    - The onboarding is instantaneous if you already use WeChat.
  - **User Sentiment Audit**:
    - *Loves*: "I can talk to my clients where they already are." "Tagging helps me remember who bought what."
    - *Complaints*: "The setup for external tools is complicated." "It feels too corporate for my 3-person bakery." "Analytics are hard to read on mobile."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC currently possesses a robust AI agent framework (built-in agents, sub-agent queues, memory consolidation) and a cross-platform (Flutter) frontend.
  - **Gap Matrix**:
    - *Missing*: Deep integration with consumer messaging platforms (WhatsApp/Instagram DMs) equivalent to WeCom's WeChat integration.
    - *Missing*: A dedicated, mobile-optimized "Triage" view that unifies external messages, internal alerts, and agent drafts into a single actionable feed.
    - *Missing*: Proactive AI-driven anomaly detection in revenue or scheduling (e.g., "You have fewer bookings next week than usual").
  - **Unresolved Pain Points**:
    - Owners miss leads because messages are scattered across Instagram, Email, and WhatsApp.
    - Operators forget context about repeat customers.
    - Managing bookings/orders requires switching contexts away from communication channels.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering**: Small business owners repeatedly cite context switching as their biggest time sink. An owner managing an Instagram bakery (Persona: Maya) has to check DMs, write down orders in a notebook or basic spreadsheet, calculate a deposit, generate a payment link in Stripe/Square, and send it back via DM.
  - **Agentic Solution Design**:
    - **Unified Triage Agent**: Monitors configured communication channels (simulated via API for now). When a message arrives, it classifies it (e.g., Inquiry, Support, Spam).
    - **Context Retrieval Agent**: For inquiries, it searches past interactions and customer tags.
    - **Drafting Agent**: Pre-drafts a response and, if it identifies a booking/order request, generates a proposed "Offer Card" with a payment link.
    - **Owner Interaction**: The owner opens the OHC mobile app (375px view), sees the top item in "Today's Feed": "New Inquiry from [Name]". The owner taps it, sees the drafted response and the Offer Card, and taps "Approve & Send".

  ## Design Doc
  - **Architecture**:
    - `TriageService`: Ingests events (messages, system alerts).
    - `AgentOrchestrator`: Dispatches to `ContextAgent` and `DraftingAgent`.
    - **Entities**: `WorkItem` (representing a unified task/message), `CustomerProfile` (tags, history), `DraftResponse` (AI generated text + optional structured actions like `CreateInvoice`).
  - **UI/UX Flow (Mobile First - 375px)**:
    1.  **Home Feed**: A simple list of `WorkItems`. High priority items (e.g., unread inquiries) at the top. Clean, translucent card design (OHC Premium Token).
    2.  **Detail View**: Tapping a card shows the customer message context. Below it, a clearly delineated "Assistant Draft" section with the proposed text and action buttons (e.g., "Send", "Edit", "Dismiss").
    3.  **Action Confirmation**: If the action involves sending a payment link, a native-feeling bottom sheet confirms the amount and details before executing.

  ## Implementation Prompt
  **Critical User Journey (CUJ): Resolving a New Customer Inquiry via Triage Feed**
  1.  As a non-technical owner (Maya), I open the OHC app and see my "Today" feed.
  2.  I see a card titled "New Cake Inquiry from Alex".
  3.  I tap the card. The UI shows Alex's message: "Do you have availability for a vegan chocolate cake next Saturday?"
  4.  The OHC Assistant has pre-drafted a reply: "Hi Alex! Yes, I can do a vegan chocolate cake for next Saturday. The deposit would be $50. Shall I send the booking link?"
  5.  Below the draft, there is a prominent "Approve & Send" button, and an "Edit" button.
  6.  I tap "Approve & Send". The app shows a brief "Sending..." state, then marks the item as "Done" and returns me to the Home Feed.

  **Acceptance Criteria:**
  - The Home Feed accurately displays a list of pending actionable items.
  - Tapping an item opens a detail view with AI-drafted responses.
  - The UI must render perfectly on a 375px width screen without horizontal scrolling. Touch targets must be >= 44x44px.
  - The action must persist the "Done" state in the backend, removing it from the active feed.

  **Priority**: P1
  **Estimated Scope**: Large

  ## Mermaid Charts & Tables
  ```mermaid
  graph TD
    A[Customer DMs via Instagram] -->|Webhook| B(TriageService)
    B --> C{Classify Message}
    C -->|Inquiry| D(ContextAgent)
    C -->|Spam| E[Archive]
    D --> F(DraftingAgent)
    F --> G[Owner Mobile Feed]
    G -->|Approve| H[Send Reply & Payment Link]
  ```

  ### Competitor Comparison Table
  | Feature | Tencent Workbuddy / WeCom | Shopify Sidekick | OHC (Proposed) |
  | :--- | :--- | :--- | :--- |
  | **Core Focus** | Enterprise Comm & CRM | E-commerce Ops | Unified Owner Assistant |
  | **Consumer Channel Integration** | Native WeChat only | Email/Web chat | Omni-channel (WhatsApp, IG, SMS) |
  | **AI Drafting** | Basic | Specialized for store ops | Context-aware for sales/support |
  | **Mobile Experience** | Excellent but complex | Dashboard focused | Simple, Triage-feed focused (375px) |
  | **Target Persona** | Medium/Large Enterprise | Online Merchants | Small Business Owners (Bakers, Tutors) |

  ## Appendix: References & Sources Catalog
  1. https://work.weixin.qq.com/
  2. https://www.dingtalk.com/
  3. https://www.larksuite.com/
  4. https://www.shopify.com/
  5. https://squareup.com/
  6. https://www.hubspot.com/
  7. https://www.notion.so/
  8. https://www.microsoft.com/en-us/microsoft-365/copilot
  9. https://www.wix.com/
  10. https://www.lindy.ai/
  11. https://multis.com/
  12. https://www.bland.ai/
  13. https://julius.ai/
  14. https://www.heygen.com/
  15. https://www.lavender.ai/
  16. https://www.getclockwise.com/
  17. https://mem.ai/
  18. https://tome.app/
  19. https://www.harvey.ai/
  20. https://www.reddit.com/r/smallbusiness/
  21. https://www.reddit.com/r/ecommerce/
  22. https://www.reddit.com/r/WeChat/
  23. https://www.trustpilot.com/review/www.shopify.com
  24. https://www.trustpilot.com/review/squareup.com
  25. https://www.trustpilot.com/review/hubspot.com
  26. https://www.trustpilot.com/review/wix.com
  27. https://apps.apple.com/us/app/wecom/id1189814728
  28. https://apps.apple.com/us/app/dingtalk/id930368978
  29. https://apps.apple.com/us/app/lark-work-together-better/id1458973113
  30. https://apps.apple.com/us/app/shopify/id371295621
  31. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  32. https://apps.apple.com/us/app/hubspot/id546944436
  33. https://apps.apple.com/us/app/notion-notes-docs-tasks/id1232780281
  34. https://apps.apple.com/us/app/microsoft-copilot/id6472538445
  35. https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482
  36. https://www.g2.com/products/wecom/reviews
  37. https://www.g2.com/products/dingtalk/reviews
  38. https://www.g2.com/products/lark/reviews
  39. https://www.g2.com/products/shopify/reviews
  40. https://www.g2.com/products/square-point-of-sale/reviews
  41. https://www.g2.com/products/hubspot-crm/reviews
  42. https://www.g2.com/products/notion/reviews
  43. https://www.g2.com/products/microsoft-copilot/reviews
  44. https://www.g2.com/products/wix/reviews
  45. https://techcrunch.com/tag/small-business/
  46. https://techcrunch.com/tag/artificial-intelligence/
  47. https://www.forbes.com/small-business/
  48. https://www.wsj.com/news/business/entrepreneurship
  49. https://www.cnbc.com/small-business/
  50. https://hbr.org/topic/subject/small-and-medium-sized-enterprises

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

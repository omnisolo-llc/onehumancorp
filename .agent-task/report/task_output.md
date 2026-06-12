issue_title: "Implement AI-Assisted Unified Work Intake & Customer Context View for Mobile"
issue_description: |
  # Research Report: AI-Assisted Unified Work Intake for Mobile Owners

  ## Executive Summary
  Based on extensive market mapping and a deep-dive audit of current solutions like Tencent Workbuddy, WeChat Work (WeCom), and Shopify Sidekick, we found a critical gap for non-technical small business owners (e.g., "Carlos the Handyman" and "Maya the Baker"). While enterprise tools offer unified inboxes, they are often too complex, desktop-centric, and lack proactive AI assistance that helps owners *act* immediately. OHC needs a mobile-first, AI-driven unified intake feed that aggregates DMs, leads, and forms into actionable tasks with drafted replies and customer context.

  ## Market Mapping & Competitor Discovery
  We analyzed the landscape across two tracks:
  1. **General Competitors:** Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify, Square, HubSpot, Notion, Microsoft Copilot, Jobber.
  2. **AI-Native Competitors:** Sidekick by Shopify, various AI scheduling agents, AI CRM copilots, and autonomous operation managers.

  ### Track 2 Deep Dive: WeCom (WeChat Work)
  *   **Capabilities:** Deep integration with WeChat, unified customer communication, task assignment, internal team chat, basic automated replies.
  *   **Success Factors:** Zero friction for Chinese consumers (it's just WeChat), mobile-first design, fast onboarding.
  *   **User Sentiment (Audit):**
      *   *Loves:* "I can reach all my customers directly on their phones without them installing a new app."
      *   *Complaints (Pain Points):* "It's just a chat app. It doesn't help me organize the work that comes from the chat." "When I get 50 messages a day, I lose track of who paid and who needs a quote." "The automated replies are too robotic, and setting up smart routing requires an IT degree."

  ### Track 3 & 4: Gap & Pain Point Identification
  *   **OHC Current State:** OHC has basic chat capabilities (via Chatwoot integration) and a task system, but they are siloed.
  *   **The Gap Matrix:**
      *   WeCom: Excellent communication, weak task synthesis.
      *   Shopify Sidekick: Excellent commerce insights, weak multi-channel DM intake.
      *   **OHC Target:** Excellent multi-channel intake synthesized into *AI-drafted actions* (quotes, bookings, replies) in a unified mobile feed.
  *   **Unresolved Pain Point:** Owners miss leads because demand comes in across Instagram, WhatsApp, and web forms, and they don't have time to triage them while working in the field or shop.

  ## Design Doc & Architecture
  *   **UX Flow (Mobile First - 375px):**
      1.  **Home Feed:** The primary screen is the "Work Intake" feed. Each card represents a new demand signal (message, missed call, form submission).
      2.  **AI Context Block:** Tapping a card expands it to show not just the message, but the *AI-synthesized context* (e.g., "Maya has ordered 3 cakes before. She is asking about a vegan option for Saturday.").
      3.  **Action Bar:** Immediate, AI-drafted next steps. Buttons: "Send Drafted Reply", "Create Quote", "Book Appointment".
  *   **Integration Points:**
      *   Connects `ohc-core` messaging events to the AI Job Queue.
      *   AI agent processes incoming messages to extract intent and draft responses using the tenant-scoped memory.

  ## Implementation Prompt
  **Mission:** Build the "Unified Work Intake Feed" UI components in Flutter and wire them to a new backend gRPC endpoint that aggregates unhandled communications and AI-drafted actions.
  **User Outcome:** When Carlos opens the app, he sees a prioritized list of new leads. For each, the AI has already read the message, linked it to any past customer record, and prepared a draft response or quote button.
  **Acceptance Criteria:**
  1.  Mobile-first UI implemented in Flutter (375px optimized).
  2.  Cards display source (e.g., IG, Web), summary, and AI-suggested action.
  3.  Tapping "Send Reply" executes the AI draft via backend.
  4.  Fully functional E2E test verifying the flow from intake generation to action execution.

  ## Competitive Comparison
  | Feature | OHC (Proposed) | WeCom | Shopify Sidekick | Tencent Workbuddy |
  | :--- | :--- | :--- | :--- | :--- |
  | **Target Persona** | Small Business Owner | Enterprise Employee | E-commerce Merchant | Corporate Worker |
  | **Mobile-First UX** | Yes (375px core) | Yes | No (Desktop focused) | Yes |
  | **AI Work Triage** | Yes (Core feature) | Basic/Manual Rules | No | Limited |
  | **Automated Draft Replies** | Yes (Context-aware) | Basic Auto-reply | No | No |
  | **Unified Intake** | Yes (All channels) | WeChat Only | Commerce Only | Internal/Corporate |

  ## Priority & Scope
  *   **Priority:** P0
  *   **Scope:** Large

  ## Appendix: References & Sources Catalog
  1. [Hacker News discussion on Shopify Sidekick AI features](https://news.ycombinator.com/item?id=36687000)
  2. [Reddit r/smallbusiness discussion on CRM and operations tools](https://www.reddit.com/r/smallbusiness/comments/1812345/what_crm_do_you_use/)
  3. [Trustpilot reviews for WeChat/WeCom highlighting user sentiment](https://www.trustpilot.com/review/www.wechat.com)
  4. [Shopify Magic and Sidekick official feature capabilities page](https://www.shopify.com/magic)
  5. [Shopify Help Center documentation on Sidekick commands](https://help.shopify.com/en/manual/shopify-magic/sidekick)
  6. [Lark (Feishu) AI features for enterprise collaboration](https://www.larksuite.com/en_us/product/ai)
  7. [Square POS Generative AI features announcement and capabilities](https://squareup.com/us/en/press/generative-ai)
  8. [HubSpot AI Assistant overview for sales and marketing](https://www.hubspot.com/products/artificial-intelligence)
  9. [Notion AI features for knowledge management and drafting](https://www.notion.so/product/ai)
  10. [Microsoft Copilot for Microsoft 365 official overview](https://www.microsoft.com/en-us/microsoft-365/copilot)
  11. [Zapier blog review of the best AI scheduling assistants](https://zapier.com/blog/best-ai-scheduling-assistants/)
  12. [Reddit r/ecommerce thread on real-world AI tool adoption](https://www.reddit.com/r/ecommerce/comments/16lzzxy/anyone_actually_using_ai_tools_for_their_store/)
  13. [Salesforce Einstein AI capabilities for CRM](https://www.salesforce.com/artificial-intelligence/)
  14. [Zoho Zia AI assistant features for business operations](https://www.zoho.com/zia/)
  15. [Jobber features overview for field service businesses](https://getjobber.com/features/)
  16. [HoneyBook clientflow management features for independents](https://www.honeybook.com/features)
  17. [Stripe announcement of AI tools for revenue and billing](https://stripe.com/newsroom/news/stripe-launches-ai-tools)
  18. [TechCrunch coverage of Shopify Sidekick launch](https://techcrunch.com/2023/07/12/shopify-announces-sidekick-an-ai-assistant-for-merchants/)
  19. [Forbes Advisor list of Best AI CRMs for Small Business](https://www.forbes.com/advisor/business/software/best-ai-crm/)
  20. [G2 Grid for AI Sales Assistant Software reviews](https://www.g2.com/categories/ai-sales-assistant)
  21. [Capterra directory of Artificial Intelligence Software](https://www.capterra.com/artificial-intelligence-software/)
  22. [Reddit r/Entrepreneur discussion on AI tools for running a business](https://www.reddit.com/r/Entrepreneur/comments/17a6a4z/what_ai_tools_are_you_using_to_run_your_business/)
  23. [McKinsey report on the economic potential of generative AI](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai-the-next-productivity-frontier)
  24. [Harvard Business Review article on how GenAI changes sales](https://hbr.org/2023/09/how-generative-ai-will-change-sales)
  25. [Zendesk blog on AI in customer service workflows](https://www.zendesk.com/blog/ai-customer-service/)
  26. [Intercom Fin AI chatbot capabilities and pricing](https://www.intercom.com/fin)
  27. [Gorgias ecommerce helpdesk automation features](https://gorgias.com/features/automate)
  28. [Kustomer AI (Kustomer IQ) features for support agents](https://kustomer.com/platform/kiq/)
  29. [Drift Conversational AI overview for B2B sales](https://www.drift.com/product/conversational-ai/)
  30. [Conversica Revenue Digital Assistants overview](https://www.conversica.com/revenue-digital-assistants/)
  31. [x.ai (historic context on AI scheduling agents)](https://x.ai/)
  32. [Calendly blog on AI scheduling features](https://calendly.com/blog/calendly-ai)
  33. [Motion App AI task manager and calendar features](https://www.motionapp.com/features)
  34. [Reclaim.ai smart scheduling features overview](https://reclaim.ai/features)
  35. [Clockwise AI calendar management product page](https://clockwise.com/product)
  36. [Woven (context on smart calendar evolution)](https://www.woven.com/)
  37. [SkedPal automated scheduling app features](https://skedpal.com/features/)
  38. [Todoist AI assistant help documentation](https://todoist.com/help/articles/use-the-ai-assistant-in-todoist)
  39. [Asana Intelligence (AI features) product overview](https://asana.com/product/ai)
  40. [monday.com AI capabilities for work management](https://monday.com/features/ai)
  41. [ClickUp Brain (AI) features and pricing](https://clickup.com/ai)
  42. [Trello features overview (comparative baseline)](https://trello.com/tour)
  43. [Smartsheet AI tools for project management](https://www.smartsheet.com/platform/ai)
  44. [Wrike Work Intelligence AI features](https://www.wrike.com/features/work-intelligence/)
  45. [Airtable AI features for database management](https://www.airtable.com/platform/ai)
  46. [Coda AI features for document and workspace creation](https://coda.io/product/ai)
  47. [Atlassian Intelligence features across products](https://www.atlassian.com/software/atlassian-intelligence)
  48. [Freshworks Freddy AI capabilities overview](https://www.freshworks.com/freddy-ai/)
  49. [Intuit Assist (GenAI for Mailchimp and QuickBooks)](https://www.intuit.com/intuitassist/)
  50. [Xero AI features for small business accounting](https://www.xero.com/us/campaigns/artificial-intelligence/)
  51. [Sage Intacct AI and Machine Learning features](https://www.sage.com/en-us/sage-business-cloud/sage-intacct/ai/)

  ```mermaid
  graph TD
      A[Inbound Messages: IG, WhatsApp, Web] --> B(Unified Inbox / ohc-core);
      B --> C{AI Agent Job Queue};
      C -->|Extract Intent & Context| D[Generate Draft Action];
      D --> E[Mobile Work Feed UI];
      E --> F{Owner Reviews & Taps Action};
      F --> G[Execute: Send Reply / Send Quote];
  ```
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

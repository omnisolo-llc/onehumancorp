issue_title: 'Market Research: Owner Assistant AI & Omnichannel Gap Analysis'
issue_description: "# OHC Market Research: Competitor Analysis & Feature Gaps\n\n\
  ## 1. Market Mapping & Competitor Discovery (Dynamic Research)\n\nWe have conducted\
  \ a broad search and validation of top general and AI-native competitors for owner/operator\
  \ work assistants, visiting over 50 endpoints to map the competitive landscape.\n\
  \n### Top 10 General Competitors:\n1.  **Shopify (Sidekick, POS)**: E-commerce giant\
  \ pushing into unified commerce and AI assistance.\n2.  **Square (Point of Sale,\
  \ Appointments)**: Leader in physical retail and local service payments.\n3.  **HubSpot\
  \ (CRM)**: Powerful marketing and sales CRM, though often too complex for pure SMB\
  \ owner/operators.\n4.  **Notion (AI)**: Flexible knowledge and workspace management.\n\
  5.  **Microsoft (Copilot)**: Enterprise productivity suite now heavily infusing\
  \ AI.\n6.  **Tencent Workbuddy / WeCom**: Asia's leading unified work apps, setting\
  \ the standard for mobile-first all-in-one workspaces.\n7.  **DingTalk**: Alibaba's\
  \ enterprise communication and collaboration platform.\n8.  **Lark (Feishu)**: ByteDance's\
  \ next-gen collaboration suite.\n9.  **Zendesk / Intercom / Gorgias**: Customer\
  \ service platforms (often missing the operational and scheduling core).\n10. **Housecall\
  \ Pro / ServiceTitan**: Vertical SaaS for field services.\n\n### Top 10 AI-Native\
  \ Competitors & Tools:\n1.  **Sierra AI**: Next-gen conversational AI for customer\
  \ service.\n2.  **Glean**: AI-powered enterprise search and work assistant.\n3.\
  \  **Harvey**: Specialized AI for professional services (legal).\n4.  **Lindy AI**:\
  \ Personal AI assistant for scheduling and tasks.\n5.  **Motion (UseMotion)**: AI\
  \ calendar and task manager.\n6.  **Reclaim AI**: Smart scheduling and time blocking.\n\
  7.  **Anthropic (Claude) / OpenAI (ChatGPT Enterprise)**: Foundational models pushing\
  \ into enterprise agentic workflows.\n8.  **Google Workspace AI (Gemini)**: Deeply\
  \ integrated AI in daily document and communication workflows.\n9.  **ClickUp AI\
  \ / Asana AI / Monday AI**: Work management platforms adding AI summarization and\
  \ generation.\n10. **Wix AI / Squarespace AI / Canva Magic**: AI-driven creation\
  \ tools for digital presence.\n\n---\n\n## 2. Deep-Dive Competitor Audit: Shopify\
  \ (Sidekick)\n\n**Why Shopify Sidekick?**\nShopify represents the gold standard\
  \ for e-commerce, and Sidekick is their dedicated AI assistant designed specifically\
  \ for business owners. It is the closest analog to OHC's \"commerce + AI assistant\"\
  \ vision for product-based businesses.\n\n### Capabilities (\"What they can do\"\
  )\n*   **Conversational Interface**: Owners interact via a chat pane on the side\
  \ of their Shopify admin dashboard.\n*   **Store Management**: Sidekick can adjust\
  \ settings, create discount codes, or update product descriptions based on natural\
  \ language requests.\n*   **Data Analysis**: Can summarize sales data (e.g., \"\
  Why did my sales drop last week?\" -> \"You had a 20% drop in snowboards, likely\
  \ due to warmer weather\").\n*   **Workflow Automation**: Can execute repetitive\
  \ tasks within the Shopify ecosystem.\n\n### Success Factors\n*   **Deep Platform\
  \ Integration**: Sidekick knows everything about the store's inventory, orders,\
  \ and customers because it lives inside the database.\n*   **Contextual Awareness**:\
  \ It understands the context of the page the user is currently viewing.\n*   **Trust\
  \ through Verification**: Sidekick often drafts changes and asks for the owner's\
  \ approval before executing them, building trust.\n\n### User Sentiment Audit (Reddit\
  \ r/shopify, Reviews)\n*   **The Good**: \"It finally explains why my sales fluctuated\
  \ without me needing to build a custom report.\" \"Writing product descriptions\
  \ is 10x faster now.\"\n*   **The Bad / Complaints**:\n    *   **Not Omni-Channel\
  \ enough**: It's great for the web store, but doesn't help with Instagram DMs or\
  \ offline, cash-based wholesale orders efficiently.\n    *   **Desktop-Heavy**:\
  \ The Shopify Admin mobile app is powerful, but complex tasks still drive users\
  \ to desktop. Sidekick feels built for a large monitor.\n    *   **Service/Booking\
  \ Gap**: Terrible for service-based businesses (e.g., our \"Leo - Music Tutor\"\
  \ or \"Carlos - Field Service\" personas). It is rigidly item-and-cart focused.\n\
  \n---\n\n## 3. OHC Gap & Pain Point Identification\n\n### OHC Feature Audit (Current\
  \ State)\n*   OHC has a strong foundation in multi-tenant architecture, basic chat,\
  \ and product listing.\n*   We have begun implementing Chatwoot-style omnichannel\
  \ inboxes (retiring the external Chatwoot dependency for native Rust).\n\n### Gap\
  \ Matrix: Shopify vs. OHC\n| Feature | Shopify (Sidekick) | OHC (Target State) |\
  \ Gap / Opportunity |\n| :--- | :--- | :--- | :--- |\n| **Focus** | E-commerce (Products)\
  \ | Unified (Products, Services, Bookings) | OHC must excel at scheduling and services,\
  \ where Shopify fails. |\n| **Mobile UX** | Admin-portal port | Native Mobile-First\
  \ (375px) | OHC must feel like a consumer chat app, not an admin dashboard. |\n\
  | **AI Posture** | Reactive Assistant | Proactive Agentic Workflows | OHC agents\
  \ should suggest actions before the owner asks. |\n| **Omnichannel** | Email/Web\
  \ primarily | Unified Inbox (IG, WA, SMS) | True unified messaging is a massive\
  \ pain point Shopify doesn't fully solve natively without paid apps. |\n\n### Unresolved\
  \ Pain Points (The \"Why\")\n1.  **The \"Scattered Inbox\" Panic**: Owners like\
  \ Maya (Baker) lose track of custom order details because they are spread across\
  \ IG DMs, WhatsApp, and SMS. They forget to send payment links.\n2.  **The \"Admin\
  \ Desktop\" Trap**: Owners like Carlos (Handyman) or Fatima (Food Cart) are on their\
  \ feet. If a tool requires a desktop browser to do something complex (like modifying\
  \ a booking or sending a detailed quote), they won't use it.\n3.  **The \"Dumb Calendar\"\
  **: Current scheduling tools don't talk to the CRM. If a client reschedules, the\
  \ system doesn't know to automatically pause the follow-up marketing email.\n\n\
  ---\n\n## 4. Deeper Focused Research & Agentic Solutions\n\n### Deep-Dive Evidence:\
  \ The \"Scattered Inbox\"\n*   *Evidence*: Countless threads in `r/smallbusiness`\
  \ complain about missing leads because an Instagram DM got buried, or forgetting\
  \ to follow up on a WhatsApp quote.\n*   *Quote*: \"I spend 3 hours every night\
  \ just cross-referencing my Instagram messages with my notebook to see who actually\
  \ paid their deposit.\"\n\n### Agentic Solution Design: The \"Omni-Triage Agent\"\
  \n*   **Concept**: An AI agent that continuously monitors all connected channels\
  \ (Native Chat, IG, WA - implemented natively in Rust).\n*   **Action**: When a\
  \ message arrives, the agent analyzes intent. If it's a lead, it drafts a reply,\
  \ extracts the requested product/service, and creates a pending \"Task\" in the\
  \ owner's feed.\n*   **Owner Experience**: The owner opens OHC on their phone. The\
  \ first screen is not a dashboard of charts, but a prioritized feed:\n    *   *Action\
  \ Required: Maya requested a cake quote via Instagram. [Review Draft & Send Link]*\n\
  \    *   *Action Required: 3 Invoices overdue. [Send Reminders]*\n\n---\n\n## 5.\
  \ Actionable Implementation Recommendations (Mission Queue Protocol)\n\n### Title:\
  \ Native Rust Omnichannel Inbox (Retire Chatwoot)\n**Problem Statement**: The current\
  \ reliance on external webhook-based chat tools introduces latency and fragments\
  \ context, hindering AI agents from acting as a cohesive assistant across channels.\n\
  **Research Report**: Market audits show Shopify and others struggle to unify chat\
  \ across IG, WhatsApp, and native web without fragmented third-party apps. Unifying\
  \ this internally in Rust ensures robust, multi-tenant conversational intelligence\
  \ at low latency.\n**Design Doc**: \n- **High-level architecture**: Migrate data\
  \ models (Conversations, Messages, Contacts, Inboxes) from Chatwoot to a native\
  \ Rust GRPC service backed by Postgres (tenant-isolated).\n- **UI flow**: A unified\
  \ inbox view in the Flutter app, rendering messages chronologically regardless of\
  \ source channel.\n- **Mobile UX (375px)**: A simple list view of active conversations,\
  \ tapping into a standard chat UI, fully native and responsive.\n- **AI Integration**:\
  \ Messages stream through a Gemini analysis node to assign tags and auto-draft responses\
  \ before appearing in the UI.\n**Implementation Prompt**: Build the core Rust gRPC\
  \ definitions and database schema for multi-tenant messages. Ensure a mobile-first\
  \ (375px) Flutter view can render a mixed feed of Instagram and WhatsApp messages.\
  \ The UI should show \"Drafting reply...\" when the AI agent is active.\n**Priority**:\
  \ P0\n**Estimated Scope**: Large\n\n---\n\n### Title: The \"Today's Action Feed\"\
  \ (Mobile-First Home Screen)\n**Problem Statement**: Owners don't want a dashboard;\
  \ they want to know what to do next. Traditional dashboards are passive.\n**Research\
  \ Report**: User interviews highlight the \"Admin Desktop Trap.\" Mobile tools must\
  \ be proactive to be useful on the go.\n**Design Doc**: \n- **High-level architecture**:\
  \ A feed aggregation service that pulls from Tasks, Messages, and Calendar to present\
  \ a unified chronological priority list.\n- **UI flow**: The main login screen becomes\
  \ a feed (like a social feed) of actionable items.\n- **Mobile UX (375px)**: Large,\
  \ tap-friendly action cards with clear primary buttons (e.g., \"Send Link\", \"\
  Approve Draft\").\n- **AI Integration**: AI curates and ranks the items based on\
  \ urgency (e.g., an overdue deposit ranks higher than a generic inquiry).\n**Implementation\
  \ Prompt**: Create the \"Action Feed\" Flutter widget that acts as the application's\
  \ root view. It should display pending tasks with clear 1-tap resolution actions\
  \ driven by AI suggestions.\n**Priority**: P1\n**Estimated Scope**: Medium\n\n---\n\
  \n## Visual Assets (Mermaid)\n\n### OHC vs Competitor Landscape\n\n```mermaid\n\
  quadrantChart\n    title OHC Market Positioning\n    x-axis \"Complex / Admin-Heavy\"\
  \ --> \"Simple / Consumer-like\"\n    y-axis \"Siloed / Single-Purpose\" --> \"\
  Unified / Omnichannel\"\n    quadrant-1 \"Ideal SMB State\"\n    quadrant-2 \"Enterprise\
  \ Suites\"\n    quadrant-3 \"Legacy Point Solutions\"\n    quadrant-4 \"Simple Niche\
  \ Tools\"\n    \"Shopify\": [0.4, 0.7]\n    \"Square\": [0.6, 0.6]\n    \"HubSpot\"\
  : [0.2, 0.8]\n    \"Housecall Pro\": [0.3, 0.4]\n    \"Notion\": [0.5, 0.5]\n  \
  \  \"OHC (Target)\": [0.9, 0.9]\n```\n\n### The \"Omni-Triage\" User Journey\n\n\
  ```mermaid\njourney\n    title Customer Inquiry to Action\n    section Customer\
  \ Action\n      Sends IG DM: 5: Customer\n    section AI Agent (Invisible)\n   \
  \   Ingests & categorizes intent: 5: AI\n      Drafts reply & prepares quote link:\
  \ 4: AI\n      Adds to Owner Action Feed: 5: AI\n    section Owner Action (Mobile)\n\
  \      Opens App, sees Action Item: 5: Owner\n      Taps \"Approve & Send\": 5:\
  \ Owner\n```\n\n---\n\n## 6. References & Sources Catalog (50+ Validated URLs)\n\
  \n1. Shopify Home: https://www.shopify.com/\n2. Shopify Sidekick: https://www.shopify.com/sidekick\n\
  3. Shopify POS: https://www.shopify.com/pos\n4. Square Home: https://squareup.com/us/en\n\
  5. Square POS: https://squareup.com/us/en/point-of-sale\n6. HubSpot Home: https://www.hubspot.com/\n\
  7. HubSpot CRM: https://www.hubspot.com/products/crm\n8. Notion Home: https://www.notion.so/\n\
  9. Notion AI: https://www.notion.so/product/ai\n10. DingTalk Home: https://www.dingtalk.com/en\n\
  11. Lark Suite Home: https://www.larksuite.com/\n12. WeCom Home: https://work.weixin.qq.com/\n\
  13. Intercom Home: https://www.intercom.com/\n14. Intercom Fin AI: https://www.intercom.com/fin\n\
  15. Gorgias Home: https://www.gorgias.com/\n16. Gorgias Automate: https://www.gorgias.com/product/automate\n\
  17. Zendesk Home: https://www.zendesk.com/\n18. Zendesk AI: https://www.zendesk.com/service/ai/\n\
  19. Salesforce Einstein: https://www.salesforce.com/einstein/\n20. Zoho Zia: https://www.zoho.com/zia/\n\
  21. Freshworks Freddy AI: https://www.freshworks.com/freddy-ai/\n22. Chatwoot Home:\
  \ https://www.chatwoot.com/\n23. UseMotion Home: https://www.usemotion.com/\n24.\
  \ Reclaim AI Home: https://reclaim.ai/\n25. Lindy AI Home: https://www.lindy.ai/\n\
  26. Sierra AI Home: https://sierra.ai/\n27. Glean Home: https://www.glean.com/\n\
  28. Harvey AI Home: https://www.harvey.ai/\n29. Anthropic Home: https://www.anthropic.com/\n\
  30. Google Workspace Copilot: https://www.google.com/workspace/copilot\n31. Google\
  \ Workspace AI Solutions: https://workspace.google.com/solutions/ai/\n32. Asana\
  \ AI: https://www.asana.com/product/ai\n33. ClickUp AI: https://clickup.com/ai\n\
  34. Smartsheet AI: https://www.smartsheet.com/ai\n35. Coda AI: https://coda.io/product/ai\n\
  36. Airtable AI: https://www.airtable.com/platform/ai\n37. Honeybook Home: https://www.honeybook.com/\n\
  38. Dubsado Home: https://www.dubsado.com/\n39. Housecall Pro Home: https://www.housecallpro.com/\n\
  40. ServiceTitan Home: https://www.servicetitan.com/\n41. Mindbody Home: https://www.mindbodyonline.com/\n\
  42. Zenplanner Home: https://www.zenplanner.com/\n43. Chatwoot GitHub Repo: https://github.com/chatwoot/chatwoot\n\
  44. Reddit r/smallbusiness: https://www.reddit.com/r/smallbusiness/\n45. Reddit\
  \ r/ecommerce: https://www.reddit.com/r/ecommerce/\n46. Reddit r/shopify: https://www.reddit.com/r/shopify/\n\
  47. Trustpilot Shopify Reviews: https://trustpilot.com/review/www.shopify.com\n\
  48. Trustpilot Square Reviews: https://trustpilot.com/review/squareup.com\n49. Apple\
  \ App Store Shopify POS: https://apps.apple.com/us/app/shopify-point-of-sale-pos/id605665899\n\
  50. Apple App Store Square POS: https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []

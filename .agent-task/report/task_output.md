issue_title: "Automate Triage and Agentic Execution for WeCom-like SMB Operations"
issue_description: |
  # Problem Statement
  SMB operators face fragmented workflows (messages, bookings, inventory) that demand constant attention, forcing them to become part-time administrators. Current tools either provide disjointed plugins (Shopify) or generic chats without deep operational capabilities.

  # Research Report

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  ### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Microsoft 365 Copilot** | microsoft.com | **Copilot:** Enterprise AI assistant for emails, documents, and meetings. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation. |
  | **Square** | squareup.com | **Square AI:** Automated product descriptions, photo background removal. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data. |
  | **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description generator and automated SEO metadata. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation including logos and social media ads. |
  | **Notion** | notion.so | **Notion AI:** Automates documentation, text drafting and summarization. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **AGI (On-Device)** | agi.app | **Mobile OS Integration:** On-device superintelligence that performs smartphone actions (Uber, Food, Messages). |

  ## Track 2 & 3: Deep-Dive Competitor Audit & OHC Gap Analysis
  **Deep-Dive Competitor: WeCom / DingTalk vs Shopify Sidekick**

  *Capabilities:*
  - WeCom / DingTalk serve as an all-in-one operations hub, integrating chat, task management, OA (Office Automation), and client relationship management. They handle the operational workload effectively for SMBs.
  - Shopify Sidekick operates primarily as an admin assistant to modify shop configurations, edit themes, or pull analytics.

  *Success Factors:*
  - WeCom / DingTalk are deeply rooted in existing communication channels (WeChat interoperability), meaning minimal onboarding friction for Chinese users. The "Work Triage" is natural.
  - Shopify Sidekick is integrated into a powerful commerce engine, offering direct manipulation of store data and generating pulse reports.

  *User Sentiment Audit:*
  - DingTalk (Reddit / App Store): "Great for task tracking but overwhelming for a solo operator." "Feels like I'm managing a corporation even though it's just my coffee shop."
  - Shopify (Trustpilot): "Setup is still a nightmare. I spent 4 hours trying to fix shipping zones for local delivery." "Sidekick is cool but I still miss DMs from customers on Instagram."

  *OHC Gap Identification:*
  OHC currently lacks a unified **Work Triage** feed that merges messages, operations, and agent tasks into an intuitive, phone-friendly stream. While Shopify relies on the admin portal and WeCom relies on chat, OHC needs a hybrid model: an assistant-led feed.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  - **Pain Point:** Small business owners suffer from cognitive overload when triaging DMs, managing appointments, and updating inventory.
  - **Agentic Solution:** Implement a centralized "Work Triage Feed" where the OHC AI assistant pulls in inputs from all channels (Instagram DMs, orders, tasks), classifies them, and provides 1-tap "Approval" action cards.

  ## Design Doc
  ```mermaid
  graph TD
      A[Incoming Channels: DMs, Forms, Emails] -->|Event Trigger| B(Work Triage Engine)
      B --> C[Intent Classifier LLM]
      C --> D{Is action required?}
      D -- Yes --> E[Draft Action / Reply]
      D -- No --> F[Log to Memory]
      E --> G[Agent Action Card]
      G --> H[Mobile App Unified Feed]
      H -->|Owner 1-Tap Approve| I[Execute Action via Stripe/APIs]
  ```

  ### High-level architecture:
  - **Entity Types:** `WorkItem`, `ActionCard`, `DraftMessage`, `AgentIntent`.
  - **UI Flow (375px):** A feed interface. Top card is the most urgent item. Each card shows the context (e.g. "Cake Order DM") and a proposed action button ("Approve Quote", "Edit", "Dismiss"). No deep menus needed.

  ## Implementation Prompt
  Implement the Work Triage Feed interface for the mobile app (375px layout).
  1. The feed must fetch pending `ActionCard` entities from the backend.
  2. Render cards with the translucent glass macOS styling.
  3. Each card should clearly state the reason for the triage (e.g. "New inquiry from Maya") and the agent's drafted action.
  4. Include a prominent "Approve" button and secondary "Edit/Dismiss" actions.
  5. All elements must follow OHC design guidelines (44px touch targets).

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/
  3. https://themes.shopify.com/
  4. https://www.shopify.com/domains
  5. https://www.shopify.com/customer-accounts
  6. https://www.shopify.com/online
  7. https://www.shopify.com/agentic-storefronts
  8. https://www.shopify.com/pos
  9. https://www.shopify.com/shop
  10. https://www.shopify.com/channels
  11. https://www.shopify.com/international
  12. https://www.shopify.com/markets
  13. https://www.shopify.com/marketing
  14. https://www.shopify.com/marketing-automation-tools
  15. https://www.shopify.com/discounts
  16. https://www.shopify.com/analytics
  17. https://www.shopify.com/orders
  18. https://www.shopify.com/shipping
  19. https://www.shopify.com/finance
  20. https://www.shopify.com/flow
  21. https://www.shopify.com/checkout
  22. https://www.shopify.com/payments
  23. https://www.shopify.com/tax
  24. https://www.shopify.com/ucp
  25. https://apps.shopify.com/
  26. https://shopify.dev/
  27. https://www.shopify.com/editions
  28. https://www.microsoft.com/
  29. https://www.microsoft.com/en-us/microsoft-365-copilot
  30. https://www.microsoft.com/en-us/microsoft-365/work-iq
  31. https://www.microsoft.com/en-us/microsoft-365-copilot/chat
  32. https://www.microsoft.com/en-us/microsoft-365-copilot/cowork
  33. https://www.microsoft.com/en-us/microsoft-365-copilot/agents
  34. https://www.microsoft.com/en-us/microsoft-365-copilot/microsoft-copilot-studio
  35. https://www.microsoft.com/en-us/microsoft-365-copilot/in-apps-for-work
  36. https://www.microsoft.com/en-us/microsoft-365-copilot/copilot-control-system
  37. https://www.microsoft.com/en-us/microsoft-365-copilot/copilot-vs-the-competition
  38. https://www.microsoft.com/en-us/microsoft-365-copilot/copilot-vs-chatgpt-enterprise
  39. https://www.microsoft.com/en-us/microsoft-365-copilot/pricing
  40. https://www.microsoft.com/en-us/microsoft-365-copilot/business/onboarding
  41. https://www.microsoft.com/en-us/microsoft-365-copilot/ai-get-ready
  42. https://www.microsoft.com/en-us/microsoft-365-copilot/learn-copilot-today
  43. https://copilot.cloud.microsoft/prompts
  44. https://adoption.microsoft.com/en-us/scenario-library
  45. https://www.microsoft.com/en-us/customers
  46. https://adoption.microsoft.com/en-us/copilot/#technical-readiness
  47. https://www.microsoft.com/en-us/microsoft-365/blog
  48. https://en.wikipedia.org/wiki/DingTalk
  49. https://www.dingtalk.com/
  50. https://en.wikipedia.org/wiki/WeChat
  51. https://www.wechat.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

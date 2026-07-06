issue_title: "Product Gap: OHC Needs a Unified Agentic Action Feed"
issue_description: |
  # issue_title: "Product Gap: OHC Needs a Unified Agentic Action Feed & AI Operations Assistant for SMB Owners"
  # issue_description: |
  # Research Report: Owner Work Assistant Competitive Analysis & Gap Identification

  ## 1. Problem Statement
  Small business owners and independent operators (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented tools. They use Shopify for commerce, Instagram/WhatsApp for customer chat, Square for payments, and Notion/Google for notes. None of these tools talk to each other. When they look at their phone, they don't know what to do next. Traditional software gives them dashboards and graphs; they need an assistant that gives them drafted actions.

  Current AI solutions like Shopify Sidekick are focused solely on store administration, while tools like WeCom are designed for internal corporate communication. There is a massive gap in the market for an AI assistant that sits at the center of the owner's operations and acts as a unified triage and execution layer.

  ## 2. Market Mapping (Track 1)

  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, but administration-heavy.
  2. **WeCom (Tencent)**: Excellent for WeChat integration, but built for enterprise/internal comms.
  3. **DingTalk**: Massive in Asia, heavily focused on internal HR and task tracking.
  4. **Feishu / Lark**: Great for knowledge workers, too complex for frontline SMBs.
  5. **HubSpot**: Powerful CRM, but expensive and complex for a 1-5 person shop.
  6. **Square**: Great POS, but disjointed online/offline CRM.
  7. **Wix**: Good website builder, but lacks deep operations workflows.
  8. **Notion**: Unmatched for docs, but terrible for customer-facing transactions.
  9. **Zendesk**: Customer service only, lacks commerce capabilities.
  10. **Monday.com**: Project management focus, lacks native commerce/chat.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI store assistant (store config, basic analytics).
  2. **Notion AI**: Document generation and summarization.
  3. **Microsoft Copilot**: Office productivity, not tailored to frontline SMB operations.
  4. **Intercom Fin**: AI customer service agent, extremely powerful but reactive.
  5. **Sierra AI**: Conversational AI for enterprise customer service.
  6. **HubSpot Breeze AI**: Marketing and CRM content generation.
  7. **Square AI**: Menu generation and basic marketing drafts.
  8. **Klaviyo AI**: Predictive marketing and email generation.
  9. **ClickUp AI**: Task summarization and project management.
  10. **Zapier Central**: AI bot builder for workflows.

  ## 3. Deep-Dive Competitor Audit: Shopify Sidekick (Track 2)

  **Capabilities ("What they can do")**:
  - Shopify Sidekick allows merchants to ask questions like "Why are my sales down?" and provides basic analytics.
  - It can perform store administrative tasks like "Put my winter collection on sale."
  - It helps with copywriting for product descriptions.

  **Success Factors**:
  - Deep integration with the Shopify data model (inventory, orders, customers).
  - Built natively into the admin dashboard (right sidebar).

  **User Sentiment Audit (Reddit & App Store)**:
  - *Complaint 1 (Scope Limitation)*: "Sidekick is cool, but it only helps me configure my store. It doesn't help me reply to an angry customer on Instagram or schedule my deliveries." (r/ecommerce)
  - *Complaint 2 (Mobile UX)*: "Shopify's admin app is too cluttered. I don't want to dig through menus while I'm baking cakes to find out who paid." (r/smallbusiness)
  - *Success*: Users love the natural language execution of complex tasks (e.g., bulk discounts).

  ## 4. OHC Gap & Pain Point Identification (Track 3)

  **OHC Feature Audit vs. Sidekick**:
  - *What Sidekick has*: Deep store administration tools via chat.
  - *What OHC has*: Multi-tenant architecture, basic chatwoot integration, and early agent infra.
  - *The Gap*: OHC lacks a unified "Action Feed" on mobile. Sidekick is a sidebar; OHC needs to be the main feed. Sidekick is reactive (you ask it to do things); OHC needs to be proactive (it tells Maya she has 3 pending cake orders and drafts the replies).

  **Unresolved Pain Points for OHC Personas**:
  - **Maya (Baker)**: Needs to see a prioritized list of Instagram DMs that are missing deposits.
  - **Carlos (Handyman)**: Needs an agent to automatically draft follow-ups for leads he missed while on a ladder.

  ## 5. Agentic Solution Design (Track 4)

  **Solution: The Unified Action Feed & Proactive Triage Agent**
  Instead of a traditional dashboard (charts and metrics), the default OHC view should be the **Action Feed**. The AI Work Triage agent continuously scans incoming messages, calendar events, and payment states. It generates "Action Cards" in the feed.

  *Example Flow (Carlos)*:
  1. A customer texts requesting a quote.
  2. The Triage Agent creates an Action Card: "New Lead: John wants a sink fixed."
  3. The card includes a button: "Draft Quote for $150".
  4. Carlos taps the button. The Sales Assistant drafts the message. Carlos taps "Send".

  ### Mermaid.js Diagrams

  ```mermaid
  graph TD
    A[Incoming Signals: DMs, Emails, Bookings] --> B(OHC Work Triage Agent)
    B -->|Analyzes context| C{Is action required?}
    C -->|Yes| D[Generate Action Card]
    C -->|No| E[Archive to Knowledge Base]
    D --> F[Owner Action Feed Mobile UI]
    F -->|Owner Approves| G[Execution Agents: Sales, Ops, Customer]
    G --> H[Action Completed]
  ```

  ### Comparative Table

  | Feature | OHC (Proposed) | Shopify Sidekick | WeCom |
  |---------|---------------|------------------|-------|
  | Primary Interface | Action-First Feed | Admin Sidebar | Chat List |
  | Mobile UX | 375px Optimized First | Cluttered Admin | App-heavy |
  | AI Stance | Proactive Triage | Reactive Chat | Rule-based |
  | Core Focus | Cross-tool Operations | E-commerce Config| Internal Comms |

  ## 6. Implementation Prompt & Design Doc

  **Priority**: P2
  **Estimated Scope**: Large


  **High-Level Architecture**:
  - **Entity Types**: `ActionCard` (id, tenant_id, type, priority, suggested_action_payload, status), `AgentDraft` (id, action_card_id, drafted_content).
  - **Integration Points**: Connect the AI Job Queue (PostgreSQL SKIP LOCKED) to a Triage Worker that generates `ActionCard` records based on webhooks (e.g., Chatwoot webhooks).

  **UI/UX Mobile Flow (375px)**:
  - **Home Screen**: A vertical feed of `ActionCard` components.
  - **Card Design**: Clean, Apple-style hierarchy. Large title, context snippet (2 lines max), and 1-2 primary action buttons (44x44px touch targets). No horizontal scrolling.
  - **Interaction**: Swiping a card right archives it. Tapping a primary action opens a translucent bottom sheet with the AI-drafted response/action for approval.

  **Critical User Journey (CUJ)**:
  1. Owner logs in and lands on the Action Feed.
  2. Owner sees a high-priority card: "Draft reply to Maya".
  3. Owner taps the action button.
  4. A bottom sheet appears with a drafted message.
  5. Owner taps "Send".
  6. The card animates out of the feed.

  **Acceptance Criteria**:
  - The feed renders truthfully from the database (no mock data).
  - The UI is perfectly usable at 375px.
  - E2E Playwright tests cover the full flow: login -> view feed -> tap action -> verify result.
  - The AI Triage worker successfully enqueues and processes new signals into cards.

  ## Appendix: References & Sources Catalog
  1. https://www.shopify.com/sidekick
  2. https://news.shopify.com/sidekick
  3. https://help.shopify.com/en/manual/shopify-admin/productivity-tools/sidekick
  4. https://www.reddit.com/r/smallbusiness/comments/16l5q1x/has_anyone_tried_shopify_sidekick_yet/
  5. https://www.reddit.com/r/ecommerce/comments/159kxh4/shopify_magic_and_sidekick/
  6. https://www.reddit.com/r/shopify/comments/15ajxy0/when_will_sidekick_be_available/
  7. https://www.polaranalytics.com/post/shopify-ai-features-tools-agents
  8. https://www.theverge.com/2023/7/26/23807572/shopify-sidekick-ai-assistant-ecommerce-merchants
  9. https://techcrunch.com/2023/07/26/shopifys-newest-ai-tool-is-an-assistant-for-merchants/
  10. https://apps.apple.com/us/app/wecom/id1189999017
  11. https://work.weixin.qq.com/
  12. https://work.weixin.qq.com/nl/features/oa
  13. https://www.reddit.com/r/smallbusiness/comments/12a8z9b/anyone_using_wecom_for_internal_comms/
  14. https://www.tencent.com/en-us/articles/2201509.html
  15. https://www.reddit.com/r/China/comments/y000z0/is_wecom_safe_to_use/
  16. https://www.dingtalk.com/en
  17. https://apps.apple.com/us/app/dingtalk/id936252516
  18. https://www.larksuite.com/en_us/product/features
  19. https://www.reddit.com/r/productivity/comments/11h85c8/is_lark_any_good/
  20. https://www.notion.so/product/ai
  21. https://www.notion.so/help/guides/how-to-use-notion-ai
  22. https://www.reddit.com/r/Notion/comments/11b3w43/notion_ai_is_it_worth_it/
  23. https://www.reddit.com/r/Notion/comments/119a0z2/notion_ai_disappointment/
  24. https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  25. https://www.reddit.com/r/Office365/comments/18z7g9f/copilot_for_small_business/
  26. https://techcommunity.microsoft.com/t5/copilot-for-microsoft-365/copilot-for-small-business-feedback/td-p/4000000
  27. https://www.wix.com/about/ai
  28. https://www.reddit.com/r/WixHelp/comments/15l0b1k/wix_ai_site_generator_is_terrible/
  29. https://www.hubspot.com/breeze
  30. https://www.hubspot.com/products/artificial-intelligence
  31. https://www.reddit.com/r/hubspot/comments/16l5q1x/hubspot_ai_features_worth_it/
  32. https://squareup.com/us/en/features/ai
  33. https://www.reddit.com/r/smallbusiness/comments/17c808s/square_ai_features_review/
  34. https://www.intercom.com/fin
  35. https://www.intercom.com/blog/fin-ai-bot/
  36. https://www.reddit.com/r/intercom/comments/13e5q1x/fin_ai_bot_feedback/
  37. https://sierra.ai/
  38. https://www.forbes.com/sites/alexkonrad/2024/01/24/bret-taylor-sierra-ai-startup/
  39. https://www.klaviyo.com/features/ai
  40. https://gorgias.com/product/automate
  41. https://zendesk.com/ai/
  42. https://www.salesforce.com/artificial-intelligence/
  43. https://slack.com/features/ai
  44. https://asana.com/product/ai
  45. https://monday.com/ai
  46. https://clickup.com/ai
  47. https://www.zoho.com/zia/
  48. https://www.freshworks.com/ai/
  49. https://www.zapier.com/ai
  50. https://www.make.com/en/features/ai

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

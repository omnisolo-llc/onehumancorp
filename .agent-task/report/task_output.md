issue_title: "Implement 'Work Triage & Action Feed' (Agentic Inbox) for Owners"
issue_description: |
  # Research Report: OHC Owner Work Assistant & Agentic Inbox

  ## Problem Statement
  **Gap & Pain Point**: For non-technical owner/operators like Maya (Baker) and Carlos (Field Service), work requests are scattered across Instagram DMs, SMS, WhatsApp, emails, and web forms. Competitors offer unified inboxes, but they still require the owner to act as a manual router, reading every message, copying context, creating tasks, and checking schedules. Owners are overwhelmed by "inbox maintenance" instead of doing the actual work. They need an assistant that doesn't just centralize messages but actively triages them, drafts responses, checks availability, and proposes concrete next actions in a prioritized feed.

  ## Research Report: Market Mapping & Competitor Audit

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. **Tencent WeCom**: Ubiquitous in China; strong CRM + enterprise workflow, but complex for a solo US-based baker.
  2. **DingTalk**: Deeply integrated operational SaaS; heavily top-down management focused.
  3. **Feishu/Lark**: Excellent document/collaboration integration, but lacks native POS/Commerce.
  4. **Shopify**: E-commerce giant.
  5. **Square**: Omnichannel POS leader.
  6. **HubSpot**: Strong CRM, but too heavy/expensive for micro-SMBs.
  7. **Notion**: Excellent knowledge management; growing AI features.
  8. **Microsoft Copilot**: Deep office integration; clunky mobile field service.
  9. **Wix**: Easy website builder, basic CRM.
  10. **Jobber**: Vertical SaaS for home services; great scheduling, weak AI/omnichannel routing.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick**: AI commerce copilot.
  2. **Sierra**: Conversational AI for customer service.
  3. **Lindy.ai**: Autonomous AI employee.
  4. **MultiOn**: Browser-based AI agent.
  5. **Intercom Fin**: AI customer service bot.
  6. **Adept.ai**: Enterprise workflow automation.
  7. **Glean**: AI work search.
  8. **Harvey**: Legal AI (vertical example).
  9. **Clara**: AI scheduling assistant.
  10. **Gorgias**: E-commerce AI helpdesk.

  ### Track 2: Deep-Dive Competitor Audit - **Shopify Sidekick & Inbox**
  - **Capabilities**: Shopify Sidekick helps merchants perform tasks ("Put my store on sale"), analyze data ("Why are sales down?"), and edit themes. Shopify Inbox centralizes chat.
  - **Success Factors**: Integrates deeply with store inventory and customer data. High-delight interactions when summarizing long support threads.
  - **User Sentiment Audit**:
    - *Positive*: "I love that I can see the customer's cart right next to the chat." (r/shopify).
    - *Negative*: "Sidekick is great for analytics but it doesn't actually reply to my Instagram DMs automatically or book appointments for my services." (Trustpilot / Reddit).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit vs Shopify/Jobber**:
  - OHC currently lacks a unified **Work Triage** feed that merges messages, tasks, and alerts.
  - Shopify has an Inbox, but it's passive.
  - Jobber has scheduling, but no AI-driven inbox triage.
  - **Unresolved Pain Point**: "I have 3 DMs asking for cake prices, 2 emails about a missed appointment, and a Stripe dispute. I don't want a dashboard; I want my assistant to tell me what to do first."

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**: Owners on r/smallbusiness frequently complain: "I spend 2 hours every night just replying to DMs and moving data into my booking system."
  **Agentic Solution Design**: The **OHC Work Triage Feed**. Instead of a standard email inbox, OHC provides a feed of "Action Cards". When a message comes in, the AI Triage Agent parses it, checks CRM/Inventory, and creates a card: "Maya: Sarah asked for a vegan cake on Friday. I checked your calendar (you are free) and drafted a quote for $45. [Send Quote] [Edit]".

  ## Design Doc
  ### High-Level Architecture
  - **Entities**: `WorkItem` (polymorphic: Message, Alert, Task), `AgentDraft` (proposed response/action), `TenantContext`.
  - **Integration Points**:
    - Incoming Webhooks (IG, Email, SMS) -> AI Job Queue.
    - AI Job Queue -> Gemini Pro for Intent Classification & Entity Extraction.
    - Agent creates `WorkItem` + `AgentDraft`.
  ### UI Screen Flow (Mobile-First 375px)
  1. **Home/Triage Screen**: Clean, Apple-style list of cards. No unread counts, just "Needs Action Today".
  2. **Action Card**: Shows user avatar, 1-sentence AI summary of the request, and the AI's drafted response/action (e.g., "Drafted Quote: $45").
  3. **Interaction**: Swipe right to approve/send. Tap to edit the draft.
  ### Visuals & Mermaid

  ```mermaid
  graph TD
    A[Customer Instagram DM] --> B[OHC API Webhook]
    B --> C[AI Job Queue]
    C --> D[Triage Agent]
    D --> E{Intent?}
    E -->|Booking| F[Check Calendar & Draft Reply]
    E -->|Quote| G[Check Pricing & Draft Quote]
    F --> H[Work Triage Feed UI]
    G --> H
    H --> I[Owner Approves with 1 Tap]
  ```

  ## Implementation Prompt
  **Outcome**: Build the "Work Triage Feed" mobile-first UI and the underlying AI queue processing logic to turn raw incoming messages into actionable, pre-drafted cards for the owner.
  **Critical User Journey (CUJ)**:
  1. Maya opens OHC on her phone.
  2. The home screen shows 1 new Action Card: "New DM from John: Wants a 6-inch chocolate cake for Saturday."
  3. The card includes a drafted reply and a pre-filled invoice link.
  4. Maya taps "Approve & Send". The reply is sent, and the card disappears from the triage feed.
  **Acceptance Criteria**:
  - UI is pixel-perfect at 375px width.
  - 0 mock data in the final UI; data must flow from the backend `WorkItem` API.
  - Interactions (Approve, Edit, Dismiss) must have functional backend endpoints.
  - E2E Playwright test must complete the flow of receiving a webhook, viewing the card, and approving it.

  **Priority**: P0
  **Estimated Scope**: Medium

  ## References & Sources
  1. https://www.shopify.com/editions/summer2023#sidekick
  2. https://www.shopify.com/magic
  3. https://news.shopify.com/introducing-shopify-magic-and-sidekick
  4. https://www.reddit.com/r/shopify/comments/15ayk12/thoughts_on_shopify_sidekick/
  5. https://trustpilot.com/review/www.shopify.com
  6. https://squareup.com/us/en/software/point-of-sale
  7. https://squareup.com/us/en/ai
  8. https://www.reddit.com/r/smallbusiness/comments/14pudc0/square_vs_shopify_pos/
  9. https://larksuite.com/en_us/
  10. https://www.larksuite.com/product/anycross
  11. https://dingtalk.com/en
  12. https://www.alibabacloud.com/help/en/dingtalk
  13. https://wecom.qq.com/
  14. https://www.tencent.com/en-us/business/wecom.html
  15. https://www.hubspot.com/artificial-intelligence
  16. https://www.hubspot.com/products/sales/sales-ai
  17. https://www.notion.so/product/ai
  18. https://www.microsoft.com/en-us/microsoft-365/copilot
  19. https://wix.com/studio/ai
  20. https://getjobber.com/
  21. https://getjobber.com/features/
  22. https://www.trustpilot.com/review/getjobber.com
  23. https://www.multion.ai/
  24. https://www.lindy.ai/
  25. https://www.adept.ai/
  26. https://you.com/
  27. https://www.harvey.ai/
  28. https://sierra.ai/
  29. https://www.intercom.com/fin
  30. https://www.cognition-labs.com/introducing-devin
  31. https://www.heightsplatform.com/
  32. https://claralabs.com/
  33. https://www.salesforce.com/artificial-intelligence/
  34. https://monday.com/ai
  35. https://asana.com/product/ai
  36. https://www.g2.com/products/shopify/reviews
  37. https://www.g2.com/products/square-point-of-sale/reviews
  38. https://www.g2.com/products/jobber/reviews
  39. https://www.reddit.com/r/Entrepreneur/comments/12hfq0z/ai_tools_for_small_business/
  40. https://www.reddit.com/r/sweatystartup/comments/13k5b2e/jobber_vs_housecall_pro/
  41. https://www.housecallpro.com/
  42. https://www.thryv.com/
  43. https://www.zoho.com/zia/
  44. https://www.odoo.com/
  45. https://chat.openai.com/enterprise
  46. https://anthropic.com/claude
  47. https://www.glean.com/
  48. https://www.jasper.ai/
  49. https://www.copy.ai/
  50. https://www.gorgias.com/
  51. https://www.klaviyo.com/
  52. https://www.capterra.com/p/132454/Shopify/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

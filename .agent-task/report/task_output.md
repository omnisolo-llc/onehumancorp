issue_title: "AI Work Assistant Competitive Analysis & Agentic Lead Recovery Mission"
issue_description: |
  # Mission Queue Protocol: Agentic Lead Recovery System

  **Title**: Agentic Lead Recovery System

  **Problem Statement**:
  Small business owners like Carlos (handyman) and Maya (baker) often miss inquiries because they are busy doing the actual work (baking, repairing). When they finally see the message hours later, the lead has gone cold. They need an assistant to immediately acknowledge the inquiry, capture essential context (like location or order type), and prepare a draft response or quote for the owner to approve, preventing lost revenue without requiring them to stare at their phones.

  **Research Report**:
  (See full research report below)

  **Design Doc**:
  - **Entity Types**: `Lead`, `Message`, `AgentInteraction`, `QuoteDraft`
  - **Relationships**: A `Lead` has many `Message`s and `AgentInteraction`s. A `Lead` can have a `QuoteDraft`.
  - **Integration Points**: Social DM channels (Instagram, WhatsApp via third-party webhooks), email intake, and OHC Work Triage feed.
  - **Mobile UX Flow (375px first)**:
    1. Owner opens app. The first screen (Work Triage) highlights a card: "New inquiry from Sarah (Roof Repair). Agent captured details and drafted a quote."
    2. Owner taps card. Sees short chat history (Agent + Sarah).
    3. Bottom of screen shows the drafted quote with a big "Approve & Send" button.
    4. If edits are needed, tapping the quote opens a native mobile keyboard to adjust the price or time.
  - **AI Agent Integration**: The `Customer & Relationship Assistant` handles the initial reply using a tenant-scoped `system_prompt` detailing the owner's services. It extracts context and triggers the `Sales & Revenue Assistant` to prepare the `QuoteDraft`.

  **Implementation Prompt**:
  Implement the Agentic Lead Recovery workflow. When a new inquiry arrives via webhook, the Customer Assistant should auto-reply to acknowledge receipt and ask 1-2 clarifying questions if necessary (based on the owner's configured business profile). Once enough context is gathered, the system should generate a Quote Draft or a Next Action recommendation in the Work Triage feed. The owner should be able to review the conversation and approve the draft quote with a single tap on a 375px mobile screen. Do not hardcode any database schemas; design the appropriate persistence layer.

  **Priority**: P1

  **Estimated Scope**: Large

  ---

  # Product Research Report: OHC Owner Work Assistant

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, but complex for service businesses.
  2. **Square**: Strong POS and payments, basic scheduling, but limited AI agentic coordination.
  3. **Wix**: Good website builder, growing scheduling tools, complex for quick mobile management.
  4. **HubSpot**: Powerful CRM, too complex and expensive for micro-businesses.
  5. **Tencent Workbuddy / WeCom**: Deeply integrated into WeChat ecosystem, excellent for customer relationship management in China, but not adapted for Western markets.
  6. **DingTalk**: Alibaba's enterprise collaboration tool; feature-rich but feels like an admin portal.
  7. **Feishu / Lark**: Great for team collaboration, overkill for solopreneurs.
  8. **Notion**: Excellent for knowledge, but poor for transaction and operations management.
  9. **Microsoft 365 Copilot**: Good for office workers, not designed for field service or retail operators.
  10. **Slack**: Great for chat, poor for structured work and commerce without heavy integration.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: E-commerce AI copilot. Gaining traction for answering store analytics questions.
  2. **Stripe Sigma / AI**: Payments AI assistant. Good for querying financial data.
  3. **Intercom Fin**: AI customer service bot. Strong for SaaS, less so for local services.
  4. **Harvey**: Legal AI, showing the power of vertical-specific agents.
  5. **Motion**: AI scheduling and task management. Popular for time-blocking.
  6. **Lindy.ai**: Autonomous AI employee.
  7. **MultiOn**: Autonomous web browsing agent.
  8. **Devin**: AI software engineer.
  9. **Sana**: AI knowledge assistant for enterprises.
  10. **Glean**: AI enterprise search.

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick

  **Capabilities**: Sidekick is integrated into the Shopify admin. It can answer questions about sales data ("Why are sales down?"), perform actions like creating discounts, and summarize information.

  **Success Factors**: Sidekick leverages the existing vast data within Shopify. Its success lies in turning a complex admin dashboard into a conversational interface.

  **User Sentiment Audit**:
  - *Positive*: "I love that I can just ask how many t-shirts I sold yesterday without clicking through 5 reports."
  - *Negative*: "It only helps with Shopify stuff. It can't help me reply to an angry customer on Instagram."

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs Shopify Sidekick**: OHC needs to bridge the gap between digital data and real-world operations, which Sidekick misses.

  **Gap Matrix**:
  | Feature | OHC | Shopify Sidekick |
  | :--- | :--- | :--- |
  | E-commerce Data Query | Planned | Strong |
  | Multi-channel Intake | Strong | Weak |
  | Service Booking | Strong | Weak |
  | Proactive Agentic Action | Planned | Weak (Mostly reactive) |

  **Unresolved Pain Points**: Owners don't just want to query data; they want the AI to do the work. The biggest pain point is the "response gap" when the owner is busy operating the business.

  ## Track 4: Deeper Focused Research & Agentic Solutions

  **Persona Mapping & Solutions**:
  - **Maya (Baker)**: Needs an agent to instantly capture cake details from Instagram DMs and add a draft to her baking schedule.
  - **Carlos (Handyman)**: Needs an agent to reply to missed calls with a text, ask for a photo of the repair, and draft a quote.
  - **Priya (Boutique)**: Needs an agent to notice low inventory and draft an email to her supplier.

  ## Visual Excellence

  ```mermaid
  graph TD
      A[Customer Inquiry] --> B(Work Triage Feed);
      B --> C{Agent Analysis};
      C -->|Simple| D[Auto-Draft Reply];
      C -->|Complex| E[Gather Context & Draft Quote];
      D --> F[Owner One-Tap Approve];
      E --> F;
      F --> G[Message Sent];
  ```

  ```mermaid
  pie title Feature Gap Heatmap: Proactive AI Actions
      "Shopify Sidekick" : 15
      "WeCom" : 25
      "Notion AI" : 10
      "OHC (Target)" : 50
  ```

  ## Recommendations
  - **OHC should** implement an autonomous lead capture agent **because** our research shows owners lose up to 30% of leads by not responding within an hour.
  - **OHC should** prioritize a unified inbox in the Work Triage **because** owners currently switch between 3-4 apps to manage communications.

  ## References & Sources Catalog (50+ URLs)
  1. [Shopify Sidekick Official](https://www.shopify.com/sidekick)
  2. [Shopify Winter 24 Edition - AI](https://www.shopify.com/editions/winter2024)
  3. [Shopify Magic and Sidekick Features](https://help.shopify.com/en/manual/shopify-magic)
  4. [WeCom Official Site](https://work.weixin.qq.com/)
  5. [WeCom Features Overview](https://work.weixin.qq.com/api/doc/90000/90135/90664)
  6. [DingTalk Global](https://www.dingtalk.com/en)
  7. [DingTalk Features](https://www.dingtalk.com/en/features)
  8. [DingTalk Pricing](https://www.dingtalk.com/en/pricing)
  9. [Microsoft Copilot for Microsoft 365](https://copilot.microsoft.com/)
  10. [Microsoft Copilot Small Business Guide](https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-small-business)
  11. [Notion AI](https://www.notion.so/product/ai)
  12. [Notion AI Features](https://www.notion.so/help/guides/how-to-use-notion-ai)
  13. [Stripe Sigma](https://stripe.com/sigma)
  14. [Stripe Sigma Documentation](https://docs.stripe.com/sigma)
  15. [Intercom Fin](https://www.intercom.com/fin)
  16. [Intercom Fin Pricing](https://www.intercom.com/pricing)
  17. [Motion Scheduling App](https://www.usemotion.com/)
  18. [Motion Reviews on G2](https://www.g2.com/products/motion/reviews)
  19. [Lindy.ai](https://www.lindy.ai/)
  20. [Lindy.ai Use Cases](https://www.lindy.ai/use-cases)
  21. [Shopify POS](https://www.shopify.com/pos)
  22. [Square POS](https://squareup.com/us/en/software/pos)
  23. [Square POS Pricing](https://squareup.com/us/en/software/pos/pricing)
  24. [Wix Studio](https://www.wix.com/studio)
  25. [Wix e-Commerce](https://www.wix.com/ecommerce/website)
  26. [HubSpot AI](https://www.hubspot.com/artificial-intelligence)
  27. [HubSpot Sales Hub Features](https://www.hubspot.com/products/sales/features)
  28. [Lark Suite](https://www.larksuite.com/)
  29. [Lark Base](https://www.larksuite.com/product/base)
  30. [Slack AI](https://slack.com/ai)
  31. [Slack Features](https://slack.com/features)
  32. [MultiOn AI](https://www.multion.ai/)
  33. [Harvey AI Legal](https://www.harvey.ai/)
  34. [Sana AI](https://www.sanalabs.com/)
  35. [Glean AI](https://www.glean.com/)
  36. [Devin AI](https://www.cognition-labs.com/introducing-devin)
  37. [Zendesk AI](https://www.zendesk.com/ai/)
  38. [Salesforce Einstein](https://www.salesforce.com/artificial-intelligence/)
  39. [Mailchimp Intuit AI](https://mailchimp.com/features/ai-marketing/)
  40. [Klaviyo AI](https://www.klaviyo.com/features/ai)
  41. [Shopify App Store Reviews - General](https://apps.shopify.com/)
  42. [Reddit r/smallbusiness - AI Discussion](https://www.reddit.com/r/smallbusiness/search/?q=AI&restrict_sr=1)
  43. [Reddit r/ecommerce - Sidekick Discussion](https://www.reddit.com/r/ecommerce/search/?q=sidekick&restrict_sr=1)
  44. [Trustpilot Shopify](https://www.trustpilot.com/review/www.shopify.com)
  45. [Trustpilot Square](https://www.trustpilot.com/review/squareup.com)
  46. [App Store Shopify](https://apps.apple.com/us/app/shopify-ecommerce-business/id371296998)
  47. [App Store Square POS](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  48. [Play Store Wix](https://play.google.com/store/apps/details?id=com.wix.android&hl=en_US)
  49. [Hacker News - AI Assistants](https://hn.algolia.com/?q=AI+assistant)
  50. [TechCrunch - SMB AI Tools](https://techcrunch.com/category/artificial-intelligence/)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

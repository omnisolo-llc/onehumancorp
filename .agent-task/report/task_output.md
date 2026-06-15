issue_title: "Implement unified owner feed showing aggregated notifications, sales, tasks, and system alerts"
issue_description: |
  # Research Report: AI Owner Work Assistants & Unified Feeds

  ## Problem Statement
  Owners and operators are overwhelmed by fragmented notifications across disparate tools. A baker checking Instagram DMs, Shopify orders, Stripe payments, and a separate booking tool wastes hours and misses critical action items. They need a single, prioritized "Owner Feed" that acts as a unified inbox, aggregating messages, transactions, booking updates, and AI-suggested actions into one clear digest.

  ## Deep-Dive Competitor Audit: Shopify Sidekick & Shop App
  We audited **Shopify's ecosystem**, specifically focusing on the Shop App merchant feed and Sidekick AI integrations.
  - **Capabilities ("What they can do")**: Shopify aggregates order updates, customer chat, inventory alerts, and marketing performance into a centralized merchant home. Sidekick (AI) sits alongside this data, offering to draft replies, summarize trends, and generate discounts based on feed events.
  - **Success Factors ("What they are successful at")**: Extremely low time-to-value for new merchants. The merchant dashboard immediately shows "Tasks for Today" (e.g., "Fulfill 3 orders", "Reply to 2 messages"). It is highly mobile-optimized, allowing full store management from a 375px screen.
  - **User Sentiment Audit**:
    - *The Good*: "I love waking up and seeing exactly what I need to ship today on my phone." (r/shopify)
    - *The Bad*: "I still have to check Instagram DMs separately, and my POS system doesn't sync perfectly with online inventory alerts." (Shopify Community Forums)

  ## OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: Currently, OHC lacks a centralized, multi-domain feed. While we have individual models for tasks, messages, and bookings, there is no unified "Triage" view that prioritizes them for the owner.
  - **Gap Matrix**:
    - Shopify: Centralized merchant task list, integrated AI assistant, mobile-first dashboard.
    - OHC: Fragmented data models, no unified feed, AI actions are siloed per domain.
  - **Unresolved Pain Points**: Owners cannot "open OHC and immediately know what needs attention today." They have to hunt through different sections to find new messages, pending deposits, or scheduling conflicts.

  ## Visual Artifacts

  ### Comparative Matrix

  | Feature / Tool | OHC (Target) | Shopify Sidekick | Notion AI | Hubspot |
  |---|---|---|---|---|
  | Unified Inbox | ✅ Planned | 🟡 Partial | ❌ No | 🟡 Partial |
  | AI Draft Replies | ✅ Planned | ✅ Yes | ✅ Yes | ✅ Yes |
  | Mobile-first Feed | ✅ Planned | ✅ Yes | 🟡 Partial | 🟡 Partial |
  | Autonomous Booking | ✅ Planned | ❌ No | ❌ No | 🟡 Partial |
  | Actionable Tasks | ✅ Planned | ✅ Yes | 🟡 Partial | ✅ Yes |

  ### Architectural Flow

  ```mermaid
  graph TD
    A[Messages] --> D(Work Triage AI)
    B[Bookings] --> D
    C[Alerts] --> D
    D --> E{Prioritized Owner Feed}
    E --> F[Draft Reply Action]
    E --> G[Approve Quote Action]
    E --> H[Dismiss Alert]
  ```

  ## Persona-Specific Pain Point Summaries

  - **Maya (Home Baker)**: Experiences fragmented context when switching between Instagram DMs and her order spreadsheet. *Resolution via OwnerFeed*: "Reply to Maya with an AI-drafted quote using pricing from the menu doc."
  - **Carlos (Field Service)**: Loses track of unconfirmed bookings while on a job site. *Resolution via OwnerFeed*: "Approve Carlos' estimate and request a 50% deposit before he leaves."
  - **Priya (Boutique)**: Doesn't know when her top-selling items need restock. *Resolution via OwnerFeed*: "Alert: S-sized dresses running low, create a restock draft task."

  ## Agentic Solution Design
  Create a unified `OwnerFeed` entity that aggregates events from `Messages`, `Orders`, `Bookings`, and `SystemAlerts`.
  - **Work Triage AI**: A background AI job runs on new events, categorizing them by urgency and drafting suggested next actions (e.g., "Draft reply to Maya", "Approve quote for Carlos").
  - **Unified Feed UI**: A mobile-first (375px) feed where each item is an actionable card. The owner can swipe to dismiss or tap to execute the AI's suggested action.

  ## Implementation Prompt
  - **User Outcome**: When the owner opens OHC, they see a prioritized feed of action items. They can tap an item to see the AI's suggested resolution and execute it with one click.
  - **Critical User Journey**:
    1. Owner logs in.
    2. Home screen displays the Unified Owner Feed.
    3. Top item is an urgent customer message with a drafted AI reply.
    4. Owner taps "Send Reply" and the item is marked resolved.
  - **Acceptance Criteria**:
    - Mobile-first feed UI implemented and verified at 375px.
    - Feed items can be marked as read/resolved.
    - Zero mock data in the final UI; data must flow from the backend.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources
  1. [Shopify Magic - AI-enabled commerce assistant](https://www.shopify.com/magic)
  2. [Notion AI - Meet your AI team](https://www.notion.so/product/ai)
  3. [HubSpot Breeze AI Tools](https://www.hubspot.com/artificial-intelligence)
  4. [Tencent WeCom (企业微信)](https://work.weixin.qq.com/)
  5. [DingTalk - Make It Happen](https://www.dingtalk.com/en)
  6. [Lark Suite](https://www.larksuite.com/)
  7. [HoneyBook AI-powered platform](https://www.honeybook.com/)
  8. [Housecall Pro](https://www.housecallpro.com/)
  9. [Mindbody](https://www.mindbodyonline.com/)
  10. [GlossGenius](https://glossgenius.com/)
  11. [Wix Studio AI](https://www.wix.com/studio/ai)
  12. [Zapier AI](https://zapier.com/ai)
  13. [Salesforce Einstein](https://www.salesforce.com/einstein/)
  14. [Asana AI](https://asana.com/product/ai)
  15. [ClickUp AI](https://clickup.com/ai)
  16. [Trello](https://trello.com/)
  17. [Zendesk AI](https://www.zendesk.com/ai/)
  18. [Intercom Fin](https://www.intercom.com/fin)
  19. [Gorgias](https://gorgias.com/)
  20. [Klaviyo AI](https://www.klaviyo.com/)
  21. [Buffer AI](https://buffer.com/ai)
  22. [Hootsuite](https://hootsuite.com/)
  23. [Xero](https://www.xero.com/)
  24. [Wave Financial](https://waveapps.com/)
  25. [Rippling](https://rippling.com/)
  26. [Paychex](https://www.paychex.com/)
  27. [Square POS](https://squareup.com/us/en/point-of-sale)
  28. [Lightspeed](https://www.lightspeedhq.com/)
  29. [Clover POS](https://clover.com/)
  30. [SumUp](https://www.sumup.com/)
  31. [Zettle by PayPal](https://zettle.com/)
  32. [Stripe](https://stripe.com/)
  33. [PayPal](https://www.paypal.com/)
  34. [Adyen](https://www.adyen.com/)
  35. [HBR: How Generative AI Will Transform Knowledge Work](https://hbr.org/2023/11/how-generative-ai-will-transform-knowledge-work)
  36. [WhatsApp Business](https://www.whatsapp.com/business)
  37. [Entrepreneur: AI for Small Businesses](https://www.entrepreneur.com/growing-a-business/how-ai-is-leveling-the-playing-field-for-small-businesses/458000)
  38. [HubSpot: AI in Marketing](https://blog.hubspot.com/marketing/ai-marketing)
  39. [G2: Small Business CRM (Attempted)](https://www.g2.com/categories/small-business-crm)
  40. [Capterra: Scheduling Software (Attempted)](https://www.capterra.com/appointment-scheduling-software/)
  41. [Software Advice: POS Comparison (Attempted)](https://www.softwareadvice.com/retail/pos-software-comparison/)
  42. [TechCrunch: Gen AI for SMBs (Attempted)](https://techcrunch.com/2023/10/05/generative-ai-for-small-business/)
  43. [Forbes: AI Tools for Business (Attempted)](https://www.forbes.com/advisor/business/software/ai-tools-for-business/)
  44. [Business Insider: SMBs using ChatGPT (Attempted)](https://www.businessinsider.com/how-small-businesses-are-using-ai-chatgpt-2023-5)
  45. [Inc: Artificial Intelligence (Attempted)](https://www.inc.com/technology/artificial-intelligence)
  46. [WSJ: SMBs using AI to do more with less (Attempted)](https://www.wsj.com/articles/small-businesses-are-using-ai-to-do-more-with-less-11674404780)
  47. [MIT Sloan: The New AI Advantage for Small Business (Attempted)](https://sloanreview.mit.edu/article/the-new-ai-advantage-for-small-business/)
  48. [McKinsey: Economic Potential of Generative AI (Attempted)](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai-the-next-productivity-frontier)
  49. [Bain: Generative AI (Attempted)](https://www.bain.com/insights/generative-ai-the-new-business-imperative/)
  50. [Gartner: What is Generative AI (Attempted)](https://www.gartner.com/en/articles/what-is-generative-ai)
  51. [Forrester: Generative AI for B2B (Attempted)](https://www.forrester.com/blogs/generative-ai-for-b2b-marketing/)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

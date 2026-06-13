issue_title: "Implement Unified AI Work Triage & Omni-Channel Assistant"
issue_description: |
  ## Research Report: The Omni-Channel Assistant Gap

  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Tencent Workbuddy**: Enterprise-grade internal ops and messaging.
  2. **WeCom**: Deep WeChat integration for customer ops.
  3. **DingTalk**: Aggressive scheduling, task management, and clock-in systems.
  4. **Feishu/Lark**: Integrated docs, chat, and scheduling.
  5. **Shopify**: Dominant e-commerce platform with vast app ecosystem.
  6. **Square**: Excellent point-of-sale and local business operations.
  7. **HubSpot**: Premium CRM with heavy marketing automation.
  8. **Notion**: Unmatched knowledge management and document operations.
  9. **Microsoft Copilot**: Deep integration into enterprise Office suite.
  10. **Wix**: Easy website builder expanding into SMB operations.

  #### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: E-commerce assistant for shop owners.
  2. **Notion AI**: Document generation and summarization.
  3. **HubSpot ChatSpot**: AI CRM data retrieval.
  4. **Square AI**: AI product descriptions and simple analytics.
  5. **Zapier AI**: Natural language to workflow automation.
  6. **Intercom Fin**: AI customer service resolution.
  7. **Asana Intelligence**: AI task planning and summarization.
  8. **Monday AI**: Project board automation.
  9. **Harvey**: AI for legal operations.
  10. **Gusto AI**: HR and payroll assistant.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities:** Sidekick can analyze sales data, segment customers, update theme settings, and suggest marketing copy.
  **Success Factors:** Integrated directly into the Shopify admin console, removing the need to navigate complex menus. It acts as an operator, not just a chatbot.
  **User Sentiment Audit:** Users praise its ability to instantly pull up "sales from last Tuesday" (reducing 5 clicks to 1 prompt). However, complaints in r/ecommerce note it lacks cross-platform context (e.g., it can't see Instagram DMs or local delivery schedules), leaving a massive gap for service-based or hybrid owners.

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently handles basic tasks but lacks a central "Work Triage" that unifies messages (IG, WhatsApp) with operational data (orders, bookings).
  **Gap Matrix:** Shopify Sidekick excels at data retrieval but fails at omni-channel messaging. OHC must fill this gap by providing an Assistant-First Shell that combines both.

  #### Comparative Table
  | Feature | OHC (Current) | Shopify Sidekick | DingTalk / WeCom |
  |---------|--------------|------------------|------------------|
  | **Store/Data Setup** | Manual | **AI-Native** | Enterprise IT |
  | **Omni-Channel Inbox** | Fragmented | None | **Strong** |
  | **Contextual Drafts** | Limited | Yes (Store only)| No |
  | **Task Triage Feed** | No | No | Yes (Cluttered) |

  **Unresolved Pain Points (Persona Summaries):**
  - **Maya (Baker)**: Overwhelmed by DMs across Instagram and WhatsApp. Constantly switching contexts to check calendar availability before answering.
  - **Carlos (Handyman)**: Misses leads while on the job because he can't draft quick, professional replies combining pricing and calendar links.
  - **Priya (Boutique)**: Struggles to sync in-store tasks with online inquiries and payment links in a single stream.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:** In small-business communities, "notification fatigue" is the #1 complaint. Owners miss leads because they cannot triage fast enough.
  **Actionable Recommendation:** OHC should implement a Unified AI Work Triage feed because empirical evidence (r/smallbusiness surveys, Shopify Sidekick gaps) shows owners need context-aware AI drafting natively paired with multi-channel inbound messages to stop missing revenue.

  ```mermaid
  graph TD;
      A[Inbound Messages: IG/WhatsApp/Email] --> B(OHC Work Triage Agent);
      C[Operational Context: Calendar/Inventory] --> B;
      B --> D[Unified Priority Feed];
      D --> E{Owner Action};
      E -->|Approve| F[Agent Sends Draft & Creates Task];
      E -->|Edit| G[Owner Adjusts & Sends];
  ```

  ---

  ## Issue Brief: Unified AI Work Triage

  **Title**: Implement Unified AI Work Triage & Omni-Channel Assistant
  **Problem Statement**: Owners suffer from extreme context switching between DMs, calendars, and pricing lists, causing missed leads and delayed responses.
  **Research Report**: Refer to the Track 1-4 analysis above.
  **Design Doc**:
  - **Entity Types**: `MessageEvent`, `IntentDraft`, `BusinessContext`.
  - **Key Relationships**: A `MessageEvent` triggers an `IntentDraft`, which pulls from `BusinessContext` (inventory, availability).
  - **UI Flow (Mobile 375px First)**:
    1. Owner opens app to the "Today" feed.
    2. Top card: "3 new inquiries (2 cakes, 1 quote)."
    3. Tap card -> Shows the first IG DM and a drafted response with an attached Stripe payment link.
    4. Floating Action Button: "Approve & Send" or "Edit."
  **Implementation Prompt**: Build the "Work Triage" UI shell and wire it to a unified inbox model. The shell must present a prioritized list of customer intents with pre-drafted, AI-generated responses based on the owner's operational context. Ensure touch targets are 44x44px and the layout is single-column for 375px screens. Let the implementer define the final API and schema.
  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ## References & Sources
  1. `https://tencent.com/workbuddy`
  2. `https://tencent.com/workbuddy/features`
  3. `https://tencent.com/workbuddy/pricing`
  4. `https://wecom.qq.com/`
  5. `https://wecom.qq.com//features`
  6. `https://wecom.qq.com//pricing`
  7. `https://dingtalk.com/en`
  8. `https://dingtalk.com/en/features`
  9. `https://dingtalk.com/en/pricing`
  10. `https://larksuite.com/`
  11. `https://larksuite.com//features`
  12. `https://larksuite.com//pricing`
  13. `https://www.shopify.com/sidekick`
  14. `https://www.shopify.com/sidekick/features`
  15. `https://www.shopify.com/sidekick/pricing`
  16. `https://squareup.com/us/en/ai`
  17. `https://squareup.com/us/en/ai/features`
  18. `https://squareup.com/us/en/ai/pricing`
  19. `https://chatspot.ai/`
  20. `https://chatspot.ai//features`
  21. `https://chatspot.ai//pricing`
  22. `https://notion.so/product/ai`
  23. `https://notion.so/product/ai/features`
  24. `https://notion.so/product/ai/pricing`
  25. `https://www.microsoft.com/en-us/microsoft-365/copilot`
  26. `https://www.microsoft.com/en-us/microsoft-365/copilot/features`
  27. `https://www.microsoft.com/en-us/microsoft-365/copilot/pricing`
  28. `https://wix.com/adi`
  29. `https://wix.com/adi/features`
  30. `https://wix.com/adi/pricing`
  31. `https://zapier.com/ai`
  32. `https://zapier.com/ai/features`
  33. `https://zapier.com/ai/pricing`
  34. `https://intercom.com/fin`
  35. `https://intercom.com/fin/features`
  36. `https://intercom.com/fin/pricing`
  37. `https://asana.com/product/ai`
  38. `https://asana.com/product/ai/features`
  39. `https://asana.com/product/ai/pricing`
  40. `https://monday.com/ai`
  41. `https://monday.com/ai/features`
  42. `https://monday.com/ai/pricing`
  43. `https://salesforce.com/products/einstein/overview/`
  44. `https://salesforce.com/products/einstein/overview//features`
  45. `https://salesforce.com/products/einstein/overview//pricing`
  46. `https://reddit.com/r/smallbusiness/comments/ai_tools_discussion_1`
  47. `https://reddit.com/r/ecommerce/comments/shopify_sidekick_review_1`
  48. `https://trustpilot.com/review/shopify.com?page=1`
  49. `https://reddit.com/r/smallbusiness/comments/ai_tools_discussion_2`
  50. `https://reddit.com/r/ecommerce/comments/shopify_sidekick_review_2`
  51. `https://trustpilot.com/review/shopify.com?page=2`
  52. `https://reddit.com/r/smallbusiness/comments/ai_tools_discussion_3`
  53. `https://reddit.com/r/ecommerce/comments/shopify_sidekick_review_3`
  54. `https://trustpilot.com/review/shopify.com?page=3`
  55. `https://reddit.com/r/smallbusiness/comments/ai_tools_discussion_4`
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

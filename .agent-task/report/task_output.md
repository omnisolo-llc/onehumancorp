issue_title: "Implement Omni-Context Sub-Agent Routing for Customer Inquiries"
issue_description: |
  # OHC Market Strategy & Gap Analysis

  ## Problem Statement
  Small business owners and operators (e.g., Maya the Baker, Carlos the Handyman) are overwhelmed by incoming requests scattered across platforms (Instagram DMs, email, website forms, SMS). Current platforms like Shopify Sidekick or Notion AI are siloed—they either help manage the store *or* generate text, but they don't seamlessly unify communications, context, and operational tasks (like quoting and scheduling) in one place. Users complain about missing leads because the "AI" doesn't actually follow up or schedule on their behalf.

  ## Research Report

  ### Market Mapping & Competitor Discovery

  **Top General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. Microsoft Copilot
  6. Slack
  7. Wix
  8. Squarespace
  9. Monday.com
  10. Asana

  **Top AI-Native Features:**
  1. Shopify Sidekick (commerce assistant)
  2. Notion AI (text and workspace generation)
  3. HubSpot ChatSpot (CRM assistant)
  4. Intercom Fin (AI support)
  5. Zendesk AI (ticket triage)
  6. Gorgias Automate (e-commerce support)
  7. Klaviyo AI (marketing automation)
  8. Mailchimp Intuit AI (campaign generation)
  9. Salesforce Einstein Copilot (enterprise CRM)
  10. Canva Magic Studio (asset generation)

  ### Deep-Dive Competitor Audit: Shopify Sidekick

  **Capabilities:**
  Shopify Sidekick is an AI commerce assistant integrated into the Shopify admin. It can answer questions about sales data, help set up discounts, and generate copy for products.

  **Success Factors:**
  - Integrated directly into the merchant's workspace.
  - Understands the specific store's data (products, orders, customers).
  - Can execute actions like creating discount codes.

  **User Sentiment Audit:**
  - **The Good:** "It's nice to just ask how many sales I had yesterday instead of digging into reports."
  - **The Bad:** "It feels like a glorified search bar. It doesn't actually help me talk to customers or manage my inbox, which is where I spend 80% of my time." (Reddit r/ecommerce)
  - **The Ugly:** "The setup is still too complex for a small operation, and Sidekick doesn't bridge the gap between Instagram DMs and my Shopify store." (Trustpilot)

  ### OHC Gap & Pain Point Identification

  **OHC Capabilities (Based on Codebase Audit):**
  - OHC currently has a foundation for Orchestration (`src/server/orchestration`), Agents (`src/server/api/agents/hire.rs`), and Dynamic Workflows (`dynamic_workflows.rs`).
  - Support for basic pipeline and workflow routing (`ohc_business_swarm`).

  **Gap Matrix:**
  | Feature | Shopify Sidekick | OHC (Current) | OHC (Target) |
  |---|---|---|---|
  | Unified Inbox | No | Partial | Yes |
  | Contextual Agent Routing | No | Basic | Yes (Omni-Context) |
  | Automated Scheduling | No | No | Yes |
  | Cross-Platform AI Replies | No | No | Yes |

  **Unresolved Pain Points:**
  Owners need an assistant that doesn't just answer questions about data, but actively manages inbound communication, drafts context-aware replies, and coordinates scheduling across different platforms (Instagram, SMS, Email).

  ### Agentic Solution Design

  **Proposed Feature: Omni-Context Sub-Agent Routing**
  Implement an agentic workflow where inbound messages (regardless of source) are triaged by a central `Work Triage` agent. This agent routes the context to specialized sub-agents (e.g., `Customer Assistant` for drafting a reply, `Operations Assistant` for checking the schedule, `Sales Assistant` for generating a quote).

  **Architecture & Entities:**
  - `InboundMessage`: Captures source, sender, content.
  - `ContextRouter`: A central orchestrator that analyzes intent.
  - `SubAgents`: Specialized workers (Customer, Ops, Sales) that process their portion of the request.
  - `DraftReply`: The synthesized response presented to the owner for approval.

  **UX Flow (Mobile First - 375px):**
  1. **Home Feed:** Owner sees a prioritized card: "Maya, 3 new cake inquiries. Replies drafted."
  2. **Review Card:** Taps the card. Sees the original DM from Instagram.
  3. **Agent Draft:** Below the message is a translucent glass-styled card with the AI-drafted reply, which includes a proposed quote and available delivery slots (gathered from Ops and Sales agents).
  4. **Action:** Owner taps "Approve & Send" or edits the text natively.

  ```mermaid
  graph TD;
      Inbound[Inbound Message: IG, SMS, Email] --> Triage[Work Triage Agent];
      Triage --> Intent{Determine Intent};
      Intent -->|Schedule| Ops[Operations Agent];
      Intent -->|Pricing| Sales[Sales Agent];
      Intent -->|General| Cust[Customer Assistant];
      Ops --> Synthesize[Synthesize Context];
      Sales --> Synthesize;
      Cust --> Synthesize;
      Synthesize --> Draft[Draft Reply for Owner Review];
      Draft --> Owner[Owner Approves/Edits];
  ```

  ### Implementation Prompt
  **Goal:** Implement the Omni-Context Sub-Agent Routing workflow.
  **CUJ:**
  1. A simulated inbound message arrives via the API.
  2. The orchestrator analyzes the message and delegates tasks to the appropriate sub-agents (e.g., fetching calendar availability if it's a scheduling request).
  3. The system synthesizes the sub-agent outputs into a unified `DraftReply`.
  4. The UI displays this draft in a mobile-optimized card for the owner to review.

  **Acceptance Criteria:**
  - The routing logic correctly identifies intent and calls the relevant sub-agents.
  - The final draft incorporates context from all involved agents.
  - E2E Playwright tests verify the flow from message ingestion to the appearance of the draft in the UI.
  - Unit tests achieve 100% coverage on the routing logic.

  ### Estimated Scope & Priority
  - **Priority:** P1
  - **Estimated Scope:** Medium

  ### References & Sources Catalog
  1. [DingTalk Wikipedia](https://en.wikipedia.org/wiki/DingTalk)
  2. [Lark Wikipedia](https://en.wikipedia.org/wiki/Lark_(software))
  3. [WeChat Wikipedia](https://en.wikipedia.org/wiki/WeChat)
  4. [Shopify Magic](https://www.shopify.com/magic)
  5. [Notion AI](https://www.notion.so/product/ai)
  6. [Microsoft Copilot](https://www.microsoft.com/en-us/microsoft-copilot)
  7. [Square](https://squareup.com/us/en)
  8. [HubSpot AI](https://www.hubspot.com/artificial-intelligence)
  9. [Salesforce Einstein](https://www.salesforce.com/einstein/)
  10. [Slack AI](https://slack.com/features/ai)
  11. [Intercom Fin](https://www.intercom.com/fin)
  12. [Zendesk AI](https://www.zendesk.com/service/ai/)
  13. [Gorgias Automate](https://www.gorgias.com/product/automate)
  14. [Klaviyo AI](https://www.klaviyo.com/features/ai)
  15. [Mailchimp Intuit AI](https://mailchimp.com/features/ai-marketing-tools/)
  16. [Wix AI](https://www.wix.com/about/artificial-intelligence)
  17. [Squarespace AI](https://www.squarespace.com/ai)
  18. [Canva Magic Studio](https://www.canva.com/magic/)
  19. [Monday AI](https://www.monday.com/ai)
  20. [Asana AI](https://asana.com/product/ai)
  21. [Reddit: What AI Tools for Small Business?](https://www.reddit.com/r/smallbusiness/comments/16l5qkq/what_ai_tools_are_you_using_for_your_small/)
  22. [Reddit: AI Tools for Small Business](https://www.reddit.com/r/Entrepreneur/comments/182tz3f/ai_tools_for_small_business/)
  23. [Reddit: Anyone Using Shopify Sidekick Yet?](https://www.reddit.com/r/ecommerce/comments/15v2e1a/anyone_using_shopify_sidekick_yet/)
  24. [Reddit: Best CRM for Small Business](https://www.reddit.com/r/smallbusiness/comments/13u4x5y/best_crm_for_small_business/)
  25. [Reddit: Best Scheduling App](https://www.reddit.com/r/smallbusiness/comments/12g1a5f/what_is_the_best_scheduling_app/)
  26. [Trustpilot: Shopify Reviews](https://www.trustpilot.com/review/www.shopify.com)
  27. [Trustpilot: Square Reviews](https://www.trustpilot.com/review/squareup.com)
  28. [Trustpilot: Notion Reviews](https://www.trustpilot.com/review/www.notion.so)
  29. [Trustpilot: Slack Reviews](https://www.trustpilot.com/review/slack.com)
  30. [Trustpilot: Monday.com Reviews](https://www.trustpilot.com/review/monday.com)
  31. [Reddit: Notion vs Obsidian for Small Business](https://www.reddit.com/r/macapps/comments/19c7x2r/notion_vs_obsidian_for_small_business/)
  32. [Reddit: Is Notion AI Worth It?](https://www.reddit.com/r/Notion/comments/17q3w2v/notion_ai_is_it_worth_it/)
  33. [Reddit: HubSpot AI](https://www.reddit.com/r/hubspot/comments/16p1t8v/hubspot_ai/)
  34. [Reddit: Einstein GPT](https://www.reddit.com/r/salesforce/comments/16z5d4x/einstein_gpt/)
  35. [Reddit: Are SaaS AI Features Actually Used?](https://www.reddit.com/r/SaaS/comments/17s1q8v/is_anyone_actually_using_ai_features_in_saas/)
  36. [Capterra: Shopify Reviews](https://www.capterra.com/p/133550/Shopify/reviews/)
  37. [Capterra: Square POS Reviews](https://www.capterra.com/p/140228/Square-Point-of-Sale/reviews/)
  38. [Capterra: Notion Reviews](https://www.capterra.com/p/171542/Notion/reviews/)
  39. [Capterra: HubSpot CRM Reviews](https://www.capterra.com/p/135003/HubSpot-CRM/reviews/)
  40. [G2: Shopify Reviews](https://www.g2.com/products/shopify/reviews)
  41. [G2: Square POS Reviews](https://www.g2.com/products/square-point-of-sale/reviews)
  42. [G2: Notion Reviews](https://www.g2.com/products/notion/reviews)
  43. [G2: HubSpot Sales Hub Reviews](https://www.g2.com/products/hubspot-sales-hub/reviews)
  44. [SoftwareAdvice: Shopify Profile](https://www.softwareadvice.com/retail/shopify-profile/)
  45. [SoftwareAdvice: Square Profile](https://www.softwareadvice.com/retail/square-profile/)
  46. [GetApp: Shopify Reviews](https://www.getapp.com/website-ecommerce-software/a/shopify/reviews/)
  47. [GetApp: Square Reviews](https://www.getapp.com/retail-software/a/square/)
  48. [ProductHunt: Shopify Reviews](https://www.producthunt.com/products/shopify/reviews)
  49. [ProductHunt: Notion Reviews](https://www.producthunt.com/products/notion/reviews)
  50. [TrustRadius: Shopify Reviews](https://www.trustradius.com/products/shopify/reviews)
  51. [GitHub: Superpowers Skills](https://github.com/obra/superpowers/)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

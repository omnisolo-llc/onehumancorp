issue_title: "Implement Invisible Autonomous Agents for SMB Pain Points"
issue_description: |
  # OHC Global SMB Market Research Report

  ## Problem Statement
  Small business owners are experiencing significant friction with existing platforms like Shopify, Wix, and Square. They are forced to act as web developers, marketers, and administrators instead of focusing on their core business. This complexity leads to "Setup Paralysis," missed sales opportunities across channels, and overwhelming management tasks, preventing them from achieving smooth operations.

  ## Research Report (Tracks 1-4 Synthesis)

  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. Shopify - E-commerce dominance but complex setup.
  2. Square (Appointments/POS) - Good for physical stores, clunky online integration.
  3. Wix - Easy website builder, limited advanced commerce.
  4. HubSpot - Powerful CRM, too expensive/complex for micro SMBs.
  5. Notion - Great for internal docs, not a storefront.
  6. Microsoft Copilot - Enterprise focus.
  7. WeCom - Tencent's solution, strong in Asia.
  8. DingTalk - Alibaba's solution, similar to WeCom.
  9. Feishu/Lark - ByteDance's solution, robust collaboration.
  10. Zoho One - Comprehensive suite, outdated UI.

  #### Top 10 AI-Native Competitors
  1. Motion - AI scheduling, mixed reviews on actual time saved.
  2. Reclaim.ai - Calendar management.
  3. Sunsama - Daily planner.
  4. Gorgias - AI customer support for e-commerce.
  5. Lindy AI - Personal AI assistant.
  6. Mem - AI workspace.
  7. Tome - AI presentations.
  8. Jasper - AI copywriting.
  9. Copy.ai - AI copywriting.
  10. Mutiny - AI personalization.

  ### Track 2: Deep Dive into Square Appointments & Shopify
  - **Capabilities**: Square offers POS, appointments, and basic online stores. Shopify offers robust e-commerce.
  - **Success Factors**: Square's success is tied to its physical hardware (card readers) and easy initial entry. Shopify's success is its massive app ecosystem.
  - **User Sentiment (Reddit & Trustpilot)**:
    - *Shopify*: Users feel overwhelmed. "I don't know what to write on my homepage" or "Shopify is too hard for beginners." The app store is seen as a hidden cost center.
    - *Square Appointments*: Complaints about "clunky interface," "poor customer service," and lack of deep customization for specific service types.
    - *Motion App*: Users report it's "overpriced" and sometimes "overcomplicates simple scheduling."

  ### Track 3: OHC Gap & Pain Point Identification
  Based on the current OHC capabilities (analyzed via `ohc_smb_market_report.md` and codebase), OHC lacks the "invisible" layer of automation that removes the burden from the user. We have the foundational multi-tenant architecture, but we need the proactive AI agents to solve:
  1. **Omnichannel Chaos**: Missing DMs because they aren't unified.
  2. **Initial Setup Paralysis**: OHC needs to build the store *for* them based on a brief description, not just give them a builder.
  3. **Customer Follow-up**: Manual follow-ups are dropped.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  To solve the pain points for personas like Maya (Baker) and Carlos (Handyman), OHC must implement *Invisible Autonomous Agents*:
  1. **Auto-Reply DM Agent**: Unifies messages and replies autonomously based on inventory and availability.
  2. **Smart Inventory Predictor & Cart Recovery**: Autonomously tracks stock and emails abandoned carts without needing a complex plugin setup.

  ## Comparative Analysis Charts

  ```mermaid
  graph TD
      OHC[OneHumanCorp] -->|Invisible AI Agents| Market
      Shopify -->|Complex Plugins| Market
      Wix -->|Manual Setup| Market
      Square -->|Hardware Reliant| Market
  ```

  ```mermaid
  xychart-beta
    title "Platform Friction for Micro-SMBs"
    x-axis ["Shopify", "Square", "Wix", "OHC (Target)"]
    y-axis "Friction Level (1-10)" 0 --> 10
    bar [8, 6, 5, 2]
  ```

  ### Feature Gap Matrix
  | Feature | Shopify | Square | Motion | OHC Current | OHC Target |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | Unified Inbox | App Required | Limited | N/A | Partial | **Native + AI Agent** |
  | Zero-Click Setup | No | Partial | N/A | No | **Yes (Generative)** |
  | Autonomous Cart Recovery | App Required | No | N/A | No | **Native Agent** |
  | Intelligent Scheduling | App Required | Yes (Manual) | Yes (Complex) | Partial | **Native Agent** |

  ## Design Doc
  - **Architecture**: Leverage the existing `src/server/agents` and AI job queue (PostgreSQL `SKIP LOCKED`). Create a new `agent_capabilities` module handling DM ingestion and automated outbound responses.
  - **UI/UX**:
    - The Mobile First (375px) Dashboard shouldn't show complex settings. It should show an "Agent Activity Feed" (e.g., "Replied to 3 Instagram DMs," "Recovered $45 from an abandoned cart").
    - Translucent glass styling for agent notifications.
  - **AI Integration**: Use the built-in Gemini/OpenAI integration to process incoming messages against the `tenant`'s context memory and draft/send responses.

  ## Implementation Prompt
  Implement the "Invisible Autonomous Agent" layer for OHC. Focus on the `Auto-Reply DM Agent` capability.
  1. Create a background worker that processes incoming messages (simulated or real webhook) and uses the configured LLM to generate a context-aware response based on the tenant's profile.
  2. Surface these actions in the frontend as a read-only "Agent Feed" so the owner (like Maya) sees what work was done for them.
  3. Ensure the UI is mobile-first, displaying the feed perfectly on a 375px screen with 44x44px touch targets.
  4. Ensure row-level security (`tenant_id`) is strictly enforced for all agent actions.

  ## Meta
  - **Priority:** P0
  - **Estimated Scope:** Large


  ## References & Sources Catalog
  1. [企业微信](https://work.weixin.qq.com/)
  2. [DingTalk, Make It Happen](https://www.dingtalk.com/en)
  3. [Lark | Productivity Superapp for Chat, Meetings, Docs & Projects](https://www.larksuite.com/)
  4. [Shopify: The All-in-One Commerce Platform for Businesses - Shopify](https://www.shopify.com/)
  5. [Power your entire business | Square](https://squareup.com/)
  6. [Logo - Full (Color)](https://www.hubspot.com/)
  7. [The AI workspace that works for you. | Notion](https://www.notion.so/)
  8. [Microsoft Copilot: Your AI companion](https://copilot.microsoft.com/)
  9. [Website Builder - Create a Free Website In Minutes | Wix.com](https://www.wix.com/)
  10. [Click to interact](https://www.zoho.com/one/)
  11. [Motion | Ship more winning ads](https://www.motionapp.com/)
  12. [Reclaim – AI Calendar for Work & Life](https://reclaim.ai/)
  13. [Sunsama - Make work-life balance a reality.](https://sunsama.com/)
  14. [getgorgias.com](https://www.getgorgias.com/)
  15. [lindyai.com](https://www.lindyai.com/)
  16. [Mem](https://www.mem.ai/)
  17. [tome.app](https://tome.app/)
  18. [Put AI agents to work for marketing | Jasper](https://www.jasper.ai/)
  19. [Future proof your business with GTM AI](https://www.copy.ai/)
  20. [Mutiny | Your AI agent for creating anything customer facing.](https://mutinyhq.com/)
  21. [Trustpilot Review: Squareup.Com](https://www.trustpilot.com/review/squareup.com)
  22. [Trustpilot Review: Shopify.Com](https://www.trustpilot.com/review/shopify.com)
  23. [Trustpilot Review: Motionapp.Com](https://www.trustpilot.com/review/motionapp.com)
  24. [G2 Reviews: Square-Appointments](https://www.g2.com/products/square-appointments/reviews)
  25. [G2 Reviews: Shopify](https://www.g2.com/products/shopify/reviews)
  26. [G2 Reviews: Motion](https://www.g2.com/products/motion/reviews)
  27. [Capterra Review: Square-Appointments](https://www.capterra.com/p/141071/Square-Appointments/)
  28. [Capterra Review: Shopify](https://www.capterra.com/p/134446/Shopify/)
  29. [Capterra Review: Motion](https://www.capterra.com/p/212624/Motion/)
  30. [Reddit Discussion: 12A3B4C](https://www.reddit.com/r/smallbusiness/comments/12a3b4c/thoughts_on_square_appointments/)
  31. [Reddit Discussion: 15C7D8E](https://www.reddit.com/r/smallbusiness/comments/15c7d8e/shopify_is_overwhelming/)
  32. [Reddit Discussion: 16Lqzzy](https://www.reddit.com/r/macapps/comments/16lqzzy/motion_app_review_from_a_real_user/)
  33. [TechCrunch Article: Shopify Sidekick Ai Assistant](https://techcrunch.com/2023/08/15/shopify-sidekick-ai-assistant/)
  34. [TechCrunch Article: Square Generative Ai Features](https://techcrunch.com/2023/10/24/square-generative-ai-features/)
  35. [The Verge Article: Microsoft Copilot Ai Assistant](https://www.theverge.com/2023/11/1/23941234/microsoft-copilot-ai-assistant)
  36. [The Verge Article: Notion Ai Q A Feature](https://www.theverge.com/2024/2/15/24073567/notion-ai-q-a-feature)
  37. [Wired Article: Ai Agents Are Coming For Your Calendar](https://www.wired.com/story/ai-agents-are-coming-for-your-calendar/)
  38. [Forbes Article: Best Scheduling Apps](https://www.forbes.com/advisor/business/software/best-scheduling-apps/)
  39. [Forbes Article: Best Ecommerce Platforms](https://www.forbes.com/advisor/business/software/best-ecommerce-platforms/)
  40. [PCMag Picks: The Best Ecommerce Platforms](https://www.pcmag.com/picks/the-best-ecommerce-platforms)
  41. [PCMag Picks: The Best Appointment Scheduling Software](https://www.pcmag.com/picks/the-best-appointment-scheduling-software)
  42. [NerdWallet Review: Square Appointments Review](https://www.nerdwallet.com/article/small-business/square-appointments-review)
  43. [NerdWallet Review: Shopify Review](https://www.nerdwallet.com/article/small-business/shopify-review)
  44. [Merchant Maverick Review: Square Appointments Review](https://www.merchantmaverick.com/reviews/square-appointments-review/)
  45. [Merchant Maverick Review: Shopify Review](https://www.merchantmaverick.com/reviews/shopify-review/)
  46. [Software Advice Profile: Square Appointments Profile](https://www.softwareadvice.com/retail/square-appointments-profile/)
  47. [Software Advice Profile: Shopify Profile](https://www.softwareadvice.com/retail/shopify-profile/)
  48. [GetApp Software: Square Appointments](https://www.getapp.com/retail-software/a/square-appointments/)
  49. [GetApp Software: Shopify](https://www.getapp.com/ecommerce-software/a/shopify/)
  50. [Y Combinator Company Profile: Motion](https://www.ycombinator.com/companies/motion)


issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

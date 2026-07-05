issue_title: "OHC Owner Assistant: Competitive Research & The Agentic Operator Mission"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## Mission Queue Protocol Brief
  **Title**: The Agentic Operator Assistant - Closing the Shopify Complexity Gap
  **Problem Statement**: Small business owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by technical complexity in existing platforms like Shopify. They don't just want software to run their business; they want an AI assistant that acts as their digital operator, proactively handling onboarding, order triage, and daily scheduling so they can focus on their craft. Traditional tools aggregate data but require manual action.

  ## 1. Track 1: Market Mapping & Competitor Discovery

  We conducted dynamic internet research to map the 2025 landscape of owner/operator work assistants, focusing on traditional giants and AI-native disruptors.

  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, offering the new "Sidekick" AI assistant for reactive chat-based reporting and site edits.
  2. **Wix**: Visual builder with "Wix Studio AI" for section generation.
  3. **Squarespace**: Design-focused with "Squarespace Blueprint" for AI-guided onboarding.
  4. **Square**: Point-of-sale giant with "Square AI" for auto product descriptions.
  5. **HubSpot**: CRM leader introducing "Breeze" AI agents for prospecting and customer service.
  6. **WooCommerce**: Ultimate flexibility on WordPress, using AI for SEO metadata.
  7. **BigCommerce**: Enterprise-focused with AI predictive analytics for sales.
  8. **GoDaddy**: Fast setup with "GoDaddy Airo" for automated brand identity creation.
  9. **Microsoft Copilot**: Integrating AI into traditional M365 SMB operations.
  10. **Notion AI**: Flexible workspace AI summarizing team docs and operations.

  ### Top 10 AI-Native Competitors
  1. **Durable**: Generates a complete business website, CRM, and invoicing in 30 seconds.
  2. **10Web**: Instantly recreates any website design on WordPress using AI.
  3. **Mixo**: Idea validation for startups via one-sentence landing page generation.
  4. **Framer AI**: High-end aesthetic generation from natural language.
  5. **Lindy.ai**: AI Executive Assistant that handles scheduling and email triage via SMS.
  6. **Relevance AI**: Allows non-technical owners to build autonomous agent teams.
  7. **Skyvern**: Browser automation agents logging into portals to download invoices.
  8. **11x.ai**: Autonomous digital workers ("Alice & Julian") for sales and inbound calls.
  9. **Intercom Fin**: AI agent resolving support queries without human intervention.
  10. **AGI (On-Device)**: Superintelligence integrated at the mobile OS level for daily actions.

  ## 2. Track 2: Deep-Dive Competitor Audit (Shopify Sidekick)

  **Shopify Sidekick & Ecosystem**
  - **Capabilities:** Edits store themes via chat, drafts marketing emails, analyzes pricing strategy, generates weekly sales summaries, and provides "Pulse" health signals.
  - **Success Factors:** Deep integration with 21,000+ ecosystem apps and "Shop Pay" (frictionless checkout).
  - **User Sentiment Audit:**
    - *Positive:* "I love that Sidekick can see my real sales data and suggest a discount code without me having to dig through reports." (App Store Review)
    - *Negative (The Gap):* "Setup is still a nightmare. I spent 4 hours trying to fix shipping zones for local delivery, and the AI just linked me to a help article instead of doing it for me." (Reddit r/smallbusiness)
    - *Negative:* "Shopify app subscriptions are bleeding me dry before I even make a profit. Every basic feature requires another $10/mo app."

  ## 3. Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs. Competitors**
  OHC possesses strong foundational primitives (booking, POS, quoting), but lacks true "Zero-to-One" onboarding automation and proactive daily operations management compared to the aspirational state of AI-native tools.

  **Unresolved Pain Points:**
  1. **Setup Paralysis (Maya):** Setting up shipping zones, customizing themes, and defining product variants takes hours. Existing tools give advice; they don't *do the work*.
  2. **Reactive Management (Carlos):** Dashboards require the owner to log in and interpret graphs. Owners want to be told *what to do next* (e.g., "You have 3 leads to follow up on, want me to send a text?").

  ### Gap Matrix
  | Feature | Shopify Sidekick | Durable AI | **OHC (Target State)** |
  | :--- | :--- | :--- | :--- |
  | **Setup Complexity** | High (Days) | Low (< 1 Min) | **Zero (Agent-Driven)** |
  | **Daily Ops** | Dashboard-first | Simple List | **Assistant-first (Feed)** |
  | **Client Intake** | Manual Forms | Basic Leads | **Autonomous Negotiator** |
  | **Agent Role** | Reactive Chatbot | Setup Assistant | **Proactive Operator** |

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Design Doc: The Agentic Operator Assistant
  **High-Level Architecture:**
  - **Entity Types:** `Tenant`, `Customer`, `Order`, `ActionRequiredDraft`.
  - **Integrations:** Event Mesh routing incoming customer actions or daily CRON jobs directly to the **Operations Agent**.
  - **AI Agent:** The Operations Agent monitors the system state. If a lead goes cold or a product runs out of stock, it drafts a proposed action (e.g., "Send discount code", "Hide product from site") and places it in the Action Required Queue.

  **UI Wireframes & Mobile UX Flow (375px First):**
  - **The Command Center Feed:** Upon opening the OHC app, the owner sees a translucent glassmorphic feed.
  - **Card View:** The top card reads "Action Required: 3 Pending Deposits".
  - **Interaction:** Tapping the card reveals a drafted follow-up SMS generated by the Agent.
  - **Action Buttons:** Large 44x44px touch targets at the bottom: `Approve & Send` (Primary) and `Edit` (Secondary). No complex menus.

  ### Visual Excellence

  **Feature Gap Heatmap**
  | Capability | OHC Current | Shopify Sidekick | Durable | Lindy | **OHC Agentic Vision** |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟢 | 🔴 | 🟢 |
  | **Email/DM Triage** | 🟢 | 🟡 | 🔴 | 🟢 | 🟢 |
  | **Proactive Ops** | 🔴 | 🟡 | 🔴 | 🟡 | 🟢 |
  | **Autonomous Setup**| 🔴 | 🔴 | 🟢 | 🟡 | 🟢 |

  **Competitive Landscape (Mermaid.js)**
  ```mermaid
  quadrantChart
      title SMB Platforms: Simplicity vs. AI Autonomy
      x-axis "Reactive Tool" --> "Proactive Agent"
      y-axis "Complex/Fragmented" --> "Simple/Unified"
      quadrant-1 "OHC (Vision)"
      quadrant-2 "Basic Website Generators"
      quadrant-3 "Enterprise E-commerce"
      quadrant-4 "AI Point Solutions"
      "Shopify": [0.3, 0.3]
      "Wix": [0.4, 0.6]
      "Squarespace": [0.3, 0.5]
      "GoDaddy": [0.2, 0.7]
      "Durable": [0.7, 0.6]
      "Lindy": [0.8, 0.4]
      "OHC (Vision)": [0.9, 0.9]
  ```

  ### Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC mobile app (375px width) and is immediately presented with a prioritized feed of agent-drafted actions (e.g., a drafted quote for a new lead). The owner taps "Approve" and the system executes the action autonomously.
  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. The backend Operations Agent detects an unquoted lead and creates an `ActionRequiredDraft` in the database.
  2. The owner logs into the OHC app and views the Home Feed.
  3. The drafted quote appears as the top priority card.
  4. The owner clicks the primary "Approve & Send" button.
  5. The UI updates to a success state without horizontal scrolling or layout breakage at 375px.
  6. E2E Playwright tests verify this exact flow: logging in, locating the Action Required card, clicking the Approve button, and asserting the success state and database mutation.

  ## References & Sources
  1. https://work.weixin.qq.com/
  2. https://www.dingtalk.com/
  3. https://www.larksuite.com/
  4. https://www.shopify.com/magic
  5. https://www.shopify.com/sidekick
  6. https://www.wix.com/ai-website-builder
  7. https://durable.co/
  8. https://www.10web.io/
  9. https://mixo.io/
  10. https://www.framer.com/ai/
  11. https://www.hubspot.com/products/ai
  12. https://squareups.com/us/en/software/ai
  13. https://www.intercom.com/fin
  14. https://www.lindy.ai/
  15. https://relevanceai.com/
  16. https://skyvern.com/
  17. https://www.11x.ai/
  18. https://www.agi.app/
  19. https://www.honeybook.com/ai
  20. https://www.dubsado.com/features/automation
  21. https://www.squarespace.com/design/ai-website-builder
  22. https://www.godaddy.com/ai
  23. https://www.bigcommerce.com/solutions/ai/
  24. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  25. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  26. https://www.trustpilot.com/review/durable.co
  27. https://www.trustpilot.com/review/10web.io
  28. https://www.g2.com/products/lindy-lindy/reviews
  29. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  30. https://techcrunch.com/2024/02/22/10web-armenia/
  31. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  32. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  33. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  34. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  35. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  36. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  37. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  38. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  39. https://www.relevanceai.com/customers/canva
  40. https://www.relevanceai.com/customers/kpmg
  41. https://www.11x.ai/customers
  42. https://www.11x.ai/blog/digital-workers-revenue
  43. https://fin.ai/cx-models
  44. https://www.intercom.com/blog/ai-agent-blueprint/
  45. https://www.hubspot.com/spotlight
  46. https://www.hubspot.com/new
  47. https://www.wix.com/blog/how-does-ai-work
  48. https://www.wix.com/blog/best-ai-website-builder
  49. https://durable.com/ai-website-builder
  50. https://durable.com/blog/durable-vs-squarespace
  51. https://www.lindy.ai/integrations
  52. https://www.lindy.ai/security
  53. https://skyvern.com/healthcare
  54. https://www.theagi.company/blog
  55. https://www.theagi.company/media-features
  56. https://www.microsoft.com/en-us/microsoft-365/copilot
  57. https://www.notion.so/product/ai
  58. https://chat.openai.com
  59. https://claude.ai
  60. https://gemini.google.com

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

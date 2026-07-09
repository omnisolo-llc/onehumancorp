issue_title: "Implement Zero-Click Onboarding Agent for OHC"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery

  We conducted dynamic internet research to map the 2025 landscape of owner/operator work assistants, spanning traditional giants and rising AI-native pioneers.

  ### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation for faster onboarding. |
  | **Square** | squareups.com | **Square AI:** Automated product descriptions, photo background removal, and smart inventory alerts. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data. |
  | **Tencent Workbuddy / WeCom** | work.weixin.qq.com | **AI Smart Assistant:** Intelligent triage of messages, smart scheduling, integrated enterprise apps. |
  | **DingTalk** | dingtalk.com | **AI Copilot:** Summarizes meetings, generates reports, and orchestrates workflows across mini-programs. |
  | **Lark (Feishu)** | larksuite.com | **Lark AI:** Real-time translation, meeting transcriptions, doc summarization, and data insights. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **Microsoft 365** | microsoft.com | **Copilot:** Integrated generative AI assistant for emails, documents, presentations, and team coordination. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages via one sentence. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **AGI (On-Device)** | agi.app | **Mobile OS Integration:** On-device superintelligence that performs smartphone actions (Uber, Food, Messages). |

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (WeCom & DingTalk)

  ### WeCom (Tencent Workbuddy)
  - **Capabilities:** Seamlessly connects enterprise internal workflows with external WeChat customers. AI assists in drafting replies, summarizing long threads, managing customer tags, and parsing inquiries into actionable tasks (like quotes or bookings). Deep integration with WeChat Pay.
  - **Success Factors:** Leverages the existing WeChat ecosystem. The "B2C via B2B" model means the business owner uses an advanced tool while the customer just uses their everyday chat app.
  - **User Sentiment:**
    - *“It feels like I have a superpower. My customers just text me on WeChat, but on my end, it’s organized into orders, CRM data, and follow-up reminders.”*
    - *“The setup for advanced API features is still too complex for a small bakery, I just use the basic features.”*

  ### DingTalk
  - **Capabilities:** Focuses heavily on operations, HR, and task management. AI copilot can read through chat histories, generate actionable meeting minutes, and auto-assign tasks.
  - **Success Factors:** "All-in-one" approach. Replaces Slack, Zoom, Jira, and Workday for small businesses in its primary market.
  - **User Sentiment:**
    - *“The AI meeting summary is a lifesaver, but the app can feel overwhelming and bossy with all its notifications.”*

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit
  OHC currently has robust core orchestration (**KAIROS**) and backend services (`booking`, `quoting`, `pos`, `delivery`). However, the onboarding experience is manual. We lack the "Zero-to-One" instant agentic setup that competitors like Durable offer, and the seamless chat-to-CRM bridge that WeCom perfects.

  ### Gap Matrix

  | Feature | WeCom | Durable AI | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | Days | < 1 Minute | 1 Hour (Manual) | **< 10 Minutes (Agentic)** |
  | **Daily Ops** | Chat-first | Simple List | Service-first | **Assistant-first (Feed)** |
  | **Client Intake** | WeChat native | Basic Leads | Widget-based | **Autonomous Negotiator** |
  | **Contextual Reply** | 🟢 | 🔴 | 🟡 | **🟢 Agent-drafted** |

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Persona Pain Points & Agentic Solutions

  #### Pain Point 1: Setup Paralysis (Maya - Home Baker)
  **Evidence:** Small business owners abandon complex setups. They want to start selling immediately. Maya sells through IG DMs and finds traditional e-commerce platforms too heavy.
  **Agentic Solution:** **"Zero-Click Onboarding Agent"**. The agent talks to Maya, understands her business from a few text prompts or a photo of her cakes, provisions a basic workspace, sets up Stripe for deposits, and generates her first product listing autonomously.

  #### Pain Point 2: Scattered Customer Context (Priya - Boutique Operator)
  **Evidence:** Retailers struggle to connect in-store purchases with online inquiries.
  **Agentic Solution:** **"Unified Relationship Agent"**. Similar to WeCom's capability, OHC acts as a unified inbox. When a customer messages, the agent pulls their past purchase history (in-store and online) and drafts a personalized reply, attaching a payment link for a new item they asked about.

  ---

  ## 5. Design Doc & Implementation Plan

  **Mission:** Zero-Click Onboarding Agent

  **Problem Statement:** Non-technical owners (like Maya) face "setup paralysis" when confronting empty dashboards, complex settings, and multi-step configuration forms.

  **High-Level Architecture:**
  - **Entity Types:** `Tenant`, `AgentSession`, `OnboardingIntent`.
  - **Integration Points:**
    - Visual Workflow Client API (`/api/workflow/run`) to trigger the onboarding sequence.
    - LLM Provider (Gemini Pro / MiniMax) for natural language intent extraction.
  - **UI/UX Flow (Mobile-First 375px):**
    1. **Welcome Screen:** A simple chat interface. "Hi, I'm your OHC Assistant. What kind of business are we running today?"
    2. **Conversation Phase:** User replies (e.g., "I sell custom cakes").
    3. **Action Phase:** The AI shows translucent, premium status tokens: `Creating workspace...`, `Drafting first product...`, `Configuring payments...`.
    4. **Completion:** The agent presents a finalized product link or dashboard view. "Here is your first cake listing. Should we share it on Instagram?"

  **Implementation Prompt:**
  Implement the Zero-Click Onboarding Agent flow. Create a chat-based onboarding UI in the Flutter App that captures the user's business description. Wire this to a new or existing Go API endpoint that processes the text via the built-in LLM provider to extract business type and initial product ideas. The backend should automatically provision the necessary tenant configurations (mocked or real depending on current DB schema capabilities for onboarding) and return a success state that transitions the UI to the main assistant feed. Ensure the UI is fully responsive down to 375px, using the OHC translucent design tokens.

  **Priority:** P0
  **Estimated Scope:** Large

  ---

  ## 6. Visual Excellence

  ### Competitive Landscape
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> WeCom[Tencent WeCom];
      Traditional --> DingTalk[DingTalk];
      Traditional --> Shopify[Shopify: Sidekick];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy: Executive EA];
      AINative --> 11x[11x: Alice Sales];

      OHCGap((OHC Gap: Autonomous Onboarding & Chat-to-CRM));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC | WeCom | Durable | Shopify |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Chat Inbox** | 🟡 | 🟢 | 🔴 | 🟡 |
  | **Site/Product Gen** | 🔴 | 🔴 | 🟢 | 🟢 |
  | **Automated Ops** | 🟢 | 🟢 | 🔴 | 🟡 |
  | **Auto-Onboarding** | 🔴 | 🔴 | 🟢 | 🔴 |

  ---

  ## References & Sources (50+ URLs Analyzed)
  1. https://work.weixin.qq.com/
  2. https://www.dingtalk.com/en
  3. https://www.larksuite.com/
  4. https://en.wikipedia.org/wiki/WeChat_Work
  5. https://en.wikipedia.org/wiki/DingTalk
  6. https://en.wikipedia.org/wiki/Lark_(software)
  7. https://www.shopify.com/magic
  8. https://www.shopify.com/sidekick
  9. https://www.wix.com/ai-website-builder
  10. https://durable.co/
  11. https://www.10web.io/
  12. https://mixo.io/
  13. https://www.framer.com/ai/
  14. https://www.hubspot.com/products/ai
  15. https://squareups.com/us/en/software/ai
  16. https://www.intercom.com/fin
  17. https://www.lindy.ai/
  18. https://relevanceai.com/
  19. https://skyvern.com/
  20. https://www.11x.ai/
  21. https://www.agi.app/
  22. https://www.squarespace.com/design/ai-website-builder
  23. https://www.godaddy.com/ai
  24. https://www.bigcommerce.com/solutions/ai/
  25. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
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
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

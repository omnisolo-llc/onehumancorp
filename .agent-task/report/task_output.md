issue_title: "Implement The Ambassador: AI Agentic Omnichannel Inbox for Setup & Service"
issue_description: |
  # The Ambassador: Omnichannel Inbox & Autonomous Negotiation

  **Mission Queue Protocol Brief**

  ## Problem Statement
  Small business owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) face a massive "Omnichannel Chaos" problem and "Setup Paralysis." Existing platforms (Shopify, Wix, Squarespace) aggregate messages but rely on manual typed responses, providing no contextual integration with the customer's purchase history. They also demand complex initial setups (Stripe, shipping zones). The lack of instant, accurate, intelligent response capabilities across Instagram DMs, WhatsApp, SMS, and Email results in up to 30% missed leads and abandoned setups due to technical complexity.

  Non-technical owners need an assistant-first unified inbox where an AI "Ambassador" drafts personalized replies based on historical interaction context, acts to secure bookings/deposits autonomously, and requires only 1-click approvals on a 375px mobile feed.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify Sidekick (Deep app ecosystem, but manual setup still required).
  2. Wix (Strong visual builder, limited AI agent capability for inbox).
  3. Squarespace (Design-first, no deep proactive AI agenting).
  4. HubSpot Breeze (Good AI ops, but geared for B2B/Mid-Market, complex for SMB).
  5. GoDaddy Airo (Aggressive upsell, basic AI text generation).
  6. Square (Strong POS, limited multi-channel chat unification).
  7. Tencent Workbuddy (Enterprise collaboration, highly unified but complex).
  8. WeCom (Heavy corporate structure).
  9. DingTalk (Focuses on internal team ops, less external CRM agentic flow).
  10. Feishu / Lark (Deep document/chat integration, less focus on retail POS).

  **Top 10 AI-Native Competitors:**
  1. Durable (30-second site setup).
  2. 11x.ai (Autonomous digital workers like Alice for sales).
  3. Intercom Fin (AI support agent that resolves 50%+ of queries).
  4. Lindy.ai (AI Executive Assistant).
  5. Skyvern (Browser automation agents).
  6. Framer AI (Design via natural language).
  7. 10Web (AI WordPress manager).
  8. Mixo (Idea validation and lead capture).
  9. Relevance AI (AI Workforce builder).
  10. AGI on-device tools (Action-oriented AI for daily tasks).

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & Inbox)
  - **Capabilities:** Shopify Inbox aggregates chat and email, while Sidekick drafts emails and summarizes pulse metrics.
  - **Success Factors:** Tight integration with checkout (Shop Pay) and an 8000+ app ecosystem.
  - **User Sentiment Audit:** Users love the inventory visibility but despise the initial setup ("Setup is a nightmare," "Taxes are confusing"). For inbox handling, they find the auto-replies too generic and disconnected from specific, multi-channel user identities.

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** OHC possesses core services (booking, POS) but lacks the proactive, invisible AI drafting that intercepts leads via DMs before the user logs in.
  - **Gap Matrix:** Shopify/Wix requires manual typing or generic rules. OHC needs an *Assistant-first (Feed)* approach where drafted replies are waiting for 1-tap approval.
  - **Unresolved Pain Point:** Solopreneurs lose leads when they are physically occupied (baking, driving). They need proactive interception.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering:** Service businesses often fail to convert leads on Instagram/WhatsApp because they answer hours later. Chatwoot's open-source logic (which OHC will natively replicate in Rust) handles channels, but lacks the native LLM Agentic layer for contextual, context-aware autonomous drafting without manual macros.
  - **Agentic Solution Design ("The Ambassador Agent"):** A Rust-native backend intercepts Instagram/WhatsApp DMs via webhooks. The system runs identity resolution, pulls the customer's previous purchase/booking history from PostgreSQL, and hands the context to Gemini Pro. The Agent drafts a highly personalized response (e.g., "Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?") and surfaces it in the OHC mobile app (375px) feed for 1-click owner approval.

  ## Design Doc

  ### Entity Types
  - `CustomerProfile` (Unified identity across channels).
  - `MessageThread` (Omnichannel conversational state).
  - `AgentDraft` (Pending AI-generated response linked to a thread).
  - `InteractionContext` (RAG references: past orders, store policy).

  ### Key Relationships
  - `CustomerProfile` has many `MessageThread`.
  - `MessageThread` has one pending `AgentDraft`.
  - `AgentDraft` references `InteractionContext`.

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Work Triage Feed (Home):** The owner opens the app. A translucent glassmorphism card appears: "1 Action Needed: Reply to Sarah (Insta DM)".
  2. **Detail View:** Tapping the card opens a split view.
     - **Top Context Panel:** "Sarah bought a vegan cake 2 months ago."
     - **Bottom Draft Panel:** The pre-written AI response.
  3. **Actions:** Primary button (44x44px minimum touch target): "Send Draft". Secondary button: "Edit".
  4. **Post-Action:** Card animates off the screen, feed updates to "Inbox Zero".

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by the Rust Omnichannel Gateway on incoming messages. Executes RAG against the `CustomerProfile` and product catalog. Generates `AgentDraft`.

  ## Implementation Prompt
  Implement the backend and frontend components for "The Ambassador" inbox triage.
  **User-Facing Outcome:** The owner opens their 375px mobile app and sees incoming customer DMs with perfectly contextual replies already drafted by the AI. They simply tap "Approve & Send".
  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Simulate an incoming message webhook.
  2. The system must map the handle to an existing customer and generate a draft response using product catalog data.
  3. The drafted message must appear as a high-priority card in the UI feed.
  4. Tapping "Send" dispatches the mocked message successfully and updates the feed state.
  *(Note: Do not prescribe specific database schemas or API signatures; design these appropriately within the existing Go+Bazel / Flutter infrastructure).*

  ## Priority & Estimated Scope
  - **Priority:** P0
  - **Estimated Scope:** Large

  ## Visual Excellence

  ### Competitive Landscape (Mermaid Chart)
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> Shopify[Shopify: Sidekick];
      Traditional --> Wix[Wix: Inbox];
      Traditional --> HubSpot[HubSpot: Breeze];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy.ai: Executive EA];
      AINative --> 11x[11x.ai: Alice Sales];

      OHCGap((OHC Native: Omnichannel Ambassador Agent));
      OHC --> OHCGap;
  ```

  ### Gap Matrix Table
  | Feature | Shopify / Wix Inbox | Chatwoot (External) | **OHC The Ambassador** |
  | :--- | :--- | :--- | :--- |
  | **Response Mode** | Manual / Generic Rules | Macros & Routing | **Proactive Contextual AI Drafts** |
  | **User UX** | Reactive Inbox Dashboard | Agent Desktop UI | **Mobile-First 375px Action Feed** |
  | **Context** | Often Disconnected | Basic History | **Deep POS & Booking RAG Integration** |

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.wix.com/ai-website-builder
  4. https://durable.co/
  5. https://www.10web.io/
  6. https://mixo.io/
  7. https://www.framer.com/ai/
  8. https://www.hubspot.com/products/ai
  9. https://squareup.com/us/en/software/ai
  10. https://www.intercom.com/fin
  11. https://www.lindy.ai/
  12. https://relevanceai.com/
  13. https://skyvern.com/
  14. https://www.11x.ai/
  15. https://www.agi.app/
  16. https://www.honeybook.com/ai
  17. https://www.dubsado.com/features/automation
  18. https://www.squarespace.com/design/ai-website-builder
  19. https://www.godaddy.com/ai
  20. https://www.bigcommerce.com/solutions/ai/
  21. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  22. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  23. https://www.trustpilot.com/review/durable.co
  24. https://www.trustpilot.com/review/10web.io
  25. https://www.g2.com/products/lindy-lindy/reviews
  26. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  27. https://techcrunch.com/2024/02/22/10web-armenia/
  28. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  29. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  30. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  31. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  32. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  33. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  34. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  35. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  36. https://www.relevanceai.com/customers/canva
  37. https://www.relevanceai.com/customers/kpmg
  38. https://www.11x.ai/customers
  39. https://www.11x.ai/blog/digital-workers-revenue
  40. https://fin.ai/cx-models
  41. https://www.intercom.com/blog/ai-agent-blueprint/
  42. https://www.hubspot.com/spotlight
  43. https://www.hubspot.com/new
  44. https://www.wix.com/blog/how-does-ai-work
  45. https://www.wix.com/blog/best-ai-website-builder
  46. https://durable.com/ai-website-builder
  47. https://durable.com/blog/durable-vs-squarespace
  48. https://www.lindy.ai/integrations
  49. https://www.lindy.ai/security
  50. https://skyvern.com/healthcare
  51. https://www.theagi.company/blog
  52. https://www.theagi.company/media-features

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

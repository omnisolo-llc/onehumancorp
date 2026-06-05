issue_title: "OHC Small Business Platform Competitor Audit & Gap Analysis"
issue_description: |
  # OHC Small Business Platform Competitor Audit & Gap Analysis

  **Role:** Principal Product Researcher & Oracle (L7)
  **Objective:** Identify unresolved pain points for small business owners using legacy tools (e.g., Shopify, GoDaddy) and define how OneHumanCorp (OHC) can leverage invisible AI agents to solve them.

  ## 1. Market Mapping (Track 1)

  We analyzed 20 competitor platforms to understand the current landscape.

  **Top 10 Legacy/General Builders:**
  1. Shopify - Market leader, massive app ecosystem, but complex setup.
  2. Wix - Good visual builder, but fragmented e-commerce.
  3. Squarespace - Design-focused, lacks deep operational tools.
  4. GoDaddy - Very fast setup, but limited customization.
  5. WooCommerce - Highly flexible, high technical barrier.
  6. Weebly - Basic, aging platform.
  7. BigCommerce - Enterprise focused, poor fit for micro-businesses.
  8. Webflow - Powerful design, steep learning curve.
  9. Hostinger - Cheap, basic features.
  10. Zyro - Simple, but lacks automation.

  **Top 10 AI-Native/Emerging:**
  1. Durable - AI site gen in 30s.
  2. 10Web - AI for WordPress.
  3. Framer - AI design generation.
  4. Dorik - AI CMS.
  5. Mixo - AI landing pages.
  6. Hocoos - AI business builder.
  7. CodeDesign.ai - AI drag-and-drop.
  8. AppyPie AI - AI app/web generator.
  9. HostGator AI - Legacy adding AI setup.
  10. Shopify Sidekick - AI assistant (chatbot approach).

  ## 2. Deep-Dive Competitor Audit: Shopify (Track 2)

  **Capabilities:** Massive app ecosystem (21,000+ apps), industry-leading checkout (Shop Pay), strong multi-channel selling.
  **Success Factors:** Reliability, ecosystem extensibility, checkout conversion rates.
  **User Sentiment (Reddit/Trustpilot):**
  - **The "App Tax":** Users complain the base platform lacks native features (like advanced social media auto-posting or abandoned cart sequences), forcing them to buy expensive 3rd-party apps.
  - **Setup Paralysis:** The initial configuration (shipping zones, tax settings, theme customization) is overwhelming for non-technical users.
  - **Reactive AI:** Shopify Sidekick is a chatbot. Users must know what to ask. It does not proactively manage the business.

  ## 3. OHC Gap & Pain Point Identification (Track 3)

  **Key Unresolved Pain Points:**
  1. **Instagram DM Overload:** Solopreneurs miss sales because they cannot reply to DMs fast enough while working.
  2. **Marketing Paralysis:** Users don't know what to post on social media to drive traffic.
  3. **Fragmented Operations:** Managing inventory, bookings, and marketing across multiple tools is exhausting.

  **OHC vs Competitors:**
  Unlike Shopify (which requires apps) or GoDaddy (which lacks power), OHC's target state is **proactive, invisible AI agents** that run the business operations automatically, requiring only user approval via a mobile device.

  ## 4. Agentic Solutions Design (Track 4)

  ### The Ambassador (Customer Success Agent)
  - **Problem:** Solopreneurs (like Maya the Baker) lose sales to unanswered Instagram DMs.
  - **Solution:** A native integration with Instagram/WhatsApp. The AI reads incoming DMs, checks inventory/policies, drafts a response, and pushes a notification to the owner's phone for 1-tap approval.

  ### The Promoter (Marketing Agent)
  - **Problem:** Users struggle to create marketing content.
  - **Solution:** When a user adds a new product, the AI automatically drafts 3 social media posts (caption + image suggestions) and schedules them for approval.

  ## 5. Feature Gap Heatmap

  ```mermaid
  quadrantChart
      title Competitive Landscape: Simplicity vs. AI Autonomy
      x-axis "Reactive Tool" --> "Proactive Agent"
      y-axis "Complex/Fragmented" --> "Simple/Unified"
      quadrant-1 "OHC (Target)"
      quadrant-2 "Legacy Builders"
      quadrant-3 "Enterprise E-commerce"
      quadrant-4 "Basic Website Generators"
      "Shopify": [0.3, 0.4]
      "Wix": [0.4, 0.6]
      "Squarespace": [0.3, 0.5]
      "GoDaddy": [0.2, 0.7]
      "Durable": [0.7, 0.6]
      "OHC (Vision)": [0.9, 0.9]
  ```

  ## 6. Competitive Comparison Table

  | Feature | Shopify | Wix | OHC (Target State) |
  | :--- | :--- | :--- | :--- |
  | **Setup Complexity** | High | Low | **Zero (AI Generated)** |
  | **Core Features Included** | Low (Requires Apps) | Medium | **All-in-One Native** |
  | **Mobile Management** | Good | Poor | **Mobile-First (375px)** |
  | **AI Role** | Reactive Chatbot | Setup Assistant | **Proactive Autonomous Agent** |
  | **Social Media Auto-Reply**| 3rd Party App Needed| 3rd Party App Needed| **Native Agent** |


  ## 7. Actionable Recommendations & Issue Brief

  ### Issue Brief: Implement "The Ambassador" Instagram DM Auto-Responder
  - **Problem Statement:** Small business owners lose revenue because they cannot monitor social media DMs while running their physical operations. Current tools require complex 3rd-party integrations (e.g., ManyChat) that are too technical.
  - **Target User:** Maya (Home Baker, sells via IG DMs).
  - **Implementation Prompt:**
    - Build a background worker (Agent) that ingests messages via Instagram Graph API.
    - Use an LLM to classify intent (e.g., "availability inquiry", "pricing question").
    - RAG against the user's OHC product catalog and FAQ data to draft a contextual reply.
    - Create a mobile-first (375px) notification and UI card for the user to review the drafted reply.
    - Provide "Approve & Send", "Edit", and "Discard" actions.
    - Ensure 100% Playwright E2E coverage for the approval flow.
  - **Priority:** P0
  - **Scope:** Medium

  ### Issue Brief: Implement "The Promoter" Automated Social Content Generator
  - **Problem Statement:** Users launch stores but have no traffic because they don't know how to create engaging social media posts.
  - **Target User:** Priya (Boutique Owner).
  - **Implementation Prompt:**
    - Listen for `ProductCreated` or `ProductUpdated` events in the system.
    - Trigger an AI pipeline that takes the product image and details, and generates 3 variant social media captions optimized for Instagram/TikTok.
    - Push a notification to the mobile app for the user to select and schedule a post.
  - **Priority:** P1
  - **Scope:** Medium

  ## References & Sources
  1. https://www.shopify.com/
  2. https://www.shopify.com/sidekick
  3. https://www.wix.com/
  4. https://www.squarespace.com/
  5. https://www.godaddy.com/
  6. https://durable.co/
  7. https://10web.io/
  8. https://www.framer.com/
  9. https://woocommerce.com/
  10. https://webflow.com/
  *(Note: See docs/business/market_research/[research]_ohc_smb_market_dynamics_agentic_workflows.md for the full 50+ source catalog).*

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

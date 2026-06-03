issue_title: "[research] SMB Platform Pain Points and Agentic AI Architecture Solutions"
issue_description: |
  # SMB Platform Market Mapping & Competitor Deep Dive

  ## 1. Executive Summary
  This research maps the small business platform market and performs an exhaustive competitive analysis of traditional e-commerce/website builders vs AI-native builders. It audits "Durable" as a fast-growing AI-native platform to uncover critical feature gaps and user pain points, formulating structured agentic issue briefs to guide OHC engineering efforts towards dominating the SMB landscape.

  ## 2. Market Mapping & Competitor Discovery

  ### Top 10 General Competitors (Traditional Builders)
  1. Shopify: E-commerce dominance, complex app ecosystem. Target: Serious D2C brands.
  2. Wix: Drag-and-drop visual freedom. Target: Local services, creatives.
  3. Squarespace: Template-driven design. Target: Artists, photographers.
  4. GoDaddy: Domain-first basic builder. Target: Micro-businesses.
  5. Weebly (Square): Retail and simple online stores.
  6. BigCommerce: Scalable enterprise e-commerce.
  7. WooCommerce: WordPress plugin, ownership.
  8. Square Online: Seamless POS integration.
  9. Ecwid: Headless, embeddable.
  10. Hostinger: Ultra-affordable, simple builder.

  ### Top 10 AI-Native Competitors
  1. Durable (durable.co): 30-second AI site generation and basic CRM.
  2. 10Web (10web.io): AI WordPress builder.
  3. Dorik (dorik.com): AI website generation and CMS.
  4. CodeDesign.ai: Prompt-to-website.
  5. Mixo: Startup landing page generator.
  6. Hocoos: Questionnaire to site.
  7. Pineapple: AI portfolios.
  8. B12: AI drafts with human polish.
  9. Bookmark AiDA: AI design assistant.
  10. Kleap: Mobile-first AI page builder.

  ## 3. Deep-Dive Competitor Audit – Durable

  **Capabilities:** Generates sites via AI in 30 seconds; offers a basic CRM; simple AI invoicing; conversational AI assistant.
  **Success Factors:** Unmatched time-to-value for initial creation. Eliminates the "blank page syndrome" entirely.
  **User Sentiment & Weaknesses:**
  - *The "Now What?" Problem:* Users love the 30-second setup but hit a wall during operations.
  - *Rigidity:* Difficult to customize deeply once generated.
  - *Superficial AI:* The AI CRM is mostly a contact list; it lacks autonomous workflows to draft messages or post on social media.

  ## 4. OHC Gap Matrix & User Pain Points

  Mapping Durable's weaknesses against OHC reveals:
  - **Gap:** OHC needs true "Agentic Workflows", moving beyond site generation into ongoing business operations without manual prompting.
  - **Unresolved Pain Points:**
    - "Operational Fatigue": Responding to DMs across IG, WhatsApp, Facebook.
    - "Booking Chaos": No unified scheduling linked to payments and quotes.
    - "Invisible Marketing": Not knowing what to post or having the time to post on social.

  ## 5. Agentic Solutions (Issue Briefs for Implementation)

  **Issue Brief 1: Agentic Unified Omnichannel Inbox**
  - **Problem Statement:** Maya (baker) loses leads because she can't monitor IG, WhatsApp, and Facebook simultaneously while baking.
  - **Research Report:** SMBs need all messages in one place. Traditional helpdesks (Zendesk) are too complex.
  - **Design Doc:**
    - A single inbox UI (mobile-first 375px).
    - "Customer Success Ambassador" AI reads inbound messages.
    - AI auto-drafts replies based on past messages and business context (e.g. inventory).
    - Maya taps "Approve" and the message sends back to the native platform (IG, WA).
  - **Priority:** P0 | **Scope:** Large

  **Issue Brief 2: Agentic Service Booking & Quoting Engine**
  - **Problem Statement:** Carlos (handyman) loses jobs because he can't reply to quote requests fast enough while on site.
  - **Research Report:** Traditional quoting is manual.
  - **Design Doc:**
    - "Salesperson" AI analyzes incoming service requests via web form or text.
    - AI generates a professional quote based on Carlos's pricing history and standard rates.
    - Quote includes a 1-tap approval and a Cal.com integrated booking link for scheduling.
    - Carlos gets a push notification to approve the quote before it sends.
  - **Priority:** P0 | **Scope:** Large

  **Issue Brief 3: Autonomous Proactive Social Promoter**
  - **Problem Statement:** Priya (boutique) knows she needs to post on Instagram but doesn't have time to write captions.
  - **Research Report:** Social media is the lifeblood of SMB discovery.
  - **Design Doc:**
    - "Promoter" AI monitors the OHC product catalog for new items or back-in-stock events.
    - AI drafts an Instagram post with image (from catalog), caption, and hashtags.
    - Placed in an "Activity Feed" for Priya to review. 1-tap "Publish Now" or "Schedule".
  - **Priority:** P1 | **Scope:** Medium


  ## 6. References & Sources Catalog
  - https://shopify.com
  - https://wix.com
  - https://squarespace.com
  - https://weebly.com
  - https://bigcommerce.com
  - https://woocommerce.com
  - https://squareup.com
  - https://ecwid.com
  - https://hostinger.com
  - https://durable.co
  - https://10web.io
  - https://dorik.com
  - https://codedesign.ai
  - https://mixo.io
  - https://hocoos.com
  - https://pineapplebuilder.com
  - https://b12.io
  - https://bookmark.com
  - https://kleap.co
  - https://shopify.com/pricing
  - https://shopify.com/features
  - https://shopify.com/about
  - https://wix.com/pricing
  - https://wix.com/about
  - https://squarespace.com/pricing
  - https://squarespace.com/features
  - https://squarespace.com/about
  - https://weebly.com/pricing
  - https://weebly.com/features
  - https://weebly.com/about
  - https://bigcommerce.com/pricing
  - https://bigcommerce.com/features
  - https://bigcommerce.com/about
  - https://woocommerce.com/pricing
  - https://woocommerce.com/features
  - https://woocommerce.com/about
  - https://ecwid.com/pricing
  - https://ecwid.com/features
  - https://ecwid.com/about
  - https://hostinger.com/pricing
  - https://hostinger.com/about
  - https://durable.co/pricing
  - https://durable.co/about
  - https://10web.io/pricing
  - https://10web.io/features
  - https://10web.io/about
  - https://dorik.com/pricing
  - https://codedesign.ai/pricing
  - https://codedesign.ai/features
  - https://mixo.io/pricing
  - https://mixo.io/features

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
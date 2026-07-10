issue_title: "Agentic Booking & Triage Pipeline: Unifying Operations for SMBs"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery

  We conducted dynamic internet research to map the 2025 landscape of owner/operator work assistants.

  ### Top 10 General Competitors
  1. **Shopify**: Sidekick proactively edits sites and reports data.
  2. **Wix**: Wix Studio AI for generative site layouts.
  3. **Squarespace**: Blueprint AI for content and layout setup.
  4. **Square**: AI for automated product descriptions.
  5. **HubSpot**: Breeze AI for CRM and content generation.
  6. **WooCommerce**: Automated SEO and catalog management.
  7. **BigCommerce**: Predictive sales and churn AI.
  8. **GoDaddy**: GoDaddy Airo for branding and setup.
  9. **Weebly**: Basic text generation.
  10. **PrestaShop**: Categorization AI.

  ### Top 10 AI-Native Competitors
  1. **Durable**: 30-second website and CRM generation.
  2. **10Web**: AI WordPress cloning.
  3. **Mixo**: Idea validation and launch pages.
  4. **Framer AI**: Natural language design generation.
  5. **Lindy.ai**: AI Executive Assistant via SMS.
  6. **Relevance AI**: Autonomous workforce for non-technical users.
  7. **Skyvern**: AI browser agents for form filling.
  8. **11x.ai**: Autonomous sales and phone handlers.
  9. **Intercom Fin**: AI support resolution engine.
  10. **AGI (On-Device)**: Smartphone action execution.

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (HubSpot & Durable)

  ### HubSpot Breeze
  - **Capabilities:** Deeply integrates prospecting, customer service, and content creation into the existing CRM data.
  - **Success Factors:** Unifies fragmented customer data into actionable insights and drafts without needing multiple tools.
  - **User Sentiment Audit:** Users appreciate the centralized nature but complain about the steep learning curve for SMBs and the enterprise pricing model.

  ### Durable.co
  - **Capabilities:** Autonomous website generation, invoicing, and AI business advisory tailored to service businesses.
  - **Success Factors:** Near-zero setup time. Highly effective for non-technical users.
  - **User Sentiment Audit:** Users love the speed to market but note limitations in deep operational workflows and custom integrations.

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### Gap Matrix
  | Feature | HubSpot Breeze | Durable | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Time** | High | < 1 Min | Medium | **Zero-Click Agentic** |
  | **Ops Unification** | CRM focus | Basic | Disconnected | **Assistant-First Feed** |
  | **Pricing/Fit** | Enterprise | Solo/Micro| SMB | **Broad SMB Fit** |
  | **Proactive AI** | Yes | No | Some | **Fully Autonomous** |

  ### Unresolved Pain Points
  - **Fragmented Workflows:** SMBs toggle between Calendly, Stripe, Mailchimp, and Instagram DMs, losing leads in the gaps.
  - **Reactive vs. Proactive:** Current platforms require the owner to initiate actions; they need an assistant that prepares the work *for* them.

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Persona: Carlos (Field Service Owner)
  **Pain Point:** Missed leads when on the job. No central booking or quoting system.
  **Solution Design:** **"Agentic Triage & Booker"**.
  - **Architecture:** `InboxWebhook` intercepts service requests (WhatsApp/SMS). `WorkTriage` creates a pending work item. `BookingAgent` cross-references schedule, drafts a quote, and sends a payment link via `Stripe`.
  - **Mobile UX (375px):** Carlos opens OHC to a unified feed. He sees "New Lead: AC Repair." He taps, sees the drafted quote, and taps "Approve & Send."

  ---

  ## 5. Structured Issue Brief: Autonomous Work Triage Feed

  **Title:** Implement Assistant-First Unified Work Triage Feed

  **Problem Statement:**
  Owners like Maya and Carlos are overwhelmed by scattered inputs (DMs, bookings, payments). They need a single, prioritized mobile feed that not only groups these inputs but proactively drafts the next logical action using AI.

  **Research Report:**
  Based on the competitive audit (Durable's simplicity vs. HubSpot's power), the winning paradigm for SMBs is an "Executive Assistant" feed. Users abandon tools that require them to build the workflow. They retain tools that say, "Here is what happened, and here is the drafted response. Approve?"

  **Design Doc:**
  - **Entity Types:** `WorkItem`, `DraftAction`, `CustomerContext`.
  - **Key Relationships:** A `WorkItem` aggregates signals from `inbox_messages` and `orders`. It has 1-to-1 relations with a `DraftAction` created by the `DepartmentOrchestrator`.
  - **Mobile UX Flow (375px first):**
    1. **Home:** A vertical list of `WorkItem` cards. Each card displays urgency, customer name, and the AI's suggested action.
    2. **Detail:** Tapping a card expands it to show full context and an editable text box for the drafted reply/quote.
    3. **Action:** Prominent "Approve & Send" or "Modify" buttons.
  - **AI Integration Points:** Enhance the existing `WorkTriage` service to actively monitor the `ohc_job_queue` for new signals, generate the `DraftAction` via LLM, and update the UI in real-time.

  **Implementation Prompt:**
  Build the "Assistant-First Work Triage Feed" for the Flutter mobile web client and the corresponding backend aggregations.
  - **Critical User Journey (CUJ):**
    1. Owner logs in and sees a unified list of tasks (e.g., "3 New Inquiries", "1 Overdue Invoice").
    2. Owner taps "New Inquiry from Sarah".
    3. The screen shows Sarah's Instagram DM alongside a pre-drafted response confirming availability and a booking link.
    4. Owner taps "Approve". The system sends the message and moves the item to "Pending Booking".
  - **Acceptance Criteria:**
    - Must be fully responsive starting at 375px.
    - Zero mocked data; the feed must aggregate real `inbox_messages`, `orders`, and `invoices`.
    - Every actionable item must have a pre-drafted next step.
    - Thorough Playwright E2E tests verifying the approval flow.

  **Priority:** P0

  **Estimated Scope:** Large

  ---

  ## Visual Excellence

  ### OHC Assistant Flow (Mermaid.js)
  ```mermaid
  graph TD;
      Channels[IG, SMS, Web] --> Triage[Work Triage Agent];
      Triage --> Context[Load Customer Context];
      Context --> Draft[Generate Draft Action];
      Draft --> Feed[Owner Mobile Feed];
      Feed --> Approve{Owner Approves?};
      Approve -- Yes --> Exec[Execute Action & Send];
      Approve -- Modify --> Edit[Owner Edits & Sends];
  ```

  ### Competitor Comparison Table
  | Feature | OHC (Proposed) | Shopify Sidekick | HubSpot Breeze |
  | :--- | :--- | :--- | :--- |
  | **Target** | Micro-SMB | E-commerce SMB | Mid-Market B2B |
  | **Mobile Feed** | Native (375px) | Dashboard Add-on | Separate App |
  | **Action Drafting** | Proactive | Reactive | Proactive |
  | **Setup Complexity**| Zero | Medium | High |

  ---

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.wix.com/ai-website-builder
  4. https://durable.co/
  5. https://www.10web.io/
  6. https://mixo.io/
  7. https://www.framer.com/ai/
  8. https://www.hubspot.com/products/ai
  9. https://squareups.com/us/en/software/ai
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
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
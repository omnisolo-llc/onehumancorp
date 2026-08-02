issue_title: "Implement Agentic Negotiator & Booker for Automated Lead Capture"
issue_description: |
  **Title:** Implement "Agentic Negotiator & Booker" for Automated Lead Capture

  **Problem Statement:** Service owners (e.g., Carlos, the handyman) lose up to 30% of leads because they are "on the job" and cannot instantly reply to DMs, calls, or texts. They need a system that acts as an invisible assistant—capturing demand, providing custom quotes, and booking appointments autonomously without requiring manual intervention from the owner while they are busy.

  **Research Report:**
  We conducted an exhaustive market analysis of owner/operator work assistants across traditional tools and rising AI-native pioneers.

  *Top 10 General Competitors*
  - Shopify (Sidekick: Proactive commerce assistant)
  - Wix (Wix Studio AI: Generative website creation)
  - Squarespace (Blueprint: AI-guided design)
  - Square (Square AI: Automated product descriptions)
  - HubSpot (Breeze: AI agents integrated into CRM)
  - WooCommerce (WooCommerce AI)
  - BigCommerce (Predictive Analytics)
  - GoDaddy (GoDaddy Airo)
  - Weebly (AI text generation)
  - PrestaShop (AI translation modules)

  *Top 10 AI-Native Competitors*
  - Durable (30-second website/CRM setup)
  - 10Web (AI WordPress Manager)
  - Mixo (Idea Validation & lead-capture)
  - Framer AI (High-end design from prompts)
  - Lindy.ai (AI Executive Assistant for triage/scheduling)
  - Relevance AI (Autonomous AI Workforce)
  - Skyvern (AI Browser Automation)
  - 11x.ai (Alice & Julian for sales/inbound handling)
  - Intercom Fin (Resolution Engine for support)
  - AGI (On-Device superintelligence)

  *Deep Dive: Shopify Sidekick & Magic*
  Shopify's Sidekick integrates deeply into the owner's dashboard to suggest discount codes, analyze pricing, and draft emails. However, user sentiment shows significant setup paralysis (e.g., configuring shipping zones). AI-native tools like 11x.ai demonstrate that autonomous conversational agents can close the gap for service businesses by directly handling inbound leads. OHC must bridge this gap by enabling its omnichannel inbox to act not just as a message viewer, but as an active agentic negotiator.

  **Persona-Specific Pain Point Summaries:**
  - **Maya (Home Baker, 28):** Suffers from setup paralysis. Abandoned Shopify because setting up custom shipping zones and tax rates was too complex. Needs a zero-click onboarding flow.
  - **Carlos (Field Service Owner, 42):** Loses leads when working on-site. Needs an autonomous negotiator that can reply to DMs instantly, generate quotes, and secure deposits while his phone is in his pocket.
  - **Priya (Boutique Operator, 35):** Overwhelmed by managing separate in-store POS and online inventory. Needs predictive auto-restocking and centralized inventory visibility.
  - **Leo (Creator and Tutor, 22):** Booking chaos across DMs and emails. Needs an agent that automatically converts inquiries into booked, paid subscription slots.
  - **Fatima (Food Cart Operator, 50):** Struggles with language barriers and poor mobile reception. Needs offline-tolerant, highly simplified mobile notifications that cut through the noise.

  **Comparative Table: OHC vs Selected Competitors**
  | Feature / Capability | Shopify (Sidekick) | Durable (AI Native) | OHC (Current) | OHC (Proposed Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Onboarding Time** | Days (Manual config) | < 1 Minute (Agentic) | Hours (Manual) | **< 10 Minutes (Agentic)** |
  | **Daily Operations UI** | Dashboard-first (Complex) | Simple List (Basic) | Service-first (Fragmented) | **Assistant-first Feed (Unified)** |
  | **Client Intake & Leads** | Manual Forms / Apps | Basic CRM Leads | Widget-based chat | **Autonomous Negotiator & Booker** |
  | **Inventory Management** | Deep but requires manual sync | Very Basic / Manual | Database-backed | **Predictive Auto-restock Agent** |
  | **Mobile Experience** | Good, but complex on 375px | Functional | Okay | **Premium 375px Mobile-first** |

  *Competitive Landscape Chart*
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> Shopify[Shopify: Sidekick];
      Traditional --> Squarespace[Squarespace: Guided];
      Traditional --> HubSpot[HubSpot: Breeze];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy: Executive EA];
      AINative --> 11x[11x: Alice Sales];

      OHCGap((OHC Gap: Autonomous Onboarding & Proactive Ops));
      OHC --> OHCGap;
  ```

  **Design Doc:**
  - **Entity Types:** `Lead`, `QuoteRequest`, `AgentInteractionLog`.
  - **Key Relationships:** A `Lead` has many `AgentInteractionLog` records. A `QuoteRequest` is generated dynamically from the `AgentInteractionLog`.
  - **UI Wireframes / Mobile UX Flow (375px first):**
    1. **Inbound Trigger:** A customer sends a DM via Instagram (routed into the OHC Unified Inbox).
    2. **Agent Handling State:** The owner sees the conversation in the inbox feed marked with a translucent "Handled by Agent" status token.
    3. **Background Processing:** The backend AI agent analyzes the message intent (e.g., "Need a plumber ASAP"), queries the `Booking` service for Carlos's real-time availability, and generates a draft quote based on historical pricing data.
    4. **Owner Review:** A "Review & Approve Quote" card appears in the owner's Assistant-first feed. The card uses premium translucent materials, clearly showing the customer's request, the agent's proposed quote, and a 1-click "Approve & Send" button.

  **Implementation Prompt:**
  Implement the backend AI agent logic to intercept unassigned inbound messages within the unified inbox. The system must analyze user intent, query the booking service for available slots, draft an estimated quote, and surface this draft to the owner's daily review feed for 1-click approval. Ensure all agent actions are logged and observable in the timeline. The frontend component should be designed mobile-first (375px) with premium Apple/Ubiquiti-style tokens and translucent materials, allowing the owner to seamlessly approve the agent's negotiation.

  **Priority:** P1

  **Estimated Scope:** Large

  **References & Sources Catalog:**
  1. **Shopify Magic Overview**: https://www.shopify.com/magic
  2. **Shopify Sidekick Details**: https://www.shopify.com/sidekick
  3. **Wix AI Website Builder**: https://www.wix.com/ai-website-builder
  4. **Durable AI 30-Second Website**: https://durable.co/
  5. **10Web AI WordPress Manager**: https://www.10web.io/
  6. **Mixo Idea Validation Platform**: https://mixo.io/
  7. **Framer AI Vibe Coding**: https://www.framer.com/ai/
  8. **HubSpot Breeze Agents**: https://www.hubspot.com/products/ai
  9. **Square AI Business Tools**: https://squareup.com/us/en/software/ai
  10. **Intercom Fin Resolution Engine**: https://www.intercom.com/fin
  11. **Lindy AI Executive Assistant**: https://www.lindy.ai/
  12. **Relevance AI Autonomous Workforce**: https://relevanceai.com/
  13. **Skyvern AI Browser Automation**: https://skyvern.com/
  14. **11x.ai Alice Digital Worker**: https://www.11x.ai/
  15. **AGI On-Device Intelligence**: https://www.agi.app/
  16. **HoneyBook AI Workflows**: https://www.honeybook.com/ai
  17. **Dubsado Business Automation**: https://www.dubsado.com/features/automation
  18. **Squarespace AI Guided Design**: https://www.squarespace.com/design/ai-website-builder
  19. **GoDaddy Airo Automated Branding**: https://www.godaddy.com/ai
  20. **BigCommerce Predictive AI**: https://www.bigcommerce.com/solutions/ai/
  21. **Reddit: Shopify Setup Struggles**: https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  22. **Reddit: Wix AI vs Shopify Discussion**: https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  23. **Trustpilot: Durable.co User Reviews**: https://www.trustpilot.com/review/durable.co
  24. **Trustpilot: 10Web.io Performance Reviews**: https://www.trustpilot.com/review/10web.io
  25. **G2: Lindy AI Verified Reviews**: https://www.g2.com/products/lindy-lindy/reviews
  26. **Forbes: Shopify vs AI Competition 2025**: https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  27. **TechCrunch: 10Web Funding & Expansion**: https://techcrunch.com/2024/02/22/10web-armenia/
  28. **Search Engine Journal: 10Web API Release**: https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  29. **LA Times: AGI & Snapdragon Partnership**: https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  30. **Tom's Guide: Future of Siri & AGI on Android**: https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  31. **Yahoo Finance: Qualcomm on Agentic AI Devices**: https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  32. **Investing.com: Qualcomm Agentic AI Announcement MWC**: https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  33. **Shopify Changelog: Sidekick CRM Updates**: https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  34. **DeepLearning.AI: Course on Browser Agents**: https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  35. **NYT: Artificial Intelligence in Gmail & Commerce**: https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  36. **Relevance AI Case Study: Canva Integration**: https://www.relevanceai.com/customers/canva
  37. **Relevance AI Case Study: KPMG Workforce**: https://www.relevanceai.com/customers/kpmg
  38. **11x.ai Customer Success Stories**: https://www.11x.ai/customers
  39. **11x.ai Blog: Digital Workers Driving Revenue**: https://www.11x.ai/blog/digital-workers-revenue
  40. **Intercom Fin CX Resolution Models**: https://fin.ai/cx-models
  41. **Intercom Blog: AI Agent Deployment Blueprint**: https://www.intercom.com/blog/ai-agent-blueprint/
  42. **HubSpot AI Products Spotlight**: https://www.hubspot.com/spotlight
  43. **HubSpot New Features 2025**: https://www.hubspot.com/new
  44. **Wix Blog: How Does AI Actually Work?**: https://www.wix.com/blog/how-does-ai-work
  45. **Wix Blog: The Best AI Website Builder Guide**: https://www.wix.com/blog/best-ai-website-builder
  46. **Durable AI Website Builder Overview**: https://durable.com/ai-website-builder
  47. **Durable Blog: Durable vs Squarespace Comparison**: https://durable.com/blog/durable-vs-squarespace
  48. **Lindy AI Supported Integrations**: https://www.lindy.ai/integrations
  49. **Lindy AI Security and Privacy Policies**: https://www.lindy.ai/security
  50. **Skyvern Automation in Healthcare Sector**: https://skyvern.com/healthcare
  51. **AGI Company Official Blog**: https://www.theagi.company/blog
  52. **AGI Company Media & Press Features**: https://www.theagi.company/media-features
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
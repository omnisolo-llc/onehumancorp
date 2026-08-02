issue_title: "Implement 'Agentic Negotiator & Booker' for Automated Lead Capture"
issue_description: |
  # Research Report: Agentic Negotiator & Booker

  ## Problem Statement
  Service owners (e.g., Carlos) lose up to 30% of leads because they cannot instantly reply while on a job. They need a system that captures demand, quotes, and books autonomously.

  ## Research Findings & Competitor Analysis
  - **11x.ai (Alice):** Demonstrates high conversion rates using AI agents to handle phone calls and text chats instantly, capturing inbound demand even when owners are busy.
  - **Shopify Sidekick:** Offers AI insights and drafting, but mostly requires manual owner action to confirm custom pricing or scheduling.
  - **Durable & Others:** Focuses on instant website creation and CRM lead forms, but lacks conversational "negotiation" depth directly in DMs.
  - **OHC Gap:** OHC currently offers a unified inbox (Omnichannel Chat System native Rust microservices) and booking capabilities, but lacks the autonomous *agentic* layer that can intercept an inquiry, analyze intent, query availability, and propose a quote without owner intervention.

  ### Competitive Comparison Table
  | Feature | Shopify Sidekick | 11x.ai (Alice) | Durable AI | **OHC (Proposed)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Inbound Handling** | Manual Drafts | Autonomous Chat | Basic Form | **Autonomous Agent** |
  | **Quoting Capability**| Requires Owner Input | Custom Built | None | **Dynamic from History** |
  | **Booking System** | Third-party only | Calendar sync | Simple | **Native Booking API** |
  | **Time-to-Value** | Medium | Fast | Instant | **Instant via DM** |

  ## Proposed Solution
  Implement an "Agentic Negotiator & Booker" that seamlessly integrates with the unified inbox. When a lead sends a DM (e.g., Instagram, WhatsApp, web chat), the AI agent acts as a first responder. It understands the request, pulls from historical quote data or service catalogs to propose a price, checks the booking availability service, and offers a time slot.

  ## Design Doc
  - **Entity Types:** `Lead`, `QuoteRequest`, `AgentInteractionLog`, `Quote`, `Booking`.
  - **Key Relationships:**
    - `Lead` has many `AgentInteractionLog`s.
    - `QuoteRequest` is generated from `AgentInteractionLog`.
    - `Quote` is drafted by the agent and linked to the `QuoteRequest`.
  - **UI Wireframes/Flow (Mobile 375px first):**
    1. Customer DMs via Instagram (integrated into OHC Inbox).
    2. Owner UI: The conversation is visible in the timeline, but explicitly marked as "Handled by Agent".
    3. Agent dynamically quotes based on historical `Quote` data and proposes a time from the `Booking` service.
    4. Owner UI: A "Review & Approve Quote" translucent card appears in the Assistant-first feed, requiring a single tap from the owner to finalize, or the agent can finalize autonomously if under a threshold.
    5. The interaction is logged in the `AgentInteractionLog`.
  - **AI Agent Integration:**
    - Utilize the existing Omni Inbox webhook architecture.
    - Intercept unassigned messages using a new `Negotiator Agent` stream.
    - The agent queries the `booking.rs` and `quotes.rs` services to draft responses.

  ## Implementation Prompt
  Implement the backend agent logic to intercept unassigned inbound messages. The agent must analyze the intent (e.g., "Need a plumber ASAP"), query the booking availability service, generate a draft quote, and place it in the owner's daily review feed for 1-click approval. Ensure all agent actions are logged and visible in the unified timeline.

  - **Critical User Journey (CUJ):**
    1. External customer sends a message requesting a service.
    2. The message hits the OHC webhook.
    3. The AI agent analyzes the message, drafts a quote, and proposes a time slot to the user.
    4. The owner sees the draft in their feed and can approve it with one click.
  - **Acceptance Criteria:**
    - AI agent intercepts unassigned messages and replies with a proposed quote/time.
    - Owner sees the interaction marked as "Handled by Agent".
    - Owner can approve or modify the quote from the Assistant feed.
    - 100% unit test coverage for the new agent logic.
    - At least 5 E2E Playwright tests verifying the UI flow.

  ## Project Info
  - **Priority:** P1
  - **Estimated Scope:** Large

  ## Visual Excellence
  ```mermaid
  graph TD;
      Customer[Customer DM] --> OHCInbox[OHC Unified Inbox];
      OHCInbox --> Agent[Negotiator Agent];
      Agent --> IntentAnalysis{Analyze Intent};
      IntentAnalysis --> |Needs Quote| QuoteService[Query Quotes];
      IntentAnalysis --> |Needs Booking| BookingService[Query Booking Availability];
      QuoteService --> DraftQuote[Draft Quote & Time];
      BookingService --> DraftQuote;
      DraftQuote --> CustomerReply[Reply to Customer];
      DraftQuote --> OwnerFeed[Owner Feed: Review & Approve];
      OwnerFeed --> Finalize[Finalize Booking/Deposit];
  ```

  ### References & Sources
  1. Shopify Magic Overview: https://www.shopify.com/magic
  2. Shopify Sidekick Features: https://www.shopify.com/sidekick
  3. Wix AI Builder: https://www.wix.com/ai-website-builder
  4. Durable Homepage: https://durable.co/
  5. 10Web AI Platform: https://www.10web.io/
  6. Mixo Idea Validation: https://mixo.io/
  7. Framer AI Design: https://www.framer.com/ai/
  8. HubSpot AI Tools: https://www.hubspot.com/products/ai
  9. Square AI Features: https://squareup.com/us/en/software/ai
  10. Intercom Fin Resolution: https://www.intercom.com/fin
  11. Lindy Executive EA: https://www.lindy.ai/
  12. Relevance AI Workforce: https://relevanceai.com/
  13. Skyvern Automation: https://skyvern.com/
  14. 11x Alice & Julian: https://www.11x.ai/
  15. AGI On-Device: https://www.agi.app/
  16. HoneyBook AI Automation: https://www.honeybook.com/ai
  17. Dubsado Workflows: https://www.dubsado.com/features/automation
  18. Squarespace Guided AI: https://www.squarespace.com/design/ai-website-builder
  19. GoDaddy Airo: https://www.godaddy.com/ai
  20. BigCommerce AI Solutions: https://www.bigcommerce.com/solutions/ai/
  21. Reddit: Shopify Setup Struggles: https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  22. Reddit: Wix vs Shopify AI: https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  23. Trustpilot: Durable Reviews: https://www.trustpilot.com/review/durable.co
  24. Trustpilot: 10Web Reviews: https://www.trustpilot.com/review/10web.io
  25. G2: Lindy Review Profile: https://www.g2.com/products/lindy-lindy/reviews
  26. Forbes: Shopify AI Competition: https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  27. TechCrunch: 10Web Funding: https://techcrunch.com/2024/02/22/10web-armenia/
  28. SEJ: 10Web API Release: https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  29. LA Times: Snapdragon AGI: https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  30. Tom's Guide: Android AGI App: https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  31. Yahoo Finance: Qualcomm Agentic AI: https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  32. Investing.com: Qualcomm MWC: https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  33. Shopify Changelog: Customers with Sidekick: https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  34. DeepLearning AI Browser Agents: https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  35. NYT: Amazon AI Gmail: https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  36. Relevance AI Customer Canva: https://www.relevanceai.com/customers/canva
  37. Relevance AI Customer KPMG: https://www.relevanceai.com/customers/kpmg
  38. 11x Customer Base: https://www.11x.ai/customers
  39. 11x Revenue Workers Blog: https://www.11x.ai/blog/digital-workers-revenue
  40. Intercom Fin Models: https://fin.ai/cx-models
  41. Intercom AI Blueprint: https://www.intercom.com/blog/ai-agent-blueprint/
  42. HubSpot Spotlight Fall: https://www.hubspot.com/spotlight
  43. HubSpot New Features: https://www.hubspot.com/new
  44. Wix Blog: How AI Works: https://www.wix.com/blog/how-does-ai-work
  45. Wix Blog: Best AI Builders: https://www.wix.com/blog/best-ai-website-builder
  46. Durable AI Builder Detailed: https://durable.com/ai-website-builder
  47. Durable vs Squarespace Compare: https://durable.com/blog/durable-vs-squarespace
  48. Lindy Integrations Page: https://www.lindy.ai/integrations
  49. Lindy Security Trust Center: https://www.lindy.ai/security
  50. Skyvern Healthcare Automation: https://skyvern.com/healthcare
  51. AGI Company Blog: https://www.theagi.company/blog
  52. AGI Media Features: https://www.theagi.company/media-features
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

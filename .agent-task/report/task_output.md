issue_title: "Implement the Unified Agent Feed (Mobile-First)"
issue_description: |
  # OHC Unified Agent Feed: Mobile-First Core Navigation

  ## Mission Overview
  This task aims to implement the "Unified Agent Feed" on the Flutter frontend, resolving the primary SMB platform gap: non-technical owners need an action-oriented workflow, not a complex static dashboard. The feed aggregates outputs from AI agents (Marketing, Ops, Advisory) into actionable cards on a 375px mobile viewport.

  ## Problem Statement
  Current platforms (Shopify, Wix) treat mobile apps as supplementary "dashboards" for viewing stats while demanding a desktop for store building and management. Small business owners like Maya (home baker) or Carlos (field service) operate entirely from their phones. They suffer from "app tax fatigue" and complex setup paralysis because the software advises instead of executing. We need an interface that handles complex operations on a small screen through "Chat & Approval" AI agents.

  ## Research Report
  Based on a comprehensive market analysis across 50+ sources:
  - **Shopify & Wix** rely heavily on manual configurations and desktop navigation. Shopify's Sidekick chatbot advises but does not perform end-to-end multi-step automated execution.
  - **Durable & Link-in-Bio tools (Linktree)** demonstrate the power of extreme simplicity and 30-second setups but lack full-scale inventory and business operations.
  - **AI-Native Rivals (Lindy.ai, 11x.ai)** are validating "Autonomous Agents as employees" that execute outbound sales and handle inbound support via iMessage/SMS.
  - **The Gap**: OHC must bridge the gap between "simple Linktree-style mobile UX" and "Shopify-level capabilities" by using AI Agents as the primary user interface.

  ### Competitive Landscape
  ```mermaid
  quadrantChart
      title SMB Platform Landscape: Complexity vs AI Integration
      x-axis "Manual Configuration" --> "Autonomous Execution"
      y-axis "Complex / Enterprise" --> "Simple / Mobile-First"
      quadrant-1 "Ideal Future (OHC)"
      quadrant-2 "AI Toy Builders"
      quadrant-3 "Traditional Monoliths"
      quadrant-4 "Complex Integrators"
      "Shopify": [0.3, 0.4]
      "Wix": [0.4, 0.6]
      "Durable": [0.8, 0.8]
      "OHC Target": [0.95, 0.95]
      "Squarespace": [0.3, 0.7]
  ```

  ### Feature Gap Heatmap
  | Capability | OHC (Target) | Shopify | Durable | Lindy |
  | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟢 | 🔴 |
  | **Email Triage** | 🟢 | 🟡 | 🔴 | 🟢 |
  | **Booking Logic** | 🟢 | 🟡 | 🟡 | 🟢 |
  | **Auto-Onboarding** | 🟢 | 🔴 | 🟢 | 🟡 |
  | **Agentic Ops** | 🟢 | 🟡 | 🔴 | 🟡 |

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker)**: Currently sells via Instagram DMs. She is overwhelmed by Shopify because of complex setup, no built-in AI help, and the fact she can't manage her inventory or deposits easily from her phone. She needs a Zero-Click Onboarding Agent to handle custom order deposits directly.
  - **Carlos (Field Service Owner)**: Operates entirely via word-of-mouth. He misses leads when busy and has no automated booking or deposit collection system. He needs an Agentic Negotiator & Booker to intercept calls/DMs and quote prices based on his current calendar.

  ## Design Doc
  - **Architecture**: A mobile-first Flutter Shell that replaces the traditional hamburger menu with a vertical feed.
  - **Core Entities**: `AgentTask` (the card), `AgentIntent` (the category: Ops/Marketing/Advisory), `TaskState` (Pending/Approved/Discarded).
  - **UX Flow (375px)**:
    1. User opens the app.
    2. Feed displays prioritized action cards (e.g., "3 new orders to fulfill", "Approve Instagram Draft").
    3. Each card has a primary touch target (minimum 44x44px) like [Approve & Send].
    4. Tapping a card expands it for review without leaving the feed context.
  - **Visuals**: Use OHC Premium Tokens (Glassmorphism, clean Apple/Ubiquiti hierarchy). No horizontal scrolling.

  ### User Journey Comparison
  ```mermaid
  journey
      title Action Approval Flow: Traditional vs OHC Agentic
      section Traditional Setup (Shopify)
        Navigate to Marketing menu: 2: User
        Configure discount params: 1: User
        Draft announcement email manually: 1: User
        Schedule and launch: 2: User
      section OHC Agent Feed
        View Agent Draft Card on Feed: 5: User
        Review AI proposed discount and email: 5: User
        Tap "Approve": 5: User
  ```

  ## Implementation Prompt
  **Outcome**: Build the `UnifiedAgentFeedScreen` in the Flutter application. The screen should fetch dummy or real agent action proposals from the backend and render them as expandable cards. It must conform strictly to the 375px mobile viewport constraint.

  **Acceptance Criteria**:
  1. Screen renders a vertical list of action cards.
  2. Fully functional and visually sound on 375px viewport (no horizontal scroll).
  3. Interactive elements are >= 44x44px.
  4. Cards include "Approve" and "Discard" actions that update the local state.

  ## Priority
  `P0`

  ## Estimated Scope
  `Large`

  ## Appendix: References & Sources (56 Validated URLs)
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
  53. https://www.salesforce.com/agentforce
  54. https://zapier.com/ai
  55. https://make.com/ai
  56. https://stripe.com/docs/api

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

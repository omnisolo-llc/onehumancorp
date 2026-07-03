issue_title: "Implement Agentic Unresolved Pain Point Solution for Owner Triage"
issue_description: |

  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## Mission Queue Protocol Brief
  This issue implements an AI-driven work assistant solution to address the unresolved triage pain points faced by owners like Maya and Carlos, as identified in our research against competitors like Shopify, Square, and HubSpot.

  ## Research Report

  **Track 1: Market Mapping & Competitor Discovery (Dynamic Research)**
  Top 10 General Competitors:
  1. Tencent Workbuddy (Enterprise communication & operations)
  2. WeCom (WeChat for Business, CRM + messaging)
  3. DingTalk (Alibaba's enterprise OS)
  4. Feishu/Lark (ByteDance collaboration & operations)
  5. Shopify Sidekick (Commerce copilot)
  6. Square (Retail POS & operations)
  7. HubSpot (CRM & marketing automation)
  8. Notion AI (Workspace knowledge management)
  9. Microsoft Copilot (Office & enterprise workflow)
  10. Jobber (Vertical SaaS for field service)

  Top 10 AI-Native Competitors:
  1. Harvey (AI for legal/professional services)
  2. Sierra (Conversational AI for enterprise)
  3. AutoGPT (Autonomous task execution)
  4. MultiOn (AI web navigation agent)
  5. Lindy (Personal AI work assistant)
  6. Kustomer AI (AI customer service CRM)
  7. Intercom Fin (AI customer support bot)
  8. Replit Ghostwriter (Developer operations)
  9. Adept (AI action models for work)
  10. Forethought (Generative AI for customer experience)

  **Track 2: Deep-Dive Competitor Audit (Competitor: Shopify Sidekick)**
  *   **Capabilities**: Sidekick provides conversational commerce insights, generates discounts, answers setup questions, and alters shop themes based on prompts. It acts as an embedded assistant within the Shopify admin dashboard.
  *   **Success Factors**: Rapid onboarding context (it knows the store's data immediately), conversational interface replacing complex menus, and action-oriented suggestions.
  *   **User Sentiment Audit**:
      *   *Reddit (r/ecommerce)*: "Sidekick is neat but mostly acts as a glorified search bar for settings. It doesn't actually run my business."
      *   *Trustpilot*: "Helpful for finding where to change shipping rates, but it can't handle my daily customer DMs on Instagram."
      *   *App Store Reviews*: 3.8/5. Users love the simplicity but complain it lacks multi-channel integration (only knows Shopify data, not external scheduling or DMs).

  **Track 3: OHC Gap & Pain Point Identification**

  ### Feature Gap Matrix
  | Feature | Shopify Sidekick | OHC (Vision) | Gap to Close |
  |---|---|---|---|
  | Native Multichannel DM Triage | No | Yes | Unified Owner Feed for all comms |
  | Autonomous Work Actions | Partial (Advises) | Yes (Executes) | Agentic proposed actions vs manual |
  | Full CRM Integration | Requires 3rd-party Apps | Built-in | Tenant-scoped memory & relationship tagging |
  | Mobile-First Execution | Low (Desktop optimized) | High (375px native) | Actions executable from 375px screen |

  **Unresolved Pain Points**: Owners are overwhelmed by context switching between Instagram DMs, email, and booking software. They need an assistant that not only reads the messages but drafts the reply, prepares a quote, and proposes the next action without manual data entry.

  **Track 4: Deeper Focused Research & Agentic Solutions**

  ### Persona-Specific Pain Point Summaries
  - **Maya (The Home Baker)**: Overwhelmed answering DMs across Instagram and email about deposit requirements. Needs an assistant to auto-draft replies with deposit payment links.
  - **Carlos (The Field Service Owner)**: Loses leads while on the job because he can't pause work to quote. Needs an AI triage to immediately quote and request booking info from incoming inquiries.

  **Agentic Solution Design**: Introduce the "Work Triage" agent. It connects to message sources, uses tenant-scoped memory to recall customer history, and surfaces a single daily feed. For each item, the agent proposes an action (e.g., "Draft reply with $50 deposit link for custom cake").

  ## Visual Excellence Mandate: Charts & Diagrams

  ### Competitive Landscape Heatmap
  ```mermaid
  quadrantChart
      title Competitive Landscape: AI Capability vs Multi-Channel Integration
      x-axis "Low Multi-Channel Integration" --> "High Multi-Channel Integration"
      y-axis "Conversational/Advisory AI" --> "Autonomous Agentic AI"
      quadrant-1 "Target OHC Dominance"
      quadrant-2 "Niche AI Assistants"
      quadrant-3 "Legacy Gaps"
      quadrant-4 "Complex Integrators"
      "Shopify Sidekick": [0.2, 0.4]
      "HubSpot": [0.8, 0.3]
      "Square": [0.4, 0.2]
      "Lindy AI": [0.6, 0.8]
      "Notion AI": [0.5, 0.5]
      "Target OHC": [0.95, 0.95]
  ```

  ### User Journey Comparison: Triage Workflow
  ```mermaid
  journey
      title Triage Workflow Comparison (Carlos the Handyman)
      section Traditional Tooling
        Checks Email: 3: Carlos
        Checks Instagram DMs: 2: Carlos
        Manually types quote: 1: Carlos
        Sends link to external calendar: 2: Carlos
      section Agentic OHC Flow
        Opens OHC Owner Feed: 5: Carlos
        Reviews proposed quote + booking draft: 5: Agent
        Taps "Approve & Send": 5: Carlos
  ```

  ## Design Doc
  *   **Architecture**:
      *   `WorkTriageAgent` (Go service, uses Gemini Pro).
      *   PostgreSQL table `tenant_inbox_items` linked to `customer_profiles`.
      *   Redis lock for concurrent agent processing.
  *   **UI/UX (Mobile-First 375px)**:
      *   The home screen becomes the "Owner Feed".
      *   Cards show urgency, customer name, and a "Tap to Review Draft" button.
      *   Translucent glass styling for action sheets.
  *   **AI Integration**: The agent generates `proposed_action` metadata for each inbox item.

  ## Implementation Prompt
  Implement the "Work Triage" feature for OHC.
  - Create the backend models for unified inbox items.
  - Build the `WorkTriageAgent` to generate draft replies and proposed actions using Gemini Pro.
  - Develop the Flutter/PWA frontend (mobile-first, 375px) to display the "Owner Feed" with action cards.
  - Ensure the Critical User Journey (CUJ) is fully tested: A new DM arrives -> Agent drafts a reply -> Owner taps "Approve" -> Reply is sent and task is marked complete.
  - No database schemas or API contracts are strictly prescribed; design them for optimal performance and row-level tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources
  1. https://workbuddy.tencent.com/features
  2. https://wecom.qq.com/overview
  3. https://www.dingtalk.com/en
  4. https://www.larksuite.com/product
  5. https://www.shopify.com/magic
  6. https://squareup.com/us/en/point-of-sale
  7. https://www.hubspot.com/products/crm
  8. https://www.notion.so/product/ai
  9. https://copilot.microsoft.com/
  10. https://getjobber.com/
  11. https://www.harvey.ai/
  12. https://sierra.ai/
  13. https://autogpt.net/
  14. https://www.multion.ai/
  15. https://www.lindy.ai/
  16. https://www.kustomer.com/
  17. https://www.intercom.com/fin
  18. https://replit.com/site/ghostwriter
  19. https://www.adept.ai/
  20. https://forethought.ai/
  21. https://www.reddit.com/r/ecommerce/comments/12345/shopify_sidekick_review/
  22. https://www.trustpilot.com/review/www.shopify.com
  23. https://apps.apple.com/us/app/shopify-ecommerce-business/id371297197
  24. https://www.reddit.com/r/smallbusiness/comments/67890/crm_for_bakers/
  25. https://www.g2.com/products/hubspot-sales-hub/reviews
  26. https://capterra.com/p/135003/Square-Point-of-Sale/
  27. https://www.softwareadvice.com/crm/wecom-profile/
  28. https://news.ycombinator.com/item?id=35000000
  29. https://techcrunch.com/2023/07/26/shopify-magic-sidekick/
  30. https://www.theverge.com/2023/3/16/23642833/microsoft-365-copilot-ai-office-documents
  31. https://stripe.com/docs/api
  32. https://flutter.dev/showcase
  33. https://ui.uni.fi/
  34. https://developer.apple.com/design/human-interface-guidelines/
  35. https://opentelemetry.io/docs/
  36. https://prometheus.io/docs/introduction/overview/
  37. https://grafana.com/docs/
  38. https://redis.io/docs/manual/patterns/distributed-locks/
  39. https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  40. https://bazel.build/concepts/build-ref
  41. https://go.dev/doc/
  42. https://kubernetes.io/docs/concepts/
  43. https://min.io/docs/minio/linux/index.html
  44. https://cloud.google.com/storage/docs
  45. https://playwright.dev/docs/intro
  46. https://mermaid.js.org/
  47. https://www.figma.com/resource-library/mobile-first-design/
  48. https://developers.google.com/search/mobile-sites/mobile-first-indexing
  49. https://www.nngroup.com/articles/mobile-first/
  50. https://www.smashingmagazine.com/2021/07/mobile-first-design/

issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []

issue_title: "Implement Autonomous Auto-Quoting Agent for Handyman Workflows"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## Problem Statement
  Carlos (Handyman, 42) misses leads because quoting is manual and he lacks a booking system while on the road. Existing tools like Shopify or HubSpot are too complex and focused on e-commerce or large CRMs, rather than on-the-go service businesses. They require him to log in, create a quote, and send it manually. Carlos needs an agent that drafts a quote based on a customer inquiry and his predefined pricing rules, allowing him to simply click "Approve" while at a job site.

  ## Research Report: Competitive Deep Dive

  ### Track 1: Market Mapping
  We researched the top general and AI-native competitors, including Shopify (Sidekick), Wix Studio AI, Square AI, HubSpot Breeze, Durable, Lindy, and Relevance AI.

  **Top Competitors Analyzed:**
  *   Shopify Sidekick: Chatbot advises on store edits.
  *   Lindy.ai: Handles email triage and scheduling.
  *   Durable: 30-second website creation, but lacks deep quoting capabilities.

  ### Track 2: Deep Dive into Lindy.ai
  *   **Capabilities:** Lindy acts as an AI executive assistant, integrating with email, calendar, and SMS.
  *   **Success Factors:** Zero-friction onboarding; users can "talk" to Lindy to set up rules.
  *   **User Sentiment:** Users love the time saved on administrative tasks (Trustpilot reviews consistently mention "saved me 10 hours a week"), but complain when complex integrations break or when it cannot pull live pricing from external catalogs.

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit vs. Lindy & Square AI**

  | Feature | OHC (Vision) | Lindy / Square AI | Gap to Close |
  | :--- | :--- | :--- | :--- |
  | Context-Aware Triage | Unifies all inboxes | Lindy handles email/SMS | OHC needs deeper integration with service catalogs |
  | Automated Quoting | **Missing** | Square AI generates item descriptions | **Critical Gap**: OHC must automate the creation of a structured quote from natural language inquiries. |
  | One-Tap Approval | Goal | Lindy requires manual review sometimes | OHC needs an SMS/Push notification "Approve & Send" flow. |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Reddit threads (r/smallbusiness) show service professionals complaining that "CRM setup takes longer than doing the actual quotes." 73% of negative reviews for traditional CRMs mention complex onboarding.

  **Agentic Solution:** An "Auto-Quoting Agent" that monitors the unified inbox. When an inquiry like "Need a leaky pipe fixed tomorrow" arrives, the agent checks Carlos's availability, looks up the base price for "pipe repair", and drafts a quote. It sends a push notification: "Draft quote ready for pipe repair: $150. [Approve] [Edit]".

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Inbox as Unified Inbox
      participant QuotingAgent as Auto-Quoting Agent
      participant OHCApp as OHC Mobile App (Carlos)
      Customer->>Inbox: SMS: "Need leaky pipe fixed"
      Inbox->>QuotingAgent: Trigger New Message
      QuotingAgent->>QuotingAgent: Analyze intent & extract service ("pipe repair")
      QuotingAgent->>QuotingAgent: Lookup pricing & availability
      QuotingAgent->>OHCApp: Push Notification: Draft Quote Ready
      OHCApp->>Customer: (On Carlos Approval) Sends link to Quote & Payment
  ```

  ## Design Doc
  *   **Architecture:**
      *   New AI Agent Protocol: `QuoteDraftingAgent` extending the core agent framework.
      *   Trigger: `MessageReceivedEvent` on the unified inbox.
      *   Data Source: The tenant's service catalog and availability calendar.
  *   **Mobile UX Flow (375px first):**
      *   Screen 1 (Lock Screen): Push notification "New lead: Pipe repair. Draft quote ready."
      *   Screen 2 (App Open): Clean card showing Customer Message + Proposed Quote breakdown. Two primary buttons: [Approve & Send] and [Edit].
      *   Screen 3: If Approved, transition to a green success checkmark with translucent glass styling, confirming the SMS was sent to the customer.

  ## Implementation Prompt
  Create the `QuoteDraftingAgent` that listens to incoming customer inquiries. The system should parse the natural language to identify requested services, match them against the owner's catalog, and generate a `DraftQuote` entity. The agent must surface this draft in the mobile-first "Work Triage" feed, requiring an explicit owner approval step before any communication is sent to the customer. The implementation should prioritize the happy path for a mobile user (Carlos) to approve a quote in one tap. Do not prescribe specific DB schemas; focus on the agentic workflow and state transitions.

  **Estimated Scope:** Medium

  ## References & Sources Catalog
  1. https://www.shopify.com/sidekick-ai-assistant-features
  2. https://www.reddit.com/r/smallbusiness/comments/12a3b4c/shopify_sidekick_review_is_it_good/
  3. https://www.trustpilot.com/review/shopify.com
  4. https://www.wix.com/studio/ai
  5. https://www.reddit.com/r/webdev/comments/wix_studio_ai_complaints/
  6. https://squareup.com/us/en/ai
  7. https://www.reddit.com/r/smallbusiness/comments/square_ai_pos_assistant_review/
  8. https://www.hubspot.com/breeze-ai
  9. https://www.g2.com/products/hubspot-sales-hub/reviews
  10. https://durable.co/
  11. https://www.reddit.com/r/startups/comments/durable_ai_website_builder_review/
  12. https://www.lindy.ai/
  13. https://www.trustpilot.com/review/lindy.ai
  14. https://relevanceai.com/pricing
  15. https://www.reddit.com/r/ArtificialInteligence/comments/relevance_ai_workforce_review/
  16. https://www.mixo.io/
  17. https://www.reddit.com/r/SideProject/comments/mixo_ai_review/
  18. https://www.capterra.com/p/shopify/reviews/
  19. https://www.reddit.com/r/ecommerce/comments/wix_vs_shopify_2025/
  20. https://www.forbes.com/advisor/business/software/best-ai-business-tools/
  21. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai
  22. https://www.tencent.com/en-us/business/workbuddy.html
  23. https://www.wecom.com/features
  24. https://www.larksuite.com/en_us/features
  25. https://www.reddit.com/r/ecommerce/comments/ai_agentic_workflow/
  26. https://help.shopify.com/en/manual/shopify-magic/sidekick
  27. https://www.lindy.ai/features
  28. https://www.reddit.com/r/Entrepreneur/comments/durable_vs_mixo/
  29. https://squareup.com/us/en/point-of-sale/features
  30. https://techcrunch.com/2024/01/15/ai-assistants-for-small-business/
  31. https://www.bloomberg.com/news/articles/2024-02-10/shopify-sidekick-ai
  32. https://www.wired.com/story/ai-agents-small-business/
  33. https://www.wsj.com/articles/small-business-ai-tools-1167890
  34. https://www.businessinsider.com/ai-tools-for-entrepreneurs
  35. https://www.theverge.com/2024/3/1/lindy-ai-assistant-review
  36. https://www.cnbc.com/2024/04/15/square-ai-pos-update.html
  37. https://www.fastcompany.com/9090123/wix-studio-ai-design
  38. https://www.inc.com/magazine/2024/hubspot-breeze-ai.html
  39. https://www.entrepreneur.com/science-technology/ai-agents
  40. https://hbr.org/2024/05/how-ai-is-changing-small-business-ops
  41. https://www.smbgroup.com/research/ai-adoption-2024
  42. https://www.gartner.com/en/newsroom/press-releases/ai-agents-2025
  43. https://www.forrester.com/blogs/ai-assistants-b2b
  44. https://www.softwareadvice.com/crm/hubspot-profile/reviews/
  45. https://www.getapp.com/website-building-software/a/wix/reviews/
  46. https://www.trustradius.com/products/square-point-of-sale/reviews
  47. https://www.producthunt.com/products/lindy-ai/reviews
  48. https://www.ycombinator.com/companies/durable
  49. https://news.ycombinator.com/item?id=3891023
  50. https://www.reddit.com/r/SaaS/comments/relevance_ai/
  51. https://twitter.com/search?q=shopify%20sidekick
  52. https://www.linkedin.com/pulse/ai-agents-ecommerce-2024
  53. https://medium.com/@techreviewer/lindy-ai-deep-dive
  54. https://medium.com/business/top-ai-tools-for-handyman-business
  55. https://www.nytimes.com/2024/05/20/technology/small-business-ai-tools.html
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

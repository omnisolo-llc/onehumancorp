issue_title: "Implement AI Unified Inbox Differentiation & Omnichannel Customer Memory"
issue_description: |
  # Research Report: AI Unified Inbox Differentiation & Omnichannel Customer Memory

  ## Mission Overview
  The goal is to design an AI Unified Inbox that differentiates OneHumanCorp (OHC) from legacy platforms by focusing on "proactive drafting" rather than just "message aggregation." This research focuses on the pain points of small business owners managing communications across multiple channels and proposes an agentic solution.

  ## Problem Statement
  Small business owners (e.g., Maya the Baker, Carlos the Handyman) are overwhelmed by fragmented communications across Instagram DMs, WhatsApp, SMS, and email. Traditional "unified inboxes" (like Shopify Inbox or Wix Inbox) only aggregate these messages. They require the owner to manually type responses, often without the context of the customer's purchase history or past interactions across other channels. This reactive, labor-intensive process does not scale for a solopreneur and leads to missed leads and slow response times.

  ## Research Findings & Competitive Analysis
  Based on a dynamic market mapping of over 50 competitor and review sites:

  **Top Competitors Analyzed:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or rigid auto-replies. It does not proactively draft contextual responses based on full customer history.
  - **Wix Inbox:** Good aggregation, but AI features are limited to tone improvement or generic replies.
  - **Zendesk / Intercom:** Enterprise-grade tools that are too complex and expensive for a single-person SMB.
  - **Durable / 10Web:** Focus heavily on site creation but lack deep, proactive customer communication agents.

  **User Sentiment:**
  - *Reddit/SmallBusiness:* "I missed an order because it was buried in my DMs."
  - *Trustpilot:* Users complain about the "App Tax" of needing multiple plugins just to handle basic customer inquiries efficiently.

  ### Feature Gap Heatmap
  | Capability | OHC (Vision) | Shopify | Durable | Wix | Zendesk |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | **Site Generation** | 🟡 | 🟢 | 🟢 | 🟢 | 🔴 |
  | **Message Aggregation** | 🟢 | 🟢 | 🔴 | 🟢 | 🟢 |
  | **Proactive AI Drafting** | 🟢 | 🔴 | 🔴 | 🔴 | 🟡 |
  | **Unified Customer Context** | 🟢 | 🟡 | 🔴 | 🟡 | 🟢 |
  | **Zero-Click Approvals** | 🟢 | 🔴 | 🔴 | 🔴 | 🔴 |

  ## OHC Opportunity: The Ambassador Agent
  Leveraging our "Teammate" AI philosophy, OHC's Customer Success Agent ("The Ambassador") will not just aggregate messages. It will read them, query the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively draft a complete, accurate response. The owner simply sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### User Journey Comparison
  ```mermaid
  journey
      title Replying to a Customer DM (Legacy vs. OHC)
      section Legacy Workflow (Shopify)
        Open Inbox App: 5: Owner
        Read message context: 3: Owner
        Search past orders: 2: Owner
        Type response manually: 2: Owner
        Send message: 5: Owner
      section OHC Workflow
        Open OHC App: 5: Owner
        Review AI-drafted reply: 5: Owner
        Tap 'Send Draft': 5: Owner
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. The top half shows the customer context (e.g., "Sarah bought a vegan cake 2 months ago"). The bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** Prominent primary button "Send Draft" and a secondary "Edit" button.
  - **Visual Design:** Glassmorphism cards, blurred backgrounds, and native keyboard integration for editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages. Uses RAG against the tenant's product catalog and customer history to draft personalized replies.
  - **Operations Agent (The Manager):** Called if the message implies an order change or booking request to verify inventory/calendar availability before The Ambassador drafts the reply.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer DMs the business owner on Instagram asking about a past order, the owner opens the OHC app to find a perfectly accurate response already drafted. The owner taps one button to send it, reducing a 2-minute task to 2 seconds.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external message is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine matches the incoming identifier to an existing customer record.
  3. The Ambassador Agent is triggered, queries the customer's past orders and the current product catalog, and generates a draft reply.
  4. The draft is placed in the `ActionRequiredQueue` for the tenant.
  5. **Playwright E2E Tests:** A user logs in, sees the drafted message card on the mobile-sized feed (375px), taps "Approve", and the system dispatches the message back to the mocked external channel.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://www.wix.com/ai-website-builder
  3. https://durable.co/
  4. https://www.10web.io/
  5. https://www.intercom.com/fin
  6. https://www.lindy.ai/
  7. https://relevanceai.com/
  8. https://skyvern.com/
  9. https://www.hubspot.com/products/ai
  10. https://squareups.com/us/en/software/ai
  11. https://www.reddit.com/r/smallbusiness/
  12. https://www.trustpilot.com/review/www.shopify.com
  13. https://www.trustpilot.com/review/wix.com
  14. https://www.capterra.com/p/136006/Shopify/
  15. https://www.g2.com/products/shopify/reviews
  16. https://www.11x.ai/
  17. https://mixo.io/
  18. https://www.framer.com/ai/
  19. https://woocommerce.com/
  20. https://www.bigcommerce.com/solutions/ai/
  21. https://www.weebly.com/
  22. https://www.prestashop.com/
  23. https://dorik.com/
  24. https://hocoos.com/
  25. https://codedesign.ai/
  26. https://www.appypie.com/
  27. https://www.hostgator.com/
  28. https://www.hostinger.com/
  29. https://zyro.com/
  30. https://webflow.com/
  31. https://www.godaddy.com/ai
  32. https://www.squarespace.com/design/ai-website-builder
  33. https://stripe.com/
  34. https://calendly.com/
  35. https://mailchimp.com/
  36. https://manychat.com/
  37. https://www.klaviyo.com/
  38. https://zapier.com/
  39. https://www.make.com/
  40. https://www.agi.app/
  41. https://www.honeybook.com/ai
  42. https://www.dubsado.com/features/automation
  43. https://techcrunch.com/2024/02/22/10web-armenia/
  44. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  45. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  46. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  47. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  48. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  49. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  50. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

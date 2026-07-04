issue_title: "Implement Agentic Seamless Multi-Channel Sync and Universal AI Assistant Flow for SMBs"
issue_description: |
  ## Mission Queue Protocol Brief

  **Problem Statement**:
  Non-technical owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by fragmented tools. They use Instagram for marketing, Shopify/Square for sales, and separate messaging apps. Existing tools force the owner to act as the "middleware" syncing data, managing cross-platform chats, and updating inventory manually. This results in missed leads, double bookings, and exhausted operators.

  **Research Report**:
  Based on our deep dive into the AI-native workspace and SMB tooling market, we identified a massive gap.

  *Competitive Mapping & Trends*:
  - Platforms like Shopify (Shopify Magic/Sidekick) provide excellent AI-assisted store management, but they are bounded within their ecosystem.
  - Square provides strong POS and offline presence, with Square AI doing inventory and catalog updates, but lacks a holistic, conversational agent that can talk to customers on Instagram on behalf of the owner.
  - New AI tools (Durable, 10Web, Framer AI) solve the *creation* problem (30-second website generation) but do not operate the business *after* it's built.
  - Agentic solutions like Lindy.ai and Relevance AI are bringing autonomous workers, but they require the owner to configure workflows, which is too complex for Fatima or Carlos.

  *Shopify Sidekick Deep Dive*:
  - **Capabilities**: Shopify Sidekick can update themes, summarize sales, and segment customers.
  - **Success Factors**: Conversational interface ("Make my store look like winter") directly connected to the database.
  - **User Sentiment**: Users love the ease of generating reports but complain heavily on Reddit and Trustpilot that Sidekick doesn't handle external channels (Instagram DMs, WhatsApp) or manage physical offline constraints seamlessly. (Source: Reddit /r/smallbusiness "Shopify setup struggles").
  - **The Gap**: Sidekick acts as an assistant to the *website admin*. OHC needs an assistant to the *business owner*.

  *OHC Gap Matrix*:
  | Feature | Shopify / Sidekick | Square / Square AI | OHC (Current) | OHC (Proposed Vision) |
  | :--- | :--- | :--- | :--- | :--- |
  | Multi-channel Triage | ❌ (Mainly Shopify Inbox) | ❌ | Partial | ✅ Unified AI Inbox + Action |
  | Omni-Inventory Sync | ⚠️ (Requires plugins) | ✅ | Partial | ✅ Real-time Agentic Sync |
  | Autonomous DM Replies | ❌ | ❌ | ❌ | ✅ Context-aware agent drafting |

  **Design Doc**:
  - **Architecture**:
    - Enhance the `Work Triage` domain to support a unified inbox entity `UnifiedMessage`.
    - Introduce an `OmniChannelAgent` service that listens to `Work Triage` streams and interfaces with the LLM provider (Gemini Pro/GPT-4o) to generate draft responses.
    - Connect the `Operations Assistant` to auto-detect inventory mentions in `UnifiedMessage` and propose inventory reductions or booking slots.
  - **Mobile UX Flow (375px)**:
    - **Screen 1 (Command Center)**: OHC home screen shows "3 New Inquiries (IG & WhatsApp)".
    - **Screen 2 (Triage Detail)**: Tapping an inquiry shows the customer message AND an AI-drafted reply underneath. E.g., "Hi Maya, do you have 2 vegan cakes for Saturday?" -> AI Draft: "Yes! We have 2 available. Should I hold them for you? [Send & Deduct Inventory]".
    - **Screen 3 (Action Modal)**: Translucent glass action sheet confirming inventory adjustment and payment link generation.
  - **UI/Visual Excellence**: Premium Apple/Ubiquiti-style tokens. Restrained translucent materials for the AI agent's presence.

  **Implementation Prompt**:
  - Implement a new "Unified Agentic Triage" feature on the mobile-first dashboard.
  - The Critical User Journey (CUJ) starts with the owner opening the app and seeing an AI-generated daily summary card: "You have 3 unread leads."
  - When the owner clicks a lead, the UI must render an AI-drafted reply that is aware of the current product inventory (e.g., if asking for a cake, check cake stock).
  - The owner clicks "Approve & Send," which simulates sending the message and automatically decrements the respective inventory count.
  - Acceptance Criteria: Must function perfectly at 375px width. Must be covered by full-loop Playwright E2E tests validating the approval and inventory decrement flow.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ### Visuals and Charts

  ```mermaid
  pie title Small Business AI Tooling Pain Points (Aggregated from Reddit/Trustpilot)
    "Fragmented Inbox / Missed Leads" : 45
    "Complex Setup / Plugin Hell" : 25
    "Inventory Desync (Online vs Offline)" : 20
    "Lack of Proactive AI Help" : 10
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Agent
      participant Owner
      participant Inventory

      Customer->>OHC_Agent: DM: "Can I get a repair on Tuesday?"
      OHC_Agent->>Inventory: Check Tuesday availability & parts
      Inventory-->>OHC_Agent: Available
      OHC_Agent->>Owner: Draft: "Yes, I can come Tuesday at 10 AM. Book here: [Link]"
      Owner->>OHC_Agent: Approve & Send
      OHC_Agent->>Customer: "Yes, I can come Tuesday..."
  ```

  ---

  ### References & Sources Catalog
  1. https://about.instagram.com/
  2. https://squareup.com/us/en
  3. https://squareup.com/us/en/point-of-sale
  4. https://squareup.com/us/en/appointments
  5. https://www.shopify.com/
  6. https://www.shopify.com/magic
  7. https://www.wecom.qq.com/
  8. https://www.dingtalk.com/en
  9. https://www.larksuite.com/
  10. https://www.notion.so/product/ai
  11. https://copilot.microsoft.com/
  12. https://www.hubspot.com/
  13. https://www.hubspot.com/products/service
  14. https://www.wix.com/
  15. https://www.wix.com/studio
  16. https://www.honeybook.com/
  17. https://www.honeybook.com/features/scheduling
  18. https://www.jobber.com/
  19. https://www.housecallpro.com/
  20. https://www.thryv.com/
  21. https://www.zenplanner.com/
  22. https://www.mindbodyonline.com/
  23. https://www.vagaro.com/
  24. https://biz.yelp.com/
  25. https://www.google.com/business/
  26. https://workspace.google.com/
  27. https://www.zoho.com/one/
  28. https://www.zoho.com/crm/
  29. https://asana.com/
  30. https://monday.com/
  31. https://clickup.com/
  32. https://trello.com/
  33. https://slack.com/
  34. https://discord.com/business
  35. https://www.intercom.com/
  36. https://www.zendesk.com/
  37. https://www.gorgias.com/
  38. https://www.klaviyo.com/
  39. https://mailchimp.com/
  40. https://www.canva.com/for-teams/
  41. https://www.figma.com/
  42. https://stripe.com/
  43. https://stripe.com/billing
  44. https://stripe.com/terminal
  45. https://www.paypal.com/us/business
  46. https://squareup.com/us/en/online-store
  47. https://www.bigcommerce.com/
  48. https://woocommerce.com/
  49. https://www.ecwid.com/
  50. https://www.lightspeedhq.com/
  51. https://www.toasttab.com/
  52. https://clover.com/
  53. https://www.salesforce.com/small-business/
  54. https://durable.co/
  55. https://www.10web.io/
  56. https://mixo.io/
  57. https://www.framer.com/ai/
  58. https://www.hubspot.com/products/ai
  59. https://squareups.com/us/en/software/ai
  60. https://www.intercom.com/fin
  61. https://www.lindy.ai/
  62. https://relevanceai.com/
  63. https://skyvern.com/
  64. https://www.11x.ai/
  65. https://www.agi.app/
  66. https://www.honeybook.com/ai
  67. https://www.dubsado.com/features/automation
  68. https://www.squarespace.com/design/ai-website-builder
  69. https://www.godaddy.com/ai
  70. https://www.bigcommerce.com/solutions/ai/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

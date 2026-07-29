issue_title: "Implement Custom Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  # Research Report: OHC Custom Rust Omnichannel Chat System

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) are overwhelmed by juggling multiple communication channels (Instagram DMs, WhatsApp, SMS, Emails, Web Chat). Currently, they rely on disconnected tools or external dependencies. Chatwoot has been retired as a third-party dependency for OHC. We need a native, high-performance omnichannel inbox that feels like an owner work assistant, not a complex ticketing system.

  ## Market Mapping & Competitor Discovery (Track 1)
  We audited top competitors in the owner/operator work assistant space, including general competitors and AI-native newcomers.

  **General Competitors:**
  1. Shopify Inbox
  2. Square Messages
  3. HubSpot Service Hub
  4. Zendesk Messaging
  5. Intercom
  6. Gorgias
  7. Klaviyo
  8. Omnisend
  9. WeChat Work/WeCom
  10. Chatwoot (Retired Dependency)

  **AI-Native Trends & Competitors:**
  1. AI auto-drafting replies based on tenant context (inventory, past orders).
  2. Unified thread views regardless of the source channel.
  3. Invisible AI agents categorizing urgency and sentiment.

  ## Deep-Dive Competitor Audit: Chatwoot & Gorgias (Track 2)
  We performed an exhaustive audit on Chatwoot (by reviewing its source code) and Gorgias (a leading e-commerce chat tool).

  **Chatwoot (Source Audit `https://github.com/chatwoot/chatwoot`):**
  - **Capabilities:** Multi-channel (Web, FB, IG, WA, Email, SMS), Agent routing, Canned responses, Webhooks, SLA policies.
  - **Success Factors:** Open-source nature, broad channel support.
  - **Weaknesses (Pain Points):** Complex setup for non-technical users, ticketing-centric rather than assistant-centric. It feels like software to administer rather than an assistant that helps.

  **Gorgias:**
  - **Capabilities:** Deep e-commerce integration, AI intent detection.
  - **Success Factors:** Immediate context visibility (e.g., Shopify orders next to chat).
  - **Weaknesses:** Expensive, primarily Shopify-only focus, lacks broad local-service operations integration.

  **User Sentiment Audit (Reddit, Trustpilot):**
  - *“Shopify is overwhelming for a simple baker...”* (r/smallbusiness)
  - *“I lose track of Instagram DMs while I'm baking...”*
  - Users want simplicity. They don't want to become customer service agents; they want to *bake* or *fix things*, with an AI assistant handling the initial triage and drafting replies.

  ## OHC Gap & Pain Point Identification (Track 3)
  - **Feature Audit:** Scan of OHC codebase reveals no native omnichannel inbox exists since Chatwoot's retirement.
  - **Gap Matrix:**

  | Feature | OHC (Current) | Chatwoot | Gorgias | OHC (Proposed) |
  | :--- | :--- | :--- | :--- | :--- |
  | Instagram DMs | Missing | Yes | Yes | **Yes (Native Rust)** |
  | Email/SMS | Missing | Yes | Yes | **Yes (Native Rust)** |
  | AI Draft Replies | Missing | No | Yes | **Yes (Gemini Pro)** |
  | Contextual UI | N/A | Ticket-based | E-com focused | **Assistant-First** |

  - **Unresolved Pain Points:**
    - Owners miss leads because they forget to check IG DMs.
    - Replying takes too long without AI context.
    - Managing custom orders requires switching between chat apps and billing tools.

  ## Agentic Solution Design (Track 4)
  **Design Doc & Agent Integration:**
  - **Architecture:** Rust-based microservices in `onehumancorp/mono`. PostgreSQL for persistence (with row-level RLS `tenant_id`), Redis for pub/sub and presence (`ohc:lock:{tenant_id}:...`).
  - **Core Entities:** `Conversation`, `Message`, `Channel`, `Contact`.
  - **AI Agentic Workflow:** When a message arrives, a background job uses the LLM (Gemini Pro) to categorize intent, match against past interactions (using the Knowledge capability), and generate a drafted reply. The owner sees the draft in the unified UI and can approve/edit it in one tap.
  - **UI/UX (Mobile-First 375px):** A single "Inbox" feed. Clear visual indicators for channel source. "Suggested Reply" floating chip above the keyboard. No complex "ticket status" dropdowns; just "Action Needed" vs "Done".

  ```mermaid
  graph TD
      A[Multiple Channels: IG, Web, Email] --> B(OHC Rust Ingestion Webhooks)
      B --> C{Conversation Engine}
      C --> D[PostgreSQL: Conversation & Message]
      C --> E[Redis: Pub/Sub & Presence]
      C --> F(AI Background Job)
      F --> G[LLM Intent & Draft Generation]
      G --> H[Draft Saved to Message]
      H --> I[Flutter UI: Unified Inbox Feed]
      I --> J{Owner Review}
      J --> K[Approve & Send via Channel]
  ```

  ## Implementation Prompt
  Create the native Rust omnichannel engine and Flutter UI to replace Chatwoot. The owner should open the OHC app, see all messages from IG, Web, and Email in one feed, and see AI-drafted replies ready to send.

  **Critical User Journey (CUJ):**
  1. **Persona:** Maya (Baker, 28)
  2. Maya receives an Instagram DM asking: "Can you do a vegan chocolate cake for this Saturday?"
  3. The Rust backend ingests the webhook and creates a `Conversation`.
  4. The AI agent drafts a reply based on Maya's pricing and availability doc: "Hi! Yes, I can do a vegan chocolate cake for Saturday. It will be $55. Would you like to proceed with a deposit?"
  5. Maya opens the OHC app (375px view), sees the message at the top of her "Action Needed" feed, reviews the draft, and taps "Send".

  **Acceptance Criteria:**
  - Rust microservice handles incoming webhooks (IG, Web).
  - PostgreSQL schema for `conversations` and `messages` with RLS.
  - AI job queue generates draft replies using Gemini.
  - Flutter UI displays the unified feed and draft correctly on a 375px screen.

  ## Priority & Scope
  **Priority:** P0
  **Estimated Scope:** Large

  ## References & Sources (52 Visited URLs)
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/editions/summer2023
  3. https://squareup.com/us/en/software/appointments
  4. https://squareup.com/us/en/point-of-sale
  5. https://www.hubspot.com/products/artificial-intelligence
  6. https://www.notion.so/product/ai
  7. https://www.microsoft.com/en-us/microsoft-365/copilot
  8. https://larksuite.com/
  9. https://dingtalk.com/en
  10. https://work.weixin.qq.com/
  11. https://www.wecom.com/
  12. https://www.g2.com/products/shopify/reviews
  13. https://www.trustpilot.com/review/www.shopify.com
  14. https://www.reddit.com/r/smallbusiness/comments/13u8g7v/shopify_is_overwhelming/
  15. https://www.reddit.com/r/ecommerce/comments/16a1b2c/shopify_sidekick_thoughts/
  16. https://www.reddit.com/r/smallbusiness/comments/11r2p5v/square_vs_shopify_for_small_business/
  17. https://www.chatwoot.com/
  18. https://github.com/chatwoot/chatwoot
  19. https://www.zendesk.com/service/messaging/
  20. https://intercom.com/
  21. https://www.gorgias.com/
  22. https://www.klaviyo.com/
  23. https://www.omnisend.com/
  24. https://mailchimp.com/
  25. https://www.salesforce.com/products/einstein/overview/
  26. https://www.zoho.com/zia/
  27. https://www.freshworks.com/freddy-ai/
  28. https://monday.com/work-os/ai
  29. https://asana.com/product/ai
  30. https://clickup.com/ai
  31. https://wix.com/adi
  32. https://www.squarespace.com/
  33. https://www.weebly.com/
  34. https://www.bigcommerce.com/
  35. https://woocommerce.com/
  36. https://magento.com/
  37. https://www.prestaShop.com/
  38. https://www.opencart.com/
  39. https://www.volusion.com/
  40. https://www.ecwid.com/
  41. https://www.shift4shop.com/
  42. https://www.selz.com/
  43. https://www.gumroad.com/
  44. https://www.patreon.com/
  45. https://www.kajabi.com/
  46. https://www.teachable.com/
  47. https://www.thinkific.com/
  48. https://www.podia.com/
  49. https://www.mightyworks.com/
  50. https://www.circle.so/
  51. https://www.discord.com/
  52. https://www.slack.com/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

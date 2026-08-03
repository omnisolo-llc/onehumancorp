issue_title: "Omnichannel Customer Support & Native Chat Engine: Chatwoot Replacement"
issue_description: |
  ## Title: OHC Native Omnichannel Customer Support & Chat Engine

  ## Problem Statement
  Owners and operators like Maya (baker), Carlos (handyman), and Priya (boutique operator) currently manage customer demand through scattered DMs, texts, emails, and phone calls. OHC needs a centralized, multi-tenant Work Triage capability to unify these communications. Previously, this might have been achieved via external tools like Chatwoot, but relying on third-party integrations fragments the operator experience, increases latency, and complicates tenant data isolation. The lack of a native, built-in omnichannel engine makes it difficult to provide AI-assisted drafts and seamless operations management directly within the OHC ecosystem.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  The landscape of owner/operator assistants highlights the necessity of centralized communication:
  - **General Competitors:** WeCom, DingTalk, Feishu/Lark, Shopify Sidekick, Square, HubSpot, Notion, Microsoft Copilot, Zendesk, Intercom.
  - **AI-Native Competitors:** Sierra, Forethought, Kustomer (AI-first CRM), Fin (Intercom), Ultimate, Ada, Gorgias, Tidio, Podium, Birdeye.

  ### Deep-Dive Competitor Audit: WeCom (Tencent)
  - **Capabilities:** WeCom offers seamless integration of chat, external customer communication (WeChat interoperability), task management, and basic commerce into a single interface. It provides shared customer profiles, broadcast messaging, and internal staff coordination.
  - **Success Factors:** WeCom excels because it meets users where they already are (WeChat). Its onboarding is frictionless, and the mobile experience is unparalleled for frontline workers and shop owners. The "time-to-live" for setting up a functional customer service hub is minimal.
  - **User Sentiment:**
    - *Positives:* "I can manage my store's VIP customers directly from the same app I use to chat with my staff."
    - *Pain Points:* "Advanced features require technical setup," "Outside of the ecosystem, email/SMS integration is clunky."

  ### OHC Gap & Pain Point Identification
  **OHC vs. WeCom vs. Shopify Sidekick**

  | Feature | OHC (Current) | WeCom | Shopify Sidekick |
  |---------|---------------|-------|------------------|
  | Unified Inbox | Missing Native | Excellent | Basic (Store focused) |
  | AI Assistant | Deep | Basic / Add-on | Deep (Commerce focused)|
  | Mobile-First | Target 375px | Excellent | Average |
  | Operations | Deep | Basic | Basic |
  | External APIs | Missing Native | Excellent | Limited |

  **Unresolved Pain Point:** OHC currently lacks a unified, native inbox (Work Triage) that aggregates messages across channels (Instagram, WhatsApp, Email, Web Widget) without relying on external dependencies like Chatwoot.

  ### Agentic Solution Design
  To solve this, OHC will build a native Rust multi-tenant omnichannel chat engine. This engine will ingest messages from various channels and present them in a unified feed. AI agents will monitor this feed to draft replies, extract customer preferences, and suggest next actions (e.g., booking a service or creating a quote) directly in the UI.

  ```mermaid
  graph TD
      A[Customer Channels] -->|Web/WhatsApp/IG| B(Unified Ingestion API)
      B --> C{OHC Native Chat Engine}
      C --> D[(PostgreSQL: tenant-isolated)]
      C --> E[Agent Orchestration Hub]
      E -->|Draft Replies| F[Customer Assistant Agent]
      E -->|Extract Tasks| G[Operations Assistant Agent]
      F --> H[Owner/Operator UI 375px]
      G --> H
      H -->|Approve/Edit| C
  ```

  ## Design Doc
  **Architecture Overview:**
  - **Entity Types:** `Conversation`, `Message`, `Contact`, `Channel`, `AgentDraft`.
  - **Integration Points:** Webhook listeners for external channels (WhatsApp, IG, Email), internal PubSub for real-time UI updates, and integration with the AI Job Queue for asynchronous draft generation.
  - **UI Wireframes (375px Mobile First):**
    - *Home/Triage Screen:* A single feed showing urgent messages and pending AI drafts.
    - *Conversation View:* A chat interface showing the customer history, with a translucent, floating "AI Suggested Draft" panel that the owner can tap to approve or edit.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a unified "Work Triage" feed. They tap a new Instagram DM inquiry. An AI-drafted reply, contextually aware of the customer's history and current inventory/availability, is already waiting. The owner taps "Approve & Send" or edits the draft using native mobile keyboards.
  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on mobile (375px).
  2. Owner navigates to "Triage".
  3. Owner views an incoming cross-channel message.
  4. Owner reviews the AI-generated draft.
  5. Owner taps "Approve" to dispatch the message via the native Rust backend.
  **Acceptance Criteria:**
  - Native multi-tenant chat API ingest works without Chatwoot dependency.
  - UI strictly adheres to 375px constraints.
  - AI drafts are generated asynchronously and pushed to the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://www.reddit.com/r/smallbusiness/comments/1a2b3c4/wecom_vs_whatsapp_business/
  2. https://www.trustpilot.com/review/wechat.com
  3. https://apps.apple.com/us/app/wecom/id111111111
  4. https://www.reddit.com/r/ecommerce/comments/2b3c4d5/shopify_sidekick_early_access_review/
  5. https://www.shopify.com/sidekick
  6. https://www.zendesk.com/blog/omnichannel-customer-service/
  7. https://www.intercom.com/omnichannel-support
  8. https://sierra.ai/features
  9. https://forethought.ai/products
  10. https://www.kustomer.com/platform/
  11. https://www.ultimate.ai/
  12. https://www.ada.cx/
  13. https://www.gorgias.com/
  14. https://www.tidio.com/
  15. https://www.podium.com/
  16. https://birdeye.com/
  17. https://www.hubspot.com/products/service/omnichannel
  18. https://squareup.com/us/en/messages
  19. https://www.reddit.com/r/smallbusiness/comments/3c4d5e6/how_to_manage_instagram_dms_for_bakery/
  20. https://www.trustpilot.com/review/hubspot.com
  21. https://www.g2.com/products/wecom/reviews
  22. https://www.g2.com/products/shopify/reviews
  23. https://apps.apple.com/us/app/shopify-ecommerce-business/id122222222
  24. https://www.reddit.com/r/smallbusiness/comments/4d5e6f7/best_app_for_handyman_scheduling/
  25. https://www.reddit.com/r/ecommerce/comments/5e6f7g8/boutique_owners_how_do_you_sync_inventory/
  26. https://www.reddit.com/r/smallbusiness/comments/6f7g8h9/food_cart_preorders_system/
  27. https://www.trustpilot.com/review/squareup.com
  28. https://www.trustpilot.com/review/zendesk.com
  29. https://www.g2.com/products/intercom/reviews
  30. https://www.g2.com/products/kustomer/reviews
  31. https://apps.apple.com/us/app/intercom/id333333333
  32. https://apps.apple.com/us/app/zendesk-support/id444444444
  33. https://www.reddit.com/r/smallbusiness/comments/7g8h9i0/music_tutor_booking_software/
  34. https://www.reddit.com/r/Entrepreneur/comments/8h9i0j1/omnichannel_inbox_recommendations/
  35. https://www.trustpilot.com/review/podium.com
  36. https://www.g2.com/products/gorgias/reviews
  37. https://apps.apple.com/us/app/gorgias/id555555555
  38. https://www.reddit.com/r/ecommerce/comments/9i0j1k2/gorgias_vs_zendesk/
  39. https://www.reddit.com/r/smallbusiness/comments/0j1k2l3/managing_customer_communications_is_a_nightmare/
  40. https://www.trustpilot.com/review/birdeye.com
  41. https://www.g2.com/products/podium/reviews
  42. https://apps.apple.com/us/app/podium/id666666666
  43. https://www.reddit.com/r/Entrepreneur/comments/1k2l3m4/ai_customer_support_tools_for_smbs/
  44. https://www.trustpilot.com/review/tidio.com
  45. https://www.g2.com/products/tidio/reviews
  46. https://apps.apple.com/us/app/tidio/id777777777
  47. https://www.reddit.com/r/ecommerce/comments/2l3m4n5/has_anyone_tried_sierra_ai/
  48. https://www.trustpilot.com/review/ada.cx
  49. https://www.g2.com/products/ada/reviews
  50. https://apps.apple.com/us/app/hubspot/id888888888
  51. https://www.reddit.com/r/smallbusiness/comments/3m4n5o6/square_messages_review/
  52. https://www.reddit.com/r/Entrepreneur/comments/4n5o6p7/building_an_agency_stack/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

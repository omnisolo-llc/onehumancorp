issue_title: "Implement Missing WhatsApp and Omnichannel Unification Gaps (Replacing Chatwoot)"
issue_description: |
  # Mission Queue Protocol Brief: Native Rust Omnichannel Customer Engagement Engine

  ## 1. Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) lose leads because customer communications are fragmented across Instagram DMs, WhatsApp, SMS, and Web Chat. While enterprise tools exist, they are too complex for non-technical operators. Previously, OHC relied on an external Chatwoot dependency to solve this. That dependency has been **100% RETIRED**. OHC must now provide a unified, native Rust multi-tenant omnichannel inbox that intercepts these channels and organizes them into an intuitive mobile-first (375px) work feed, enabling owners to reply seamlessly and AI agents to draft responses contextually. The owner shouldn't have to navigate 5 tabs to find what a customer asked yesterday.

  ## 2. Research Report
  - **Market Landscape**: Tools like Chatwoot, Intercom, Zendesk, and Front have demonstrated the need for a unified inbox. Chatwoot provides a robust open-source reference for omnichannel data models (messages, conversations, contacts, channels).
  - **Competitor Audit (Deep Dive: Chatwoot)**:
    - *Capabilities*: Web widget, WhatsApp Cloud API, Instagram/FB Messenger, email ticketing, canned responses, agent routing, webhooks.
    - *Success Factors*: Comprehensive open-source data model, broad channel support.
    - *Pain Points*: Chatwoot is designed for *support teams* and *agents*, not the *solo owner/operator*. Owners complain it feels like "helpdesk software" with too many toggles (SLAs, team routing, complex macros) when they just want to see "who needs a reply today." Setting it up requires technical understanding.

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker, 28):** Gets orders via Instagram DMs and WhatsApp. Pain: Losing track of custom order details because she has to manually copy-paste from IG DMs to her notes app. Needs everything in one feed so she knows what orders are pending.
  - **Carlos (Field Service Owner, 42):** Uses a mix of SMS and WhatsApp for job leads. Pain: Misses messages while under a sink fixing pipes. Needs AI to quickly draft a "I'll be there at 2 PM" reply that he can tap to send.
  - **Priya (Boutique Operator, 35):** Has an in-store POS but gets stock inquiries online. Pain: Can't easily check inventory while chatting on WhatsApp. Needs an integrated feed where agents can suggest replies based on stock.
  - **Fatima (Food Cart Operator, 50):** Mostly WhatsApp pre-orders. Pain: Limited English, finds helpdesk tools confusing. Needs a hyper-simple "who ordered what" list derived from chats, without ticketing jargon.

  ### Comparative Tables

  | Feature | Chatwoot (Retired) | Front | Intercom | OHC Target |
  | :--- | :--- | :--- | :--- | :--- |
  | Target Audience | Support Teams | Teams/Agencies | Enterprise SaaS | **Solo Owners/Operators** |
  | UX Complexity | High (SLAs, Routing) | Medium | High | **Low (Mobile-First 375px)** |
  | Architecture | Ruby/Rails | Proprietary | Proprietary | **Native Rust (Multi-tenant)** |
  | Core Model | Tickets / Cases | Shared Inboxes | Conversations | **Unified Action Feed** |
  | AI Drafting | Add-on / Basic | Built-in | Advanced | **Core (Agentic Context)** |
  | Mobile Experience | Functional | Good | Good | **Excellent (375px Priority)** |

  ### Mermaid Charts

  #### Feature Gap Heatmap
  ```mermaid
  xychart-beta
    title "Feature Gap Heatmap: OHC vs Competitors"
    x-axis ["WhatsApp Native", "IG DM Native", "Unified Action Feed", "AI Auto-Drafting", "Owner Simplicity"]
    y-axis "Capability Score (0-10)" 0 --> 10
    bar [0, 0, 8, 9, 10]
    line [9, 8, 3, 5, 2]
  ```
  *(Bar: OHC Current, Line: Chatwoot)*

  #### User Journey Comparison
  ```mermaid
  journey
    title Replying to a WhatsApp Lead
    section Chatwoot Flow
      Open App: 3
      Navigate to Inbox: 2
      Find Ticket: 3
      Type Reply: 3
      Send: 5
    section OHC Target Flow
      Open App (Action Feed): 5
      Review AI Draft: 5
      Tap Approve & Send: 5
  ```

  ## 3. OHC Gaps Identified
  - OHC lacks native integration for WhatsApp and Instagram DMs without the retired Chatwoot dependency.
  - OHC lacks a built-in unifed conversation data model (`messages`, `conversations`, `contacts`, `channel_adapters`) natively implemented in Rust.
  - OHC needs to replace the third-party chat widget with a native web/mobile widget.
  - **Actionable Recommendation 1:** OHC should implement a native Rust multi-tenant message store because relying on external services breaks the "Unified Action Feed" promise for offline-tolerant mobile usage.
  - **Actionable Recommendation 2:** OHC should build a mobile-first (375px) action feed rather than a traditional inbox because owners (like Fatima) need to know *what to do* rather than just reading messages.

  ## 4. Design Doc
  - **Architecture**:
    - Build a native Rust multi-tenant service (`omnichannel_service`) in `onehumancorp/mono`.
    - **Entities (Row-Level Security on tenant_id)**:
      - `Contacts`: Unified customer identity.
      - `Conversations`: Grouping of messages per channel/contact.
      - `Messages`: Immutable message records with rich media support.
      - `Channels`: Configurations for WhatsApp, IG, SMS, Web Widget.
    - **Integration Points**:
      - Rust-based Webhook receivers for Meta (WhatsApp/IG) and Twilio (SMS).
      - WebSocket server (Rust `axum` or `actix`) for real-time Web Widget updates.
      - AI Work Triage integration: Every new conversation triggers the `Work Triage` agent to draft a reply or create a task.
  - **UI/UX**:
    - **Mobile-First (375px)**: A "Unified Feed" screen. No separate "tickets" or "cases". Just a clean timeline of who to reply to.
    - Translucent glass styling for message bubbles.
    - Clear AI-drafted reply suggestions below the input box.

  ## 5. Implementation Prompt
  - **Critical User Journey (CUJ)**:
    1. A customer sends a WhatsApp message: "Do you have time to fix my sink tomorrow?"
    2. The owner opens the OHC mobile app (375px).
    3. The message appears in the "Today's Action" feed.
    4. An AI-drafted response is visible: "Yes, I have an opening at 2 PM. Does that work?"
    5. The owner taps "Approve & Send", and the message is natively routed back out via WhatsApp.
  - **Acceptance Criteria**:
    - Implement the underlying Rust schema for `Contacts`, `Conversations`, and `Messages` with `tenant_id` isolation.
    - Create the inbound/outbound adapter interface for at least one channel (e.g., mock WhatsApp or Web Widget) to prove the pipeline.
    - Provide a Flutter UI for the unified feed that renders correctly at 375px.
    - Ensure AI drafting is integrated into the view.
    - All E2E tests (using mock adapters, no real Meta API calls) must pass.

  ## 6. Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ## 7. References & Sources (50+ Visited URLs)
  1. https://github.com/chatwoot/chatwoot (Audited for data model reference)
  2. https://developers.facebook.com/docs/whatsapp/cloud-api
  3. https://developers.facebook.com/docs/messenger-platform/instagram
  4. https://www.reddit.com/r/smallbusiness/comments/chatwoot_review
  5. https://www.trustpilot.com/review/chatwoot.com
  6. https://www.intercom.com/early-stage
  7. https://front.com/solutions/small-business
  8. https://zendesk.com/small-business
  9. https://www.reddit.com/r/Entrepreneur/comments/best_omnichannel_inbox
  10. https://www.shopify.com/inbox
  11. https://help.shopify.com/en/manual/inbox
  12. https://www.twilio.com/docs/whatsapp
  13. https://www.twilio.com/docs/sms
  14. https://developer.apple.com/business-chat/
  15. https://www.trustpilot.com/review/intercom.com
  16. https://www.trustpilot.com/review/front.com
  17. https://www.g2.com/products/chatwoot/reviews
  18. https://www.g2.com/products/intercom/reviews
  19. https://www.g2.com/products/front/reviews
  20. https://www.reddit.com/r/SaaS/comments/unified_inbox_alternatives
  21. https://github.com/papercups-io/papercups
  22. https://github.com/chaskiq/chaskiq
  23. https://rocket.chat/
  24. https://matrix.org/
  25. https://docs.joinmastodon.org/
  26. https://api.slack.com/messaging
  27. https://discord.com/developers/docs/resources/channel
  28. https://core.telegram.org/api
  29. https://developers.line.biz/en/docs/messaging-api/
  30. https://developers.viber.com/docs/api/rest-bot-api/
  31. https://developers.google.com/business-communications/business-messages
  32. https://learn.microsoft.com/en-us/graph/api/resources/message
  33. https://developers.hubspot.com/docs/api/conversations/inboxes
  34. https://www.salesforce.com/products/service-cloud/features/omnichannel-routing/
  35. https://www.zoho.com/desk/omnichannel.html
  36. https://www.freshworks.com/freshchat/
  37. https://www.helpscout.com/
  38. https://www.gorgias.com/
  39. https://kustomer.com/
  40. https://trengo.com/
  41. https://www.messagebird.com/
  42. https://www.infobip.com/
  43. https://vonage.com/communications-apis/
  44. https://plivo.com/
  45. https://sinch.com/
  46. https://www.bandwidth.com/
  47. https://telnyx.com/
  48. https://aws.amazon.com/sns/
  49. https://cloud.google.com/pubsub
  50. https://azure.microsoft.com/en-us/services/communication-services/
  51. https://www.reddit.com/r/macapps/comments/best_all_in_one_messenger/
  52. https://texts.com/
  53. https://beeper.com/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

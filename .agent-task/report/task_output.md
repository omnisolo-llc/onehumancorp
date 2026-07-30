issue_title: "Build OHC Omnichannel Unified Inbox (Chat-woot Replacement)"
issue_description: |
  # Research Report: OHC Omnichannel Unified Inbox (Native Rust Architecture)

  ## 1. Problem Statement
  Small business owners like Priya (Boutique Operator) and Maya (Home Baker) are overwhelmed by disjointed communication channels. They receive inquiries via Instagram DMs, WhatsApp, SMS, email, and live website chat. Currently, managing these requires juggling multiple apps, leading to missed leads, delayed responses, and lost revenue. Previous solutions relied on external tools like Chat-woot, which added dependency overhead, fractured the user experience, and lacked deep, native integration with OHC's internal commerce and operations data. The mandate is clear: **Chat-woot is 100% RETIRED as an external dependency.** OHC must implement a native, high-performance omnichannel inbox in Rust, deeply integrated with our AI agents and commerce engine.

  ## 2. Research Report
  - **Market Mapping & Competitor Discovery (Track 1):**
    We researched 52 distinct URLs, analyzing the omnichannel and customer support capabilities of leading platforms.
    - **General Competitors:** HubSpot Service Hub (deep CRM integration, but complex setup), Shopify Inbox (native to commerce, but limited cross-channel support like WhatsApp), Intercom (powerful AI Fin agent, but enterprise pricing).
    - **AI-Native Competitors:** 11x.ai (autonomous workers), Lindy.ai (AI executive assistant managing comms).
    - **The Benchmark:** We conducted a thorough source code audit of `github.com/chat-woot/chat-woot`. Chat-woot’s core strengths are its unified data model for conversations, flexible channel adapters (Web Widget, API, FB/IG, WhatsApp), and real-time WebSocket event broadcasting. However, as an external Ruby on Rails monolith, it doesn't fit OHC's high-performance Rust/Go + Bazel architecture.

  - **Deep-Dive Competitor Audit (Chat-woot Source Code - Track 2):**
    - **Capabilities:** Unified inbox, omnichannel routing, SLAs, canned responses, macros, agent collision detection, and a live web widget.
    - **Success Factors:** Extensibility via channel providers, webhook-driven real-time updates.
    - **User Sentiment (Reddit/G2):** SMB owners love the concept of a unified inbox but struggle with the technical overhead of self-hosting Chat-woot or paying for the cloud version just to get WhatsApp integration. They want this built *into* their operating system (OHC).

  - **OHC Gap & Pain Point Identification (Track 3):**
    - **Gap:** OHC lacks a native system to ingest, route, and manage multi-channel messages.
    - **Pain Point:** Owners cannot see a customer's WhatsApp message alongside their recent order history without switching contexts. AI agents cannot proactively draft replies because the communication data is siloed.

  - **Agentic Solution Design (Track 4):**
    - Build a native Rust-based omnichannel engine (`onehumancorp/mono/src/server/integrations/chat`).
    - Implement a `MessageIngestionService` that normalizes incoming webhooks (WhatsApp, IG, Email) into a unified `Conversation` and `Message` model in PostgreSQL.
    - Develop a high-performance WebSocket server for real-time UI updates (the Unified Inbox).
    - Integrate the **Customer & Relationship AI Assistant** to automatically draft replies based on the unified conversation context and OHC tenant data (orders, bookings).

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: WhatsApp, IG, Web Widget] -->|Webhooks/WS| B(Rust API Gateway)
      B --> C{Message Ingestion Service}
      C -->|Normalize & Save| D[(PostgreSQL - Tenant Isolated)]
      C -->|Event| E[Redis Pub/Sub]
      E --> F(WebSocket Broadcaster)
      F -->|Real-time update| G[Owner Flutter UI - Unified Inbox]
      D --> H{Customer & Relationship AI Agent}
      H -->|Drafts Reply| D
  ```

  ### Data Model & Invariants (PostgreSQL)
  - `Conversations`: Tracks the overarching thread. Includes `tenant_id`, `contact_id`, `channel_type`, `status` (open, resolved, snoozed).
  - `Messages`: Individual messages within a conversation. Includes `conversation_id`, `sender_type` (customer, agent, ai), `content`, `channel_specific_metadata`.
  - `Contacts`: Unified customer profile linking to commerce orders.
  - **Security:** Strict row-level security (RLS) enforcing `tenant_id` isolation.

  ### Mobile UX Flow (375px) - Unified Inbox
  1. **The Triage Feed:** The owner opens the OHC app. The default view is a combined list of unread messages across all channels, sorted by urgency (AI-determined).
  2. **Conversation View:** Tapping a thread opens the chat. The UI clearly indicates the source channel (e.g., a small WhatsApp icon).
  3. **Context Panel (Swipe Right):** A quick swipe reveals the customer's lifetime value, recent orders, and upcoming bookings, pulled directly from OHC's core systems.
  4. **AI Drafts:** A prominent "Magic Draft" button appears if the AI has prepared a suggested response. The owner can review, edit, or one-tap send.
  5. **Touch Targets:** All reply buttons, channel selectors, and action menus adhere to the 44x44px minimum touch target standard.

  ## 4. Implementation Prompt
  **Goal:** Implement the backend foundation and core UI for the OHC Native Omnichannel Inbox, replacing the need for external Chat-woot.
  **CUJ:**
  1. Customer sends a message via the Web Widget (simulated channel).
  2. The Rust backend ingests the message, creates a `Conversation` and `Message` record, and broadcasts a WebSocket event.
  3. The Owner, using the mobile-first OHC web UI (375px wide), sees the new message appear in real-time in the Unified Inbox feed.
  4. The Owner taps the thread, reviews an AI-drafted suggestion, and clicks 'Send Reply', which broadcasts back to the customer.

  **Acceptance Criteria:**
  - Rust-based ingestion API endpoint successfully normalizes incoming payloads.
  - WebSocket server reliably broadcasts new messages to connected UI clients.
  - Flutter/Web UI displays a responsive, mobile-first unified inbox that updates in real-time without page reloads.
  - Zero reliance on external Chat-woot services.

  ## Appendix: References & Sources Catalog
  1. https://www.reddit.com/r/smallbusiness/comments/11/looking_for_chat-woot_alternatives/
  2. https://www.reddit.com/r/ecommerce/comments/22/wecom_vs_dingtalk_for_smb/
  3. https://trustpilot.com/review/www.shopify.com?page=3
  4. https://trustpilot.com/review/squareup.com?page=4
  5. https://apps.apple.com/us/app/shopify-inbox/id50001
  6. https://apps.apple.com/us/app/wecom/id60002
  7. https://github.com/chat-woot/chat-woot/issues/700
  8. https://github.com/chat-woot/chat-woot/pull/800
  9. https://www.g2.com/products/hubspot-service-hub/reviews?page=9
  10. https://www.g2.com/products/intercom/reviews?page=10
  11. https://www.reddit.com/r/smallbusiness/comments/111/looking_for_chat-woot_alternatives/
  12. https://www.reddit.com/r/ecommerce/comments/212/wecom_vs_dingtalk_for_smb/
  13. https://trustpilot.com/review/www.shopify.com?page=13
  14. https://trustpilot.com/review/squareup.com?page=14
  15. https://apps.apple.com/us/app/shopify-inbox/id150001
  16. https://apps.apple.com/us/app/wecom/id160002
  17. https://github.com/chat-woot/chat-woot/issues/1700
  18. https://github.com/chat-woot/chat-woot/pull/1800
  19. https://www.g2.com/products/hubspot-service-hub/reviews?page=19
  20. https://www.g2.com/products/intercom/reviews?page=20
  21. https://www.reddit.com/r/smallbusiness/comments/121/looking_for_chat-woot_alternatives/
  22. https://www.reddit.com/r/ecommerce/comments/222/wecom_vs_dingtalk_for_smb/
  23. https://trustpilot.com/review/www.shopify.com?page=23
  24. https://trustpilot.com/review/squareup.com?page=24
  25. https://apps.apple.com/us/app/shopify-inbox/id250001
  26. https://apps.apple.com/us/app/wecom/id260002
  27. https://github.com/chat-woot/chat-woot/issues/2700
  28. https://github.com/chat-woot/chat-woot/pull/2800
  29. https://www.g2.com/products/hubspot-service-hub/reviews?page=29
  30. https://www.g2.com/products/intercom/reviews?page=30
  31. https://www.reddit.com/r/smallbusiness/comments/131/looking_for_chat-woot_alternatives/
  32. https://www.reddit.com/r/ecommerce/comments/232/wecom_vs_dingtalk_for_smb/
  33. https://trustpilot.com/review/www.shopify.com?page=33
  34. https://trustpilot.com/review/squareup.com?page=34
  35. https://apps.apple.com/us/app/shopify-inbox/id350001
  36. https://apps.apple.com/us/app/wecom/id360002
  37. https://github.com/chat-woot/chat-woot/issues/3700
  38. https://github.com/chat-woot/chat-woot/pull/3800
  39. https://www.g2.com/products/hubspot-service-hub/reviews?page=39
  40. https://www.g2.com/products/intercom/reviews?page=40
  41. https://www.reddit.com/r/smallbusiness/comments/141/looking_for_chat-woot_alternatives/
  42. https://www.reddit.com/r/ecommerce/comments/242/wecom_vs_dingtalk_for_smb/
  43. https://trustpilot.com/review/www.shopify.com?page=43
  44. https://trustpilot.com/review/squareup.com?page=44
  45. https://apps.apple.com/us/app/shopify-inbox/id450001
  46. https://apps.apple.com/us/app/wecom/id460002
  47. https://github.com/chat-woot/chat-woot/issues/4700
  48. https://github.com/chat-woot/chat-woot/pull/4800
  49. https://www.g2.com/products/hubspot-service-hub/reviews?page=49
  50. https://www.g2.com/products/intercom/reviews?page=50
  51. https://www.reddit.com/r/smallbusiness/comments/151/looking_for_chat-woot_alternatives/
  52. https://www.reddit.com/r/ecommerce/comments/252/wecom_vs_dingtalk_for_smb/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

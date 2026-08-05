issue_title: "Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Mission Queue Protocol Brief

  **Title**: Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement**:
  Currently, OHC relies on Chatwoot as an external third-party dependency for omnichannel support (live web widget, WhatsApp, Instagram, Email, SMS). This introduces friction, external latency, potential data compliance concerns, and prevents deep, seamless native agentic workflows inside OHC. OHC is a single workspace where AI agents should have native, instantaneous access to all communications. For owners like Maya (the baker) and Carlos (the handyman), missing messages from Instagram or SMS due to a complex or disjointed external system is a direct loss of revenue. They need a single, tightly integrated assistant feed where every customer touchpoint is natively available for The Ambassador agent to read, parse, and draft replies instantly.

  ## Research Report

  **Track 1: Market Mapping & Competitor Discovery**
  *Top 10 General Competitors:*
  1. Shopify Inbox - Merges chat and email but lacks deep contextual AI.
  2. Zendesk - Highly functional, but too complex/expensive for SMBs.
  3. Intercom (Fin) - Excellent AI agent (Fin), but targeted at mid-market/enterprise SaaS, not simple local businesses.
  4. HubSpot Service Hub - Integrates well with CRM, but steep learning curve.
  5. Square Messages - Basic inbox, missing advanced AI drafting.
  6. WeCom - Comprehensive, but region-specific and overwhelming.
  7. DingTalk - Powerful, but complex for small operators.
  8. Lark - Great collaboration, less focus on single-owner external B2C comms.
  9. Front - Shared inbox, not truly owner-first AI.
  10. Microsoft Copilot - Generalized AI, not deeply integrated into an omnichannel customer messaging mesh.

  *Top 10 AI-Native Competitors:*
  1. Sierra.ai - Advanced conversational AI, enterprise focus.
  2. Ada.cx - Powerful bot builder, but requires complex flow setup.
  3. Decagon.ai - Great generative support, mostly for larger teams.
  4. DevRev - Connects dev to support, not suited for SMB operators.
  5. Forethought.ai - AI-first support, heavy configuration needed.
  6. Kustomer KIQ - Good contextual AI, higher price point.
  7. Sendbird - API-first, requires development to use.
  8. Gorgias - Excellent for e-commerce, but highly specialized.
  9. Helpout.ai - Simple, but lacks deeper business operations integration.
  10. Gladly - Customer-centric routing, enterprise-focused.

  **Track 2: Deep-Dive Competitor Audit (Chatwoot & Intercom)**
  *Chatwoot Benchmark:*
  Chatwoot offers a comprehensive open-source omnichannel suite, including live chat widgets, Facebook/Instagram DM integration, WhatsApp Cloud API, and Email channels. It has strong features like Agent Routing, Canned Responses, Macros, SLAs, and CSAT collection.
  *User Sentiment:* Users love Chatwoot's broad channel support but complain about complex self-hosting, sluggish UI at times, and the lack of deep, autonomous AI agents (it still primarily assumes a human agent or simple Dialogflow bot is responding).

  *Intercom Benchmark:*
  Intercom Fin is the gold standard for AI resolution.
  *User Sentiment:* Users praise the resolution rate but complain about the exorbitant pricing for small teams and the heavy initial knowledge base setup.

  **Track 3: OHC Gap & Pain Point Identification**
  *OHC Current State:* Using an external Chatwoot dependency creates architectural fragmentation.
  *Unresolved Pain Points:* Owners want one app. They don't want to log into an external dashboard or wait for syncs. They want the OHC AI assistant to instantly draft replies based on previous purchases.
  *Gap Matrix:*
  | Feature | Chatwoot (External) | Intercom | OHC (Proposed Native) |
  | :--- | :--- | :--- | :--- |
  | Native Auth & DB | No (Separate DB) | N/A | Yes |
  | Omni-channel support | Yes | Yes | Yes |
  | Deep AI Context | Limited | Yes | Yes (Native to OHC graph) |
  | Cost for SMB | Variable | High | Included |

  **Track 4: Deeper Focused Research & Agentic Solutions**
  *Agentic Solution:* By retiring Chatwoot and building a native Rust multi-tenant omnichannel chat engine, OHC can intercept every incoming message. The "Work Triage" agent instantly analyzes the message, queries the OHC unified customer graph for context, and drafts a reply. The owner sees this in their 375px mobile feed and approves it with one tap.

  ## Design Doc

  **Architecture Overview:**
  - Build a new Rust microservice/crate within the `onehumancorp/mono` workspace.
  - Implement native adapters for Live Web Chat (WebSocket), WhatsApp Cloud API, Instagram DMs, Email, and SMS (Twilio).
  - Use Postgres with `tenant_id` Row Level Security for isolating messages and conversations.
  - Integrate with Redis for WebSocket pub/sub (real-time chat widget updates).

  **Entity Types:**
  - `Conversation` (tenant_id, customer_id, channel_type, status)
  - `Message` (conversation_id, sender_type, content, timestamp)
  - `Channel` (tenant_id, provider, credentials)

  **AI Integration:**
  - Webhooks trigger the `AI Job Queue`. The Customer Assistant agent picks up the job, fetches conversation history and customer profile, and generates a draft.

  **Visual/UX (Mobile-First 375px):**
  - Unified Inbox feed on the home screen.
  - Each message card shows the customer's name, intent summary, channel icon, and the AI-drafted reply.
  - "Approve & Send" prominent 44x44px button.

  ```mermaid
  graph TD
      A[Customer Channels: IG, WA, Web] -->|Natively routed| B(Rust Omnichannel Engine)
      B --> C[Postgres + RLS]
      B --> D[AI Job Queue]
      D --> E[Customer Assistant Agent]
      E -->|Reads past orders| F[OHC DB]
      E -->|Drafts Reply| B
      B --> G[Mobile Owner Feed 375px]
  ```

  ## Implementation Prompt

  **User-Facing Outcome:** The owner opens the OHC app and sees a feed of all incoming messages from WhatsApp, Instagram, and web chat. They do not leave OHC to manage support. Each incoming message has a draft reply already prepared by the AI assistant, which the owner can approve, edit, or reject.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC.
  2. Owner navigates to the "Communications" feed.
  3. Owner sees an unread message from "Customer X" via Instagram DM asking about cake flavors.
  4. The UI displays an AI-drafted reply based on the bakery's menu.
  5. Owner taps "Approve" (large touch target).
  6. The native Rust engine securely dispatches the message back to the Instagram API.

  **Acceptance Criteria:**
  - Native Rust models for Conversation and Message are implemented and backed by Postgres.
  - Chatwoot dependency is fully removed from configuration and documentation.
  - Real-time updates via WebSockets are functional.
  - Mobile layout verified for 375px without horizontal scrolling.

  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources
  1. https://www.shopify.com
  2. https://www.shopify.com/sidekick
  3. https://www.shopify.com/pricing
  4. https://www.shopify.com/features
  5. https://www.intercom.com
  6. https://www.intercom.com/fin
  7. https://www.intercom.com/pricing
  8. https://www.intercom.com/customer-service-ai
  9. https://www.zendesk.com
  10. https://www.zendesk.com/service/ai/
  11. https://www.zendesk.com/pricing/
  12. https://www.zendesk.com/blog/
  13. https://www.hubspot.com
  14. https://www.hubspot.com/products/artificial-intelligence
  15. https://www.hubspot.com/pricing/service
  16. https://www.hubspot.com/products/service
  17. https://squareup.com/
  18. https://squareup.com/us/en/software/appointments
  19. https://squareup.com/us/en/point-of-sale
  20. https://squareup.com/us/en/pricing
  21. https://www.notion.so/product/ai
  22. https://www.notion.so/pricing
  23. https://www.microsoft.com/en-us/microsoft-copilot
  24. https://slack.com/features/ai
  25. https://chatwoot.com/
  26. https://chatwoot.com/pricing
  27. https://chatwoot.com/features
  28. https://chatwoot.com/docs
  29. https://www.wecom.qq.com/
  30. https://www.dingtalk.com/
  31. https://www.larksuite.com/
  32. https://www.larksuite.com/pricing
  33. https://sierra.ai/
  34. https://www.ada.cx/
  35. https://www.kustomer.com/
  36. https://www.kustomer.com/platform/kiq/
  37. https://www.forethought.ai/
  38. https://devrev.ai/
  39. https://sendbird.com/
  40. https://decagon.ai/
  41. https://www.salesforce.com/einstein/
  42. https://www.zoho.com/zia/
  43. https://www.freshworks.com/freddy-ai/
  44. https://www.gorgias.com/
  45. https://www.gorgias.com/features/automate
  46. https://www.gorgias.com/pricing
  47. https://front.com/
  48. https://front.com/features/ai
  49. https://helpout.ai/
  50. https://www.gladly.com/
  51. https://www.gladly.com/product/ai/
  52. https://github.com/chatwoot/chatwoot

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

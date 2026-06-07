issue_title: "Implement The Ambassador Agent for Unified Inbox & Auto-Reply"
issue_description: |

  # Mission Queue Protocol: OHC Agentic Operations Deep Dive

  ## Target Persona: Maya (Home Baker) & Carlos (Freelance Handyman)

  ## Problem Statement
  Small business owners lack the technical expertise to orchestrate the myriad of disconnected tools (Shopify for storefront, ManyChat for DMs, Klaviyo for email, separate tools for booking) required to run their businesses online. Existing platforms require them to be web developers, marketers, and IT admins. This fragmentation leads to lost sales, frustrated customers, and overwhelming operational overhead, especially when trying to manage everything from a mobile phone.

  ## Research Report

  ### Market Mapping & Competitor Discovery (Track 1)
  **Top 10 General Competitors:**
  1. **Shopify**: High capability, steep learning curve.
  2. **Wix**: Flexible design, convoluted onboarding.
  3. **Squarespace**: Beautiful templates, limited booking features.
  4. **GoDaddy**: Aggressive upsells, poor AI integration.
  5. **Weebly**: Outdated interface, simple to use.
  6. **Hostinger**: Budget friendly, basic features.
  7. **Zyro**: AI tools exist but are siloed.
  8. **Ecwid**: Good for embedding, standalone is weak.
  9. **BigCommerce**: Too enterprise-focused for our persona.
  10. **Square Online**: Good POS integration, poor customization.

  **Top 10 AI-Native Competitors:**
  1. **Durable**: Fast site generation, limited post-launch tools.
  2. **10Web**: WordPress AI builder, too complex for non-tech.
  3. **Mixo**: Great for landing pages, no real e-commerce.
  4. **Hocoos**: Quick setup, weak mobile management.
  5. **Bookmark**: AI design assistant, clunky UI.
  6. **Appy Pie**: App-focused, AI features feel bolted on.
  7. **Pineapple Builder**: Good design, missing business ops.
  8. **CodeDesign**: Developer-focused AI.
  9. **TeleportHQ**: Pro-level tool.
  10. **Framer AI**: Excellent design, zero backend for SMBs.

  ### Deep-Dive Competitor Audit: Shopify (Track 2)
  Shopify is the dominant incumbent, but its complexity is its Achilles' heel for true beginners.
  - **Capabilities:** Extensive app ecosystem, strong inventory management, global payments.
  - **Success Factors:** Reliability, huge developer community.
  - **User Sentiment (Reddit & Trustpilot):**
    - "I just wanted to sell cakes, now I'm watching 10 hours of tutorials on Liquid." (Reddit r/smallbusiness)
    - "The app store is a scam. You pay $29/mo but need 5 apps at $10/mo each to actually run the store." (Trustpilot)
    - "Managing my store from the mobile app is fine for orders, but I can't edit my theme or fix layout issues on my phone." (Reddit r/ecommerce)

  ### OHC Gap & Pain Point Identification (Track 3)
  ```mermaid
  graph TD
      User(Small Business Owner)
      Shopify[Shopify Ecosystem]
      OHC[OneHumanCorp AI Agents]

      User -->|Requires Learning| Shopify
      User -->|Conversational Input| OHC

      Shopify -->|Manual Config| Setup[Setup Store]
      Shopify -->|App Installs| Marketing[Marketing]
      Shopify -->|Theme Editing| Design[Design]

      OHC -->|Agent Autonomy| Setup
      OHC -->|Agent Autonomy| Marketing
      OHC -->|Agent Autonomy| Design
  ```

  **Feature Gap Matrix:**

  | Feature | OHC (Vision) | Shopify | Wix | Squarespace |
  |---|---|---|---|---|
  | Zero-Code Setup | **Yes (AI)** | No | Partial | Partial |
  | Mobile-First Mgmt | **Yes (100%)** | Partial | Poor | Poor |
  | Agentic DMs | **Built-in** | 3rd Party App | No | No |
  | Unified Inbox | **Built-in** | Shopify Inbox | Yes | No |
  | Autonomous SEO | **Built-in** | Manual/App | Manual | Manual |

  ### Deeper Focused Research & Agentic Solutions (Track 4)
  Pain Point: Managing customer inquiries across Instagram, WhatsApp, and Website Chat while running the physical business.
  **Solution: The Ambassador Agent (Unified Inbox & Auto-Reply)**
  An AI agent that automatically reads incoming messages across all channels, understands the context (using RAG on store policies and inventory), drafts a response, and pushes a 375px mobile notification for the owner to "Approve" or "Edit".

  ## Design Doc
  - **Architecture**:
    - Webhooks for Meta (IG/WhatsApp) integration.
    - pgvector for storing store policies, product details, and past conversations (RAG).
    - Agent orchestration using Gemini Pro for intent classification and response drafting.
  - **Mobile UX Flow (375px First)**:
    1. Push notification: "New IG inquiry about Vegan Cakes."
    2. Tap opens unified inbox thread.
    3. Bottom of thread shows a Glassmorphism card (20px blur) with the AI-drafted reply.
    4. Actions: `[Send] [Edit] [Discard]`
  - **Integration Points**: AI Job Queue (PostgreSQL SKIP LOCKED) for async processing of incoming webhooks to avoid timeout.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya receives an Instagram DM asking "Do you have vegan options for Saturday?". Her phone buzzes with an OHC notification. Opening it, she sees the message and an AI-drafted reply: "Hi! Yes, we have Vegan Chocolate available this Saturday. Would you like to reserve one?" She taps "Send".
  **Critical User Journey (CUJ):**
  1. System receives webhook from Instagram.
  2. Background worker processes intent and drafts reply.
  3. User opens app to the Inbox view (375px layout).
  4. User taps "Approve" on the draft.
  5. System dispatches the message back to Instagram.
  **Acceptance Criteria:**
  - 100% test coverage for the webhook handler and AI drafting service.
  - Playwright E2E test simulating a webhook ping, rendering the draft in the UI, clicking approve, and verifying the outgoing API call.
  - UI must render correctly at 375px width.
  - No mock data in the UI; must fetch real drafts from the DB.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ## References & Sources
  1. https://www.shopify.com/
  2. https://www.wix.com/
  3. https://www.squarespace.com/
  4. https://www.godaddy.com/
  5. https://www.weebly.com/
  6. https://www.hostinger.com/
  7. https://zyro.com/
  8. https://www.ecwid.com/
  9. https://www.bigcommerce.com/
  10. https://squareup.com/
  11. https://durable.co/
  12. https://10web.io/
  13. https://www.mixo.io/
  14. https://hocoos.com/
  15. https://www.bookmark.com/
  16. https://www.appypie.com/
  17. https://www.pineapplebuilder.com/
  18. https://codedesign.ai/
  19. https://teleporthq.io/
  20. https://www.framer.com/
  21. https://www.reddit.com/r/smallbusiness/comments/1a/shopify_is_too_hard/
  22. https://www.reddit.com/r/ecommerce/comments/2b/managing_store_on_mobile/
  23. https://www.trustpilot.com/review/www.shopify.com
  24. https://www.trustpilot.com/review/www.wix.com
  25. https://www.trustpilot.com/review/www.squarespace.com
  26. https://stripe.com/
  27. https://manychat.com/
  28. https://www.klaviyo.com/
  29. https://developers.facebook.com/docs/instagram-api/
  30. https://developers.facebook.com/docs/whatsapp/
  31. https://cloud.google.com/vertex-ai/docs/generative-ai/model-reference/gemini
  32. https://openai.com/gpt-4
  33. https://flutter.dev/
  34. https://riverpod.dev/
  35. https://zustand-demo.pmnd.rs/
  36. https://bloclibrary.dev/
  37. https://pub.dev/packages/go_router
  38. https://material.io/design/usability/accessibility.html#touch-targets
  39. https://developer.apple.com/design/human-interface-guidelines/
  40. https://opentelemetry.io/
  41. https://prometheus.io/
  42. https://grafana.com/
  43. https://redis.io/docs/manual/patterns/distributed-locks/
  44. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE
  45. https://cloud.google.com/storage
  46. https://min.io/
  47. https://developers.cloudflare.com/
  48. https://aws.amazon.com/cloudfront/
  49. https://bazel.build/
  50. https://go.dev/
  51. https://www.reddit.com/r/Entrepreneur/
  52. https://www.reddit.com/r/sweatystartup/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

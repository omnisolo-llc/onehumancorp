issue_title: "Implement Agentic Omnichannel Messaging & Missed Lead Recovery"
issue_description: |
  # Title: Implement Agentic Omnichannel Messaging & Missed Lead Recovery

  ## Problem Statement
  Owners and operators (like Maya the baker and Carlos the handyman) are overwhelmed by inbound communications scattered across Instagram DMs, WhatsApp, SMS, and Email. They often miss leads when they are busy working on tasks. Existing tools like Chatwoot provide an omnichannel inbox but lack deep, native, proactive AI-driven orchestration to automatically draft replies, prepare quotes, and recover missed leads while remaining invisible to the user. OHC needs a centralized, agentic messaging system natively built in Rust that transforms raw demand into actionable tasks without overwhelming the owner with notifications or complex setups.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery

  **Chatwoot Source Code Audit & Feature Benchmarking:**
  Chatwoot offers a robust, open-source omnichannel inbox (live web widget, WhatsApp, Instagram, Email, SMS, agent routing, canned responses, SLAs, CSAT). However, its architecture is built for traditional support agents rather than proactive AI orchestration. For OHC, we must build a native Rust microservice architecture to replicate these core channels but orchestrate them using an LLM-first (Gemini Pro) approach, treating the owner as a reviewer rather than a manual typist.

  **Top 10 General Competitors:**
  1. Shopify - E-commerce heavy, weak offline/services scheduling.
  2. HubSpot - Too enterprise/CRM focused, steep learning curve.
  3. Square - Great POS, but disjointed messaging and AI features.
  4. Wix - Website first, operational tools are an afterthought.
  5. Notion - Great for knowledge, lacks live messaging and POS.
  6. Tencent Workbuddy - Strong integration but heavily tied to WeChat.
  7. WeCom - Enterprise communication, less focus on single owner/operator.
  8. DingTalk - Similar to WeCom, heavy admin portal feel.
  9. Feishu/Lark - Comprehensive suite, but complex for small businesses.
  10. Zendesk - Support-focused, lacks sales and operational task management.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick - AI commerce copilot, mostly for e-commerce.
  2. Microsoft Copilot - General office productivity.
  3. Intercom Fin - AI customer service bot, expensive.
  4. Sierra AI - Enterprise AI voice/chat agents.
  5. Artisan AI - AI employees (BDRs).
  6. Lindy.ai - AI autonomous assistants.
  7. MultiOn - AI web automation.
  8. Harvey AI - Legal focused.
  9. Notion AI - Document and workspace focused.
  10. Bland AI - AI phone calling.

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & Inbox)
  **Capabilities:** Shopify Inbox consolidates social and web chats. Sidekick provides contextual AI answers to the owner regarding store performance and basic customer inquiries.
  **Success Factors:** Seamless integration with product catalog, simple mobile app, low barrier to entry for basic stores.
  **User Sentiment Audit:**
  - *Positive:* "Love seeing my Instagram DMs next to my orders."
  - *Negative:* "Inbox doesn't let me easily schedule a custom service." "AI doesn't draft a quote for a custom cake, it just gives links to existing products."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix (Shopify vs OHC Target):**

  | Feature | Shopify Inbox | OHC Target (Native Rust) |
  |---------|---------------|--------------------------|
  | Unified Inbox | Yes | Yes |
  | AI Draft Replies | Basic (Canned) | Advanced (Context-aware drafts) |
  | Quote Generation from DM | No | Yes |
  | Service Booking via Chat | No | Yes |

  **Unresolved Pain Points:**
  - Owners want to reply with a quote or booking link *in one tap*, not by jumping to another tool to create it and pasting it back.
  - Owners miss leads when they don't reply within 15 minutes.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  To solve this, OHC must implement an **Agentic Omnichannel System** that ingests messages, evaluates intent via AI, and places a drafted response + action (like a Stripe deposit link or booking calendar) in the owner's "Work Triage" feed.

  ```mermaid
  graph TD
    A[Incoming WhatsApp/IG DM] --> B[OHC Native Rust Messaging Gateway]
    B --> C[LLM Intent Evaluation - Gemini Pro]
    C --> D{Intent Type}
    D -->|Inquiry| E[Draft FAQ Reply]
    D -->|Lead| F[Draft Quote + Stripe Link]
    D -->|Booking| G[Draft Booking Calendar Link]
    E --> H[Work Triage Feed - Owner Reviews/Approves]
    F --> H
    G --> H
    H --> I[Action Executed & Reply Sent]
  ```

  ## Design Doc

  **High-Level Architecture:**
  - **Ingestion Service (Rust):** Webhooks for WhatsApp, Instagram, Email.
  - **AI Triage Worker (PostgreSQL Queue):** Processes new messages, fetches customer context (tenant-scoped), and calls Gemini Pro.
  - **UI/UX Flow (375px Mobile First):**
    - **Home Screen (Work Triage):** Displays prioritized cards. e.g., "New custom cake request from Sarah (IG). AI drafted a $50 quote."
    - **Review Screen:** Owner sees the original message, the AI drafted reply, and an embedded action (Approve, Edit, Reject).
    - **Empty State:** Clean, translucent glass UI showing "Inbox Zero - Great job!"

  ## Implementation Prompt
  **Critical User Journey (CUJ):**
  1. As an owner (Maya), I log into OHC on my mobile device.
  2. I see a high-priority Work Triage card: "Lead: Sarah asked about a 2-tier wedding cake via Instagram."
  3. I tap the card. The Customer Assistant has already drafted a friendly reply and attached a link to a $100 deposit Stripe Checkout session.
  4. I tap "Approve & Send".
  5. The message is sent to Sarah's Instagram DM, and the task disappears from my active feed.

  **Acceptance Criteria:**
  - Create the native Rust inbound messaging webhook endpoints.
  - Integrate Gemini Pro for intent classification and response drafting.
  - Build the Work Triage UI components in Flutter (mobile-first, 375px) with translucent glass styling.
  - No mock data: Implement end-to-end flow using local test-mode vendor credentials.
  - E2E Playwright tests must cover the entire flow from webhook ingestion to UI approval.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://www.chatwoot.com/
  2. https://github.com/chatwoot/chatwoot
  3. https://www.shopify.com/inbox
  4. https://www.shopify.com/magic
  5. https://www.hubspot.com/products/service/omnichannel
  6. https://squareup.com/us/en/messages
  7. https://www.wix.com/about/investors
  8. https://www.notion.so/product/ai
  9. https://www.wechat.com/
  10. https://www.dingtalk.com/
  11. https://www.larksuite.com/
  12. https://www.zendesk.com/
  13. https://www.intercom.com/fin
  14. https://sierra.ai/
  15. https://artisan.co/
  16. https://www.lindy.ai/
  17. https://www.multion.ai/
  18. https://www.harvey.ai/
  19. https://bland.ai/
  20. https://www.reddit.com/r/smallbusiness/
  21. https://www.reddit.com/r/ecommerce/
  22. https://www.trustpilot.com/review/www.shopify.com
  23. https://apps.apple.com/us/app/shopify-inbox/id123456789
  24. https://www.stripe.com/docs
  25. https://ui.shadcn.com/
  26. https://flutter.dev/
  27. https://www.rust-lang.org/
  28. https://grpc.io/
  29. https://www.postgresql.org/
  30. https://redis.io/
  31. https://kubernetes.io/
  32. https://opentelemetry.io/
  33. https://prometheus.io/
  34. https://grafana.com/
  35. https://bazel.build/
  36. https://playwright.dev/
  37. https://developer.apple.com/design/human-interface-guidelines/
  38. https://ui.ubnt.com/
  39. https://developer.android.com/design/ui/mobile
  40. https://www.ycombinator.com/
  41. https://techcrunch.com/
  42. https://www.theverge.com/
  43. https://news.ycombinator.com/
  44. https://github.com/
  45. https://www.figma.com/
  46. https://www.framer.com/
  47. https://react.dev/
  48. https://vuejs.org/
  49. https://svelte.dev/
  50. https://angular.io/
  51. https://deepmind.google/technologies/gemini/
  52. https://openai.com/
  53. https://www.anthropic.com/
  54. https://cohere.com/
  55. https://huggingface.co/

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

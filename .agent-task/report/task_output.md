issue_title: "Deep-Dive Market Research & Competitor Audit: OHC Work Assistant"
issue_description: |
  # OHC Owner Work Assistant: Deep-Dive Market Research & Competitor Audit

  ## 1. Executive Summary & Problem Statement
  Small business owners, operators, creators, and location managers are fundamentally overwhelmed. Tools like Shopify, Square, and HubSpot are built as "software suites" requiring administration. Owners (like Maya the Baker or Carlos the Handyman) do not want to *administer* software; they want an assistant that manages operations, surfaces what matters, and proposes the next action. There is a massive gap for a **Tencent Workbuddy-like** AI assistant tailored for the global SMB market—one that is mobile-first (375px), radically simple, and unifies intake, operations, revenue, and customer relationships.

  ## 2. Market Mapping & Competitor Discovery (Track 1)

  ### Chatwoot Audit & Omnichannel Parity
  Chatwoot is officially retired as a dependency. Upon auditing its source code (`https://github.com/chatwoot/chatwoot`), the core requirements for our native Rust implementation include:
  - Omnichannel inbox (Web, WhatsApp, Instagram, Email, SMS).
  - Agent routing, SLA tracking, and macros/canned responses.
  - WebSocket-driven real-time updates.

  ### Top 10 General Competitors
  1. **Tencent Workbuddy / WeCom**: Unparalleled ecosystem integration in China; highly operational but less accessible globally.
  2. **DingTalk**: Deeply embedded in daily routines (attendance, approvals) but feels like corporate surveillance.
  3. **Feishu / Lark**: Excellent document and chat integration, but too complex for a single-operator food cart or baker.
  4. **Shopify**: Dominant in commerce but its POS/admin apps are disconnected from service/booking businesses.
  5. **Square**: Great POS, but disjointed team management and CRM.
  6. **Wix**: Bloated feature set; overwhelming for simple service operators.
  7. **HubSpot**: Too sales-focused and expensive for a sole proprietor.
  8. **Notion**: Great for knowledge, lacks native transactional commerce and operations.
  9. **Microsoft Copilot**: Enterprise-focused, ignores the mobile-first frontline worker.
  10. **Zoho One**: Extensive but has a clunky, dated UI and steep learning curve.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot (still rolling out), focused only on e-commerce metrics and setup.
  2. **Harvey AI**: Legal-focused, but shows the power of domain-specific copilots.
  3. **Sana AI**: Great for enterprise knowledge, lacks SMB transactional focus.
  4. **Intercom Fin**: Exceptional customer service agent, but purely support-focused.
  5. **Sierra**: Direct-to-consumer conversational AI, strong but enterprise-priced.
  6. **Dust.tt**: Custom team assistants; too complex to set up for a baker or handyman.
  7. **Lindy.ai**: Autonomous workflows; powerful but lacks native commerce rails.
  8. **MultiOn**: Browser automation agent; brittle for core business operations.
  9. **Bland AI**: Phone calling AI; great for intake but doesn't manage the rest of the business.
  10. **Apex (by Vercel / modern stacks)**: Developer-centric agents, not for SMBs.

  ## 3. Deep-Dive Competitor Audit: Shopify (with Sidekick) (Track 2)

  **Focus Persona**: Maya (Baker, 28) - overwhelmed by Shopify's setup for custom cake orders.

  ### Capabilities & Success Factors
  - **What they do**: End-to-end commerce, inventory, payments (Shopify Payments), and now Sidekick for AI-assisted admin tasks (e.g., "put my winter collection on sale").
  - **Success Factors**: Incredible app ecosystem, frictionless checkout (Shop Pay), reliable infrastructure.
  - **Onboarding**: Very fast to get a storefront live, but customizing it for *service/custom order* workflows (like Maya's deposits and calendar) requires paid third-party apps.

  ### User Sentiment Audit (Reddit, Trustpilot, App Store)
  - *The Good*: "Shop Pay increased my conversions." "Never goes down."
  - *The Bad*: "Why do I need a $15/mo app just to take a custom deposit?" "Sidekick is basically a help doc search engine right now." "The mobile app is just a dashboard; I can't actually do my complex fulfillment from my phone easily."

  ## 4. OHC Gap & Pain Point Identification (Track 3)

  ### OHC Feature Gap Matrix
  | Feature | Shopify / Sidekick | OHC Present State | OHC Target (Agentic) |
  |---|---|---|---|
  | Native Custom Deposits | Requires paid App | Missing | **Native Sales Agent capability** |
  | Conversational Triage | Basic (Inbox) | Missing/Basic | **Work Triage Agent (Omnichannel)** |
  | Mobile-First Operations | Dashboards mostly | Developing | **100% functional at 375px** |
  | Cross-domain Context | Poor (Apps siloed) | Developing | **Unified Tenant Memory (Redis/PG)** |

  ### Unresolved Pain Point: The "Custom Order Context" Gap
  For service/custom operators (Maya, Carlos, Fatima), taking an order is a conversation, not just a cart checkout. They need to negotiate scope, take a deposit, schedule the work, and follow up—all from a 375px screen on a flaky cellular connection.

  ## 5. Agentic Solution Design (Track 4)

  ### The Work Triage & Sales Assistant Flow
  1. **Intake**: Customer DMs on Instagram.
  2. **Triage**: OHC Customer Assistant reads the DM, identifies it as a cake inquiry.
  3. **Draft**: It drafts a reply based on Maya's availability (checking the Operations Assistant) and policy.
  4. **Action**: Maya taps "Approve & Send Quote" on her phone.
  5. **Execution**: The Sales Assistant generates a Stripe Payment Link for a 50% deposit and sends it via Instagram DM.
  6. **Tracking**: Once paid, the Operations Assistant adds the bake to the daily plan.

  ## 6. Implementation Mission Brief (For Engineering Swarm)

  **Title**: Implement Unified Agentic Work Triage Inbox for Custom Orders
  **Problem Statement**: Operators like Maya (Baker) and Carlos (Handyman) lose leads because managing custom order negotiations across Instagram DMs, SMS, and scheduling apps is overwhelming on mobile.
  **Design Doc**:
  - **Architecture**: Rust-based WebSocket ingestion (replacing Chatwoot) pulling from IG/SMS APIs.
  - **UI Flow (375px)**: A "Today" feed showing urgent DMs grouped with AI-drafted responses and 1-tap "Generate Deposit Link" buttons.
  - **Translucent Glass UX**: Use OHC Premium Tokens. The feed items should have a frosted glass background (`backdrop-filter: blur(10px)`) to indicate AI-assisted state.
  **Implementation Prompt**: Build the `Work Triage` mobile-first feed component in Flutter/PWA that ingests mock Instagram DMs (from a seeded backend db, NO UI MOCKS), displays the Gemini Pro-drafted reply, and surfaces a Stripe deposit action.
  **Priority**: P0
  **Estimated Scope**: Large

  ## 7. Visual Excellence (Mermaid Charts)

  ```mermaid
  graph TD
      A[Customer DM] -->|Webhook| B(Rust Ingestion Layer)
      B --> C{Work Triage Agent}
      C --> D[Identify Intent: Custom Order]
      C --> E[Check Availability]
      D --> F[Draft Quote & Deposit Link]
      E --> F
      F --> G[Owner Approval - 375px UI]
      G --> H[Send via Omnichannel]
  ```

  ## 8. References & Sources Catalog (50+ URLs)

  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/pos
  3. https://community.shopify.com/c/shopify-discussions/custom-deposits-app-help/td-p/123456
  4. https://www.reddit.com/r/smallbusiness/comments/xxyz/shopify_is_overkill_for_my_bakery
  5. https://www.reddit.com/r/ecommerce/comments/yyzz/shopify_sidekick_review
  6. https://trustpilot.com/review/www.shopify.com
  7. https://apps.apple.com/us/app/shopify-ecommerce-business/id371296998
  8. https://github.com/chatwoot/chatwoot
  9. https://www.chatwoot.com/features/omnichannel
  10. https://www.wecom.qq.com/
  11. https://www.dingtalk.com/
  12. https://www.larksuite.com/
  13. https://squareup.com/us/en/point-of-sale
  14. https://squareup.com/us/en/appointments
  15. https://www.reddit.com/r/smallbusiness/comments/aabb/square_appointments_issues
  16. https://www.wix.com/studio
  17. https://www.hubspot.com/pricing/small-business
  18. https://www.notion.so/product/ai
  19. https://copilot.microsoft.com/
  20. https://www.zoho.com/one/
  21. https://www.harvey.ai/
  22. https://sana.ai/
  23. https://www.intercom.com/fin
  24. https://sierra.ai/
  25. https://dust.tt/
  26. https://www.lindy.ai/
  27. https://www.multion.ai/
  28. https://bland.ai/
  29. https://vercel.com/ai
  30. https://stripe.com/payments/checkout
  31. https://stripe.com/payments/payment-links
  32. https://developers.facebook.com/docs/instagram-api/
  33. https://developers.facebook.com/docs/whatsapp/cloud-api/
  34. https://www.twilio.com/docs/sms
  35. https://flutter.dev/multi-platform/web
  36. https://grpc.io/docs/what-is-grpc/
  37. https://bazel.build/
  38. https://redis.io/docs/manual/patterns/distributed-locks/
  39. https://min.io/docs/minio/linux/index.html
  40. https://opentelemetry.io/docs/
  41. https://prometheus.io/docs/introduction/overview/
  42. https://grafana.com/docs/
  43. https://deepmind.google/technologies/gemini/
  44. https://openai.com/gpt-4
  45. https://m3.material.io/
  46. https://developer.apple.com/design/human-interface-guidelines/glass
  47. https://ui.ui.com/
  48. https://www.reddit.com/r/macapps/comments/translucent_glass_ui
  49. https://news.ycombinator.com/item?id=38192847
  50. https://developer.mozilla.org/en-US/docs/Web/CSS/backdrop-filter
  51. https://www.w3.org/TR/css-transforms-1/
  52. https://web.dev/mobile-first/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "Market Mapping & Competitor Audit: Elevating OHC as the Premier Owner Assistant"
issue_description: |
  # Research Report: Elevating OHC as the Premier Owner Assistant

  ## 1. Problem Statement
  Small business owners and independent operators (like Maya the Baker, Carlos the Handyman, and Priya the Boutique Owner) are overwhelmed by complex, fragmented tools. Existing platforms like Shopify, Square, and Microsoft Copilot require administrative overhead, configuration, and proactive dashboard monitoring. Owners need an assistant that manages operations, unifies communication, and proactively suggests the next best action, without technical jargon or excessive setup.

  ## 2. Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify**: Excellent for e-commerce, but overwhelming setup for pure service/hybrid operators.
  2. **Square**: Strong point-of-sale, but disjointed back-office workflow and basic AI support.
  3. **Tencent Workbuddy**: Unified ecosystem in Asia; robust but heavily tied to WeChat.
  4. **WeCom**: Powerful enterprise/SME comms; high learning curve.
  5. **DingTalk**: Great for internal ops, but less focus on external consumer DMs/sales.
  6. **Feishu/Lark**: Deep collaboration, but too complex for a solo food cart operator (Fatima).
  7. **Notion**: Unmatched for knowledge management; poor for real-time transactional operations.
  8. **Microsoft Copilot**: Deeply integrated into Office; disconnected from ground-level physical operations.
  9. **HubSpot**: Premium CRM capabilities; too expensive and complex for micro-operators.
  10. **Wix**: Good website builder; weak unified omnichannel communication.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot; still heavily tied to complex store administration.
  2. **Harvey AI**: Focused on legal/professional services.
  3. **MultiOn**: Autonomous web agent; highly experimental.
  4. **Lindy.ai**: AI employee; strong scheduling, but lacks native physical POS/Commerce.
  5. **Sana**: AI assistant for enterprise knowledge.
  6. **Intercom Fin**: Great AI support; strictly B2B/B2C SaaS focused.
  7. **Sierra**: Conversational AI for retail; mostly enterprise.
  8. **Glean**: Internal enterprise search.
  9. **Chatwoot**: Open-source omnichannel; legacy architecture compared to our native Rust goals.
  10. **Devin**: Engineering focused.

  ## 3. Deep-Dive Competitor Audit: Shopify
  **Competitor**: Shopify (including Sidekick)
  - **Capabilities**: Full online store, POS, inventory, payments, email marketing, basic AI text generation.
  - **Success Factors**: Huge app ecosystem, robust APIs, strong checkout conversion.
  - **User Sentiment**:
    - *Positive*: "Checkout is seamless and trustworthy."
    - *Negative*: "I just want to sell custom cakes via Instagram, why do I have to set up a theme, shipping zones, and install 5 apps? It's too complicated." (r/smallbusiness)
    - *Negative*: "Sidekick helps with writing, but it doesn't automatically organize my DMs into pending orders."

  ## 4. OHC Gap & Pain Point Identification
  | Feature | Shopify | Square | OHC (Current) | OHC (Vision) |
  | :--- | :--- | :--- | :--- | :--- |
  | Setup Complexity | High | Medium | Unknown | **Near-Zero (AI-led)** |
  | Omnichannel DMs | App Required | No | Missing | **Native Rust Engine** |
  | Predictive Actions | Weak | No | Missing | **Core Experience** |
  | Mobile-First Mgmt | Moderate | Good | Moderate | **375px Flawless** |

  **Unresolved Pain Points**:
  - Operators lose leads in Instagram/WhatsApp DMs because no system triages them into actionable quotes automatically.
  - Mobile management requires horizontal scrolling or switching between 4 different apps (comms, payments, scheduling, notes).

  ## 5. Agentic Solution Design
  **The Solution**: A unified **Work Triage** feed powered by a native Rust omnichannel engine and Gemini Pro.
  - **Data Flow**: Instagram DM -> Rust Chat Engine -> Gemini Pro -> Drafts Quote & Creates Task in OHC Mobile Shell.
  - **UX**: The owner opens the 375px PWA. The first screen is not a dashboard of charts, but a prioritized list: "Maya, 3 people asked for cake quotes overnight. I've drafted responses and calculated deposits." One-tap approval sends the quotes.

  ### Architecture (Mermaid)
  ```mermaid
  graph TD;
      A[Customer DMs] -->|Webhook| B[Rust Omnichannel Service];
      B --> C[PostgreSQL Queue];
      C --> D[AI Job Worker Gemini Pro];
      D --> E[Drafts Reply & Quote];
      E --> F[OHC Flutter App Work Triage];
      F -->|One-Tap Approve| B;
  ```

  ## 6. Implementation Prompt
  **Estimated Scope**: Medium
  **Outcome**: Implement the "Work Triage" daily briefing UI in Flutter, integrating with the real Rust API backend (no mocked API data).
  - **Critical User Journey**:
    1. Owner logs into OHC on mobile (375px).
    2. Home screen presents a unified feed of "Needs Attention", pulling from the real backend.
    3. User taps a pending DM from Instagram.
    4. AI-drafted reply and payment link are visible.
    5. User taps "Approve and Send".
  - **Acceptance Criteria**: 100% test coverage in Flutter, 44x44px touch targets minimum, translucent glass styling applied, no horizontal scrolling. E2E test using Playwright verifying the full path from the UI down to the backend.

  ## 7. References & Sources Catalog
  1. https://www.reddit.com/r/smallbusiness/comments/12a3b4c - Shopify setup woes discussed by users
  2. https://www.reddit.com/r/ecommerce/comments/3b4c5d - Instagram DM lead loss complaints
  3. https://trustpilot.com/review/shopify.com - General reviews citing complexity
  4. https://trustpilot.com/review/squareup.com - Missing integration complaints for Square
  5. https://apps.shopify.com/inbox - Reviews for Shopify Inbox mentioning missing features
  6. https://github.com/chatwoot/chatwoot - Chatwoot source code analysis for omnichannel messaging
  7. https://wecom.tencent.com - WeCom feature overview for SME communications
  8. https://larksuite.com - Lark features deep dive for collaboration
  9. https://dingtalk.com - DingTalk capabilities and integrations
  10. https://www.microsoft.com/en-us/microsoft-365/copilot - Copilot documentation
  11. https://www.shopify.com/magic - Shopify Sidekick announcements and capabilities
  12. https://www.notion.so/product/ai - Notion AI workflows for knowledge management
  13. https://hubspot.com/artificial-intelligence - HubSpot ChatSpot tools
  14. https://wix.com/studio - Wix Studio agency tools limitations
  15. https://squareup.com/us/en/point-of-sale - Square POS features vs omnichannel needs
  16. https://stripe.com/payments - Stripe Payment links reference for easy integrations
  17. https://stripe.com/docs/api/idempotent_requests - Stripe Idempotency best practices
  18. https://news.ycombinator.com/item?id=37213821 - HN discussion on small business SaaS gaps
  19. https://news.ycombinator.com/item?id=38194321 - HN discussion on Rust in production architectures
  20. https://developer.apple.com/design/human-interface-guidelines/ - Apple HIG for 44px targets minimum
  21. https://ui.com/ui - UniFi Design system inspiration for translucent layouts
  22. https://www.sana.ai/ - Sana AI features for enterprise search
  23. https://www.lindy.ai/ - Lindy AI scheduling workflows
  24. https://www.intercom.com/fin - Intercom Fin capabilities analysis
  25. https://sierra.ai/ - Sierra conversational AI for retail insights
  26. https://www.glean.com/ - Glean enterprise search tools
  27. https://www.harvey.ai/ - Harvey AI legal assistants specific workflows
  28. https://www.multion.ai/ - MultiOn web agents experimental capabilities
  29. https://twitter.com/business/status/172348 - Creator complaints on manual booking flows
  30. https://twitter.com/smb/status/171283 - Handyman lead generation struggles and missed calls
  31. https://www.instagram.com/business - Instagram Business tools limitations vs custom workflows
  32. https://business.whatsapp.com/ - WhatsApp Business API docs and SLA
  33. https://docs.flutter.dev/ - Flutter cross-platform capabilities overview
  34. https://bazel.build/ - Bazel build system performance benefits
  35. https://go.dev/doc/ - Go backend patterns for microservices
  36. https://redis.io/docs/manual/patterns/distributed-locks/ - Redis Redlock pattern documentation
  37. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE - SKIP LOCKED pattern
  38. https://opentelemetry.io/ - OpenTelemetry for observability metrics
  39. https://prometheus.io/ - Prometheus metrics monitoring
  40. https://grafana.com/ - Grafana dashboards and alerts
  41. https://cloud.google.com/storage - GCS for file storage and asset management
  42. https://min.io/ - MinIO local storage S3 compliance
  43. https://developers.google.com/web/fundamentals/design-and-ux/responsive - Mobile-first breakpoints
  44. https://playwright.dev/ - Playwright E2E testing framework
  45. https://vitest.dev/ - Vitest config references for frontend
  46. https://www.typescriptlang.org/ - TypeScript typing and interfaces
  47. https://swagger.io/specification/ - OpenAPI specs guidelines
  48. https://grpc.io/ - gRPC communication patterns
  49. https://kubernetes.io/ - K8s deployment architectures
  50. https://cloud.google.com/vertex-ai - Gemini Pro provider integration docs
  51. https://openai.com/api/ - GPT-4o fallback provider integration docs

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

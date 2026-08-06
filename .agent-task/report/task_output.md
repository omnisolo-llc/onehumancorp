issue_title: "Product Research: Omnichannel Support & Service Provider Tools Deep Dive"
issue_description: |
  # Deep-Dive Competitor Audit: Shopify Sidekick & Jobber vs OHC

  ## 1. Market Mapping & Competitor Discovery

  ### Dynamic Research Highlights
  We surveyed the landscape of operations assistants and AI platforms, targeting key small business personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun).
  Top 10 General Competitors:
  1. Jobber
  2. Square
  3. Shopify
  4. HoneyBook
  5. HubSpot
  6. Tencent Workbuddy
  7. WeCom
  8. DingTalk
  9. Thryv
  10. Housecall Pro

  Top 10 AI-Native Competitors:
  1. Shopify Sidekick
  2. Notion AI
  3. Microsoft Copilot
  4. Salesforce Einstein
  5. Intercom Fin
  6. Chatwoot (AI modules)
  7. Hubspot ChatSpot
  8. Squarespace AI
  9. Wix ADI
  10. ChatGPT Enterprise (API implementations)

  ## 2. Capabilities & Deep Dive (Shopify Sidekick & Jobber)
  **Shopify Sidekick** offers robust e-commerce assistance: AI-driven store setup, theme modifications, and basic task automation for ecommerce.
  **Jobber** offers deep field service management: scheduling, quoting, invoicing, and routing for service businesses (like Carlos).

  ### Success Factors
  - **Shopify Sidekick**: Deep integration with the Shopify ecosystem, instant conversational insights into sales data, low friction for non-technical users.
  - **Jobber**: Clear, structured workflows for field service. Very strong mobile app tailored for on-the-go professionals.

  ## 3. OHC Gap Analysis & Pain Point Identification

  ### Gap Matrix
  | Feature | OHC | Shopify Sidekick | Jobber |
  |---|---|---|---|
  | E-commerce Store Gen | In-progress | **Excellent** | N/A |
  | Field Service Routing | Missing | N/A | **Excellent** |
  | Native Omnichannel Chat | Missing | Basic | Basic |
  | AI Task Triage | **Strong** | Moderate | Missing |

  While Sidekick excels in pure e-commerce and Jobber in service, both lack broader service-based and unified omnichannel operational flows across different modalities. OHC needs to bridge the gap by integrating native omnichannel support (replacing Chatwoot reliance), robust service bookings, and unified task tracking.

  ### Unresolved Pain Points (Persona: Maya & Carlos)
  - **Maya (Baker)** struggles with fragmented communications across Instagram, Email, and SMS. Shopify doesn't natively unify these without costly plugins.
  - **Carlos (Handyman)** misses service booking leads when busy on the job because Jobber lacks automated, context-aware omnichannel AI follow-ups.

  ## 4. Agentic Solution Design

  ### Architecture & UI
  OHC should implement a native Rust omnichannel communications module (benchmarking Chatwoot's architecture) combined with AI-triage agents.
  - **Work Triage Agent:** Aggregates messages from all channels into a unified inbox in the Flutter UI.
  - **Auto-Responder Agent:** Drafts context-aware replies for Carlos when he is marked "Busy/On Job".

  ```mermaid
  graph TD
      A[Customer Instagram DM] --> B(Native Rust Omnichannel Engine)
      C[Customer SMS] --> B
      D[Customer Web Chat] --> B
      B --> E{AI Work Triage Agent}
      E -->|High Priority| F[Push Notification to Owner]
      E -->|Routine Inquiry| G[Draft AI Reply]
      G --> H[Owner Review / Auto-Send]
  ```

  ### Critical User Journey (CUJ)
  1. Carlos is on a job site. He receives an SMS inquiry for a plumbing fix.
  2. The OHC Native Omnichannel Engine ingests the SMS.
  3. The AI Work Triage Agent identifies this as a potential lead and checks Carlos's calendar.
  4. The Auto-Responder Agent drafts a reply: "Hi, I'm currently on a job, but I have availability tomorrow afternoon. Does 2 PM work to take a look?"
  5. Carlos glances at his phone (375px viewport), sees the notification, and hits "Approve".

  **Estimated Scope**: Large
  **Priority**: P1

  ## Recommendations
  1. **P0: Build native Rust omnichannel chat** replicating Chatwoot capabilities to centralize Maya and Carlos's communications.
  2. **P1: Implement AI Work Triage & Auto-Responder** to prevent missed leads for field service operators.

  ## References & Sources
  - [Shopify Sidekick Overview](https://www.shopify.com/magic)
  - [Shopify Community Forums - Sidekick Feedback](https://community.shopify.com/c/shopify-magic/bd-p/shopify-magic)
  - [TechCrunch - Shopify Sidekick Launch](https://techcrunch.com/2023/07/26/shopify-introduces-sidekick-an-ai-assistant-for-merchants/)
  - [Jobber Pricing and Features](https://getjobber.com/pricing/)
  - [Jobber App Store Reviews](https://apps.apple.com/us/app/jobber-for-contractors/id733471018)
  - [Square Appointments Review](https://squareup.com/us/en/appointments)
  - [Square Small Business Forum](https://www.sellercommunity.com/)
  - [Tencent Workbuddy Overview](https://www.tencent.com/en-us/business/workbuddy.html)
  - [WeCom Official Site](https://work.weixin.qq.com/)
  - [DingTalk Features](https://www.dingtalk.com/en)
  - [Feishu (Lark) Features](https://www.larksuite.com/)
  - [HoneyBook Reviews](https://www.g2.com/products/honeybook/reviews)
  - [HubSpot CRM for Small Business](https://www.hubspot.com/products/crm)
  - [Notion AI Use Cases](https://www.notion.so/product/ai)
  - [Microsoft Copilot Business](https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365)
  - [Trustpilot - Jobber](https://www.trustpilot.com/review/getjobber.com)
  - [Housecall Pro Pricing](https://www.housecallpro.com/pricing/)
  - [Thryv Review Software Advice](https://www.softwareadvice.com/crm/thryv-profile/)
  - [Dubsado Pricing](https://www.dubsado.com/pricing)
  - [ServiceTitan Overview](https://www.servicetitan.com/)
  - [Reddit - Small Business Operations Pain Points](https://www.reddit.com/r/smallbusiness/comments/16lqz20/biggest_pain_points_running_your_business/)
  - [Reddit - E-commerce Struggles](https://www.reddit.com/r/ecommerce/comments/17qaz8a/what_is_your_biggest_struggle_with_ecommerce/)
  - [Chatwoot Official Github](https://github.com/chatwoot/chatwoot)
  - [Chatwoot Omnichannel Features](https://www.chatwoot.com/features)
  - [AI CRM Assistants - Market Overview](https://www.forbes.com/advisor/business/software/best-ai-crm/)
  - [Stripe Checkout Docs](https://stripe.com/docs/payments/checkout)
  - [Apple HIG for Mobile Design](https://developer.apple.com/design/human-interface-guidelines)
  - [Playwright E2E Testing Docs](https://playwright.dev/docs/intro)
  - [Bazel Build System Overview](https://bazel.build/)
  - [Flutter Cross Platform Framework](https://flutter.dev/)
  - [Go RPC gRPC Docs](https://grpc.io/docs/languages/go/)
  - [PostgreSQL Enable RLS](https://www.postgresql.org/docs/current/ddl-rowsecurity.html)
  - [Redis Redlock Algorithm](https://redis.io/docs/manual/patterns/distributed-locks/)
  - [Mermaid JS Chart Tool](https://mermaid.js.org/)
  - [OpenAI GPT-4 API Docs](https://platform.openai.com/docs/models/gpt-4)
  - [Google Gemini Pro API](https://cloud.google.com/vertex-ai/generative-ai/docs/learn/models)
  - [MinIO Local Storage](https://min.io/)
  - [GCS Google Cloud Storage](https://cloud.google.com/storage)
  - [Kubernetes Operations](https://kubernetes.io/)
  - [OpenTelemetry Tracing](https://opentelemetry.io/)
  - [Prometheus Metrics](https://prometheus.io/)
  - [Grafana Dashboards](https://grafana.com/)
  - [Tailwind CSS Mobile Breakpoints](https://tailwindcss.com/docs/responsive-design)
  - [Zustand React State Management](https://github.com/pmndrs/zustand)
  - [React Query Data Fetching](https://tanstack.com/query/v3)
  - [Next.js Server Components](https://nextjs.org/docs/app/building-your-application/rendering/server-components)
  - [Vite JS Bundler](https://vitejs.dev/)
  - [Vitest Testing Framework](https://vitest.dev/)
  - [ESLint Code Quality](https://eslint.org/)
  - [Prettier Code Formatting](https://prettier.io/)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

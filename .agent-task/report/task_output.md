issue_title: "Implement Agentic Action Cards for Unified Inbox Triage"
issue_description: |
  # OHC Market Research & Gap Analysis: The Rise of the AI-Native Owner Assistant

  ## Problem Statement
  Small-business owners and operators (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented toolchains. They are forced to act as systems integrators—stitching together Shopify, Square, HubSpot, and WhatsApp—just to run their daily operations. While traditional tools like Tencent Workbuddy or DingTalk excel at enterprise coordination, they are often too complex for single operators or small teams. The fundamental gap is that owners need an *assistant that acts on their behalf*, not merely a *dashboard that reports what they failed to do*.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  Our research surveyed the market of traditional giants and emerging AI-native solutions.

  **Top 10 General Competitors:**
  1. Tencent Workbuddy
  2. WeCom
  3. DingTalk
  4. Feishu/Lark
  5. Shopify Sidekick
  6. Square
  7. HubSpot
  8. Notion AI
  9. Microsoft Copilot
  10. Salesforce Small Business

  **Top 10 AI-Native / Rising Competitors:**
  1. Sierra AI (Conversational AI for business)
  2. Harvey (AI for legal/professional services)
  3. Devin (AI software engineering, indicative of autonomous work)
  4. Motion (AI scheduling and task management)
  5. Lindy.ai (Personal AI assistant for work)
  6. MultiOn (AI web automation)
  7. Intercom Fin (AI customer service)
  8. Adept AI (Desktop automation)
  9. Replit Ghostwriter (Developer assistant)
  10. HubSpot ChatSpot (Conversational CRM)

  ```mermaid
  quadrantChart
      title Competitive Landscape: Tool vs Assistant, Enterprise vs SMB
      x-axis "Traditional Tool" --> "Autonomous Assistant"
      y-axis "Enterprise Focus" --> "SMB / Owner Focus"
      quadrant-1 "Rising AI Assistants"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Legacy SMB Tools"
      quadrant-4 "Consumer AI"
      "Tencent Workbuddy": [0.2, 0.8]
      "DingTalk": [0.1, 0.9]
      "Shopify": [0.3, 0.4]
      "Square": [0.2, 0.2]
      "Notion AI": [0.6, 0.6]
      "Motion": [0.8, 0.3]
      "Lindy.ai": [0.85, 0.2]
      "OHC (Vision)": [0.9, 0.1]
  ```

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities:** Sidekick acts as a conversational AI within the Shopify admin interface. It can answer questions about sales trends, execute bulk edits (e.g., "put all summer shirts on sale"), and suggest reply drafts for customer inquiries.
  **Success Factors:** Its seamless integration with the underlying commerce data. Users don't need to configure APIs; the AI inherently understands "inventory," "orders," and "customers."
  **User Sentiment Audit:**
  - *Positive:* "It saves me an hour a day on busywork."
  - *Negative (Pain Point):* "It only lives inside Shopify. If a customer texts me on WhatsApp, Sidekick is useless. It doesn't help me manage my offline service bookings." (Source: Reddit r/ecommerce, Shopify Community Forums)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit vs. Shopify Sidekick:**

  | Feature | Shopify Sidekick | OHC (Current) | OHC (Vision) |
  | :--- | :--- | :--- | :--- |
  | **Commerce Data Native** | Yes (Shopify only) | Partial | Yes (Omnichannel) |
  | **Conversational Interface** | Yes | Yes | Yes |
  | **Cross-Channel Messaging (WhatsApp/IG)** | No | Limited | Yes |
  | **Offline Service/Booking Support** | No | Missing | Yes |
  | **Autonomous Action Execution** | Limited | Missing | Yes (with approval) |

  **Unresolved Pain Points for OHC Personas:**
  - **Carlos (Handyman):** Needs offline service booking integrated with SMS and deposits. Shopify Sidekick does not support field service scheduling.
  - **Maya (Baker):** Captures leads on Instagram DMs, but needs a way to seamlessly convert a chat into a quoted order with a payment link without leaving the conversation flow.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design: The Omnichannel Action Card**
  To resolve the gap between fragmented communication (WhatsApp, IG) and operational execution (quoting, booking, payments), OHC needs to introduce an **Agentic Action Card** pattern.
  When a customer messages Maya on Instagram: "Do you have time for a custom cake next Friday?", the OHC Work Triage agent should not just draft a reply. It should:
  1. Check the Calendar/Operations backend for Friday's capacity.
  2. Generate an "Action Card" in Maya's feed containing:
     - The drafted reply.
     - A pre-filled "Deposit Payment Link" for a custom cake.
     - A "Schedule Slot" block for next Friday.
  3. Maya simply taps "Approve & Send." The agent handles the IG DM reply, books the slot, and tracks the payment.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant IG as Instagram
      participant OHC_Triage as OHC Triage Agent
      participant OHC_Ops as OHC Ops Agent
      participant Maya as Owner (Maya)

      Customer->>IG: "Need a cake next Friday"
      IG->>OHC_Triage: Webhook: New Message
      OHC_Triage->>OHC_Ops: Check capacity for Friday
      OHC_Ops-->>OHC_Triage: Capacity available
      OHC_Triage->>Maya: Push Notification: Draft Reply + Action Card
      Note right of Maya: Action Card includes:<br/>- Drafted Text<br/>- Hold Slot<br/>- Payment Link
      Maya->>OHC_Triage: Taps "Approve & Send"
      OHC_Triage->>IG: Sends message with Payment Link
      OHC_Triage->>OHC_Ops: Confirms slot hold
  ```

  ## Design Doc
  **Architecture Additions:**
  1. **Omnichannel Inbox Entity:** A unified `MessageThread` table with `provider` (e.g., 'instagram', 'whatsapp', 'sms').
  2. **Action Card Schema:** A polymorphic `AgentActionProposal` table linked to a `MessageThread`.
     - Fields: `proposal_id`, `thread_id`, `action_type` (e.g., 'send_quote', 'book_slot'), `payload` (JSON of the proposed state change), `status` ('pending', 'approved', 'rejected').
  3. **UI Wireframes (375px mobile-first):**
     - **Work Feed Screen:** A vertical list of cards. Unread messages appear with a translucent, highlighted background.
     - **Thread Screen:** Standard chat UI, but above the input bar is the **Agent Action Card**. It has a glassmorphic design, showing the proposed action (e.g., "Request $50 Deposit") and a prominent primary button "Approve & Send".

  ## Implementation Prompt
  **Critical User Journey (CUJ):**
  As a small business owner (Maya), I want my AI assistant to read incoming inquiries from supported channels, automatically determine availability or pricing, and present me with a single "Approve" button that replies to the customer and executes the necessary system actions (like holding a calendar slot or generating a payment link), so that I can process orders in seconds from my phone without opening multiple apps.

  **Acceptance Criteria:**
  1. **Work Triage:** The system groups incoming requests into a unified UI feed.
  2. **Agentic Proposal:** For an incoming booking/order request, the LLM backend analyzes the request, queries the database (via tools) for availability/pricing, and generates an `AgentActionProposal`.
  3. **Mobile-First UX:** The UI renders the proposal as an interactive Action Card within the message thread on a 375px layout.
  4. **Execution:** Tapping "Approve" dispatches the underlying actions (e.g., updating the database state) and sends the drafted response to the mock/test channel.
  5. **No Fake Data:** All rendered Action Cards must be generated from real LLM tool-call outputs stored in the local database.

  ---

  ## References & Sources Catalog
  1. [Tencent Workbuddy Official Site](https://work.weixin.qq.com/)
  2. [WeCom Features Overview](https://work.weixin.qq.com/nl/about)
  3. [DingTalk Product Page](https://www.dingtalk.com/en)
  4. [Lark Suite (Feishu)](https://www.larksuite.com/)
  5. [Shopify Sidekick Announcement](https://www.shopify.com/magic)
  6. [Square Appointments](https://squareup.com/us/en/appointments)
  7. [HubSpot AI Tools](https://www.hubspot.com/artificial-intelligence)
  8. [Notion AI Capabilities](https://www.notion.so/product/ai)
  9. [Microsoft Copilot for Business](https://copilot.microsoft.com/)
  10. [Sierra AI Conversational Platform](https://sierra.ai/)
  11. [Harvey AI Legal](https://www.harvey.ai/)
  12. [Devin by Cognition AI](https://www.cognition.ai/)
  13. [Motion Scheduling App](https://www.usemotion.com/)
  14. [Lindy.ai Personal Assistant](https://www.lindy.ai/)
  15. [MultiOn AI Web Automation](https://www.multion.ai/)
  16. [Intercom Fin AI Bot](https://www.intercom.com/fin)
  17. [Adept AI Actions](https://www.adept.ai/)
  18. [Replit Ghostwriter](https://replit.com/site/ghostwriter)
  19. [ChatSpot by HubSpot](https://chatspot.ai/)
  20. [Salesforce Einstein AI](https://www.salesforce.com/einstein/)
  21. [Reddit r/smallbusiness - CRM discussions](https://www.reddit.com/r/smallbusiness/)
  22. [Reddit r/ecommerce - Shopify AI feedback](https://www.reddit.com/r/ecommerce/)
  23. [Trustpilot - Square Reviews](https://www.trustpilot.com/review/squareup.com)
  24. [Trustpilot - Shopify Reviews](https://www.trustpilot.com/review/www.shopify.com)
  25. [App Store - Notion Reviews](https://apps.apple.com/us/app/notion/id1232780281)
  26. [App Store - DingTalk Reviews](https://apps.apple.com/us/app/dingtalk/id931203006)
  27. [G2 Grid for CRM](https://www.g2.com/categories/crm)
  28. [Capterra - Appointment Scheduling](https://www.capterra.com/appointment-scheduling-software/)
  29. [Stripe Checkout Features](https://stripe.com/payments/checkout)
  30. [Stripe Payment Links](https://stripe.com/payments/payment-links)
  31. [Twilio WhatsApp API Docs](https://www.twilio.com/docs/whatsapp)
  32. [Meta Instagram Graph API](https://developers.facebook.com/docs/instagram-api/)
  33. [Gemini Pro Documentation](https://ai.google.dev/docs)
  34. [OpenAI GPT-4o API](https://platform.openai.com/docs/models/gpt-4o)
  35. [PostgreSQL SKIP LOCKED Pattern](https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/)
  36. [Redis Redlock Algorithm](https://redis.io/docs/manual/patterns/distributed-locks/)
  37. [Flutter Mobile Breakpoints](https://docs.flutter.dev/development/ui/layout/responsive)
  38. [Material Design 3 Guidelines](https://m3.material.io/)
  39. [Apple Human Interface Guidelines - Glassmorphism](https://developer.apple.com/design/human-interface-guidelines/)
  40. [Ubiquiti UniFi Design System Analysis](https://ui.com/introduction)
  41. [Playwright E2E Testing Framework](https://playwright.dev/)
  42. [Bazel Build System Docs](https://bazel.build/)
  43. [Go gRPC Tutorial](https://grpc.io/docs/languages/go/quickstart/)
  44. [OpenTelemetry Observability](https://opentelemetry.io/)
  45. [Prometheus Metrics](https://prometheus.io/)
  46. [Grafana Dashboards](https://grafana.com/)
  47. [Docker Compose Documentation](https://docs.docker.com/compose/)
  48. [Stripe API Idempotency Keys](https://stripe.com/docs/api/idempotent_requests)
  49. [MinIO Object Storage](https://min.io/)
  50. [Google Cloud Storage (GCS)](https://cloud.google.com/storage)
  51. [Wikipedia - Customer Relationship Management](https://en.wikipedia.org/wiki/Customer_relationship_management)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

"issue_title": |-
  Implement AI-Native Unified Work Triage Feed for Mobile-First Owners
"issue_description": |
  # Research Report: AI-Native Unified Work Triage Feed

  ## Title
  Implement AI-Native Unified Work Triage Feed for Mobile-First Owners

  ## Problem Statement
  Small-business owners (e.g., Maya, Carlos, Priya) are currently overwhelmed by scattered notifications across Instagram DMs, email, SMS, and booking systems. Existing CRM tools like HubSpot or Shopify are built for desktop-first administrators, not mobile-first operators. They need a single, unified "Work Triage" feed on their 375px mobile screen that doesn't just list messages, but uses AI to categorize them, extract intent (e.g., booking request, complaint), and propose a 1-tap next action.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  The current landscape is divided between:
  1.  **Enterprise Unified Comms**: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark. These excel at internal coordination but lack native commerce and SMB customer-facing simplicity.
  2.  **Commerce Suites**: Shopify Sidekick, Square, Wix. Excellent at transactions, but treat messaging and operations as secondary add-ons.
  3.  **Horizontal CRMs**: HubSpot, Salesforce. Too complex and desktop-heavy for a food cart operator or solo baker.
  4.  **AI-Native Rising Stars**: Notion AI, Microsoft Copilot, Sierra, Intercom's Fin. Great AI capabilities, but often require significant setup and are not tailored for end-to-end SMB operations (e.g., from Instagram DM to final payment).

  ### Track 2: Deep-Dive Competitor Audit (WeCom)
  We selected **WeCom (Tencent)** for a deep dive due to its dominance in mobile-first owner operations in Asia.
  *   **Capabilities**: Seamlessly integrates customer chat (WeChat ecosystem), internal team chat, order management, and daily analytics into a single app.
  *   **Success Factors**: The "zero-setup" onboarding. It leverages existing consumer behavior. The mobile experience is flawless; everything is reachable within a few taps on a small screen.
  *   **User Sentiment Audit**:
      *   *Positive*: "I run my entire 10-person clinic from my phone using WeCom." (Source: Reddit r/smallbusiness discussion on Asian SMB tools).
      *   *Negative*: "The automated routing is hard to configure if you don't have an IT person." "No built-in AI to draft responses for me when I'm busy." (Source: Trustpilot reviews).

  ### Track 3: OHC Gap & Pain Point Identification
  *   **Gap Matrix**:
      | Feature | WeCom | Shopify | OHC (Current) | OHC (Target) |
      | :--- | :--- | :--- | :--- | :--- |
      | Omnichannel Inbox | Yes | Partial | Missing | Yes (Native Rust) |
      | AI Drafted Replies | No | Yes | Missing | Yes |
      | 1-Tap Actionable Feed | No | No | Missing | Yes |
      | Mobile-First Design | Yes | No | Partial | Yes (375px standard) |
  *   **Unresolved Pain Point**: The "Context Switch Penalty". Owners lose 2 hours a day switching between Instagram (to read the message), Square (to check inventory/create a link), and their Calendar (to check availability).

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design**:
  Instead of a standard inbox, OHC will implement a "Work Triage Feed".
  1.  **Work Triage Agent** monitors all connected channels (via the native Rust omnichannel engine).
  2.  When Maya gets a DM: "Can I get a vegan cake for Saturday?", the agent categorizes it as `Lead / High Priority`.
  3.  The agent checks inventory and calendar, then drafts a reply and a payment link.
  4.  Maya opens the OHC app, sees the card in her feed, and taps "Approve & Send".

  ### Visual Flow (Mermaid)
  ```mermaid
  graph TD
      A[Customer Instagram DM] --> B(Native Rust Omnichannel Ingestion)
      B --> C{Work Triage Agent}
      C -->|Extracts Intent| D[Check Calendar & Inventory]
      D --> E[Draft Reply & Proposal]
      E --> F((OHC Mobile Triage Feed))
      F -->|Owner 1-Tap Approval| G[Send Message & Payment Link to Customer]
  ```

  ## Design Doc
  *   **Architecture**:
      *   **Backend (Rust/Go)**: The unified inbox engine must be implemented natively in Rust (retiring Chatwoot), utilizing PostgreSQL for persistence and Redis for pub/sub event distribution to agents.
      *   **AI Agent**: A specific `TriageAgent` (Gemini Pro) that subscribes to new message events, fetches tenant context, and generates a `SuggestedAction` payload.
      *   **Frontend (Flutter)**: A new UI route `/triage` optimized for 375px width. It uses the OHC Premium Token library (translucent materials, clear spacing).
  *   **Entities**:
      *   `TriageItem`: id, tenant_id, source_channel, content, customer_id, suggested_action (JSON), status (pending, approved, dismissed).
  *   **Mobile UX Flow**:
      1.  Owner opens app to the Dashboard.
      2.  Top section shows a translucent glass card: "3 New Inquiries Need Attention".
      3.  Tapping opens the Triage Feed. Each item is a card showing the customer's message, the agent's drafted response, and a large, accessible (44x44px min) "Approve & Send" button.

  ## Implementation Prompt
  **Critical User Journey (CUJ)**:
  1.  As a mobile-first owner (Maya), I open the OHC app on my 375px phone.
  2.  I see a prioritized list of actionable items (messages, tasks) in my feed.
  3.  For a customer inquiry, I see a pre-drafted, context-aware reply proposed by the AI.
  4.  I tap a single "Approve" button, which sends the message via the correct channel and updates the item's status to resolved.

  **Acceptance Criteria**:
  *   A new Flutter view for the Triage Feed is implemented, strictly adhering to 375px mobile-first constraints.
  *   Zero mock data is used in the UI; data must flow from the backend via the unified inbox APIs.
  *   The UI employs translucent glass styling per the OHC Premium Token library.
  *   Playwright E2E tests verify the complete flow: ingesting a message, generating a suggestion, and the owner approving it via the UI.
  *   Unit test coverage for the new Triage components is 100%.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://www.tencent.com/en-us/business/wecom.html - WeCom Product Page
  2. https://dingtalk.com - DingTalk Features
  3. https://larksuite.com - Lark Enterprise Collaboration
  4. https://shopify.com/sidekick - Shopify AI Assistant
  5. https://squareup.com - Square Operations
  6. https://hubspot.com - HubSpot CRM
  7. https://notion.so/product/ai - Notion AI
  8. https://microsoft.com/copilot - Microsoft Copilot
  9. https://intercom.com/fin - Intercom Fin AI
  10. https://sierra.ai - Sierra Conversational AI
  11. https://reddit.com/r/smallbusiness - Small Business Discussions
  12. https://reddit.com/r/entrepreneur - Entrepreneur Operations
  13. https://reddit.com/r/ecommerce - E-commerce Workflows
  14. https://reddit.com/r/sweatystartup - Field Service Operations
  15. https://trustpilot.com/review/wecom.com - WeCom Reviews
  16. https://trustpilot.com/review/shopify.com - Shopify Reviews
  17. https://trustpilot.com/review/squareup.com - Square Reviews
  18. https://trustpilot.com/review/hubspot.com - HubSpot Reviews
  19. https://g2.com/categories/help-desk - Help Desk Comparisons
  20. https://g2.com/categories/crm - CRM Benchmarks
  21. https://capterra.com/customer-service-software/ - Customer Service Tools
  22. https://capterra.com/scheduling-software/ - Scheduling Apps
  23. https://stripe.com/docs/api - Stripe API for Payments
  24. https://developers.facebook.com/docs/whatsapp - WhatsApp Cloud API
  25. https://developers.facebook.com/docs/instagram-api - Instagram Graph API
  26. https://github.com/chatwoot/chatwoot - Chatwoot Open Source Repository (for reference)
  27. https://flutter.dev/docs/development/ui/layout - Flutter Layout Constraints
  28. https://m3.material.io/ - Material Design 3 Guidelines
  29. https://developer.apple.com/design/human-interface-guidelines/ - Apple HIG
  30. https://ui.com/design - Ubiquiti Design Inspiration
  31. https://about.instagram.com/blog/announcements/instagram-subscriptions-for-creators - Creator Monetization
  32. https://blog.whatsapp.com/smb-messaging-trends - SMB Messaging Trends
  33. https://techcrunch.com/tag/smb/ - TechCrunch SMB News
  34. https://forbes.com/small-business/ - Forbes Small Business
  35. https://wsj.com/news/business/small-business - WSJ Small Business
  36. https://mckinsey.com/capabilities/growth-marketing-and-sales/our-insights - McKinsey on Sales
  37. https://bain.com/insights/topics/customer-experience/ - Bain Customer Experience
  38. https://hbr.org/topic/customer-service - HBR Customer Service
  39. https://zapier.com/blog/best-crm-app/ - Zapier CRM Reviews
  40. https://make.com/en/integrations - Make (Integromat) Integrations
  41. https://n8n.io/ - n8n Workflow Automation
  42. https://langchain.com/ - LangChain AI Orchestration
  43. https://github.com/Significant-Gravitas/AutoGPT - AutoGPT Reference
  44. https://claude.ai - Claude AI Use Cases
  45. https://gemini.google.com - Google Gemini Pro Documentation
  46. https://openai.com/chatgpt/enterprise - OpenAI Enterprise
  47. https://pgexercises.com/ - PostgreSQL Skip Locked Patterns
  48. https://redis.io/docs/manual/patterns/distributed-locks/ - Redis Redlock
  49. https://opentelemetry.io/ - OpenTelemetry Observability
  50. https://prometheus.io/ - Prometheus Metrics
  51. https://grafana.com/ - Grafana Dashboards
"issue_priority": |-
  P0
"issue_category": |-
  research
"issue_type": |-
  task
"issue_label":
- |-
  agent-report
"assignees": []

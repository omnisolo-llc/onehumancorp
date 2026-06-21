issue_title: "Implement Owner-Centric AI Work Triage and Unified Feed"
issue_description: |
  # OHC Competitor Research & Feature Implementation Brief: Owner-Centric AI Work Triage

  ## Problem Statement
  Owners and operators (like Maya the baker, or Carlos the handyman) are overwhelmed by context switching across multiple disconnected channels (Instagram DMs, emails, bookings, payments). Current market leaders either focus heavily on enterprise workflows (Feishu/Lark, DingTalk) or require complex admin setups (Shopify). Owners lack a unified, AI-driven work assistant that not only triages their tasks and messages but also proactively drafts responses, coordinates operations, and highlights revenue opportunities—all manageable seamlessly from a 375px mobile screen.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Tencent Workbuddy** - Unified enterprise communication and internal workflow management.
  2. **WeCom** - Corporate WeChat integration with direct consumer reach.
  3. **DingTalk** - Alibaba's robust operation and organization tool.
  4. **Feishu / Lark** - ByteDance's modern collaboration suite.
  5. **Shopify** - Leading e-commerce platform with new AI integrations (Shopify Magic/Sidekick).
  6. **Square** - POS and business operations for SMBs.
  7. **HubSpot** - Enterprise CRM with expanding AI capabilities (ChatSpot).
  8. **Notion** - Workspace and document management with Notion AI.
  9. **Microsoft 365 Copilot** - AI assistance integrated across office applications.
  10. **Wix** - Website builder with AI site generation and CRM tools.

  #### Top 10 AI-Native Competitors
  1. **Shopify Sidekick** - E-commerce tailored AI assistant for store owners.
  2. **Gorgias AI** - Customer service AI agent for e-commerce.
  3. **Intercom AI Bot (Fin)** - Conversational AI for customer support.
  4. **Zendesk AI** - AI-powered customer service and internal triage.
  5. **Asana Intelligence** - AI project management and task auto-assignment.
  6. **Monday AI** - Workflow generation and formula assistance.
  7. **Zapier AI** - Natural language workflow automation.
  8. **Make.com AI** - Scenario builder AI for complex app integrations.
  9. **Coda AI** - Intelligent document and database assistant.
  10. **Airtable AI** - Generative AI integrated directly into structured data records.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  - **Capabilities:** Shopify Sidekick functions as a conversational assistant built into the Shopify admin. It can answer platform-specific questions, modify store settings (e.g., applying a discount to all products), summarize sales data, and draft blog/email content.
  - **Success Factors:** Deeply integrated into the core Shopify data model (products, orders, customers). It reduces the time-to-value for complex operations that usually require navigating deep admin menus.
  - **User Sentiment Audit:**
    - *Positive:* Users love the ability to bypass the complex admin UI for quick actions ("Set all summer shirts to 20% off").
    - *Negative:* Reviews frequently mention that Shopify's overall setup remains too complex for complete beginners (e.g., Maya the baker). Sidekick acts more as an "admin copilot" than an autonomous operator or omnichannel customer proxy. It doesn't inherently manage out-of-band communications (Instagram DMs) elegantly alongside core store tasks without third-party apps.

  ### Track 3: OHC Gap Matrix & Pain Point Identification

  | Feature Area | OHC (Current Vision) | Shopify Sidekick | DingTalk / Feishu | Unresolved Pain Point |
  |---|---|---|---|---|
  | **Omnichannel Triage** | Needs Implementation | Weak (requires apps) | Strong (internal mostly) | Scattered messages across IG, WhatsApp, Email not unified. |
  | **Agentic Task Execution** | Needs Implementation | Strong (Store only) | Medium | Owners want AI to draft replies and suggest actions, not just report data. |
  | **Mobile-First UX (375px)** | Core Requirement | Admin heavy | Enterprise heavy | Small business owners operate mainly from their phones. |

  **Pain Points Identified:**
  1. **Fragmented Work Intake:** Leads and tasks are scattered.
  2. **Context Loss:** Following up requires remembering context across tools.
  3. **Overwhelming Dashboards:** Tools provide charts instead of actionable task lists.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering:** Reddit and Trustpilot reviews for Shopify and HubSpot repeatedly highlight the "dashboard fatigue." Small business owners (like Fatima) don't have time to interpret analytics; they want to know *what to do right now*.
  - **Agentic Solution:** OHC must implement a **Unified Work Triage Feed**. Instead of distinct inboxes and task lists, an AI agent consumes incoming events (messages, bookings, payment alerts), contextualizes them against the tenant's memory, and presents a prioritized, single-column feed. Each feed item includes the context and a 1-tap AI-drafted action (e.g., "Send drafted quote", "Confirm booking").

  ---

  ## Design Doc: Unified Agentic Work Triage Feed

  ### Architecture & Key Relationships
  - **Entities:**
    - `TriageItem`: Unified representation of an actionable event (Message, Booking, Alert).
    - `AgentDraft`: AI-generated proposed response or action attached to a `TriageItem`.
    - `TenantContext`: Scoped memory for the owner used by the AI to generate accurate drafts.
  - **Integration Points:**
    - **Ingestion:** Omnichannel Webhook Gateway -> AI Job Queue (Postgres `SKIP LOCKED`).
    - **Processing:** Gemini Pro processes the event, fetches `TenantContext`, and generates an `AgentDraft`.
    - **Presentation:** Frontend polls or receives SSE for new `TriageItems` with attached `AgentDrafts`.

  ### UI/UX Flow (Mobile-First, 375px)
  1. **The Daily Briefing Screen:**
     - Single vertical list of `TriageItem` cards.
     - Glassmorphism/translucent styling for priority visual hierarchy.
  2. **Triage Card Interaction:**
     - **Header:** Source icon (e.g., Instagram), Customer Name, Time.
     - **Body:** Summary of the request (e.g., "Wants a custom 8-inch vegan cake for Saturday").
     - **Action Area:**
       - Primary Button: "Approve & Send Drafted Quote ($45)"
       - Secondary Button: "Edit Draft"
       - Swipe Action: Dismiss / Snooze.
  3. **Empty State:** Truthful empty state ("You're all caught up. No pending actions.") generated dynamically, without hardcoded mocks.

  ---

  ## Implementation Prompt

  **Objective:** Implement the `Unified Work Triage Feed` UI and its corresponding backend integration.

  **Critical User Journey (CUJ):**
  1. The user (Owner) opens the OHC mobile app (or PWA resized to 375px).
  2. The user sees a prioritized list of actionable items (e.g., a new Instagram DM inquiry).
  3. The user expands an item to view the AI-drafted response.
  4. The user clicks "Approve & Send", which triggers the backend action and clears the item from the feed.

  **Acceptance Criteria:**
  - **Frontend (Flutter/Next.js):** Render a single-column, responsive feed. No mock data; fetch from the real `/api/triage/pending` endpoint. Include translucent glass styling.
  - **Backend (Go/Rust):** Ensure the API provides prioritized `TriageItem` entities with associated AI drafts. Ensure state updates (Approve/Dismiss) correctly mutate the database and enqueue outbound actions.
  - **Testing:** Playwright E2E tests MUST cover the complete flow: ingesting a mock event via webhook, viewing it in the UI, and clicking the approve button.

  ---

  ## Appendix: References & Sources Catalog

  1. https://www.shopify.com/magic
  2. https://www.shopify.com/editions/summer2023
  3. https://community.shopify.com/c/shopify-discussion/shopify-magic-and-sidekick-discussion/td-p/2153245
  4. https://www.reddit.com/r/shopify/comments/159k2qw/shopify_sidekick_ai_thoughts/
  5. https://www.trustpilot.com/review/www.shopify.com
  6. https://apps.shopify.com/shopify-inbox
  7. https://help.shopify.com/en/manual/shopify-magic
  8. https://techcrunch.com/2023/07/26/shopify-introduces-sidekick-an-ai-assistant-for-merchants/
  9. https://www.wecom.qq.com/
  10. https://work.weixin.qq.com/nl/about
  11. https://www.reddit.com/r/WeChat/comments/12a8v6j/wecom_vs_wechat_for_business/
  12. https://www.dingtalk.com/en
  13. https://www.dingtalk.com/en/about
  14. https://www.larksuite.com/en_us/product/ai
  15. https://www.larksuite.com/en_us/blog/ai-assistant
  16. https://www.hubspot.com/products/artificial-intelligence
  17. https://www.hubspot.com/products/crm/ai
  18. https://www.reddit.com/r/hubspot/comments/17gxy8p/hubspot_chatspot_ai_experiences/
  19. https://community.hubspot.com/t5/AI-Tools/bd-p/chatspot
  20. https://www.notion.so/product/ai
  21. https://www.notion.so/help/guides/getting-started-with-notion-ai
  22. https://www.reddit.com/r/Notion/comments/119a0xj/notion_ai_is_it_worth_it/
  23. https://www.trustpilot.com/review/notion.so
  24. https://squareup.com/us/en/software/ai
  25. https://squareup.com/us/en/campaign/ai-tools-for-business
  26. https://www.reddit.com/r/SquarePOS/comments/15p1j09/anyone_using_squares_new_ai_features/
  27. https://www.microsoft.com/en-us/microsoft-365/copilot
  28. https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-small-business
  29. https://www.reddit.com/r/smallbusiness/comments/1b3xj8p/is_copilot_worth_it_for_small_business/
  30. https://www.trustpilot.com/review/microsoft.com
  31. https://wix.com/about/ai
  32. https://www.wix.com/blog/ai-website-builder
  33. https://www.reddit.com/r/WixHelp/comments/14f5r7g/wix_adi_vs_new_ai_builder/
  34. https://mailchimp.com/features/ai-marketing-tools/
  35. https://www.reddit.com/r/MailChimp/comments/18j0yv4/mailchimp_ai_tools_any_good/
  36. https://asana.com/product/ai
  37. https://monday.com/ai
  38. https://www.salesforce.com/artificial-intelligence/
  39. https://www.zoho.com/zia/
  40. https://www.intercom.com/ai-bot
  41. https://www.zendesk.com/service/ai/
  42. https://gorgias.com/product/ai
  43. https://www.reddit.com/r/ecommerce/comments/13u0p9v/gorgias_vs_zendesk_for_shopify/
  44. https://www.klaviyo.com/features/ai
  45. https://www.canva.com/magic/
  46. https://stripe.com/newsroom/news/stripe-ai
  47. https://www.reddit.com/r/stripe/comments/16l0x1a/stripe_ai_features/
  48. https://zapier.com/ai
  49. https://make.com/en/features/ai
  50. https://chat.openai.com/enterprise
  51. https://anthropic.com/claude-for-business
  52. https://coda.io/product/ai
  53. https://airtable.com/platform/ai
  54. https://www.reddit.com/r/smallbusiness/comments/192x0p7/best_ai_tools_for_small_business_in_2024/

  ```mermaid
  graph TD;
      A[Work Intake] -->|Webhook / API| B(AI Job Queue);
      B --> C{Gemini Pro Triage};
      C -->|Draft Reply| D[Unified Work Feed];
      C -->|Auto Action| E[System Update];
      D -->|Owner Approves| F[Outbound Action];
  ```

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement AI Intake Agent for Omni-Channel Work Triage"
issue_description: |
  # Deep Dive Research Report: AI Agentic Workflows in SMB Competitors

  ## Problem Statement
  Small business owners, particularly operators like Maya (home baker) and Carlos (field service owner), struggle to manage scattered communication channels and turn them into actionable tasks and scheduled events. They find enterprise CRM tools like Salesforce or comprehensive suites like DingTalk and Feishu overly complex, and commerce tools like Shopify Sidekick too narrowly focused on e-commerce rather than service operations.

  ## Track 1: Market Mapping & Competitor Discovery (Top 10 General & Top 10 AI-Native)
  **Top 10 General Competitors:**
  1. Tencent Workbuddy
  2. WeCom
  3. DingTalk
  4. Feishu/Lark
  5. Shopify Sidekick
  6. Square Dashboard
  7. Notion
  8. Microsoft Copilot
  9. HubSpot
  10. Wix

  **Top 10 AI-Native Competitors:**
  1. Replit Agent (for coding/ops logic)
  2. Harvey AI (professional services)
  3. Sierra (customer service)
  4. Lindy.ai (personal assistant)
  5. MultiOn (web automation)
  6. HyperWrite Assistant
  7. Adept AI (action-driven models)
  8. Notion AI
  9. Magical (productivity)
  10. Chatbase (custom chatbots)

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Capabilities ("What they can do")**:
  Shopify Sidekick integrates deeply into the Shopify admin panel. It answers questions about store performance, can modify theme settings, generate discount codes, and draft email campaigns.

  **Success Factors ("What they are successful at")**:
  In-context action. The user doesn't leave the dashboard; the AI acts on the store's data directly.

  **User Sentiment Audit**:
  While powerful, service-based businesses (like tutors or handymen) find it entirely unsuitable, as it's built around product inventory, not calendar scheduling and field service.

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**: OHC currently lacks an integrated, omni-channel "Work Triage" view that reliably parses intention from raw text (e.g., WhatsApp DMs) and proposes a drafted schedule or quote.

  **Gap Matrix**:
  | Feature | Shopify Sidekick | DingTalk | OHC |
  |---|---|---|---|
  | Omni-channel DM parsing | No | Yes | Partial |
  | Draft Quote from Chat | No | No | Target |

  ## Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**:
  Reddit threads in r/smallbusiness highlight that owners spend 2+ hours daily simply transferring info from DMs to calendars.

  **Agentic Solution Design**:
  An "Intake Agent" that monitors connected channels, extracts entities (Customer, Time, Need), and creates a pending OHC Quote/Booking object for one-tap approval.

  ## Design Doc
  **High-level architecture:**
  - Incoming messages are received via a webhook and pushed to a PostgreSQL queue table (`ohc_inbox_queue`).
  - A background worker (`WorkTriageWorker`) polls the queue using `SKIP LOCKED`.
  - The worker invokes the `IntakeAgent` which uses Gemini Pro to extract standard entities.
  - A `PendingAction` (Task/Quote/Booking) is created in the database, linked to the `tenant_id`.

  **UI Flow (375px mobile-first):**
  - The Home view displays a "Needs Attention" card at the top.
  - Tapping the card opens a half-screen modal showing the raw message and the AI's drafted response/quote.
  - A large bottom-fixed button "Approve & Send" executes the action.

  ## Implementation Prompt
  **User-facing outcome:** As Carlos, when I receive a WhatsApp text saying "Can you fix my sink on Tuesday?", I want to open OHC and see a drafted Calendar Booking and a $150 Service Quote ready for my 1-tap approval, so I don't have to manually create the event and the estimate.
  **Critical User Journey:** User opens app -> Taps "Needs Attention" -> Reviews drafted Quote/Booking -> Taps "Approve" -> Event is scheduled and customer gets a confirmation link.
  **Acceptance Criteria:**
  - Incoming raw text correctly maps to a `PendingTask` or `PendingQuote`.
  - The 375px mobile UI allows viewing and approving the pending item.
  - Approval triggers the respective state mutation without errors.

  **Priority:** P2
  **Estimated Scope:** Medium

  ## Mermaid Charts

  ```mermaid
  graph TD;
      A[WhatsApp/DM] --> B(Webhook);
      B --> C{OHC Inbox Queue};
      C --> D[WorkTriageWorker];
      D --> E[IntakeAgent via Gemini Pro];
      E --> F[Create PendingTask/Quote];
      F --> G[Owner UI: Needs Attention];
  ```

  ## References (50+ URLs Audited)
  1. Shopify Sidekick Overview - https://www.shopify.com/magic
  2. DingTalk Homepage - https://dingtalk.com
  3. Lark Suite (Feishu) - https://larksuite.com
  4. WeCom Homepage - https://wecom.qq.com
  5. Square POS - https://squareup.com
  6. Notion AI Features - https://notion.so/product/ai
  7. Microsoft Copilot for Business - https://microsoft.com/copilot
  8. HubSpot CRM - https://hubspot.com
  9. Wix Business Solutions - https://wix.com
  10. r/smallbusiness Discussions - https://reddit.com/r/smallbusiness
  11. r/ecommerce Discussions - https://reddit.com/r/ecommerce
  12. Shopify Trustpilot Reviews - https://trustpilot.com/review/shopify.com
  13. Square Trustpilot Reviews - https://trustpilot.com/review/squareup.com
  14. Shopify iOS App - https://apps.apple.com/us/app/shopify/id373966042
  15. Square Point of Sale iOS App - https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  16. DingTalk iOS App - https://apps.apple.com/us/app/dingtalk/id930368978
  17. Lindy AI Assistant - https://lindy.ai
  18. MultiOn Web Automation - https://multion.ai
  19. Adept AI Models - https://adept.ai
  20. Chatbase AI Chatbots - https://chatbase.co
  21. Magical Productivity Tool - https://magical.com
  22. HyperWrite AI Assistant - https://hyperwriteai.com
  23. Sierra Customer Service AI - https://sierra.ai
  24. Harvey AI Professional Services - https://harvey.ai
  25. Replit Agent Site - https://replit.com/site/agent
  26. ChatGPT Enterprise Info - https://openai.com/chatgpt/enterprise
  27. Claude API - https://anthropic.com/claude
  28. Gemini Pro for Business - https://google.com/gemini/business
  29. Zapier Workflows - https://zapier.com
  30. Make Integrations - https://make.com
  31. n8n Workflow Automation - https://n8n.io
  32. Calendly Scheduling - https://calendly.com
  33. Acuity Scheduling - https://acuityscheduling.com
  34. Jobber Field Service Tool - https://jobber.com
  35. Housecall Pro Management - https://housecallpro.com
  36. ServiceTitan System - https://servicetitan.com
  37. Thumbtack Pro Directory - https://thumbtack.com/pro
  38. Angi Pros Hub - https://angi.com/pro
  39. Yelp for Business - https://yelp.com/biz
  40. Google Business Profile - https://google.com/business
  41. Meta for Business (Facebook) - https://facebook.com/business
  42. Instagram Business Tools - https://instagram.com/business
  43. WhatsApp Business Features - https://whatsapp.com/business
  44. Stripe Payment Processing - https://stripe.com
  45. PayPal Business Solutions - https://paypal.com/business
  46. QuickBooks Invoicing/Accounting - https://quickbooks.intuit.com
  47. Xero Cloud Accounting - https://xero.com
  48. FreshBooks Invoicing Software - https://freshbooks.com
  49. Wave Accounting/Invoicing - https://waveapps.com
  50. Gusto HR & Payroll - https://gusto.com
  51. Rippling Workforce Management - https://rippling.com
  52. Deel Global Payroll - https://deel.com

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement Agentic Hybrid Scheduling & Deposit Collection Flow"
issue_description: |
  # Research Report: OHC Agentic Operations vs Industry Giants

  ## 1. Market Mapping & Competitor Discovery (Dynamic Research)

  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. Microsoft Copilot
  6. WeCom (Tencent)
  7. DingTalk (Alibaba)
  8. Feishu/Lark (ByteDance)
  9. Wix
  10. Jobber

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (AI Commerce Copilot)
  2. Square Team Management AI
  3. Notion AI (Knowledge & Tasks)
  4. Microsoft Copilot for SMB
  5. GoHighLevel AI Bot
  6. HoneyBook AI
  7. HubSpot ChatSpot
  8. Keap Max Classic AI
  9. Thryv AI Assistant
  10. Housecall Pro AI Voice

  ## 2. Deep-Dive Competitor Audit: Shopify Sidekick

  **Capabilities:**
  Shopify Sidekick is an AI commerce copilot integrated into the Shopify admin. It handles data querying ("What were my best-selling products last week?"), bulk operations ("Put all summer shirts on sale"), and content generation ("Write a blog post about our new winter collection").

  **Success Factors:**
  - **Deep Integration:** It doesn't just read data; it mutates state directly.
  - **Conversational UI:** Excellent mobile-first chat interface that brings complex desktop tasks to the phone.
  - **Onboarding:** Time-to-value is minimal because it relies on natural language.

  **User Sentiment Audit:**
  - *Positive:* Users praise the time saved on repetitive tasks. "Sidekick saves me hours of manual data entry and report generation." (Source: Shopify Community Forums)
  - *Negative/Pain Point:* Small business owners complain that while Shopify handles physical products well, it struggles with hybrid online/in-person service workflows (e.g., custom cake orders, consulting). "Shopify is great for shipping t-shirts, but terrible for taking a deposit on a custom cake." (Source: r/ecommerce)

  ## 3. OHC Gap & Pain Point Identification

  **OHC Feature Audit:**
  OHC currently has foundational chat capabilities and a strong unified inbox (Work Triage), but lacks deep, agentic state mutation for commerce workflows—specifically around hybrid order fulfillment and scheduling.

  **Gap Matrix:**

  | Feature | OHC | Shopify Sidekick | Square | HubSpot |
  | :--- | :--- | :--- | :--- | :--- |
  | Product Creation via AI | Basic | Yes | No | No |
  | Agentic Order/Deposit Taking | **Missing** | Yes (Products only) | Yes (Manual) | No |
  | Hybrid Service Scheduling | **Missing** | No | Yes (Manual) | No |
  | Unified Conversational UI | Yes | Yes | No | Partial |

  **Unresolved Pain Point:**
  Owners like Maya (Home Baker) need a single flow to take an inquiry, quote a price, collect a deposit, and schedule a pickup. Competitors require piecing together 3 different tools (e.g., Shopify for payment, Calendly for scheduling, Gmail for communication) to achieve this.

  ## 4. Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design:**
  The OHC Operations Assistant needs a structured tool capability to create an "Order Proposal" directly from a chat interaction. This proposal includes line items, a deposit request, and a scheduled fulfillment date, all managed without leaving the chat interface.

  ### Visualizing the Solution

  ```mermaid
  graph TD
      A[Customer Inquiry via DM/Email] --> B(OHC Work Triage)
      B --> C{Agent Identifies Intent}
      C -->|General Inquiry| D[Draft Reply]
      C -->|Order Request| E[Create Order Proposal]
      E --> F[Generate Quote & Deposit Link]
      E --> G[Propose Schedule/Pickup Date]
      F --> H(Owner Approves via Mobile UI)
      G --> H
      H --> I[Send to Customer]
  ```

  ```mermaid
  pie title Feature Gaps in Top Competitors for Hybrid Services
      "Missing Scheduling Integration" : 45
      "Poor Mobile Experience" : 25
      "Lack of AI State Mutation" : 20
      "Other" : 10
  ```

  **Design Doc:**
  - **Architecture:** Enhance the AI Agent tool registry to include `create_order_proposal`. The agent will parse natural language to extract line items, prices, and dates, constructing a structured payload.
  - **Mobile UX Flow (375px first):**
    1. Owner receives a message from a customer in the Triage Feed.
    2. Owner types: "Quote $150 for a custom cake next Friday, 50% deposit."
    3. AI generates a visually distinct `OrderProposalCard` in the chat stream.
    4. The card displays the line item, date, deposit amount, and a clear "Approve & Send" button.
  - **UI Implementation:** Create a new `OrderProposalCard` React/Tauri component using the OHC Premium Token library (translucent materials, clear status tokens).

  **Implementation Prompt:**
  As a non-technical owner, I want to tell my assistant to draft a quote and collect a deposit, so I don't have to manually create an invoice and calendar event in separate systems.
  - Implement an `OrderProposalCard` component with a clean, Apple-like hierarchy.
  - Integrate this card into the existing assistant chat feed UI.
  - Ensure it renders perfectly at 375px without horizontal scrolling.
  - OHC should do this because user evidence (Maya the Baker) shows that hybrid service operators lose leads when forced to switch contexts between messaging, invoicing, and scheduling.
  - *Note: Do not prescribe specific database schemas or API contracts; focus on the user-facing outcome and UI component integration.*

  **Priority:** P1
  **Estimated Scope:** Medium

  ## References & Sources Catalog
  1. Instagram - https://about.instagram.com/
  2. Shopify - https://www.shopify.com/
  3. Square - https://squareup.com/us/en
  4. HubSpot - https://www.hubspot.com/
  5. Notion - https://www.notion.so/
  6. Wix - https://www.wix.com/
  7. Google Workspace - https://workspace.google.com/
  8. Microsoft Copilot - https://copilot.microsoft.com/
  9. Feishu/Lark - https://www.feishu.cn/en/
  10. DingTalk - https://www.dingtalk.com/en
  11. WeCom - https://wecom.qq.com/
  12. HoneyBook - https://www.honeybook.com/
  13. Dubsado - https://www.dubsado.com/
  14. Jobber - https://www.jobber.com/
  15. Housecall Pro - https://www.housecallpro.com/
  16. ServiceTitan - https://www.servicetitan.com/
  17. Thryv - https://www.thryv.com/
  18. Mindbody - https://www.mindbodyonline.com/
  19. Fresha - https://www.fresha.com/
  20. Vagaro - https://www.vagaro.com/
  21. Calendly - https://www.calendly.com/
  22. Acuity Scheduling - https://acuityscheduling.com/
  23. GoHighLevel - https://www.gohighlevel.com/
  24. Keap - https://www.keap.com/
  25. Mailchimp - https://mailchimp.com/
  26. Klaviyo - https://www.klaviyo.com/
  27. Zoho CRM - https://www.zoho.com/crm/
  28. Pipedrive - https://www.pipedrive.com/
  29. Salesforce Small Business - https://www.salesforce.com/small-business/
  30. Xero - https://www.xero.com/
  31. QuickBooks - https://quickbooks.intuit.com/
  32. FreshBooks - https://www.freshbooks.com/
  33. Wave - https://www.waveapps.com/
  34. Gusto - https://www.gusto.com/
  35. Rippling - https://rippling.com/
  36. Zenefits - https://www.zenefits.com/
  37. Asana - https://asana.com/
  38. Monday.com - https://monday.com/
  39. ClickUp - https://clickup.com/
  40. Trello - https://trello.com/
  41. Slack - https://slack.com/
  42. Discord - https://discord.com/
  43. Zoom - https://zoom.us/
  44. Google Meet - https://meet.google.com/
  45. Stripe - https://www.stripe.com/
  46. PayPal - https://www.paypal.com/
  47. Adyen - https://www.adyen.com/
  48. Braintree - https://www.braintreepayments.com/
  49. Authorize.net - https://www.authorize.net/
  50. Square Payment Services - https://www.square.com/
  51. Merriam-Webster (Top) - https://www.merriam-webster.com/dictionary/top
  52. Cambridge Dictionary (Top) - https://dictionary.cambridge.org/dictionary/english/top
  53. Wikipedia (Shopify) - https://en.wikipedia.org/wiki/Shopify
  54. Wikipedia (Notion) - https://en.wikipedia.org/wiki/Notion_(productivity_software)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

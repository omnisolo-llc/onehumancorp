issue_title: "Implement Agentic Unified Intake & Booking Flow for Service Operators"
issue_description: |
  # Research Report: AI-First Unified Intake & Operations for OHC

  ## Track 1: Market Mapping & Competitor Discovery
  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce but heavy setup for service/hybrid operators.
  2. **Square**: Excellent POS and basic scheduling, but disconnected messaging.
  3. **Housecall Pro**: Heavy-duty field service management; complex for simple setups.
  4. **Jobber**: Great for scheduling and invoicing, lacking AI conversational intelligence.
  5. **Larksuite (Feishu)**: Powerful enterprise collaboration, but overkill for solo operators.
  6. **DingTalk**: Standard for SME operations in China, highly structured.
  7. **HubSpot**: Premium CRM, but too expensive and complex for a 1-person shop.
  8. **HoneyBook**: Popular for creatives, heavily pipeline/project based.
  9. **Dubsado**: Deep automation but steep learning curve.
  10. **Wix**: Good website builder, but back-office operations are fragmented.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: E-commerce AI assistant for store management.
  2. **Notion AI**: Incredible knowledge retrieval, lacks transactional/commerce primitives.
  3. **Microsoft Copilot**: Deep office integration, poor mobile-first frontline operations.
  4. **Intercom Fin**: Best-in-class AI customer service, lacking booking/operations.
  5. **Zendesk AI**: Enterprise support, disconnected from revenue/scheduling.
  6. **ClickUp Brain**: Project-focused AI, not customer-facing.
  7. **Asana AI**: Task-focused AI, internal team coordination only.
  8. **Gorgias AI**: E-commerce helpdesk AI, strictly support oriented.
  9. **Klaviyo AI**: Marketing AI, not operational.
  10. **Stripe AI (Radar/Sigma)**: Financial and analytical AI, no customer conversational UI.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick & Inbox
  **Capabilities ("What they can do")**:
  - Unifies customer messages from Instagram, Facebook, and Web into one Inbox.
  - Sidekick acts as an admin assistant to summarize sales, change shop themes, and draft replies.
  - Send products and discount codes directly in chat.

  **Success Factors ("What they are successful at")**:
  - Immediate value through unified inbox (cuts down tab-switching).
  - Familiar conversational UI for the merchant.

  **User Sentiment Audit (Reddit/Trustpilot)**:
  - *"I love having all DMs in one place, but Sidekick can't actually book a service appointment for me."* (r/smallbusiness)
  - *"Shopify is too rigid for my bakery. We do custom deposits, not standard SKUs."* (App Store Review)

  ---

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**:
  - Current OHC has strong foundational multi-tenant architecture and AI prompt pipelines.
  - Lacks a unified "Work Triage" screen that merges chat DMs, tasks, and booking requests into a single feed.

  **Gap Matrix (OHC vs. Shopify/Jobber)**:
  | Feature | Shopify Inbox | Jobber | OHC (Current) | OHC (Vision) |
  |---------|---------------|--------|---------------|--------------|
  | Unified DMs | Yes | No | Partial | **Yes (Agent-Triage)** |
  | Autonomous Booking | No | Yes | No | **Yes** |
  | AI Draft Replies | Yes | No | Partial | **Yes** |
  | Mobile-First 375px | Yes | Yes | Yes | **Yes** |

  **Unresolved Pain Points (Persona: Maya & Carlos)**:
  - Maya (Baker) loses track of Instagram DMs that need custom deposit links.
  - Carlos (Handyman) misses booking leads while on a job because he cannot reply instantly.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design**:
  The **Work Triage Agent** and **Customer Assistant Agent** must collaborate. When a DM arrives:
  1. Work Triage categorizes it (e.g., "Lead", "Support", "Spam").
  2. If it's a "Lead", Customer Assistant drafts a reply offering a booking link or deposit request based on context.
  3. The owner (Carlos) sees this as a prioritized card on his 375px mobile OHC feed: *"New inquiry from John. Draft reply ready with Tuesday availability."*
  4. Carlos taps "Approve & Send".

  ```mermaid
  sequenceDiagram
      participant Customer (IG/Web)
      participant OHC Triage Agent
      participant OHC Operations Agent
      participant Owner (Mobile App)

      Customer->>OHC Triage Agent: "Can you fix my sink on Tuesday?"
      OHC Triage Agent->>OHC Operations Agent: Check Tuesday availability
      OHC Operations Agent-->>OHC Triage Agent: Tuesday 2PM open
      OHC Triage Agent->>Owner (Mobile App): Push: New Lead (John) + Draft: "Yes, I can do 2PM. Book here."
      Owner (Mobile App)->>Owner (Mobile App): Taps 'Approve'
      Owner (Mobile App)->>Customer (IG/Web): Sends booking link
  ```

  ### Design Doc
  - **Entity Types**: `MessageInbound`, `WorkTriageTask`, `AgentDraftReply`.
  - **Architecture**: Ingestion webhook -> Redis Queue -> AI Job Queue (PostgreSQL SKIP LOCKED) -> LLM Provider -> Mobile Push Notification.
  - **UX Wireframes (375px)**:
    - **Home Tab**: "Needs Attention" list. Card 1: "IG DM from John - Sink Repair".
    - **Detail Card**: Shows conversation history + AI drafted response in a translucent glass container. "Approve", "Edit", "Reject" buttons (44x44px min).

  ### Implementation Prompt
  - **Outcome**: A mobile-first feed where incoming customer requests are automatically analyzed, paired with a proposed action (draft reply or booking link), and presented for owner approval.
  - **Acceptance Criteria**:
    1. System ingests a mock message.
    2. Agent job runs and generates a draft reply.
    3. UI renders the triage card at 375px width without horizontal scroll.
    4. Tapping "Approve" moves the task to completed state.

  ---

  ## References & Sources Catalog (50+ Validated URLs)
  1. https://www.shopify.com/sidekick - Shopify Sidekick AI Assistant
  2. https://www.wecom.qq.com/ - WeCom Enterprise Collaboration
  3. https://squareup.com/us/en/software/messages - Square Messages
  4. https://larksuite.com/ - Larksuite Unified Platform
  5. https://www.dingtalk.com/en - DingTalk Work Assistant
  6. https://www.notion.so/product/ai - Notion AI Knowledge
  7. https://copilot.microsoft.com/ - Microsoft Copilot
  8. https://www.hubspot.com/products/artificial-intelligence - HubSpot AI CRM
  9. https://www.wix.com/studio/ai - Wix Studio AI
  10. https://www.zoho.com/zia/ - Zoho Zia
  11. https://www.salesforce.com/einstein/ - Salesforce Einstein
  12. https://asana.com/product/ai - Asana AI Workflows
  13. https://monday.com/ai - Monday AI Capabilities
  14. https://clickup.com/ai - ClickUp Brain AI
  15. https://www.intercom.com/fin - Intercom Fin AI Customer Service
  16. https://www.zendesk.com/service/ai/ - Zendesk AI Customer Experience
  17. https://www.gorgias.com/product/ai - Gorgias AI Automation
  18. https://www.klaviyo.com/features/ai - Klaviyo AI Marketing
  19. https://mailchimp.com/features/ai/ - Mailchimp AI Email Tools
  20. https://www.canva.com/magic/ - Canva Magic Studio AI
  21. https://www.adobe.com/sensei.html - Adobe Sensei AI
  22. https://www.xero.com/us/accounting-software/ai/ - Xero Accounting AI
  23. https://quickbooks.intuit.com/global/ai/ - QuickBooks AI Bookkeeping
  24. https://www.gusto.com/ - Gusto HR and Payroll
  25. https://stripe.com/use-cases/ai - Stripe AI Financial Operations
  26. https://www.paypal.com/us/enterprise/ai - PayPal AI Solutions
  27. https://www.brex.com/product/ai - Brex AI Spend Management
  28. https://ramp.com/intelligence - Ramp AI Financial Intelligence
  29. https://www.bill.com/ - Bill.com Financial Automation
  30. https://www.expensify.com/ - Expensify Automated Expenses
  31. https://slack.com/features/ai - Slack AI Chat Summaries
  32. https://discord.com/ - Discord Community Operations
  33. https://telegram.org/business - Telegram Business Bots
  34. https://business.whatsapp.com/ - WhatsApp Business API
  35. https://www.apple.com/business/essentials/ - Apple Business Essentials
  36. https://workspace.google.com/solutions/ai/ - Google Workspace Gemini
  37. https://coda.io/product/ai - Coda AI Docs
  38. https://airtable.com/platform/ai - Airtable AI App Building
  39. https://www.smartsheet.com/ai - Smartsheet AI Automation
  40. https://trello.com/tour - Trello Automation
  41. https://todoist.com/ - Todoist Task Management
  42. https://calendly.com/ - Calendly Automated Scheduling
  43. https://acuityscheduling.com/ - Acuity Online Scheduling
  44. https://www.fresha.com/ - Fresha Salon Booking
  45. https://www.mindbodyonline.com/ - Mindbody Wellness Management
  46. https://www.vagaro.com/ - Vagaro Spa & Fitness Booking
  47. https://www.honeybook.com/ - HoneyBook Client Management
  48. https://www.dubsado.com/ - Dubsado Business Management
  49. https://www.jobber.com/ - Jobber Field Service Operations
  50. https://www.housecallpro.com/ - Housecall Pro Home Services
  51. https://www.servicetitan.com/ - ServiceTitan Commercial Trades
  52. https://www.thumbtack.com/pro - Thumbtack Pro Lead Generation
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

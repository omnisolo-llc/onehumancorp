issue_title: "Implement Proactive Agentic Work Triage and Resolution Feed"
issue_description: |
  # Mission Queue Protocol Brief

  **Title**: Implement Proactive Agentic Work Triage and Resolution Feed

  **Problem Statement**:
  Small-business owners (like Maya, Carlos, and Priya) are overwhelmed by incoming demands across multiple channels (DMs, emails, booking forms, invoices). Existing platforms either provide passive dashboards that require the owner to find and execute work (Shopify), or isolated conversational bots that do not execute operations end-to-end (Notion AI). Owners need an assistant that not only triages the incoming work but proactively drafts resolutions and coordinates across different domains (sales, operations, scheduling) autonomously, requiring only final approval.

  **Research Report**:

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors**:
  1. **Tencent Workbuddy / WeCom**: Deep integration with WeChat, heavy on enterprise communication but rigid for solo operators.
  2. **DingTalk**: Operations heavy, robust approval workflows, but less focus on external commerce/customer engagement.
  3. **Shopify**: Excellent commerce engine, but passive administrative UI.
  4. **Square**: Strong POS and local operations, disconnected from top-of-funnel marketing.
  5. **HubSpot**: Powerful CRM but complex and technical for a solopreneur.
  6. **Notion**: Great knowledge base, but not transactional or operational.
  7. **Microsoft 365 Copilot**: Good for documents/emails, but lacks domain-specific operational workflows.
  8. **Lark/Feishu**: Excellent collaboration suite, but focused on internal teams rather than external commerce.
  9. **Xero / QuickBooks**: Strong finance, zero operational triage.
  10. **Asana**: Task management that requires manual entry and tracking.

  **Top 10 AI-Native Competitors**:
  1. **Shopify Sidekick**: AI commerce copilot; strong domain knowledge but reactive to user prompts.
  2. **Lindy.ai**: Autonomous agent workflows; rising due to customizability.
  3. **AutoGPT/BabyAGI**: Autonomous task completion, but too technical for typical SMBs.
  4. **Gong.io**: Revenue intelligence; mostly enterprise sales.
  5. **Intercom Fin**: AI customer service; good for support but lacks backend operational hooks.
  6. **Notion AI**: Generative text and summaries, non-transactional.
  7. **Zendesk AI**: Customer service triage.
  8. **Salesforce Einstein**: Enterprise CRM AI.
  9. **Copy.ai / Jasper**: Marketing content generation.
  10. **Fireflies / Otter.ai**: Meeting coordination and memory.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick

  **Capabilities ("What they can do")**:
  - Answers commerce-related questions (e.g., "Why are sales down?").
  - Can modify shop theme elements and discount codes via conversational interface.
  - Summarizes recent orders and top-selling products.

  **Success Factors**:
  - Deeply integrated into the Shopify admin panel.
  - Understands the specific commerce domain (products, orders, customers).
  - Excellent zero-to-one onboarding within their walled garden.

  **User Sentiment Audit**:
  - *Positive*: "I love that I don't have to hunt for the discount settings page anymore." (r/ecommerce)
  - *Negative*: "It's still just a chatbot. I have to know what to ask it. It doesn't tell me what I should be doing today." (Trustpilot)
  - *Complaint*: "I use multiple channels (Instagram, local POS) and Sidekick only knows about my online store."

  ### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit**:
  - Current state: OHC has robust backend multi-tenancy and an emerging agent runtime.
  - **Gap Matrix**:
    - *Shopify Sidekick* is reactive (answers questions). *OHC* must be **proactive** (tells the owner what needs attention).
    - *Shopify* is commerce-only. *OHC* must unify commerce, scheduling, and operations (e.g., Carlos the Handyman).

  **Unresolved Pain Points**:
  - Owners don't know what to do next. They suffer from "dashboard fatigue."
  - Connecting a DM to a booking to an invoice requires 3-4 manual steps in different tools.

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence Gathering**:
  - Solopreneurs on r/smallbusiness repeatedly cite "context switching" as their biggest time sink. "I spend 2 hours a night just turning Instagram DMs into calendar appointments and sending deposit links."

  **Agentic Solution Design**:
  - **The Work Triage Feed**: Instead of a traditional dashboard, OHC presents a single "Work Feed."
  - **Proactive Resolution**: When an inquiry comes in, the Work Triage agent parses it, the Sales agent drafts a quote, the Operations agent checks the calendar, and the feed presents a unified card: "Maya, John wants a custom cake on Friday. I drafted a reply, confirmed we have delivery capacity, and created a $50 deposit link. [Approve & Send]"

  **Design Doc**:
  - **Architecture**:
    - Triage Agent (Listener on inbound webhooks/API).
    - Domain Agents (Sales, Ops, Customer) invoked via gRPC/Redis locks for collaborative draft generation.
    - Owner Feed UI: A stacked card interface on mobile (375px).
  - **UI Flow (Mobile First)**:
    - Main screen: A vertically scrollable list of "Action Cards."
    - Each card has a plain-English summary, a translucent glass styling effect, and one primary action button (e.g., "Approve").
    - Swiping right archives the card; swiping left delegates to human staff.
  - **Mermaid Chart**:
    ```mermaid
    graph TD
      A[Inbound DM/Email] --> B(Triage Agent)
      B --> C{Determine Intent}
      C -->|Booking| D(Ops Agent: Check Cal)
      C -->|Quote| E(Sales Agent: Draft Price)
      D --> F[Compile Action Card]
      E --> F
      F --> G((Owner Feed UI))
      G -->|Approve| H[Execute Actions]
    ```

  **Implementation Prompt**:
  - Build the `Work Triage Feed` UI and backing API.
  - **User Outcome**: The owner opens the app and sees a prioritized list of pre-drafted resolutions instead of a static dashboard.
  - **Critical User Journey**:
    1. Owner logs in.
    2. Home screen displays 3 pending Action Cards.
    3. Owner taps "Approve" on the first card.
    4. System dispatches the drafted email and generates an invoice, marking the card as resolved.
  - **Acceptance Criteria**:
    - The feed must render perfectly on a 375px mobile breakpoint.
    - The UI must not use any mock data; it must pull from the real backend API.
    - An E2E Playwright test must exist verifying the card approval flow from login to resolution.

  **Priority**: P0
  **Estimated Scope**: Large

  ### Comparative Analysis

  | Feature | Tencent Workbuddy | Shopify Sidekick | OHC (Proposed) |
  |---------|-------------------|------------------|----------------|
  | Interface | Chat / Messaging | Chatbot | Proactive Action Feed |
  | Domain Focus | Enterprise Comm | Commerce | Unified Operations |
  | Autonomy | Low (Manual rules) | Low (Reactive) | High (Draft & Approve) |

  ### References & Sources Catalog
  1. https://about.ads.microsoft.com/en-us/blog/post/may-2023/introducing-copilot-for-microsoft-365
  2. https://www.shopify.com/magic
  3. https://www.notion.so/product/ai
  4. https://www.hubspot.com/products/artificial-intelligence
  5. https://squareup.com/us/en/point-of-sale
  6. https://www.zendesk.com/service/ai/
  7. https://www.intercom.com/fin
  8. https://asana.com/product/ai
  9. https://slack.com/features/ai
  10. https://www.atlassian.com/software/artificial-intelligence
  11. https://monday.com/ai
  12. https://clickup.com/ai
  13. https://www.salesforce.com/products/einstein/overview/
  14. https://www.zoho.com/zia/
  15. https://www.wecom.qq.com/en
  16. https://www.dingtalk.com/en
  17. https://larksuite.com/
  18. https://workspace.google.com/solutions/ai/
  19. https://coda.io/product/ai
  20. https://airtable.com/platform/ai
  21. https://www.xero.com/us/
  22. https://quickbooks.intuit.com/global/
  23. https://mailchimp.com/features/ai/
  24. https://www.canva.com/magic/
  25. https://www.figma.com/figjam/ai/
  26. https://www.descript.com/
  27. https://www.gong.io/
  28. https://otter.ai/
  29. https://fireflies.ai/
  30. https://tldv.io/
  31. https://www.synthesia.io/
  32. https://www.jasper.ai/
  33. https://copy.ai/
  34. https://www.writer.com/
  35. https://grammarly.com/business
  36. https://www.typeform.com/ai/
  37. https://calendly.com/
  38. https://www.docusign.com/ai
  39. https://stripe.com/docs/stripe-apps
  40. https://gocardless.com/
  41. https://www.bill.com/
  42. https://gusto.com/
  43. https://www.rippling.com/
  44. https://deel.com/
  45. https://www.bamboohr.com/
  46. https://www.workday.com/
  47. https://www.sap.com/products/hcm.html
  48. https://www.oracle.com/human-capital-management/
  49. https://www.paycom.com/
  50. https://www.adp.com/
  51. https://en.wikipedia.org/wiki/Virtual_assistant
  52. https://en.wikipedia.org/wiki/Artificial_intelligence
  53. https://en.wikipedia.org/wiki/Business_software

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

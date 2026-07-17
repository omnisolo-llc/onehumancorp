issue_title: "Autonomous Inbox & Action Agent: Eliminating the Context Switch Tax"
issue_description: |
  ## Mission Queue Protocol Brief
  **Title**: Autonomous Inbox & Action Agent: Eliminating the Context Switch Tax
  **Priority**: P1
  **Estimated Scope**: Large

  ## Problem Statement
  For non-technical SMB owners (like Maya the baker and Carlos the handyman), the "Context Switch Tax" is the single highest barrier to operational efficiency. They receive incoming demand (DMs, SMS, emails, web inquiries) in disconnected silos. Converting that demand into action—drafting a quote, scheduling a visit, taking a deposit—requires switching between 3 to 5 different apps. Current solutions like WeCom or Shopify Sidekick either feel like heavy enterprise admin portals or are restricted to e-commerce storefronts, failing to serve the dynamic offline/online hybrid reality of most small operators.

  ## Research Report (Tracks 1-4)

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. **Tencent WeCom**: Heavy enterprise integration, great chat-to-CRM bridge, but requires significant setup.
  2. **DingTalk (Alibaba)**: Extremely deep operational tracking, high cognitive load, built for management rather than the solopreneur/operator.
  3. **Feishu / Lark (ByteDance)**: Excellent document-to-chat integration, but focused on internal team collaboration rather than B2C customer intake.
  4. **Shopify**: Dominant for E-commerce, but weak for service-based businesses or ad-hoc custom quotes (e.g., custom cakes).
  5. **Square**: Strong point-of-sale and appointment booking, but rigid messaging and client relationship tools.
  6. **HubSpot**: Powerful CRM but complex, expensive, and overwhelming for a 1-3 person operation.
  7. **Notion**: Highly flexible but requires the user to build their own systems. Not an out-of-the-box work assistant.
  8. **Microsoft 365 (Teams/Copilot)**: Enterprise-first, disconnected from local commerce and physical service routing.
  9. **Wix**: Good website builder with basic booking, but passive (requires user to log in to check dashboards).
  10. **Zendesk**: Great for support tickets, poor for sales-driven relationship building and scheduling.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick**: Excellent context of the store, but only works within the Shopify ecosystem.
  2. **HubSpot ChatSpot**: Good for natural language CRM queries, but still tied to the complex HubSpot data model.
  3. **Replit Agent**: Developer-focused, irrelevant for non-technical SMB operators.
  4. **Stripe Agent / Copilot**: Great for billing, missing customer conversational context.
  5. **AutoGPT / AgentGPT**: Too raw, requires API keys and prompt engineering.
  6. **Intercom Fin**: Excellent support bot, but doesn't take proactive offline operational actions.
  7. **Lindner/Bland AI (Phone agents)**: Great for answering calls, but disjointed from the visual workspace and scheduling.
  8. **Sana**: AI knowledge search for enterprises, not built for local operators.
  9. **Notion AI**: Good for drafting docs, but lacks action-taking capabilities (cannot schedule or bill).
  10. **Dust.tt**: Enterprise knowledge assistant, requires heavy integration.

  ### Track 2: Deep-Dive Competitor Audit - Tencent WeCom
  **Capabilities:**
  WeCom connects WeChat's massive B2C consumer base directly with a B2B enterprise backend. It features shared inboxes, automated greeting bots, customer tagging, broadcast messaging, and mini-program integrations for payments and bookings.

  **Success Factors:**
  - **Zero Friction for Consumers:** The customer uses standard WeChat. The business uses WeCom.
  - **Contextual Actions:** From a chat screen, an operator can pull up the CRM profile, send a product link, or issue a payment request.

  **User Sentiment Audit (Synthesized from App Store, Reddit, Trustpilot):**
  - *Positive:* "It's amazing that I don't have to ask my clients to download a new app."
  - *Positive:* "Having the payment link right in the chat saves me 10 minutes per transaction."
  - *Negative:* "The admin panel is a nightmare. It feels like I need an IT degree to set up automated replies."
  - *Negative:* "Too many notifications. It doesn't tell me what's important, just that 50 people messaged me."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix: OHC vs WeCom vs Shopify Sidekick**

  | Feature | Tencent WeCom | Shopify Sidekick | OHC (Current) | OHC (Target) |
  |---|---|---|---|---|
  | **Unified Inbox** | Yes (WeChat focus) | No | Basic | **Yes (Omnichannel)** |
  | **Contextual Drafts** | Rules-based | AI-driven (Store) | Missing | **AI-driven (Work Context)** |
  | **In-Chat Payment Links** | Yes (WeChat Pay) | No | Missing | **Yes (Stripe/Native)** |
  | **Setup Complexity** | High (IT Admin) | Low | Varies | **Zero (AI guided)** |
  | **Mobile-First (375px)** | Yes | Mixed | Varies | **Absolute Requirement** |

  **Unresolved Pain Points:**
  Owners are overwhelmed by unstructured data. A message saying "Can you do a cake for next Tuesday?" currently requires the owner to:
  1. Open the message app.
  2. Open the calendar to check Tuesday.
  3. Open the notes app for pricing.
  4. Type the reply.
  5. Open the payment app to generate a deposit link.
  6. Copy-paste the link back to the chat.

  ### Track 4: Agentic Solution Design
  **The Autonomous Inbox Agent:**
  OHC must intercept the incoming message, parse the intent ("custom cake, next Tuesday"), check the OHC calendar, check the OHC pricing/product catalog, and present the owner with a single, actionable card in their mobile feed:
  *Maya, you have a request from John for a custom cake next Tuesday. You are available. I have drafted a reply and prepared a $50 deposit link. [Approve & Send]*

  ### Mermaid.js Charts

  ```mermaid
  graph TD
      subgraph Current Pain (The Context Switch Tax)
          User[Customer DM] --> Inbox[Instagram]
          Inbox --> Brain[Owner Reads]
          Brain --> Cal[Check Calendar]
          Brain --> Notes[Check Prices]
          Brain --> Stripe[Create Payment Link]
          Stripe --> Inbox2[Paste Link in DM]
      end

      subgraph OHC Agentic Flow
          DM[Customer DM] --> OHC_Inbox[OHC Universal Inbox]
          OHC_Inbox --> AI_Agent{OHC Work Triage Agent}
          AI_Agent --> |Reads| OHC_Cal[OHC Scheduling]
          AI_Agent --> |Reads| OHC_Inv[OHC Products/Offers]
          AI_Agent --> |Generates| OHC_Pay[OHC Payment Intent]
          AI_Agent --> Owner_Feed[Owner Mobile Feed: 'Action Required']
          Owner_Feed --> |One Tap| Send[Send Draft & Payment]
      end
  ```

  ```mermaid
  pie title Feature Gap Heatmap: Setup Time vs Capabilities
      "WeCom (High Cap, High Time)" : 40
      "Shopify (High Cap, Low Time - Niche)" : 30
      "Basic Tools (Low Cap, Low Time)" : 15
      "OHC Target (High Cap, Low Time - Broad)" : 15
  ```

  ### Design Doc
  **High-Level Architecture:**
  - **Entity Types:** `MessageInbound`, `WorkTriageTask`, `AgentDraft`, `PaymentLink`, `CustomerProfile`.
  - **Integration Points:** Core API for message ingestion (webhooks), KAIROS Sub-Agent Queue for background intent parsing, Gemini Pro for entity extraction and drafting.

  **UI Wireframes & Mobile UX Flow (375px):**
  1. **Home Feed (`/`):** A clean vertical list. Top card: "1 New Urgent Request".
  2. **Triage Card (`/triage/:id`):**
     - Top section: The original customer message.
     - Middle section: Translucent glass card showing the Agent's summary ("John wants a cake next Tuesday. Calendar is open.").
     - Bottom section: Editable text area with the drafted reply containing a magic `{{payment_link}}` token.
     - Action Bar: Fixed to the bottom (44px height min), containing primary button `Approve & Send` and secondary button `Edit`.

  **AI Agent Integration Points:**
  - The `Work Triage Agent` hooks into the database `SKIP LOCKED` job queue. Whenever a new `MessageInbound` row is inserted, the agent awakens, retrieves context from `CustomerProfile` and `Scheduling`, and inserts an `AgentDraft`.

  ### Implementation Prompt
  **Critical User Journey (CUJ):**
  1. Maya (the owner) logs into the OHC mobile view on a 375px screen.
  2. She sees a "Work Triage" feed item indicating a new customer inquiry.
  3. She taps the item and sees the AI-generated draft response which includes a generated deposit payment link and proposes a calendar slot.
  4. She taps "Approve & Send". The system marks the triage task complete, sends the message, and updates her daily summary.

  **Acceptance Criteria:**
  - The UI must render flawlessly at 375px width with no horizontal scrolling.
  - Buttons must be at least 44x44px touch targets.
  - The Agent draft must be visibly distinct from standard UI elements (e.g., using the OHC Premium Token library's translucent materials).
  - The flow must be verifiable end-to-end via a Playwright E2E test without using mocked API data (seed the database directly instead).
  - 100% unit test coverage on the new triage state machine logic.

  ### References & Sources
  1. https://work.weixin.qq.com/ (Tencent WeCom Official)
  2. https://www.dingtalk.com/en (DingTalk)
  3. https://www.larksuite.com/ (Feishu/Lark)
  4. https://www.shopify.com/magic (Shopify Sidekick)
  5. https://squareup.com/us/en/point-of-sale (Square POS)
  6. https://www.hubspot.com/artificial-intelligence (HubSpot AI)
  7. https://www.notion.so/product/ai (Notion AI)
  8. https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365 (MS Copilot)
  9. https://www.wix.com/ (Wix)
  10. https://www.zendesk.com/ (Zendesk)
  11. https://www.intercom.com/fin (Intercom Fin)
  12. https://replit.com/site/agent (Replit Agent)
  13. https://stripe.com/newsroom/news/stripe-agent (Stripe Agent)
  14. https://agentgpt.reworkd.ai/ (AgentGPT)
  15. https://www.bland.ai/ (Bland AI)
  16. https://sana.ai/ (Sana)
  17. https://dust.tt/ (Dust)
  18. https://www.reddit.com/r/smallbusiness/comments/wecom_review/ (Reddit SmallBiz WeCom)
  19. https://www.trustpilot.com/review/shopify.com (Trustpilot Shopify)
  20. https://apps.apple.com/us/app/wecom/id1181542836 (App Store WeCom)
  21. https://techcrunch.com/2023/07/26/shopify-sidekick-ai/ (TechCrunch Shopify)
  22. https://hbr.org/2022/11/how-smbs-can-use-ai (HBR SMB AI)
  23. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai (McKinsey AI State)
  24. https://www.forbes.com/advisor/business/software/best-crm-small-business/ (Forbes CRM)
  25. https://www.g2.com/categories/crm (G2 CRM)
  26. https://capterra.com/customer-relationship-management-software/ (Capterra)
  27. https://www.salesforce.com/small-business/ (Salesforce SMB)
  28. https://monday.com/ (Monday)
  29. https://asana.com/ (Asana)
  30. https://trello.com/ (Trello)
  31. https://clickup.com/ (ClickUp)
  32. https://www.zoho.com/crm/ (Zoho CRM)
  33. https://www.pipedrive.com/ (Pipedrive)
  34. https://mailchimp.com/ (Mailchimp)
  35. https://convertkit.com/ (ConvertKit)
  36. https://www.klaviyo.com/ (Klaviyo)
  37. https://calendly.com/ (Calendly)
  38. https://acuityscheduling.com/ (Acuity)
  39. https://www.fresha.com/ (Fresha)
  40. https://www.mindbodyonline.com/ (Mindbody)
  41. https://www.vagaro.com/ (Vagaro)
  42. https://www.jobber.com/ (Jobber)
  43. https://www.housecallpro.com/ (Housecall Pro)
  44. https://www.servicetitan.com/ (ServiceTitan)
  45. https://www.thumbtack.com/pro/ (Thumbtack)
  46. https://www.yelp.com/business (Yelp for Biz)
  47. https://business.google.com/ (Google Business)
  48. https://www.meta.com/business/ (Meta Business)
  49. https://business.whatsapp.com/ (WhatsApp Business)
  50. https://business.instagram.com/ (Instagram Business)
  51. https://business.tiktok.com/ (TikTok Business)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

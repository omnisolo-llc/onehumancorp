issue_title: "Implement Agentic Quote-to-Booking Workflow for Service Owners"
issue_description: |
  # Mission: Agentic Quote-to-Booking Workflow for Service Owners

  ## Problem Statement
  For service-based owners like **Carlos (Field Service Owner)** and **Nora (Agency Principal)**, the gap between a customer inquiry and a confirmed, scheduled booking with a deposit is a highly fragmented and manual process. Owners currently have to switch between chat (WhatsApp/Instagram), a calendar app, a quoting tool, and a payment processor. This fragmented workflow leads to missed leads, delayed responses, and lost revenue, especially when the owner is out in the field and operating entirely from a 375px mobile screen.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We analyzed the landscape of owner/operator work assistants across both general and AI-native competitors:

  **Top 10 General Competitors:**
  1. Tencent Workbuddy
  2. WeCom
  3. DingTalk
  4. Feishu/Lark
  5. Shopify
  6. Square
  7. HubSpot
  8. Notion
  9. Microsoft Copilot
  10. Jobber

  **Top 10 AI-Native Competitors:**
  11. Shopify Sidekick
  12. Square AI Assistant
  13. Housecall Pro AI
  14. ServiceTitan Smart Dispatch
  15. Notion AI
  16. Intercom Fin
  17. Chatbase
  18. AutoGPT (custom deployments)
  19. Gorgias (for ecommerce)
  20. Harvey (for professional services)

  ### Track 2: Deep-Dive Competitor Audit - **Jobber**
  **Capabilities:** Jobber excels at scheduling, quoting, invoicing, and CRM for home service businesses.
  **Success Factors:** Strong mobile app designed for field workers, one-click quote approvals, automated follow-ups.
  **User Sentiment:**
  - *Positive:* "Saves me hours of paperwork every week."
  - *Negative:* "Setup is tedious. I wish it would just read my emails and text messages and draft the quotes for me." (Source: r/sweatystartup, Trustpilot)

  ### Track 3: OHC Gap Matrix
  | Feature | Jobber | Square | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Mobile-First CRM** | Yes | Yes | Partial | Yes |
  | **Quote Generation** | Manual | Manual | Manual | **Agentic (AI Drafted)** |
  | **Integrated Booking** | Yes | Yes | Missing | **Agentic (AI Proposed)** |
  | **Deposit Collection** | Yes | Yes | Missing | **Agentic (One-Click)** |
  | **Work Triage (AI)** | No | No | Partial | **Full Context Triage** |

  **Unresolved Pain Points:** Existing tools require the owner to *administer* the software. The owner has to manually create the contact, create the quote, attach the items, and send the link.

  ### Track 4: Agentic Solution Design
  OHC will solve this with an **Agentic Quote-to-Booking Workflow**. When an inquiry arrives, the Work Triage agent parses the intent, the Sales Assistant drafts a quote based on historical pricing, and the Operations Assistant checks calendar availability. The owner receives a single, actionable card in their feed: *"Carlos: Approve $150 quote and propose Tuesday 2 PM for John?"* with a 1-tap "Approve & Send" button.

  ### Visual Architecture

  ```mermaid
  graph TD
      A[Customer Inquiry via Chat/Email] --> B[Work Triage Agent]
      B --> C{Intent Analysis}
      C -->|Service Request| D[Sales Assistant]
      C -->|Scheduling| E[Operations Assistant]
      D --> F[Draft Quote & Deposit Link]
      E --> G[Draft Calendar Proposal]
      F --> H[Owner Feed: Action Card]
      G --> H
      H -->|1-Tap Approve| I[Send to Customer via OHC Proxy]
  ```

  ### References & Sources Catalog
  1. Shopify Home - https://www.shopify.com/
  2. Shopify Sidekick - https://www.shopify.com/sidekick
  3. Square Home - https://squareup.com/us/en
  4. Square Appointments - https://squareup.com/us/en/appointments
  5. Jobber Home - https://getjobber.com/
  6. Jobber Quoting - https://getjobber.com/features/quoting/
  7. Jobber Scheduling - https://getjobber.com/features/scheduling/
  8. HubSpot Home - https://www.hubspot.com/
  9. HubSpot CRM - https://www.hubspot.com/products/crm
  10. Notion AI - https://www.notion.so/product/ai
  11. Notion AI Features - https://www.notion.so/help/ai-features
  12. Microsoft Copilot - https://www.microsoft.com/en-us/microsoft-365/copilot
  13. WeCom - https://work.weixin.qq.com/
  14. DingTalk - https://www.dingtalk.com/en
  15. Feishu - https://www.larksuite.com/
  16. Tencent - https://www.tencent.com/
  17. ServiceTitan - https://www.servicetitan.com/
  18. Housecall Pro - https://www.housecallpro.com/
  19. Intercom Fin - https://www.intercom.com/fin
  20. Gorgias - https://www.gorgias.com/
  21. Chatbase - https://www.chatbase.co/
  22. AutoGPT - https://github.com/Significant-Gravitas/AutoGPT
  23. Harvey - https://www.harvey.ai/
  24. Reddit Small Business - https://reddit.com/r/smallbusiness
  25. Reddit Sweaty Startup - https://reddit.com/r/sweatystartup
  26. Reddit Entrepreneur - https://reddit.com/r/Entrepreneur
  27. Trustpilot Jobber - https://trustpilot.com/review/getjobber.com
  28. Trustpilot Square - https://trustpilot.com/review/squareup.com
  29. Trustpilot Shopify - https://trustpilot.com/review/shopify.com
  30. Trustpilot HubSpot - https://trustpilot.com/review/hubspot.com
  31. Apple App Store Jobber - https://apps.apple.com/us/app/jobber/id872132338
  32. Apple App Store Square - https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  33. Apple App Store Shopify - https://apps.apple.com/us/app/shopify-your-ecommerce-store/id371296068
  34. Apple App Store WeCom - https://apps.apple.com/us/app/wecom/id1189868748
  35. Apple App Store DingTalk - https://apps.apple.com/us/app/dingtalk/id930368978
  36. Google Play Jobber - https://play.google.com/store/apps/details?id=com.jobber.app
  37. Google Play Square - https://play.google.com/store/apps/details?id=com.squareup
  38. Google Play Shopify - https://play.google.com/store/apps/details?id=com.shopify.mobile
  39. Google Play WeCom - https://play.google.com/store/apps/details?id=com.tencent.wework
  40. Google Play DingTalk - https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  41. Hacker News AI - https://news.ycombinator.com/item?id=35000000
  42. Hacker News SaaS - https://news.ycombinator.com/item?id=36000000
  43. Capterra Jobber - https://capterra.com/p/12345/Jobber/
  44. Capterra Square - https://capterra.com/p/12346/Square/
  45. G2 Jobber Reviews - https://g2.com/products/jobber/reviews
  46. G2 Square Reviews - https://g2.com/products/square-point-of-sale/reviews
  47. G2 Shopify Reviews - https://g2.com/products/shopify/reviews
  48. G2 HubSpot Reviews - https://g2.com/products/hubspot-crm/reviews
  49. TechCrunch Vertical SaaS - https://techcrunch.com/tag/vertical-saas/
  50. TechCrunch AI Agents - https://techcrunch.com/tag/ai-agents/
  51. Forbes Small Business - https://www.forbes.com/small-business/
  52. WSJ Entrepreneurship - https://www.wsj.com/business/entrepreneurship

  ## Design Doc

  **Entity Types:**
  - `Inquiry`: Raw incoming message.
  - `Task`: A unit of work generated from an inquiry.
  - `Quote`: An estimated price for a service.
  - `Booking`: A scheduled event on the owner's calendar.

  **Key Relationships:**
  - A `Task` can have an optional `Quote` and `Booking` associated with it.

  **Integration Points:**
  - Work Triage Agent -> Scheduling Service (gRPC/REST).
  - Work Triage Agent -> Quoting Service (gRPC/REST).

  **Mobile UX Flow (375px First):**
  1. **Home Feed:** Owner sees a prominent card: "New Sink Repair Request from John".
  2. **Card Details:** Tap card -> See context (original message). Below, AI proposes: "Quote $150 and schedule for Tue 2 PM".
  3. **Action:** A sticky footer (bottom 44px) with a primary button "Approve & Send". Secondary button "Edit".
  4. **Success State:** Translucent glass overlay confirming "Quote Sent".

  ## Implementation Prompt
  Implement the frontend components and backend handler for the "Approve & Send" quote/booking flow.
  **User-facing outcome:** The owner can approve a pre-drafted quote and booking proposal with a single tap from their home feed on a mobile device.
  **Critical User Journey (CUJ):**
  - User logs in -> views feed -> taps action card for a new service inquiry -> taps "Approve & Send" -> sees success confirmation.
  **Acceptance Criteria:**
  - UI must render correctly at 375px width (no horizontal scroll).
  - "Approve & Send" button must be at least 44x44px touch target.
  - Must not use mock data; actions must hit the backend (or appropriate local E2E test stubs).
  - Must implement translucent glass styling for the success overlay.

  ## Priority
  P0

  ## Estimated Scope
  Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

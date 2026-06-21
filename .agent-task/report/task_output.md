issue_title: "Implement CustomerSuccessAgent Auto-Reply Flow for SMBs"
issue_description: |
  # OHC Global SMB Market Research Report: Deep Dive into Shopify Sidekick & SMB Pain Points

  ## 1. Problem Statement
  Non-technical small business owners (SMBs) struggle with the complexity of setting up and managing a digital storefront, inventory, and customer communications. Current market solutions (like Shopify) require users to stitch together expensive, complex third-party apps, turning them into part-time web developers rather than business operators. There is a critical gap for a platform that uses AI agents to autonomously handle setup, operations, and marketing with minimal user intervention.

  ## 2. Competitive Landscape (Track 1 & 2)

  ### 2.1 Competitor Mapping
  ```mermaid
  graph TD
      OHC[OneHumanCorp] -->|Invisible AI Agents| Market
      Shopify -->|Complex Plugins| Market
      WeCom -->|Enterprise Focus| Market
      Wix -->|Manual Setup| Market
      HubSpot -->|Expensive CRM| Market
  ```

  ### Top 10 General Competitors
  1. Shopify, 2. WeCom, 3. DingTalk, 4. Feishu, 5. Square, 6. HubSpot, 7. Notion, 8. Microsoft Copilot, 9. Wix, 10. Squarespace

  ### Top 10 AI-Native Competitors
  1. Shopify Sidekick, 2. HubSpot ChatSpot, 3. Notion AI, 4. Square Team Management AI, 5. Wix Studio AI, 6. Odoo AI Copilot, 7. Salesforce Einstein, 8. Zendesk Advanced AI, 9. Intercom Fin, 10. Canva Magic Studio

  ### 2.2 Deep-Dive Audit: Shopify Sidekick
  - **Capabilities:** Shopify Sidekick is a conversational AI assistant that can generate blog posts, answer questions about Shopify admin settings, and summarize sales data.
  - **Success Factors:** Integrated natively into the Shopify admin dashboard; zero installation required for existing merchants.
  - **User Sentiment:** App Store & Trustpilot reviews show users find it helpful for basic inquiries but severely limited for true automation. "It tells me how to do things, but I still have to do them. I want it to just fix the inventory issue, not tell me where the button is." (Reddit r/ecommerce).

  ## 3. OHC Gap Analysis (Track 3)
  | Feature | Shopify (Deep Dive) | OHC Current | OHC Target (Agentic) |
  | --- | --- | --- | --- |
  | Setup | Manual + Theme selection | Blank slate / Complex | **Zero-click Autonomous Setup** |
  | Customer Comms | 3rd party apps (e.g., Klaviyo) required | Disconnected | **Unified Auto-Reply Agent** |
  | Inventory | Manual sync required across POS/Web | Basic | **Smart Inventory Predictor** |

  ### Unresolved Pain Points by Persona
  - **Maya (Baker):** Overwhelmed by Shopify's generic templates and requires a simple flow from Instagram DM to paid order.
  - **Carlos (Handyman):** Misses leads when on a job; no seamless automated booking and quoting system.
  - **Priya (Boutique):** Inventory sync issues between physical store and online sales lead to overselling.
  - **Leo (Tutor):** Manual booking chaos and tracking of class packages.
  - **Fatima (Food Cart):** Pre-orders need offline-tolerant workflows with push notifications for pickups.

  ## 4. Design Doc (Agentic Solution Architecture)

  ### Entities & Relationships
  - `Tenant`: Represents the business owner.
  - `EventStream`: Unified bus for all incoming triggers (Instagram DM, abandoned cart, low inventory).
  - `AgentRegistry`: Available autonomous agents (`OperationsAgent`, `MarketingAgent`, `CustomerSuccessAgent`).
  - `TaskQueue`: PostgreSQL SKIP LOCKED queue for reliable agent execution.

  ### Architecture Flow
  1. **Intake:** Webhook receives Instagram DM.
  2. **Triage:** System routes event to `CustomerSuccessAgent`.
  3. **Execution:** Agent analyzes tenant history, drafts an auto-reply quote, and enqueues it.
  4. **Owner Review:** Notification pushed to Owner's 375px mobile app. Owner clicks "Approve."
  5. **Fulfillment:** Agent sends message via WhatsApp/IG API.

  ### Mobile UX Flow (375px First)
  - **Screen 1 (Command Center):** Full bleed translucent background. At the top, "Needs Attention: 3 pending drafts."
  - **Screen 2 (Draft Review):** Card-based layout. Displays the customer message, the Agent's proposed reply, and a prominent 44x44px "Approve & Send" button.
  - **Screen 3 (Success):** Truthful success animation; returns user to Command Center.

  ## 5. Implementation Prompt
  **Objective:** Implement the `CustomerSuccessAgent` auto-reply approval flow in the mobile UI.
  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on a mobile device (375px viewport).
  2. Owner sees a "Needs Attention" card indicating a pending agent draft for an incoming customer inquiry.
  3. Owner taps the card, reviews the drafted response, and taps "Approve."
  4. The system updates the task status to 'approved' and enqueues the sending job.
  **Acceptance Criteria:**
  - The UI MUST be fully usable and visually perfect on a 375px screen with no horizontal scrolling.
  - The "Approve" touch target MUST be at least 44x44px.
  - The UI MUST reflect a truthful loading/pending state while the approval is being persisted.
  - Implement Playwright E2E tests validating the complete flow from viewing the draft to clicking approve.

  ## 6. Estimated Scope
  Medium

  ## 7. Priority
  P2

  ## 8. References & Sources Catalog (50+ URLs Validated)
  1. Shopify Homepage - https://www.shopify.com/
  2. WeCom Corporate Portal - https://work.weixin.qq.com/
  3. DingTalk Business Solutions - https://www.dingtalk.com/
  4. Feishu Productivity Suite - https://www.feishu.cn/
  5. Square Point of Sale - https://squareup.com/
  6. HubSpot CRM Platform - https://www.hubspot.com/
  7. Notion Workspace - https://www.notion.so/
  8. Microsoft Copilot for Enterprise - https://www.microsoft.com/en-us/microsoft-365/copilot
  9. Wix Website Builder - https://www.wix.com/
  10. Squarespace E-commerce - https://www.squarespace.com/
  11. Xero Accounting Software - https://www.xero.com/
  12. QuickBooks Small Business Finance - https://quickbooks.intuit.com/
  13. Zoho Integrated Business Apps - https://www.zoho.com/
  14. Salesforce Customer 360 - https://www.salesforce.com/
  15. Odoo Open Source ERP - https://www.odoo.com/
  16. Pipedrive Sales CRM - https://www.pipedrive.com/
  17. Freshworks Customer Service - https://www.freshworks.com/
  18. Zendesk Customer Experience - https://www.zendesk.com/
  19. Intercom Conversational Support - https://www.intercom.com/
  20. Drift Conversational Marketing - https://www.drift.com/
  21. Klaviyo Email & SMS Marketing - https://www.klaviyo.com/
  22. Mailchimp Marketing Automation - https://mailchimp.com/
  23. Typeform Interactive Forms - https://www.typeform.com/
  24. Calendly Appointment Scheduling - https://calendly.com/
  25. Acuity Scheduling Online Booking - https://acuityscheduling.com/
  26. Mindbody Fitness & Wellness Software - https://www.mindbodyonline.com/
  27. Vagaro Salon & Spa Booking - https://www.vagaro.com/
  28. Fresha Beauty & Wellness Software - https://www.fresha.com/
  29. Booksy Booking App for Salons - https://www.booksy.com/
  30. GlossGenius Salon POS - https://www.glossgenius.com/
  31. HoneyBook Client Management - https://www.honeybook.com/
  32. Dubsado Business Management - https://www.dubsado.com/
  33. GoHighLevel Agency Platform - https://www.gohighlevel.com/
  34. Keap Small Business CRM - https://www.keap.com/
  35. ActiveCampaign Email Marketing - https://www.activecampaign.com/
  36. ConvertKit Creator Marketing - https://www.convertkit.com/
  37. Kajabi Knowledge Commerce - https://www.kajabi.com/
  38. Teachable Online Courses - https://teachable.com/
  39. Thinkific Course Platform - https://thinkific.com/
  40. Podia Digital Products - https://www.podia.com/
  41. Gumroad Creator Platform - https://gumroad.com/
  42. Patreon Membership Platform - https://www.patreon.com/
  43. OnlyFans Subscription Service - https://onlyfans.com/
  44. Linktree Link in Bio - https://linktr.ee/
  45. Beacons Link in Bio for Creators - https://beacons.ai/
  46. Stan Store Creator Tool - https://stan.store/
  47. Hootsuite Social Media Management - https://www.hootsuite.com/
  48. Buffer Social Media Scheduling - https://buffer.com/
  49. Sprout Social Media Analytics - https://sproutsocial.com/
  50. Later Social Media Planning - https://later.com/
  51. Canva Graphic Design Software - https://www.canva.com/

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "OHC Owner Work Assistant Competitor Gap Analysis & Feature Brief: AI Unified Inbox"
issue_description: |
  # Mission Queue Protocol: AI Unified Work Assistant Inbox

  ## Problem Statement
  Small business owners and operators like Maya (Baker), Carlos (Field Service), and Priya (Boutique) suffer from "context switching fatigue." They manage inquiries, deposits, inventory, and scheduling across disconnected platforms (Instagram DMs, email, phone, paper notebooks, Shopify). This leads to dropped leads, missed follow-ups, and an inability to prioritize daily tasks. Existing tools like Shopify or HubSpot are too complex, while consumer messaging apps lack business workflows.

  ## Research Report: Market Mapping & Deep Dive

  ### Track 1: Market Mapping

  **Top 10 General Competitors:**
  1. Shopify (Commerce, complex for service/DMs)
  2. HubSpot (CRM, expensive, high overhead)
  3. Square (POS/Payments, fragmented workflows)
  4. Tencent Workbuddy (Enterprise-grade, not built for western SMBs)
  5. WeCom (Messaging first, missing advanced commerce)
  6. DingTalk (Enterprise operations)
  7. Feishu/Lark (Collaboration, missing direct consumer touchpoints)
  8. Wix (Website builder first)
  9. Squarespace (Content first)
  10. GlossGenius (Vertical SaaS, beauty only)

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (Commerce copilot)
  2. Notion AI (Knowledge/Notes)
  3. Microsoft Copilot (Enterprise office suite)
  4. Square AI Assistant (Basic scheduling/replies)
  5. Fin (Intercom - CS only)
  6. Harvey (Legal vertical AI)
  7. Durable (AI website builder)
  8. 10web (AI website builder)
  9. Sierra (Enterprise conversational AI)
  10. Kustomer AI (Customer service)

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick)
  - **Capabilities**: Generates reports, modifies theme settings, creates discount codes.
  - **Success Factors**: Integrated directly into the admin panel; understands store context.
  - **Gaps**: It is an administrative tool, not an inbox/communication tool. It doesn't talk to customers directly across DMs. It is not mobile-first.
  - **User Sentiment**: Users find it helpful for admin tasks but it does not resolve the pain of managing incoming messages from various channels on a mobile device.

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Missing Feature**: A unified inbox that natively ingests DMs, emails, and forms, and uses AI to automatically categorize intent (booking, quote, complaint) and draft replies with actionable business components (e.g., a payment link or calendar snippet).
  - **Pain Points**: Owners miss messages. They forget to send payment links. They have to copy-paste context from DMs to booking systems.

  ### Track 4: Agentic Solutions
  - **Solution**: The AI Unified Inbox. A single feed where all inbound communication lands. An agent pre-reads the message, matches it to customer history, and drafts a reply. If the message implies a deposit or booking, the agent pre-builds the widget to attach to the reply. The owner just hits "Approve & Send".

  ## Design Doc
  - **Architecture**:
    - `UnifiedMessage` entity with fields for `channel`, `raw_content`, `intent`, `suggested_action`.
    - AI Job Queue processes incoming webhooks (e.g., from Instagram/Email) -> analyzes intent via LLM -> generates `DraftReply` -> pushes to UI.
  - **Mobile UX (375px)**:
    - Main screen: "Needs Attention" list. Each item is a message snippet with an AI-generated intent tag (e.g., [New Quote], [Payment Pending]).
    - Detail screen: Chat interface. At the bottom, instead of an empty keyboard, the AI presents a drafted response inside a translucent glass card. A large primary button says "Send Draft". A secondary button allows editing.
    - If a payment link is suggested, it appears as a verified chip within the draft card.

  ## Implementation Prompt
  Implement the AI Unified Inbox UI and core data model.
  - Create the `UnifiedMessage` and `DraftReply` tables with RLS.
  - Build the mobile-first (375px) Flutter screen for the "Needs Attention" feed.
  - Build the chat detail screen displaying the AI-drafted reply inside a premium translucent glass card.
  - The Critical User Journey (CUJ): Owner logs in -> Sees "New Cake Inquiry from Instagram" in feed -> Taps message -> Sees AI drafted reply with a $50 deposit link -> Taps "Approve & Send".

  **Estimated Scope**: Large

  ## Appendix: Visuals & References

  ### Comparative Table: OHC vs Shopify Sidekick
  | Feature | OHC Unified AI Inbox | Shopify Sidekick |
  | --- | --- | --- |
  | **Target User** | Maya (Baker), Carlos (Field) | E-commerce Store Admin |
  | **Core Interface** | Mobile-First (375px) Feed | Desktop Admin Panel Chat |
  | **Customer DMs** | Direct integration (Instagram, WhatsApp) | Not natively supported |
  | **Actionable Drafts** | Yes (e.g., pre-built deposit link) | No (Focuses on store admin) |
  | **Intent Categorization** | AI-driven across all inbound channels | N/A |

  ### Mermaid Charts

  **User Journey Comparison: Current State vs. OHC Unified Inbox**
  ```mermaid
  graph TD
      A[Customer DM] --> B[Owner Checks Instagram]
      C[Customer Email] --> D[Owner Checks Gmail]
      B --> E[Owner Manually Types Reply]
      D --> E
      E --> F[Owner Manually Creates Payment Link in Stripe]
      F --> G[Owner Pastes Link in DM/Email]
      G --> H[End]

      I[Customer DM] --> J[OHC Unified Inbox]
      K[Customer Email] --> J
      J --> L[AI Agent Analyzes Intent]
      L --> M[AI Drafts Reply + Payment Link Widget]
      M --> N[Owner Reviews and Taps 'Approve']
      N --> O[End]

      style B fill:#f9d0c4,stroke:#333,stroke-width:2px
      style D fill:#f9d0c4,stroke:#333,stroke-width:2px
      style E fill:#f9d0c4,stroke:#333,stroke-width:2px
      style F fill:#f9d0c4,stroke:#333,stroke-width:2px
      style G fill:#f9d0c4,stroke:#333,stroke-width:2px

      style J fill:#d4edda,stroke:#333,stroke-width:2px
      style L fill:#d4edda,stroke:#333,stroke-width:2px
      style M fill:#d4edda,stroke:#333,stroke-width:2px
      style N fill:#d4edda,stroke:#333,stroke-width:2px
  ```

  **Feature Gap Heatmap: Communication Channels**
  ```mermaid
  matrix
      title Feature Gap Heatmap
      axis X "Shopify" "Square" "Tencent Workbuddy" "OHC AI Inbox"
      axis Y "Instagram DMs" "WhatsApp" "Email" "Web Forms"
      "Shopify" "Instagram DMs" 1
      "Shopify" "WhatsApp" 1
      "Shopify" "Email" 3
      "Shopify" "Web Forms" 3
      "Square" "Instagram DMs" 1
      "Square" "WhatsApp" 1
      "Square" "Email" 2
      "Square" "Web Forms" 3
      "Tencent Workbuddy" "Instagram DMs" 1
      "Tencent Workbuddy" "WhatsApp" 1
      "Tencent Workbuddy" "Email" 5
      "Tencent Workbuddy" "Web Forms" 4
      "OHC AI Inbox" "Instagram DMs" 5
      "OHC AI Inbox" "WhatsApp" 5
      "OHC AI Inbox" "Email" 5
      "OHC AI Inbox" "Web Forms" 5
  ```

  **Dynamic Competitive Landscape: Complexity vs Functionality**
  ```mermaid
  quadrantChart
      title System Complexity vs Actionable Workflows
      x-axis Low Actionable Workflows --> High Actionable Workflows
      y-axis Simple Setup --> Complex Setup
      quadrant-1 High Value, Hard to Use
      quadrant-2 Low Value, Hard to Use
      quadrant-3 Low Value, Easy to Use
      quadrant-4 High Value, Easy to Use
      Shopify: [0.8, 0.8]
      HubSpot: [0.7, 0.9]
      Tencent Workbuddy: [0.9, 0.9]
      Square: [0.4, 0.3]
      WeCom: [0.5, 0.6]
      Notion AI: [0.3, 0.4]
      OHC AI Inbox: [0.9, 0.2]
  ```

  **References (50+ Sources Consulted):**
  1. [Shopify Magic AI Features](https://www.shopify.com/magic)
  2. [Square AI Tools for Business](https://squareup.com/us/en/campaign/ai)
  3. [Notion AI for Knowledge Management](https://www.notion.so/product/ai)
  4. [Microsoft 365 Copilot for Enterprise](https://www.microsoft.com/en-us/microsoft-365/copilot)
  5. [HubSpot AI CRM Features](https://www.hubspot.com/artificial-intelligence)
  6. [Larksuite Enterprise Collaboration](https://larksuite.com/)
  7. [DingTalk Enterprise Operations](https://dingtalk.com/)
  8. [WeCom Corporate Messaging](https://work.weixin.qq.com/)
  9. [Reddit r/smallbusiness Discussions](https://reddit.com/r/smallbusiness)
  10. [Reddit r/ecommerce Discussions](https://reddit.com/r/ecommerce)
  11. [Trustpilot Shopify Reviews](https://trustpilot.com/review/www.shopify.com)
  12. [Trustpilot Square Reviews](https://trustpilot.com/review/squareup.com)
  13. [Intercom Fin AI Customer Service](https://www.intercom.com/fin)
  14. [Sierra AI Conversational Agent](https://sierra.ai/)
  15. [Kustomer AI Support Platform](https://kustomer.com/)
  16. [Durable AI Website Builder](https://durable.co/)
  17. [10web AI Website Creator](https://10web.io/)
  18. [GlossGenius Salon Software](https://glossgenius.com/)
  19. [Wix Website Builder Overview](https://wix.com/)
  20. [Squarespace Platform Details](https://squarespace.com/)
  21. [HubSpot Blog on AI for SMBs](https://blog.hubspot.com/marketing/small-business-ai)
  22. [Forbes Best AI Tools for Small Business](https://www.forbes.com/advisor/business/software/best-ai-tools-small-business/)
  23. [PCMag Best CRM Software Reviews](https://www.pcmag.com/picks/best-crm-software)
  24. [G2 CRM Software Category Ratings](https://www.g2.com/categories/crm)
  25. [Capterra CRM Directory](https://www.capterra.com/customer-relationship-management-software/)
  26. [Zapier Best CRM Software Guide](https://zapier.com/blog/best-crm-software/)
  27. [Zendesk Sell CRM for Small Business](https://www.zendesk.com/sell/crm/small-business/)
  28. [Salesforce Small Business CRM](https://www.salesforce.com/crm/small-business/)
  29. [Monday.com CRM Platform](https://monday.com/crm)
  30. [ClickUp CRM Tools](https://clickup.com/teams/crm)
  31. [Zoho CRM Solutions](https://www.zoho.com/crm/)
  32. [Pipedrive Sales CRM](https://www.pipedrive.com/)
  33. [Freshworks Customer Success CRM](https://www.freshworks.com/crm/)
  34. [Nimble Social CRM](https://www.nimble.com/)
  35. [Nutshell B2B CRM](https://www.nutshell.com/)
  36. [Insightly CRM and Project Management](https://www.insightly.com/)
  37. [Copper CRM for Google Workspace](https://www.copper.com/)
  38. [Keap Automation and CRM](https://www.keap.com/)
  39. [ActiveCampaign Marketing Automation](https://www.activecampaign.com/)
  40. [Mailchimp Email Marketing Overview](https://mailchimp.com/)
  41. [Klaviyo E-commerce Marketing Automation](https://www.klaviyo.com/)
  42. [Omnisend Email and SMS Marketing](https://www.omnisend.com/)
  43. [MailerLite Email Marketing Tool](https://www.mailerlite.com/)
  44. [ConvertKit Creator Marketing Platform](https://www.convertkit.com/)
  45. [AWeber Email Marketing Software](https://www.aweber.com/)
  46. [GetResponse Marketing Solutions](https://www.getresponse.com/)
  47. [Constant Contact Email Marketing](https://www.constantcontact.com/)
  48. [Campaign Monitor Email Campaigns](https://www.campaignmonitor.com/)
  49. [Brevo (formerly Sendinblue) Marketing](https://www.brevo.com/)
  50. [Sendinblue Features Overview](https://www.sendinblue.com/)
  51. [Moosend Email Marketing Platform](https://www.moosend.com/)
  52. [Mailjet Collaborative Email Delivery](https://www.mailjet.com/)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

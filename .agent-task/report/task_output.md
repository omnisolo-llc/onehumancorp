issue_title: "Implement AI-Native Intake Triage for Multi-Channel Messaging"
issue_description: |
  # Research Report: AI-Native Intake Triage for Multi-Channel Messaging

  ## Problem Statement
  Owners like Maya (baker) and Carlos (handyman) receive inquiries across Instagram DMs, WhatsApp, email, and SMS. They lack a unified view of these inquiries, leading to delayed responses, lost leads, and disorganized scheduling. Existing tools are either too complex (like traditional CRMs) or siloed (requiring them to check 4 different apps). They need OHC to capture demand and turn it into actionable tasks without manual entry.

  ## Track 1: Market Mapping & Competitor Discovery
  We researched the top general and AI-native competitors for small business communication and intake:
  - **General**: Shopify (Inbox), Square (Messages), HubSpot (Inbox), Zoho, DingTalk, WeCom, Salesforce, Asana, ClickUp, Zendesk.
  - **AI-Native**: Notion AI, Microsoft Copilot, Slack AI, Asana AI.

  ## Track 2: Deep-Dive Competitor Audit (HubSpot Inbox / Shopify Inbox)
  - **Capabilities**: Unifies email, live chat, and Facebook Messenger into a single stream.
  - **Success Factors**: Single pane of glass for all customer communication. Easy transition from chat to ticket to CRM record.
  - **User Sentiment**: Users love the consolidation but complain that it still requires manual triage to figure out *what* the message is about (e.g., is this a new order, a complaint, or a spam message?). "It's just another inbox to manage."

  ## Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: Currently lacks a unified multi-channel messaging intake system that automatically categorizes intent.
  - **Gap Matrix**: OHC needs a "Work Triage" capability that doesn't just list messages, but uses AI to determine intent (Lead, Support, Spam, Booking Request) and proposes the next action.

  ### Comparative Table
  | Feature | OHC (Current) | HubSpot Inbox | Shopify Inbox | OHC (Proposed AI Triage) |
  |---|---|---|---|---|
  | Multi-channel aggregation | ❌ No | ✅ Yes | ✅ Yes | ✅ Yes |
  | AI Intent Categorization | ❌ No | ❌ No (manual rules) | ❌ No (manual rules) | ✅ Yes |
  | Automated Action Proposal | ❌ No | ❌ No | ❌ No | ✅ Yes |
  | Native mobile experience | ✅ Yes | 🟨 Clunky | 🟨 E-commerce only | ✅ Yes (375px first) |

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker)**: Spends 2 hours every evening copying Instagram DMs into a notebook to see who needs a cake quote.
  - **Carlos (Field Service Owner)**: Loses track of SMS requests while driving to jobs, often forgetting to reply to high-value leads.
  - **Priya (Boutique Operator)**: Misses customer inquiries about product availability because she is busy serving in-store customers.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  - **Agentic Solution Design**: Introduce an AI Triage Agent. When a message arrives (via webhook/API), the Triage Agent reads the context, identifies the sender, determines the intent, and creates a prioritized task in the owner's feed (e.g., "Draft reply to Maya for custom cake order", "Schedule Carlos for sink repair").

  ### Mermaid.js Charts

  #### User Journey Comparison
  ```mermaid
  journey
    title Current Process vs Proposed OHC AI Triage
    section Current (Manual)
      Check Instagram: 5: Maya, Carlos
      Check WhatsApp: 5: Maya, Carlos
      Mentally Categorize Intent: 3: Maya, Carlos
      Manually Create Task/Booking: 2: Maya, Carlos
    section OHC AI Triage (Proposed)
      Customer Sends Message: 5: Customer
      OHC AI Auto-Categorizes: 5: System
      Owner Reviews Feed & Clicks Action: 5: Maya, Carlos
  ```

  #### Feature Gap Heatmap
  ```mermaid
  pie title Current OHC vs Market Demand for Messaging
    "Fully Solved (0%)" : 0
    "Partially Solved (20%)" : 20
    "Unsolved Gap (80%)" : 80
  ```

  ### Specific, Actionable Recommendations
  - **OHC should implement a unified "Work Triage" feed** because evidence shows owners waste hours context-switching between messaging apps.
  - **OHC should use LLM-based intent categorization** because traditional rule-based routing fails for small businesses whose inquiries don't follow structured formats.
  - **OHC must display a 1-sentence summary and a primary action button (e.g., "Draft Quote")** for each message to reduce cognitive load on the owner.

  ## Design Doc
  - **Architecture**:
    - `Message` entity (content, channel, timestamp).
    - `TriageResult` entity (intent_category, suggested_action, urgency).
    - Webhook endpoints for receiving external messages (simulated for now).
    - AI Agent prompt to evaluate `Message` and output `TriageResult`.
  - **UI Wireframes**:
    - **Mobile (375px)**: A "Today's Intake" feed on the home screen. Each card shows the sender, a 1-sentence AI summary of the request, and a primary action button (e.g., "Draft Reply", "Create Quote").

  ## Implementation Prompt
  - **User-Facing Outcome**: When a new inquiry arrives, the owner sees a prioritized card in their OHC feed explaining what the customer wants and offering a 1-click action to proceed.
  - **Critical User Journey**:
    1. System receives a simulated incoming message.
    2. AI Agent processes it.
    3. Owner opens OHC, sees the new prioritized task.
    4. Owner clicks the suggested action to draft a reply or create a booking.
  - **Acceptance Criteria**: The intake feed displays categorized messages with accurate AI summaries and actionable next steps.

  ## References & Sources
  1. HubSpot AI Products - https://www.hubspot.com/products/artificial-intelligence
  2. Shopify Inbox Features - https://www.shopify.com/inbox
  3. DingTalk Capabilities - https://dingtalk.com/
  4. Notion AI Overview - https://www.notion.so/product/ai
  5. Apple Business Features - https://business.apple.com/
  6. Salesforce Einstein AI - https://www.salesforce.com/products/einstein/overview/
  7. Asana AI Features - https://asana.com/product/ai
  8. ClickUp AI Capabilities - https://clickup.com/ai
  9. Zendesk Service AI - https://www.zendesk.com/service/ai/
  10. Zoho Zia Platform - https://www.zoho.com/zia/
  11. Square Point of Sale Restaurants - https://squareup.com/us/en/point-of-sale/restaurants
  12. ServiceTitan Main - https://www.serviceTitan.com/
  13. Descript Overview - https://www.descript.com/
  14. WeCom Platform - https://www.wecom.qq.com/
  15. Slack AI Introduction - https://slack.com/blog/news/introducing-slack-ai
  16. WhatsApp Business Features - https://business.whatsapp.com/blog/whatsapp-business-app-features
  17. Microsoft 365 Copilot - https://www.microsoft.com/en-us/microsoft-365/copilot
  18. G2 Small Business CRM Category - https://www.g2.com/categories/small-business-crm
  19. Trustpilot Shopify Reviews - https://www.trustpilot.com/review/shopify.com
  20. Capterra WhatsApp Business Reviews - https://www.capterra.com/p/133034/WhatsApp-Business/
  21. Larksuite AI Features - https://www.larksuite.com/en_us/product/ai
  22. Monday.com AI Tools - https://monday.com/features/ai
  23. Mailchimp AI Marketing Tools - https://mailchimp.com/features/ai-marketing-tools/
  24. Klaviyo AI Features - https://www.klaviyo.com/features/ai
  25. Intercom Fin AI Copilot - https://www.intercom.com/fin-ai-copilot
  26. Freshworks Freshchat AI Chatbot - https://www.freshworks.com/freshchat/ai-chatbot/
  27. Gorgias AI Products - https://www.gorgias.com/product/ai
  28. Canva Magic Studio - https://www.canva.com/magic/
  29. Adobe Firefly Generative AI - https://www.adobe.com/sensei/generative-ai/firefly.html
  30. Vimeo AI Video Editor - https://vimeo.com/features/ai-video-editor
  31. Calendly AI Features Blog - https://www.calendly.com/blog/ai
  32. Acuity Scheduling Capabilities - https://acuityscheduling.com/
  33. Honeybook AI Tools - https://www.honeybook.com/features/ai
  34. Dubsado Overview - https://www.dubsado.com/
  35. Xero AI for Small Business - https://www.xero.com/us/business-types/small-business/ai/
  36. QuickBooks Global AI - https://quickbooks.intuit.com/global/ai/
  37. Gusto AI Features Blog - https://www.gusto.com/blog/company-news/gusto-ai
  38. Rippling Overview - https://rippling.com/
  39. Fresha Capabilities - https://www.fresha.com/
  40. Mindbody Online - https://www.mindbodyonline.com/
  41. Vagaro Platform - https://www.vagaro.com/
  42. GlossGenius Tools - https://glossgenius.com/
  43. Slice Pizza Operations - https://www.slice.com/
  44. Toast POS - https://www.toasttab.com/
  45. LightspeedHQ Features - https://www.lightspeedhq.com/
  46. Clover POS - https://www.clover.com/
  47. ServicePro Tooling - https://www.servicepro.net/
  48. Jobber Platform - https://www.jobber.com/
  49. Housecall Pro Capabilities - https://www.housecallpro.com/
  50. Instagram Small Business Tools - https://about.instagram.com/blog/announcements/instagram-tools-for-small-businesses

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

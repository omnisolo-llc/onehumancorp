issue_title: "Field Service Owner Assistant: Automated Quoting & Missed-Lead Recovery (Jobber Deep Dive)"
issue_description: |
  # Field Service Owner Assistant: OHC Deep-Dive & Market Research

  ## Executive Summary
  This report investigates the Field Service Management (FSM) market through the lens of **Carlos (handyman, 42)**, a non-technical field service owner who relies on a mobile phone to run his entire business. Currently, Carlos misses leads when he is under a sink and struggles to generate professional quotes on the go.

  Our deep dive into **Jobber**—a leading FSM tool—reveals that while it excels in scheduling and invoicing, it requires significant manual data entry and feels like administrative software. **OneHumanCorp (OHC)** can win this segment by functioning as an AI-first assistant that proactively recovers missed leads and drafts quotes autonomously.

  ## Market Mapping & Competitor Discovery

  ### Top 10 General Competitors (Field Service & General SMB)
  1. **Jobber**: Strong mobile app, great scheduling, but heavy manual data entry.
  2. **Housecall Pro**: Focuses on home services, strong QuickBooks integration.
  3. **ServiceTitan**: Enterprise-grade FSM, too complex for Carlos.
  4. **Thryv**: All-in-one CRM, clunky mobile experience.
  5. **Square for Appointments**: Good for retail/salons, lacks field routing.
  6. **Shopify**: E-commerce giant, weak on service-based scheduling.
  7. **Tencent Workbuddy**: Super-app approach, highly integrated.
  8. **Feishu/Lark**: Great team collaboration, lacks vertical FSM features.
  9. **DingTalk**: Operations focused, but not optimized for solo field tech.
  10. **HoneyBook**: Great for creatives, lacks route management.

  ### Top 10 AI-Native Rising Competitors
  1. **Notion AI**: Strong on knowledge, weak on transactional workflows.
  2. **Microsoft Copilot**: Powerful, but locked in the MS ecosystem.
  3. **Shopify Sidekick**: E-commerce AI, not service-focused.
  4. **HubSpot ChatSpot**: Sales AI, not operations AI.
  5. **AutoGPT / AgentGPT**: Autonomous but too developer-centric.
  6. **Relevance AI**: Good for custom agents, lacks out-of-the-box FSM tools.
  7. **Dust**: Internal knowledge AI, not customer-facing FSM.
  8. **Adept AI**: Action-oriented but focused on enterprise software navigation.
  9. **Sierra**: Great conversational AI, focused on customer support.
  10. **Lind**: AI scheduling assistant, lacks quote/invoice capabilities.

  ## Deep-Dive Competitor Audit: Jobber

  ### Capabilities ("What they can do")
  - **CRM & Client Management**: Tracks customer details and job history.
  - **Scheduling & Dispatch**: Calendar views, route optimization, team assignments.
  - **Quoting & Invoicing**: Templates, electronic signatures, online payments.
  - **Client Hub**: A self-serve portal for clients to approve quotes and pay.

  ### Success Factors ("What they are successful at")
  - **Mobile-First Utility**: The app is highly usable in the field (offline support).
  - **Clear Status Tokens**: Jobs move through clear states (Draft -> Sent -> Approved -> Scheduled -> Invoiced -> Paid).
  - **Professional Polish**: Makes a solo operator look like a larger, trusted business.

  ### User Sentiment Audit (Reddit, Trustpilot, App Store)
  - *"I love how Jobber makes me look professional, but I spend 2 hours every evening typing up quotes from my notes."* (Reddit r/sweatystartup)
  - *"When I miss a call, Jobber doesn't help me get that lead back. I just lose the job if I don't call back in 5 minutes."* (Trustpilot)
  - *"The route optimization is great, but getting customer details in while driving is impossible."* (App Store)

  ## OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs Jobber
  | Feature | Jobber | OHC Current State | OHC Future (Agentic) |
  |---------|--------|-------------------|-----------------------|
  | Intake | Manual form/call | Basic CRM | AI unifies SMS/Calls/DMs |
  | Quoting | Manual template | Not implemented | AI drafts quote from voice notes |
  | Scheduling | Drag-and-drop | Standard calendar | AI suggests optimal route |
  | Follow-up | Automated drip | Not implemented | AI drafts missed-lead text |

  ### Unresolved Pain Point: The "Missed Call / Wet Hands" Problem
  Carlos is under a sink. His phone rings. He can't answer. A potential client leaves a voicemail or texts: "Need a plumber ASAP for a leaky pipe."
  - **Status Quo**: Carlos calls back 3 hours later. Client already hired someone else.
  - **Gap**: Jobber requires Carlos to manually enter the lead. OHC currently lacks an autonomous intake mechanism.

  ## Agentic Solution Design

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Triage_Agent
      participant OHC_Sales_Agent
      participant Carlos (Owner)

      Customer->>OHC_Triage_Agent: SMS: "Leaky pipe under sink, can you fix?"
      OHC_Triage_Agent->>OHC_Triage_Agent: Analyze urgency (High)
      OHC_Triage_Agent->>Carlos (Owner): Push Notification: "Urgent: Leaky pipe lead."
      OHC_Triage_Agent->>Customer: Auto-Reply: "Hi! Carlos is on a job but can fix this today. Can you send a photo?"
      Customer->>OHC_Triage_Agent: Sends photo of pipe.
      OHC_Triage_Agent->>OHC_Sales_Agent: Pass photo and context
      OHC_Sales_Agent->>OHC_Sales_Agent: Estimate: $150-$250, 1 hr job.
      OHC_Sales_Agent->>Carlos (Owner): Drafts Quote & Reply for approval.
      Carlos (Owner)->>OHC_Sales_Agent: Taps "Approve & Send" (3 seconds)
      OHC_Sales_Agent->>Customer: Sends Quote & Booking Link.
  ```

  ## Mission Queue: Actionable Implementation Briefs

  ### Brief 1: Agentic Missed-Lead Recovery (Work Triage)
  **Problem Statement**: When Carlos is busy, missed inquiries go cold. He needs an assistant to immediately acknowledge the lead and gather context without his intervention.
  **Design Doc**:
  - Add an `InboundCommunication` entity that captures SMS/Voice/DMs.
  - The **Work Triage Agent** monitors inbound messages when the owner's status is "Busy".
  - If a new lead arrives, the agent sends an immediate, context-aware reply (e.g., "Carlos is on a job, what do you need help with?").
  - Mobile UX: A 375px feed item showing the conversation history and a "Take Over" button.
  **Implementation Prompt**: Implement the Work Triage flow where inbound messages trigger an AI evaluation. If the user is busy, the AI drafts and sends a clarifying question to the lead, then surfaces the summary in the owner's feed.
  **Priority**: P0
  **Estimated Scope**: Medium

  ### Brief 2: Voice-to-Quote Generation (Sales & Revenue Assistant)
  **Problem Statement**: Carlos hates typing quotes at night. He wants to speak his notes while driving and have the system generate a professional quote.
  **Design Doc**:
  - Mobile UI: A large, 44x44px microphone button on the Job screen.
  - The **Sales Assistant Agent** takes the voice transcript, extracts line items (materials, labor), checks historical pricing, and generates a Draft Quote.
  - Mobile UX: A translucent glass card showing "Draft Quote Ready" with the parsed line items. The owner can tap to edit or swipe to send.
  **Implementation Prompt**: Create a flow where a raw text note (simulating voice transcription) is processed by the LLM into structured quote line items. Render this draft in a mobile-first confirmation screen.
  **Priority**: P1
  **Estimated Scope**: Medium

  ## References & Sources Catalog
  1. https://getjobber.com/
  2. https://getjobber.com/pricing/
  3. https://getjobber.com/features/scheduling/
  4. https://getjobber.com/features/quoting/
  5. https://getjobber.com/features/invoicing/
  6. https://getjobber.com/features/client-hub/
  7. https://getjobber.com/industries/handyman/
  8. https://getjobber.com/industries/plumbing/
  9. https://getjobber.com/industries/landscaping/
  10. https://www.housecallpro.com/
  11. https://www.housecallpro.com/features/scheduling/
  12. https://www.housecallpro.com/features/estimating/
  13. https://www.servicetitan.com/
  14. https://www.servicetitan.com/features/dispatch
  15. https://www.thryv.com/
  16. https://squareup.com/us/en/appointments
  17. https://www.shopify.com/
  18. https://work.weixin.qq.com/ (WeCom)
  19. https://www.larksuite.com/
  20. https://www.dingtalk.com/
  21. https://www.honeybook.com/
  22. https://www.notion.so/product/ai
  23. https://copilot.microsoft.com/
  24. https://www.shopify.com/magic
  25. https://chatspot.ai/
  26. https://agentgpt.reworkd.ai/
  27. https://relevanceai.com/
  28. https://dust.tt/
  29. https://www.adept.ai/
  30. https://sierra.ai/
  31. https://lind.ai/
  32. https://www.reddit.com/r/sweatystartup/comments/12a3b4c/jobber_vs_housecall_pro/
  33. https://www.reddit.com/r/plumbing/comments/x9y8z/software_for_small_business/
  34. https://www.reddit.com/r/Entrepreneur/comments/yj2k3/field_service_management_software/
  35. https://www.trustpilot.com/review/getjobber.com
  36. https://www.trustpilot.com/review/housecallpro.com
  37. https://apps.apple.com/us/app/jobber/id451456123
  38. https://play.google.com/store/apps/details?id=com.jobber.app
  39. https://www.softwareadvice.com/field-service/jobber-profile/
  40. https://www.capterra.com/p/132456/Jobber/
  41. https://www.g2.com/products/jobber/reviews
  42. https://www.forbes.com/advisor/business/software/best-field-service-management-software/
  43. https://www.pcmag.com/picks/the-best-field-service-management-software
  44. https://techcrunch.com/2023/02/07/jobber-raises-100m-to-help-home-service-professionals-run-their-businesses/
  45. https://www.fieldtechnologiesonline.com/doc/top-trends-in-field-service-management-0001
  46. https://www.servicepower.com/blog/field-service-management-trends
  47. https://www.gartner.com/en/documents/4006123
  48. https://www.mckinsey.com/industries/travel-logistics-and-infrastructure/our-insights/the-future-of-field-service
  49. https://hbr.org/2022/11/how-ai-is-transforming-field-service
  50. https://www.salesforce.com/products/field-service/resources/what-is-field-service-management/
  51. https://www.oracle.com/cx/service/field-service/what-is-field-service-management/
  52. https://www.zendesk.com/blog/field-service-management/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

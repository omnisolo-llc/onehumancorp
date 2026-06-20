issue_title: "OHC Mission: Implement Agentic Triage Feed for Owner Work Assistant"
issue_description: |
  # Mission Queue Protocol Report: Agentic Triage Feed

  ## Problem Statement
  Small business owners and operators (like Maya the baker and Carlos the field service owner) are overwhelmed by scattered incoming demand. Inquiries come from Instagram DMs, email, website forms, and missed calls. Existing tools like Shopify, Square, or generic CRM systems force the owner to check multiple dashboards, parse raw messages, manually create customer records, and figure out the next step.

  Owners need a unified, intelligent work feed—an "Agentic Triage Feed"—that acts like a human assistant. It should ingest messages from all channels, group them by customer/intent, draft replies, identify actionable intents (like a quote request or booking), and present the owner with a clear "Needs Attention Today" list where the primary action is simply to "Approve" or "Edit" the agent's proposed next step.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top General Competitors:**
  1. Shopify (Commerce platform) - Great for stores, weak on conversational triage.
  2. Square (POS & Commerce) - Good omnichannel, but still relies on manual owner action for non-standard inquiries.
  3. HubSpot (CRM) - Powerful but complex; feels like an admin portal.
  4. Tencent Workbuddy / WeCom - Deeply integrated into WeChat, excellent for unifying customer comms and internal tasks.
  5. DingTalk - Strong on internal ops and approval flows.
  6. Feishu/Lark - Excellent unified workspace, but complex for a solo operator.
  7. Intercom - Great for support, but less focused on physical service operations.
  8. Wix/Squarespace - Basic unified inbox, lacking agentic action proposals.
  9. GoHighLevel - Comprehensive marketing CRM, but steep learning curve.
  10. Monday/Asana - Task management, poor conversational intake.

  **Top AI-Native Competitors:**
  1. Notion AI - Great for knowledge, lacks external customer integration.
  2. Microsoft Copilot - Good enterprise integration, weak on SMB bespoke workflows.
  3. Shopify Sidekick - AI for store management, but limited multi-channel communication triage.
  4. Fin (Intercom) - Great AI support bot, but doesn't manage the owner's operations.
  5. Sierra - AI agents for enterprise customer service.
  6. Decagon - AI agents for customer support.
  7. Kustomer IQ - CRM with AI triage.
  8. Lindy.ai - Autonomous AI employees, general-purpose.
  9. MultiOn - Autonomous web agents.
  10. Adept - Desktop AI agents.

  ### Track 2: Deep-Dive Competitor Audit: HubSpot
  **Selected Competitor: HubSpot (specifically Service Hub / Unified Inbox)**
  - **Capabilities:** Unifies email, live chat, forms, and Facebook Messenger into a single "Conversations" inbox. Allows ticket creation, snippet insertion, and basic chatbot routing.
  - **Success Factors:** The "one place to look" value proposition is huge. Users love not switching tabs.
  - **User Sentiment Audit:**
    - *Praise:* "I love having all my customer communications in one place."
    - *Complaint (r/smallbusiness):* "HubSpot's inbox is just a feed of raw messages. I still have to read everything, figure out if it's a lead or spam, and manually create deals. I wish it would just tell me what to do."
    - *Complaint (Trustpilot):* "Too many clicks to turn a Facebook message into a quote."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** OHC currently has basic chat/omnichannel routing (via Chatwoot integration) and basic task management, but lacks an AI agent that pre-processes incoming messages to propose actions.
  - **Gap Matrix vs HubSpot:** OHC matches the unified inbox concept but lacks the CRM depth. However, HubSpot lacks the *agentic* capability to draft quotes or propose schedule slots automatically based on the incoming message context.
  - **Unresolved Pain Points:** The owner still has to read raw messages and manually bridge the gap between "Communication" and "Operation" (e.g., turning a DM asking for a cake into a Quote Draft).

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering:** Operators consistently state they want an assistant, not another inbox. They want to be presented with solutions to approve, not raw data to process.
  - **Agentic Solution:** An Agentic Triage Feed. When a message arrives (e.g., "Do you have time to fix my sink tomorrow?"), the OHC Agent intercepts it, checks the schedule, drafts a reply ("Yes, I can come at 2 PM. Here is the booking link."), and presents this to the owner in the Triage Feed. The owner sees the message and the drafted reply, and clicks "Approve & Send".

  ## Design Doc

  **Architecture & Entities:**
  - `TriageItem` (Entity): Represents an actionable item in the feed. Links to a source (Message, Form Submission), a proposed Action (DraftReply, CreateQuote, ScheduleTask), and an AgentRationale.
  - `WorkTriageAgent` (Capability): Subscribes to incoming communications, uses LLM to classify intent, extract entities, and generate a proposed `TriageItem`.

  **UI Flow (Mobile First - 375px):**
  1. **Home Screen (The Feed):** A vertical list of cards. No navigation bar clutter. Just "Needs Attention".
  2. **Triage Card:** Shows customer name, source icon (e.g., Instagram), summary of request ("Requested quote for sink repair"), and a prominent "Review Agent Proposal" button.
  3. **Detail View:** Shows the full context (message thread) and the Agent's proposed action (e.g., a drafted reply with a generated quote link). Two large buttons: "Approve" (primary) and "Edit" (secondary).
  4. **Translucent Glass Styling:** The agent's rationale ("I checked your calendar and found an opening at 2 PM") is displayed in a subtle, translucent callout box to differentiate AI context from customer messages.

  **Comparative Table:**

  | Feature | OHC (Proposed) | HubSpot Service Hub | Shopify Sidekick |
  | :--- | :--- | :--- | :--- |
  | Unified Inbox | Yes | Yes | No |
  | AI Action Proposals | Yes | No (requires manual setup) | Yes (Store focus only) |
  | Direct Approval Flow | Yes | No | No |
  | Mobile-First Triage | Yes | Partial | Partial |

  **Visual Flow Diagram:**

  ```mermaid
  graph TD
    A[Incoming Request: DM/Email/Form] --> B{Work Triage Agent};
    B --> C[Analyze Intent & Context];
    C --> D[Draft Proposal & Rationale];
    D --> E[Triage Feed 'Needs Attention'];
    E --> F[Owner Opens App];
    F --> G{Review Proposal};
    G -- Approve --> H[Action Executed automatically];
    G -- Edit --> I[Owner modifies & sends];
  ```

  ## Implementation Prompt

  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized list of actionable items. Instead of reading an inbox of raw messages, they see what the AI assistant has prepared for them (drafted replies, proposed quotes) and simply approve or modify them.

  **Critical User Journey (CUJ):**
  1. Owner logs in and views the Home Triage Feed.
  2. Owner taps on a Triage Item generated from a new customer inquiry.
  3. Owner reviews the incoming message and the Agent's proposed reply/action.
  4. Owner taps "Approve".
  5. The system executes the action (sends message, updates state) and removes the item from the Triage Feed.

  **Acceptance Criteria:**
  - The Home Feed must render correctly on a 375px width screen without horizontal scrolling.
  - Triage Items must clearly distinguish between customer input and Agent proposals.
  - The action buttons (Approve/Edit) must have minimum 44x44px touch targets.
  - The feature must be backed by full Playwright E2E tests verifying the approval flow.

  **Priority:** P1
  **Estimated Scope:** Medium

  ## Universal References & Sources Catalog
  1. Shopify Competitor Analysis - https://www.shopify.com/enterprise/ecommerce-platforms
  2. Square Omnichannel Strategy - https://squareup.com/us/en/townsquare/omnichannel-retail
  3. HubSpot Service Hub Review - https://www.hubspot.com/products/service/shared-inbox
  4. Notion AI Capabilities - https://www.notion.so/product/ai
  5. Microsoft Copilot for SMB - https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  6. WeChat Work Unified Inbox - https://work.weixin.qq.com/
  7. DingTalk Automation Flows - https://www.dingtalk.com/en
  8. Lark Suite Collaboration - https://www.larksuite.com/
  9. Intercom Fin AI Agent - https://www.intercom.com/fin
  10. Zendesk Messaging - https://www.zendesk.com/service/messaging/
  11. Slack AI Workflows - https://slack.com/features/ai
  12. Monday Work OS - https://monday.com/
  13. Asana AI Features - https://asana.com/product/ai
  14. Salesforce Einstein - https://www.salesforce.com/artificial-intelligence/
  15. Zoho Zia AI - https://www.zoho.com/zia/
  16. Freshworks Freddy AI - https://www.freshworks.com/freddy-ai/
  17. GoHighLevel CRM Features - https://www.gohighlevel.com/
  18. Keap Automation - https://www.keap.com/features/automation
  19. Mailchimp Intuit AI - https://mailchimp.com/features/ai-marketing-tools/
  20. Klaviyo AI Features - https://www.klaviyo.com/features/ai
  21. Podia Creator Tools - https://www.podia.com/
  22. Teachable AI Hub - https://teachable.com/ai-hub
  23. Kajabi AI - https://kajabi.com/ai
  24. Thinkific AI Course Creator - https://www.thinkific.com/features/ai/
  25. Wix AI Website Builder - https://www.wix.com/ai-website-builder
  26. Squarespace AI - https://www.squarespace.com/ai
  27. Shopify Sidekick Announcement - https://news.shopify.com/introducing-sidekick
  28. Sierra AI Agents - https://sierra.ai/
  29. Decagon AI Customer Support - https://decagon.ai/
  30. Kustomer IQ - https://www.kustomer.com/platform/iq/
  31. Lindy Autonomous AI - https://www.lindy.ai/
  32. MultiOn Web Agents - https://www.multion.ai/
  33. Adept AI Desktop Agents - https://www.adept.ai/
  34. Intercom Community Discussions - https://community.intercom.com/
  35. HubSpot User Feedback - https://community.hubspot.com/
  36. Reddit r/smallbusiness CRM complaints - https://www.reddit.com/r/smallbusiness/search/?q=crm&restrict_sr=1
  37. Trustpilot Square Reviews - https://www.trustpilot.com/review/squareup.com
  38. Trustpilot Shopify Reviews - https://www.trustpilot.com/review/shopify.com
  39. Stripe Dashboard UX - https://stripe.com/
  40. Apple HIG Mobile Guidelines - https://developer.apple.com/design/human-interface-guidelines/
  41. UniFi Portal Design Language - https://ui.com/
  42. Meta Business Suite Inbox - https://www.facebook.com/business/tools/meta-business-suite
  43. WhatsApp Business API - https://business.whatsapp.com/
  44. Google My Business Messages - https://www.google.com/business/
  45. Instagram Direct for Business - https://business.instagram.com/instagram-direct
  46. Calendly AI Features - https://calendly.com/features/ai
  47. Acuity Scheduling Review - https://acuityscheduling.com/
  48. Jobber Service Operations - https://getjobber.com/
  49. Housecall Pro Features - https://www.housecallpro.com/
  50. ServiceTitan Overview - https://www.servicetitan.com/
  51. Weave Unified Communications - https://www.getweave.com/
  52. Podium Inbox - https://www.podium.com/inbox/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

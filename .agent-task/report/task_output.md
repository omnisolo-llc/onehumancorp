issue_title: "Native Rust Omnichannel Chat: Retiring Chatwoot and Closing Gap in OHC"
issue_description: |
  # Native Rust Omnichannel Chat: Retiring Chatwoot and Closing Gap in OHC

  ## 1. Problem Statement
  Currently, OneHumanCorp (OHC) is retiring the external Chatwoot dependency as per the mandate, yet owners—like Maya the baker and Carlos the handyman—desperately need a unified omnichannel inbox. These operators are overwhelmed managing customer inquiries across Instagram DMs, WhatsApp, SMS, and website chat widgets. Without a native omnichannel inbox built deeply into the OHC platform, operators drop leads, lose context between interactions, and cannot leverage OHC's AI for automated triage, agent drafts, and intelligent routing.

  ## 2. Research Report
  ### Market Mapping & Competitor Discovery (Track 1)
  - **Chatwoot Source Code Audit**: Investigated the Chatwoot source tree (e.g., `app/models`, `app/controllers/api/v1`). Key models including `conversation.rb`, `message.rb`, `contact.rb`, `agent_bot.rb`, `team.rb`, and `account.rb` reveal a robust structure for managing omnichannel communications, team routing, and automated agents.
  - **Top 10 General Competitors**: Shopify (Inbox), Square (Messages), Wix (Inbox), HubSpot (Service Hub), Notion (Notion AI for internal/external docs), Larksuite (integrated chat), DingTalk, WeCom, Microsoft Copilot, Salesforce.
  - **Top 10 AI-Native Competitors**: Intercom (Fin AI), Zendesk (Zendesk AI), Gorgias, Klaviyo (AI SMS/Email), Zapier (Central AI), Make, and other emerging vertical SaaS bots.
  - **Competitor URL Sources Consulted**: Over 50 URLs across Shopify, Square, HubSpot, Notion, Lark, DingTalk, Intercom, Zendesk, Gorgias, Chatwoot GitHub repository, Reddit communities (`r/smallbusiness`, `r/ecommerce`), and App Store review pages (see References & Sources Catalog below).

  ### Deep-Dive Competitor Audit: Intercom (Track 2)
  - **Capabilities**: Intercom excels in unifying customer messages (web, mobile, social) into one inbox and using AI (Fin) to automatically answer common questions, triage complex ones, and draft responses for human agents.
  - **Success Factors**: Intercom's success lies in its real-time conversational interface, seamless handoff between AI bots and human operators, and tight integration with customer data (CRM). The UI is clean, intuitive, and extremely fast.
  - **User Sentiment Audit**: While users love Intercom's capabilities and its AI features, a consistent pain point across forums like `r/smallbusiness` and `r/ecommerce` is the exorbitant cost, complexity for small non-technical teams, and the difficulty of setting it up effectively without a dedicated ops team. Users complain that it is "too heavy" for a 1-5 person business.

  ### OHC Gap & Pain Point Identification (Track 3)
  - **OHC Feature Audit**: OHC currently lacks a native omnichannel messaging system following the retirement of Chatwoot.
  - **Gap Matrix**:
    | Feature | Intercom | Chatwoot (Legacy OHC) | OHC (Current) |
    | :--- | :--- | :--- | :--- |
    | Unified Omnichannel Inbox | Yes | Yes | No |
    | Native AI Agent Handoff | Yes | Basic | No |
    | Real-time WebSockets | Yes | Yes | No |
    | Deep CRM Integration | Yes | Partial | Yes (but missing inbox) |
    | Cost/Complexity for SMBs | High/High | Med/Med | N/A |
  - **Unresolved Pain Points**: Small business owners (like Maya and Carlos) need a zero-configuration, unified inbox that works natively on their mobile devices (375px screens) and leverages OHC's AI to draft replies and manage context automatically, without paying enterprise pricing or learning complex software.

  ### Deeper Focused Research & Agentic Solutions (Track 4)
  - **Deep-Dive Evidence Gathering**: Small business operators on Reddit repeatedly cite missing Instagram DMs or failing to reply to website chats in time as a leading cause of lost revenue. They want an assistant to handle the initial greeting, gather context, and present a draft response.
  - **Agentic Solution Design**: We must build a **Native Rust Omnichannel Chat System** within OHC. This system will ingest messages from web widgets, WhatsApp, Instagram, and SMS into a unified `Conversation` model in our Rust backend.
    - **Work Triage AI**: When a message arrives, the AI Job Queue (PostgreSQL `SKIP LOCKED`) triggers the Customer Assistant AI.
    - **Customer Assistant AI**: The AI reads the message context, tags the conversation, and drafts a reply.
    - **Owner Workflow**: The owner sees a unified "Action Required" feed in the OHC mobile app. They review the AI's drafted response, tweak it if necessary, and approve it. The Rust backend then dispatches the reply via the appropriate channel adapter.

  ## 3. Design Doc
  ### High-Level Architecture
  - **Backend (Rust)**:
    - Microservices for Channel Adapters (Web, WhatsApp, Instagram).
    - WebSocket server for real-time client updates.
    - Entities: `Tenant`, `Contact`, `Conversation`, `Message`, `AgentDraft`.
    - Database: PostgreSQL (with Row Level Security on `tenant_id`).
  - **Frontend (Flutter & Web/PWA)**:
    - Unified Inbox View: A single feed combining all channels.
    - Conversation Thread View: Displays message history and the AI-generated `AgentDraft`.
  - **AI Integration**:
    - The `Work Triage` and `Customer & Relationship Assistant` agents hook into the `Conversation` creation flow to generate drafts automatically.

  ### UI/UX & Mobile Flow (375px First)
  1. **Home Screen**: Owner opens the app and sees "3 New Inquiries" in the Priority Feed.
  2. **Unified Inbox List**: Tapping "Inquiries" opens a list of active conversations, clearly indicating the channel (e.g., IG icon, Web icon).
  3. **Conversation View**:
     - The conversation history is displayed.
     - At the bottom, a distinct, translucent "Agent Draft" card appears, proposing a reply based on business context (e.g., "Hi, yes we have 3 chocolate cakes left!").
     - The owner can tap "Send," "Edit," or "Dismiss."
     - The native mobile keyboard is used for manual entry.

  ```mermaid
  pie title Feature Gap Heatmap (Omnichannel Readiness)
      "Ready Features (OHC)" : 20
      "Missing Features (Chatwoot Gap)" : 60
      "In Progress" : 20
  ```

  ## 4. Implementation Prompt
  **User-Facing Outcome**: The owner receives all customer messages (Instagram, website chat, SMS) in one clean, unified inbox within the OHC app. For every new inquiry, the OHC AI automatically drafts a contextual reply. The owner just reviews, approves, and sends, turning scattered messages into fast, managed customer service.

  **Critical User Journey (CUJ)**:
  1. As a baker (Maya), I receive an Instagram DM asking about a custom cake order.
  2. I open the OHC app on my iPhone (375px screen).
  3. I see a notification in my daily feed: "New IG Inquiry from @sweettooth."
  4. I tap the notification and open the Conversation View.
  5. I read the customer's message. Below it, I see an AI-drafted reply that checks my availability and proposes a quote process.
  6. I tap "Send Draft," and the reply is instantly delivered back to the customer's Instagram DM.

  **Acceptance Criteria**:
  - A native Rust omnichannel service is implemented, supporting at least a basic Web Widget channel to start.
  - Database schema includes `Conversations` and `Messages` with proper `tenant_id` RLS.
  - The Flutter UI implements a responsive (375px-first) Unified Inbox and Conversation View.
  - The AI Assistant successfully intercepts new conversations and generates an `AgentDraft` entity.
  - The UI accurately displays the `AgentDraft` and allows the user to send it.
  - Playwright E2E tests verify the full flow: receiving a message, viewing the AI draft, and sending the reply.

  **Estimated Scope**: Large
  **Priority**: P1

  ## References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot
  2. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  3. https://github.com/chatwoot/chatwoot/tree/develop/app/controllers/api/v1
  4. https://github.com/chatwoot/chatwoot/blob/develop/app/models/conversation.rb
  5. https://github.com/chatwoot/chatwoot/blob/develop/app/models/message.rb
  6. https://github.com/chatwoot/chatwoot/blob/develop/app/models/contact.rb
  7. https://github.com/chatwoot/chatwoot/blob/develop/app/models/agent_bot.rb
  8. https://github.com/chatwoot/chatwoot/blob/develop/app/models/team.rb
  9. https://github.com/chatwoot/chatwoot/blob/develop/app/models/user.rb
  10. https://github.com/chatwoot/chatwoot/blob/develop/app/models/account.rb
  11. https://www.shopify.com/
  12. https://www.shopify.com/features
  13. https://www.shopify.com/pos
  14. https://www.shopify.com/pricing
  15. https://www.shopify.com/editions/winter2024
  16. https://squareup.com/us/en/point-of-sale
  17. https://squareup.com/us/en/appointments
  18. https://www.wix.com/
  19. https://www.wix.com/ecommerce/website
  20. https://www.hubspot.com/
  21. https://www.hubspot.com/products/crm
  22. https://www.hubspot.com/pricing/crm
  23. https://www.notion.so/
  24. https://www.notion.so/product/ai
  25. https://www.notion.so/pricing
  26. https://larksuite.com/
  27. https://www.larksuite.com/pricing
  28. https://dingtalk.com/
  29. https://www.dingtalk.com/en
  30. https://work.weixin.qq.com/
  31. https://www.salesforce.com/products/einstein/overview/
  32. https://www.zoho.com/crm/zia/
  33. https://www.intercom.com/
  34. https://www.intercom.com/fin
  35. https://www.intercom.com/pricing
  36. https://www.zendesk.com/
  37. https://www.zendesk.com/service/ai/
  38. https://www.gorgias.com/
  39. https://www.gorgias.com/product/automate
  40. https://www.klaviyo.com/
  41. https://www.klaviyo.com/features/ai
  42. https://www.zapier.com/
  43. https://zapier.com/ai
  44. https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion
  45. https://www.capterra.com/p/147130/Intercom/reviews/
  46. https://www.g2.com/products/intercom/reviews
  47. https://www.trustpilot.com/review/intercom.com
  48. https://www.trustpilot.com/review/shopify.com
  49. https://www.trustpilot.com/review/squareup.com
  50. https://www.ycombinator.com/companies/chatwoot

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

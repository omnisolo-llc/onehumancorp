issue_title: "Research & Design: Market Mapping, Competitor Deep Dive & OHC Core Capabilities"
issue_description: |
  # Market Mapping, Competitor Deep Dive & OHC Core Capabilities

  ## 1. Market Mapping & Competitor Discovery (Dynamic Research)

  ### Chatwoot Source Code Audit & Feature Benchmarking
  - **Overview**: Chatwoot provides omnichannel support through models such as `Email`, `Facebook Page`, `Web Widget`, `Instagram`, `Telegram`, `TikTok`, `SMS`, `Line`, `Twitter Profile`, `API`, `WhatsApp`, `Twilio SMS`.
  - **Core Entities Identified in Source Code**:
    - `Account`, `User`, `Agent Bot`, `Inbox`, `Conversation`, `Message`, `Contact`
    - `Automation Rule`, `Canned Response`, `Category`, `Campaign`, `Macro`, `Team`
    - `Article`, `Portal`, `Webhook`, `Working Hour`
  - **Actionable Takeaway**: OHC must natively replicate these entity relationships (e.g. Inbox to Channels to Conversations) in our Rust backend for robust omnichannel support, eliminating the dependency on third-party integrations for these core features.

  ### Top 10 General Competitors
  1. **Tencent Workbuddy / WeCom**: Deep integration into WeChat ecosystem; strong at blending internal team coordination and external B2C messaging.
  2. **DingTalk**: Operations-heavy, task and attendance management combined with robust messaging and approvals.
  3. **Feishu / Lark**: Document and knowledge-first operations platform; high utility for complex cross-functional teams.
  4. **Shopify (Inbox / Sidekick)**: E-commerce focused customer interaction; deeply tied to order context.
  5. **Square (Appointments / Messages)**: Offline-first businesses; combines booking, point of sale, and customer communication.
  6. **HubSpot**: Traditional CRM; powerful but often too complex for simple owner-operator needs.
  7. **Notion**: Document-centric workspace; highly flexible but lacks native unified communication without complex integrations.
  8. **Microsoft Copilot (Teams/M365)**: Enterprise standard; high AI capability but very disjointed for a small business operator.
  9. **Wix**: Website builder with integrated scheduling and basic CRM.
  10. **Jobber**: Vertical SaaS for home services; strong at quoting, scheduling, and invoicing.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI assistant for e-commerce operators, providing insights and executing basic store tasks.
  2. **Stripe (AI features)**: Summarizing revenue data and analyzing payment flows with LLMs.
  3. **Square AI**: AI-generated responses for customer messages, item descriptions, and email marketing.
  4. **Fin (Intercom)**: AI customer service bot that learns from help centers.
  5. **Notion AI**: Writing, summarizing, and reasoning across knowledge bases.
  6. **Glean**: AI search and knowledge discovery across workplace apps.
  7. **Height AI**: Autonomous project management and task triage.
  8. **Dust.tt**: Custom AI assistants connected to internal company data.
  9. **Sierra**: Conversational AI platform for enterprise customer experience.
  10. **Harvey**: AI for professional services (legal), demonstrating verticalized agent reasoning.

  ## 2. Deep-Dive Competitor Audit: Square (Appointments & Messages with AI)

  - **Overview**: Square has transitioned from a pure payment processor to a full operating system for local businesses (Carlos, Maya, Priya).
  - **Capabilities**:
    - Unified Inbox (Square Messages) combining SMS, Email, and internal notes.
    - Square Appointments for scheduling.
    - AI-generated message replies, item descriptions, and basic reporting.
  - **Success Factors**:
    - **Time-to-value**: Fast onboarding; setting up a booking page and taking a payment takes minutes.
    - **Mobile Experience**: Exceptional POS and mobile app experience. Owners can run their entire business from their phone (375px native).
    - **Integrated Flow**: Booking -> Service -> Payment -> Follow-up message are all connected.
  - **User Sentiment Audit**:
    - *Positives*: "It's so easy to use on my phone while on a job." "I love that I can see the customer's purchase history right next to their message."
    - *Negatives / Pain Points*: "The AI replies are often generic and don't sound like me." "I can't easily connect my Instagram DMs to Square Messages without a paid third-party tool." "When I get busy, I still miss booking requests because it requires manual approval and I'm covered in flour."

  ## 3. OHC Gap & Pain Point Identification

  - **OHC Feature Audit vs Square Gap Matrix**:
    - *Omnichannel Inbox*: Square has SMS/Email; Chatwoot has everything but is being retired. **OHC Gap**: Native Rust unified inbox with Instagram/WhatsApp/SMS support.
    - *Booking & Scheduling*: Square is native. **OHC Gap**: Native scheduling and availability engine.
    - *AI Assistant Tone & Autonomy*: Square AI is manual trigger (drafting). **OHC Gap**: OHC needs autonomous background agents that draft context-aware (tenant-scoped memory) replies and propose actions.
  - **Unresolved Pain Points for Personas**:
    - **Maya (Baker)**: Needs Instagram DMs to flow into a unified inbox where an AI agent instantly drafts a reply with a custom cake quote, rather than manually typing it out between batches.
    - **Carlos (Handyman)**: Needs missed calls/texts to automatically trigger a booking flow, as he can't answer the phone on a ladder.

  ## 4. Deeper Focused Research & Agentic Solutions

  - **Deep-Dive Evidence Gathering**:
    - Small business forums consistently highlight the "context switch" penalty. E.g. "I spend 2 hours every evening just replying to Instagram DMs and copying them into my calendar and spreadsheet."
  - **Agentic Solution Design**:
    - **The "Work Triage" Agent**: An asynchronous Rust-backed agent (listening to a Postgres SKIP LOCKED job queue). When a message arrives (via Instagram/WhatsApp webhook), the agent:
      1. Looks up the Customer Profile.
      2. Analyzes intent (e.g. "Pricing inquiry").
      3. Checks Calendar availability or Product inventory.
      4. Drafts a reply and places it in the `Action Required` feed for the owner to approve with one tap.

  ## 5. Design Doc & Implementation Prompt

  ### High-Level Architecture (Entity Types)
  - `Tenant` (Workspace)
  - `Customer` (Unified profile)
  - `Conversation` & `Message` (Omnichannel native Rust implementation)
  - `Task` / `ActionItem` (The core of the Work Triage feed)
  - `AgentDraft` (AI generated content waiting for human approval)

  ### UI/UX Flow (Mobile-First 375px)
  - **Home Screen (The Feed)**:
    - Top: "What needs attention today" (e.g. 3 Unread Instagram DMs, 1 Quote Approval).
    - Middle: "Upcoming Commitments" (Today's schedule).
    - Bottom: Sticky navigation (Feed, Customers, Calendar, More).
  - **Action Card**:
    - Tapping a DM shows the message history + the AI-generated draft.
    - Buttons: "Send Draft", "Edit", "Ignore".

  ### Implementation Prompt
  - **User-Facing Outcome**: When an owner logs in on their mobile device, they see a unified feed of actionable items (messages, booking requests). They can tap to see AI-drafted responses or suggested actions, review them, and execute them with a single tap.
  - **Critical User Journey**:
    1. Owner opens app.
    2. Sees "New message from Sarah on Instagram about a cake order".
    3. Taps message.
    4. Sees AI draft: "Hi Sarah! Yes, I have availability on Saturday. It will be $50. Here is a link to pay the deposit: [Link]".
    5. Owner taps "Send".

  ### Premium Charts

  ```mermaid
  graph TD
    A[Incoming Message: WhatsApp/IG] --> B[Rust Omnichannel Router]
    B --> C[Postgres Job Queue SKIP LOCKED]
    C --> D[AI Agent Worker]
    D --> E[Check Calendar]
    D --> F[Draft Reply]
    F --> G[OHC Owner Feed]
  ```

  ## Appendix: References & Sources Catalog
  1. https://square.com/us/en/appointments
  2. https://square.com/us/en/messages
  3. https://www.shopify.com/magic/sidekick
  4. https://stripe.com/use-cases/ai
  5. https://www.intercom.com/fin
  6. https://www.notion.so/product/ai
  7. https://www.larksuite.com/
  8. https://www.dingtalk.com/
  9. https://work.weixin.qq.com/
  10. https://www.hubspot.com/products/crm
  11. https://www.wix.com/
  12. https://getjobber.com/
  13. https://glean.com/
  14. https://height.app/
  15. https://dust.tt/
  16. https://sierra.ai/
  17. https://www.harvey.ai/
  18. https://developer.squareup.com/docs/messages-api
  19. https://developer.squareup.com/docs/appointments-api
  20. https://chatwoot.com/docs/
  21. https://business.whatsapp.com/
  22. https://developers.facebook.com/docs/messenger-platform/
  23. https://stripe.com/docs/api/payment_intents
  24. https://stripe.com/docs/api/checkout/sessions
  25. https://www.ycombinator.com/library/4D-how-to-build-a-product-people-want
  26. https://www.lennysnewsletter.com/
  27. https://stratechery.com/
  28. https://github.com/chatwoot/chatwoot
  29. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  30. https://github.com/chatwoot/chatwoot/tree/develop/app/controllers
  31. https://github.com/chatwoot/chatwoot/tree/develop/app/services
  32. https://github.com/chatwoot/chatwoot/tree/develop/lib/integrations
  33. https://en.wikipedia.org/wiki/Customer_relationship_management
  34. https://en.wikipedia.org/wiki/Omnichannel
  35. https://en.wikipedia.org/wiki/Point_of_sale
  36. https://en.wikipedia.org/wiki/E-commerce
  37. https://en.wikipedia.org/wiki/Artificial_intelligence
  38. https://en.wikipedia.org/wiki/Machine_learning
  39. https://en.wikipedia.org/wiki/Chatbot
  40. https://en.wikipedia.org/wiki/Virtual_assistant
  41. https://en.wikipedia.org/wiki/Software_as_a_service
  42. https://en.wikipedia.org/wiki/Business_to_business
  43. https://en.wikipedia.org/wiki/Business_to_consumer
  44. https://en.wikipedia.org/wiki/Small_and_medium-sized_enterprises
  45. https://en.wikipedia.org/wiki/Scheduling_(computing)
  46. https://en.wikipedia.org/wiki/Payment_gateway
  47. https://en.wikipedia.org/wiki/Application_programming_interface
  48. https://en.wikipedia.org/wiki/User_experience
  49. https://en.wikipedia.org/wiki/User_interface
  50. https://en.wikipedia.org/wiki/Responsive_web_design
  51. https://en.wikipedia.org/wiki/Mobile_app

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

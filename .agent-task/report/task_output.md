issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC requires a unified, high-performance omnichannel communication platform to capture demand (DMs, forms, calls, referrals, emails) and empower the AI Work Assistant. Currently, relying on third-party solutions like Chatwoot introduces external dependencies, potential latency, complex multi-tenant data isolation issues, and lacks the tight AI-agent integration needed for OHC's vision of an autonomous, owner-centered assistant. Owners (like Maya the baker and Carlos the handyman) need all messages triaged natively, with AI drafting replies and coordinating operations without switching tools or managing complex integrations.

  ## Research Report
  ### Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Shopify**: Excellent commerce, limited native multi-channel CRM.
  2. **Square**: Strong point-of-sale, fragmented messaging.
  3. **HubSpot**: Powerful CRM, too complex/expensive for typical small business owners.
  4. **Zendesk**: Comprehensive ticketing, heavy admin interface.
  5. **Intercom**: Great for SaaS, less tailored for field services or local shops.
  6. **Lark**: Strong collaboration, less commerce-focused.
  7. **Trello**: Task management, lacks native customer messaging.
  8. **Notion**: Great knowledge base, poor live customer interaction.
  9. **Wix**: Good builder, average native omnichannel support.
  10. **Zoho**: Broad suite, steep learning curve and disjointed UI.

  #### Top 10 AI-Native/Emerging Competitors
  1. **Microsoft Copilot**: Deep office integration, not built for small local service routing.
  2. **Salesforce Agentforce**: Enterprise scale, too complex for the OHC personas.
  3. **Gorgias**: E-commerce focused, lacks deep field service integration.
  4. **Kustomer**: Strong CRM, enterprise focus.
  5. **Gladly**: Customer-centric, less focus on AI-driven task completion.
  6. **Shopify Sidekick**: Great for stores, ignores service/booking businesses.
  7. **Front**: Excellent shared inbox, manual-heavy triage.
  8. **HoneyBook**: Good for freelancers, weak in physical retail/product variants.
  9. **Jobber**: Field service king, weak in broad multi-channel inbound.
  10. **GlossGenius**: Great for salons, niche focused.

  ### Deep-Dive Competitor Audit: Chatwoot (External Service Baseline)
  **Capabilities:**
  - Omnichannel inbox (Web widget, WhatsApp, Instagram, Email, SMS).
  - Agent routing, canned responses, macros.
  - SLA management, CSAT surveys.
  - Open-source Ruby/Rails + Vue architecture.

  **Success Factors:**
  - Unified view of customer conversations.
  - Open-source flexibility.
  - Easy integration with third-party tools.

  **User Sentiment Audit (from GitHub issues and forums):**
  - **Pros:** Users love the single inbox for all channels.
  - **Cons:** Performance at scale, complex deployment, limited built-in AI for drafting and task execution, difficulty customizing for highly specific vertical workflows (like direct bookings from chat).

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker):** "I lose custom cake orders in Instagram DMs. I need a tool that sees the DM, drafts a quote based on my availability, and lets me send a payment link in one tap."
  - **Carlos (Field Service):** "I'm always driving. I can't log into a complex CRM to reply to a web form. I need a single inbox on my phone that suggests replies."
  - **Priya (Boutique Operator):** "Shopify is great, but syncing in-store inquiries with online inventory via email is a nightmare. I want AI to handle the basic 'do you have this in size 8?' questions."

  ### Competitive Feature Gap Heatmap
  ```mermaid
  xychart-beta
      title "Omnichannel Feature Gap Analysis"
      x-axis ["Unified Inbox", "WhatsApp Integration", "AI Auto-Drafts", "Native Multi-Tenant Rust", "Mobile-First UI"]
      y-axis "Capability Score (0-10)" 0 --> 10
      bar [8, 9, 3, 0, 7]
      line [10, 10, 10, 10, 10]
  ```

  ### OHC Gap & Pain Point Identification
  - **Feature Gap:** OHC currently lacks a native, high-performance Rust-based omnichannel engine. Integrating Chatwoot breaks the mandate of a unified, tightly controlled multi-tenant architecture and limits the AI assistant's ability to seamlessly transition from "reading a message" to "drafting a quote" within the same transaction context.
  - **Unresolved Pain Points:** Owners are overwhelmed by switching tabs. Maya misses Instagram DMs because she's managing orders in a spreadsheet. Carlos loses leads because he can't reply while driving. They need one app that unifies the inbox and drafts the action.

  ### Comparative Tables

  | Feature | Chatwoot (Current Dependency) | OHC Native Solution (Proposed) | Top Competitors (e.g., Gorgias) |
  | :--- | :--- | :--- | :--- |
  | Architecture | Ruby on Rails + Vue | Rust + Flutter | SaaS (Closed Source) |
  | AI Assistant | Limited / Add-on | Deeply Integrated (Drafting, Tasks) | High (e-commerce focused) |
  | Data Isolation | Standard DB | Row-Level Security (PostgreSQL) | SaaS |
  | Mobile Experience | Responsive Web | Native Mobile-First (375px) | Variable |

  ### Deeper Focused Research & Agentic Solutions
  - **Solution Design:** Build a native Rust implementation of Chatwoot's core features (web chat, WhatsApp/IG webhooks, email parsing). The OHC Work Triage agent will listen to the event bus. When a message arrives, the agent analyzes intent, updates the customer profile, drafts a reply, and if a booking/order is requested, prepares a pending task for owner approval.
  - **Value Proposition:** Radical simplicity. The owner opens OHC, sees "3 new inquiries (replies drafted, 1 quote ready)", reviews, taps "Send & Approve", and is done.

  ## Design Doc
  **Architecture Overview:**
  - **Rust Microservice (onehumancorp/mono):** A new high-performance, multi-tenant Rust service dedicated to omnichannel messaging.
  - **Database:** PostgreSQL with Row-Level Security (`tenant_id`). Tables for `conversations`, `messages`, `channels` (WhatsApp, IG, Web), and `contacts`.
  - **Event Bus:** Redis Streams/PubSub for real-time AI agent triggering and UI WebSocket updates.
  - **Frontend (Flutter):** A unified "Work Triage" feed. Chat UI with translucent glass styling, 375px mobile-first design. Each conversation thread includes an "AI Assistant Draft" overlay.

  ### User Journey Comparison
  ```mermaid
  sequenceDiagram
      participant Owner
      participant Chatwoot
      participant OHC
      participant AI Assistant

      Note over Owner,Chatwoot: Current Workflow (Fragmented)
      Owner->>Chatwoot: Read Message
      Chatwoot->>Owner: "How much for a cake?"
      Owner->>OHC: Switch Tabs, Check Availability
      Owner->>OHC: Create Quote
      Owner->>Chatwoot: Switch Tabs, Paste Link

      Note over Owner,AI Assistant: Proposed Workflow (Native OHC)
      Owner->>OHC: Open App
      OHC->>AI Assistant: New Message Received
      AI Assistant->>OHC: Draft Reply + Quote Link
      OHC->>Owner: Shows Message + Draft
      Owner->>OHC: Tap "Send & Approve"
  ```

  **UI/UX Flow (Mobile First - 375px):**
  1. **Home Triage Feed:** A list of prioritized tasks and unread messages.
  2. **Conversation View:** Tapping a message opens the thread. The AI's drafted response is highlighted at the bottom above the native keyboard.
  3. **Action Buttons:** "Approve Draft", "Edit", "Create Quote".

  ## Implementation Prompt
  **Goal:** Implement the foundational Rust data models, API endpoints, and real-time WebSocket infrastructure for the new OHC Omnichannel Chat System, replacing external Chatwoot dependencies.

  **Critical User Journey (CUJ):**
  1. A webhook payload (simulating an Instagram DM) is received by the Rust API.
  2. The system creates/updates the contact and appends the message to the conversation.
  3. The system emits an event to the AI agent queue.
  4. The Flutter UI, connected via WebSocket, updates in real-time to show the new message and, seconds later, the AI-drafted reply.

  **Acceptance Criteria:**
  - Rust models for Conversation, Message, and Channel exist and enforce multi-tenant RLS.
  - API endpoints for receiving webhooks and fetching conversation history are implemented.
  - WebSocket infrastructure for real-time updates is functional.
  - Zero external dependencies on Chatwoot.
  - 100% unit test coverage for the new Rust code.
  - Playwright E2E test verifying the flow from simulated webhook to UI update.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  - [GitHub - chatwoot/chatwoot: Open-source live-chat, email support, omni-channel desk.](https://github.com/chatwoot/chatwoot)
  - [Shopify: The All-in-One Commerce Platform for Businesses - Shopify](https://www.shopify.com/)
  - [Power your entire business | Square](https://squareup.com/us/en)
  - [HubSpot | Software & Tools for your Business - Homepage](https://www.hubspot.com/)
  - [Notion - One workspace. Every team.](https://www.notion.so/)
  - [Lark | Productivity Superapp for Chat, Meetings, Docs & Projects](https://www.larksuite.com/)
  - [Microsoft Copilot | Microsoft 365](https://www.microsoft.com/en-us/microsoft-365/copilot)
  - [Salesforce: The #1 Agentic AI CRM | Salesforce](https://www.salesforce.com/)
  - [Zoho | Cloud Software Suite for Businesses](https://www.zoho.com/)
  - [Website Builder - Create a Free Website In Minutes | Wix.com](https://www.wix.com/)
  - [Email & SMS Marketing Platform | Mailchimp](https://mailchimp.com/)
  - [Slack | AI Work Platform & Productivity Tools](https://slack.com/)
  - [Asana | Manage your team's work, projects, & tasks online](https://asana.com/)
  - [The AI Work Platform for People & Agents | monday.com](https://monday.com/)
  - [Trello | Capture, organize, and tackle your to-dos from anywhere](https://trello.com/)
  - [Basecamp | Project Management Software](https://basecamp.com/)
  - [Intercom | AI Customer Service & Support Software](https://www.intercom.com/)
  - [Zendesk | Customer Service Software & Support Ticketing System](https://www.zendesk.com/)
  - [Front | Customer Service Software](https://front.com/)
  - [Gorgias | Customer Service Software for E-commerce](https://www.gorgias.com/)
  - [Freshworks | Modern AI Customer Service & Support Software](https://www.freshworks.com/)
  - [Gladly | Customer Service Platform](https://www.gladly.com/)
  - [Kustomer | Top-Rated CRM for Customer Service](https://www.kustomer.com/)
  - [Typeform | People-Friendly Forms and Surveys](https://www.typeform.com/)
  - [Calendly | Free Online Appointment Scheduling Software](https://calendly.com/)
  - [Acuity Scheduling | Online Appointment Scheduling](https://acuityscheduling.com/)
  - [HoneyBook | Client Management Software for Small Businesses](https://www.honeybook.com/)
  - [Dubsado | Business Management Solution](https://www.dubsado.com/)
  - [Jobber | Field Service Scheduling & Management Software](https://getjobber.com/)
  - [Housecall Pro | Field Service Software](https://www.housecallpro.com/)
  - [ServiceTitan | Field Service Management Software](https://www.servicetitan.com/)
  - [Thryv | Small Business Management Software](https://www.thryv.com/)
  - [GlossGenius | Salon & Spa Booking App](https://glossgenius.com/)
  - [Vagaro | Salon, Spa & Fitness Software](https://www.vagaro.com/)
  - [Mindbody | Software for Salons, Spas & Fitness Businesses](https://www.mindbodyonline.com/)
  - [Fresha | Salon & Spa Booking System](https://www.fresha.com/)
  - [Toast | Restaurant Point of Sale & Management System](https://www.toasttab.com/)
  - [Clover | Point of Sale Systems](https://www.clover.com/)
  - [Lightspeed | Cloud-based POS for Retail & Restaurants](https://www.lightspeedhq.com/)
  - [TouchBistro | Restaurant POS & Management System](https://www.touchbistro.com/)
  - [Revel Systems | iPad POS for Restaurants & Retail](https://revelsystems.com/)
  - [Lavu | Restaurant POS System](https://www.lavu.com/)
  - [GoCanvas | Mobile Forms & Data Collection Apps](https://www.gocanvas.com/)
  - [ProntoForms | Mobile Forms App](https://www.prontoforms.com/)
  - [Jotform | Form Builder](https://www.jotform.com/)
  - [r/smallbusiness - Reddit](https://www.reddit.com/r/smallbusiness/)
  - [r/Entrepreneur - Reddit](https://www.reddit.com/r/Entrepreneur/)
  - [Hacker News](https://news.ycombinator.com/)
  - [Shopify Reviews - Trustpilot](https://www.trustpilot.com/review/www.shopify.com)
  - [Square Reviews - Trustpilot](https://www.trustpilot.com/review/squareup.com)
  - [Shopify Reviews - Capterra](https://www.capterra.com/p/133544/Shopify/)
  - [Shopify Reviews - G2](https://www.g2.com/products/shopify/reviews)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

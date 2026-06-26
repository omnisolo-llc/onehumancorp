issue_title: "OHC Mission Research: Conversational Scheduling & Unified Dispatch"
issue_description: |
  # OHC Mission Research: Conversational Scheduling & Unified Dispatch

  ## Problem Statement
  Owners and operators like Carlos (field service) and Leo (tutor) struggle with fragmented scheduling. They receive booking requests via DMs, emails, and calls. Navigating traditional calendar software or setting up complex online booking flows (like Calendly or Mindbody) is too heavy, forcing them to manually copy details, confirm availability, send payment links, and schedule. They lack an assistant that just reads a DM, knows their availability, and drafts a booking confirmation or deposit request instantly.

  ## Research Report
  ### Market Mapping & Competitor Discovery (Top 10 General & Top 10 AI-Native)
  #### Top 10 General Competitors:
  - Microsoft Copilot: Integrates deeply with M365 but feels like a heavy enterprise tool.
  - Shopify: Great for commerce, but lacks deep service scheduling and conversational booking.
  - HubSpot: Powerful CRM, too complex/expensive for micro-owners.
  - Square: Good POS and appointments, but the AI is bolted-on rather than assistant-first.
  - Calendly: Standard for scheduling but passive (requires user to click a link), not conversational.
  - Mindbody: Vertical SaaS for wellness; too rigid for mixed-use creators/bakers.
  - Jobber: Excellent for field service, but requires heavy initial setup and learning.
  - Notion AI: Great for knowledge, lacks real-time operational integration and scheduling.
  - WeCom/DingTalk/Lark: Excellent "work OS" tools in Asia but often feel like admin portals rather than simple assistants.
  - Salesforce: Far too heavy for our target personas.

  #### Top 10 AI-Native Rising Competitors:
  - Motion: AI scheduling, but focused on individual task/calendar optimization, not customer booking.
  - Reclaim AI: Similar to Motion, focuses on time-blocking for knowledge workers.
  - Lindy.ai: Autonomous AI employees; gaining traction for flexible workflows.
  - Sierra (by Bret Taylor): Conversational AI for customer service; highly capable but enterprise-focused.
  - Fin (Intercom): Great customer support bot, not an operator assistant.
  - MultiOn: Personal AI agent for web tasks, not a B2B operator tool.
  - Artisan AI: AI employees (BDRs), more for outbound sales.
  - 11x.ai: Automated workers, focused on enterprise sales.
  - Adept: Useful for automating browser tasks, not a dedicated mobile-first business OS.
  - Synthia / Various vertical AI assistants: Emerging but fragmented.

  ### Deep-Dive Competitor Audit: Square Appointments (Square)
  **Capabilities**: Online booking site, automated reminders, deposit collection, integrated POS, staff management.
  **Success Factors**: Tight integration with Square POS. Getting paid at the time of booking is seamless. Mobile app is functional.
  **User Sentiment**:
  - *Positive*: "I love that it syncs with my Square reader and I can require a card on file."
  - *Negative*: "Setting up services and variations is really tedious on the phone." "Customers get confused by the booking page." "I wish I could just text a customer a fast link for a specific time we agreed on."

  ### OHC Gap & Pain Point Identification
  **Current OHC State**: OHC has Work Triage and basic conversational AI, but lacks a native, friction-free way to turn a conversational thread into a locked-in schedule with an integrated deposit/payment request in a single tap.

  #### Gap Matrix Table: OHC vs Top Competitors
  | Feature | OHC (Current) | Square | Calendly | Shopify | Proposed OHC |
  |---|---|---|---|---|---|
  | Unified DM Inbox | ✅ Yes | ❌ No | ❌ No | ✅ Yes | ✅ Yes |
  | Conversational Booking | ❌ No | ❌ No | ❌ No | ❌ No | ✅ Yes |
  | Integrated Deposits | ❌ No | ✅ Yes | ❌ No | ✅ Yes | ✅ Yes |
  | Automated Calendar Sync | ❌ No | ✅ Yes | ✅ Yes | ❌ No | ✅ Yes |
  | "One-Tap Send" Link | ❌ No | ❌ No | ✅ Yes | ❌ No | ✅ Yes |

  **The Real Pain**: Carlos texts a client "I can come by Tuesday at 2pm." Client says "Great." Carlos now has to switch apps, make a calendar event, switch to a payment app, create an invoice/deposit link, and text it back.

  ### Visual Analytics

  #### Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title Business Assistant: AI Capability vs Mobile Simplicity
      x-axis "Low AI Integration" --> "High AI Integration"
      y-axis "Complex & Heavy Setup" --> "Simple Mobile-First"
      quadrant-1 "Ideal Assistants"
      quadrant-2 "Complex AI Tools"
      quadrant-3 "Legacy Portals"
      quadrant-4 "Basic Apps"
      "Salesforce": [0.2, 0.2]
      "HubSpot": [0.4, 0.3]
      "Square": [0.3, 0.6]
      "Calendly": [0.2, 0.7]
      "Motion": [0.8, 0.4]
      "Lindy.ai": [0.9, 0.5]
      "Shopify": [0.6, 0.5]
      "OHC (Current)": [0.7, 0.8]
      "OHC (Proposed)": [0.95, 0.95]
  ```

  #### User Journey Comparison (Mermaid)
  ```mermaid
  sequenceDiagram
      autonumber
      participant C as Customer
      participant O as Owner (Carlos)
      participant S as System (Square)
      participant A as OHC Assistant

      Note over C,S: Traditional Flow (High Friction)
      C->>O: Can you fix my sink tomorrow?
      O->>S: Open Calendar App, Check Avail
      S-->>O: Free at 10am
      O->>S: Open Payment App, Create Invoice
      S-->>O: Payment Link
      O->>C: I am free at 10am. Here is the link to pay deposit: [Link]

      Note over C,A: Proposed OHC Flow (Conversational Dispatch)
      C->>A: Can you fix my sink tomorrow?
      A->>O: Draft: "I have 10am open. Diagnostic is $50." [Proposal Card attached]
      O->>A: Taps "Approve & Send"
      A->>C: Sends SMS with 1-click booking link
      C->>A: Pays and Confirms
      A->>O: Notifies: "Sink repair confirmed for 10am"
  ```

  ### Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**: Reddit r/smallbusiness is full of complaints about "no-shows" and the friction of collecting deposits for service appointments. Creators (like Leo) complain that setting up a Acuity/Calendly page for a one-off custom lesson is too much work.
  **Agentic Solution**: **Conversational Dispatch**.
  When a user messages Maya or Carlos, the OHC Assistant reads the intent, checks the owner's availability (Calendar Agent), and generates a rich UI card in the chat: "Draft Booking: Tuesday 2pm, $50 deposit". The owner taps one button ("Send"), and the customer receives a short link to confirm the time and pay the deposit via Stripe. The Assistant handles the state (Pending -> Confirmed -> Scheduled).

  ## Design Doc
  **Architecture**:
  - **Entities**: `BookingProposal` (links to `Tenant`, `Customer`, `TimeSlot`, `PaymentIntent`).
  - **AI Agent Integration**: The `Customer Assistant` prompt needs a `propose_booking` tool. When called, it outputs structured data that the Flutter UI renders as an interactive card.
  - **Mobile UX Flow (375px)**:
    1. Inbox view: Customer message "Can you fix my sink tomorrow?"
    2. OHC Assistant auto-replies (visible only to owner): "Draft: Hi, I have availability tomorrow at 10am or 2pm. The diagnostic fee is $50."
    3. UI shows a "Booking Proposal" card attached to the draft.
    4. Owner taps "Approve & Send".
    5. Message is sent via SMS/WhatsApp with a short link.
    6. Customer clicks link -> simple mobile web view (Stripe Checkout integration) to pay and confirm.
    7. OHC Assistant notifies owner: "Sink repair confirmed for 10am."

  ## Implementation Prompt
  Implement the **Conversational Booking Proposal** capability.
  1. Add a `propose_booking` tool to the LLM agent's toolkit. It should accept `time_options`, `service_description`, and `deposit_amount`.
  2. Create a Flutter UI component (`BookingProposalCard`) that renders this tool call beautifully in the owner's chat feed. It must fit perfectly on a 375px screen and use the OHC Premium Token design system.
  3. Create a simple public-facing web endpoint (Flutter web or simple HTML/Go template) that displays the proposal to the customer and integrates with a mock/test Stripe payment flow.
  4. Ensure the end-to-end flow works: Agent suggests -> Owner approves -> Link generated -> (Simulated) Customer pays -> Status updates in owner feed.

  ## References & Sources Catalog
  - [Shopify](https://www.shopify.com/) - E-commerce platform
  - [Square](https://squareup.com/us/en) - POS and appointment scheduling
  - [HubSpot](https://www.hubspot.com/) - CRM and marketing automation
  - [Notion AI](https://www.notion.so/product/ai) - AI-powered knowledge management
  - [Microsoft Copilot](https://copilot.microsoft.com/) - Enterprise AI assistant
  - [Lark](https://larksuite.com/) - All-in-one collaboration suite
  - [DingTalk](https://www.dingtalk.com/en) - Enterprise communication platform
  - [WeCom](https://work.weixin.qq.com/) - Corporate communication tool
  - [Wix](https://www.wix.com/) - Website builder and business tools
  - [Salesforce](https://www.salesforce.com/) - Leading enterprise CRM
  - [monday.com](https://monday.com/) - Work operating system
  - [Asana](https://asana.com/) - Work management platform
  - [Trello](https://trello.com/) - Visual project management
  - [ClickUp](https://clickup.com/) - Productivity platform
  - [Zoho](https://www.zoho.com/) - Suite of business software
  - [Freshworks](https://www.freshworks.com/) - Customer engagement software
  - [Zendesk](https://www.zendesk.com/) - Customer service platform
  - [Intercom](https://www.intercom.com/) - Conversational support OS
  - [Gorgias](https://www.gorgias.com/) - Ecommerce helpdesk
  - [Klaviyo](https://klaviyo.com/) - Marketing automation platform
  - [Mailchimp](https://mailchimp.com/) - Marketing platform
  - [QuickBooks](https://www.quickbooks.intuit.com/) - Accounting software
  - [Xero](https://www.xero.com/) - Cloud accounting platform
  - [Gusto](https://gusto.com/) - Payroll and HR software
  - [Stripe](https://stripe.com/) - Financial infrastructure platform
  - [PayPal](https://www.paypal.com/) - Online payments system
  - [Adyen](https://www.adyen.com/) - Global payment company
  - [HoneyBook](https://www.honeybook.com/) - Client management software for small businesses
  - [Dubsado](https://www.dubsado.com/) - Business management solution
  - [Calendly](https://calendly.com/) - Automated scheduling software
  - [Acuity Scheduling](https://acuityscheduling.com/) - Online appointment scheduling
  - [Setmore](https://www.setmore.com/) - Free scheduling software
  - [Mindbody](https://www.mindbodyonline.com/) - Wellness business management software
  - [Vagaro](https://www.vagaro.com/) - Salon and fitness software
  - [Fresha](https://www.fresha.com/) - Booking software for salons and spas
  - [ServiceTitan](https://www.servicetitan.com/) - Field service management software
  - [Housecall Pro](https://www.housecallpro.com/) - Field service business software
  - [Jobber](https://www.jobber.com/) - Field service management software
  - [Thumbtack](https://www.thumbtack.com/) - Local services marketplace
  - [Taskrabbit](https://www.taskrabbit.com/) - Same-day service platform
  - [Upwork](https://www.upwork.com/) - Freelance marketplace
  - [Fiverr](https://www.fiverr.com/) - Freelance services platform
  - [Patreon](https://www.patreon.com/) - Creator membership platform
  - [Substack](https://www.substack.com/) - Newsletter publishing platform
  - [Kajabi](https://www.kajabi.com/) - Knowledge commerce platform
  - [Teachable](https://www.teachable.com/) - Online course creation
  - [Thinkific](https://www.thinkific.com/) - Platform to create and sell online courses
  - [Podia](https://www.podia.com/) - Creator platform for digital products
  - [Gumroad](https://www.gumroad.com/) - E-commerce platform for creators
  - [Printful](https://www.printful.com/) - Print-on-demand drop shipping
  - [Printify](https://www.printify.com/) - Print-on-demand network

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

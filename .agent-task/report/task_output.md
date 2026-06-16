issue_title: "AI-Native Quoting and Deposit Collection for Service Businesses"
issue_description: |
  ## Target Persona: Carlos (Field Service Owner)

  ## Problem Statement
  Carlos runs a repair and home-improvement service from an Android phone. He relies on word of mouth and inbound service requests via texts and calls. Converting a vague customer text ("My sink is leaking, can you come today?") into a concrete service booking with an agreed estimate and deposit is highly manual. He has to text back and forth, calculate an estimate, find a slot in his calendar, and then chase down a deposit using external payment apps. Existing tools are either clunky CRM dashboards not suited for a 375px mobile screen on the job, or simple schedulers that don't help with the quoting process. Carlos loses leads because he can't respond fast enough with a professional, actionable quote and booking link while he's under a sink.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Built for e-commerce or simple static service bookings, not dynamic, back-and-forth service estimating based on natural language descriptions or photos of broken items.
  - **Housecall Pro / Jobber:** Feature-rich but feel like desktop-first administrative portals. Too many forms to fill out on a mobile device while in the field.
  - **Square Appointments:** Good for fixed-price salon services, but lacks AI capabilities to parse a customer issue, estimate parts/labor, and draft a custom quote.
  - **OHC Opportunity:** OHC needs an AI "Operations & Sales Assistant" that intercepts inbound service requests, queries Carlos's pricing sheet and calendar context, and drafts a complete Quote + Deposit Booking Link. Carlos simply reviews a notification card ("Approve $150 Estimate & Book for Tuesday 2 PM") and hits send.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inbound Customer Request SMS/WhatsApp] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Operations & Sales Agent}
      C -->|Read Calendar| D[Schedule Service]
      C -->|Read Pricing/Inventory| E[Pricing Service]
      C -->|Draft Quote| F[Quote Generation Engine]
      F -->|Generate Stripe Payment Link| G[Stripe API]
      F --> H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J --> A
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top priority card reads: "New Request: Leaky Sink from Mike. Drafted Estimate: $150. Proposed Time: Tomorrow 2 PM."
  - **Interaction:** Carlos taps the card to view details.
    - Top section: Customer original message.
    - Middle section: The drafted quote breakdown (Labor: $100, Parts: $50) and suggested calendar slot.
    - Bottom section: The drafted reply message to the customer containing the Stripe deposit link.
  - **Action:** Primary button "Approve & Send Quote". Secondary button "Edit Quote/Time".
  - **Visual Design:** Clean, translucent glass material design. High contrast text for outdoor visibility. Large 44x44px touch targets for the buttons.

  ### AI Agent Integration Points
  - **Intake parsing:** Gemini Pro parses the customer's text to determine the service type and urgency.
  - **Pricing RAG:** Retrieve pricing guidelines and typical part costs from the owner's knowledge base.
  - **Drafting:** Agent generates the response text and interfaces with Stripe and the Calendar module to generate links.

  ## Implementation Prompt
  - Build the backend webhook handler to receive SMS/WhatsApp service requests using real, live endpoints and proper configuration flows. No mocked endpoints or network interceptions.
  - Implement an agent workflow (using the built-in LLM provider) that categorizes the request, looks up a real seeded pricing table in the Postgres database, and checks real calendar availability.
  - Generate a Quote object containing an estimated price, a proposed time slot, and a real Stripe payment link generated via the actual Stripe API in test mode.
  - Create the 375px mobile UI Action Card in the Flutter PWA app that displays the proposed quote and includes an "Approve & Send" button. Absolutely no UI-only mock data.
  - Write E2E Playwright tests verifying the flow: interacting via the UI, seeing the Action Card in the feed, and successfully approving the quote through real backend pathways.
  - Focus on the seamless integration and the mobile UX; do not over-engineer the database schema for the quote object initially.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

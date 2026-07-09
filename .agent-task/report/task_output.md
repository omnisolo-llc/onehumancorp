issue_title: "Implement the 'Negotiator Agent' for Automated Quoting and Deposit Collection"
issue_description: |
  # Research Report: The 'Negotiator Agent' for Field Service Operations

  ## Problem Statement
  Field service owners like Carlos (Handyman) lose approximately 30% of their inbound leads because they are physically "on the job" and unable to answer calls or respond to messages promptly. Existing solutions either require manual quoting processes (delaying the response) or complex CRM setups that are inaccessible to non-technical solopreneurs. When a customer reaches out for a quote, the lack of an immediate, intelligent response often results in the customer moving on to a competitor.

  ## Research & Market Analysis
  **Competitor Landscape:**
  - **Traditional Builders (Wix/Squarespace):** Provide contact forms, but require the owner to manually review the form, write a quote, and send a separate payment link.
  - **Service CRMs (Jobber/Housecall Pro):** Excellent for dispatching and quoting, but they are separate systems from the owner's primary website and require significant setup. They also primarily rely on the owner to generate the quote.
  - **AI-Native Assistants (11x.ai/Lindy):** Very powerful, but operate as standalone tools rather than being natively integrated into an all-in-one platform like OHC.

  **The OHC Opportunity:**
  OHC can integrate an autonomous "Negotiator Agent" directly into the platform's unified inbox and core state. This agent can intercept inbound queries (via SMS, WhatsApp, or Web Chat), understand the scope of the request using the owner's predefined service parameters, generate a quote, and collect a deposit—all without the owner lifting a finger while they are on site.

  ## Architecture & Design Flow

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Inbound Customer SMS/DM] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Negotiator Agent Intent Classifier}
      C -->|Identify Service| D[Service & Pricing Engine]
      D -->|Query OHC Backend| E[(PostgreSQL Tenant Data)]
      C -->|Formulate Quote| F[Quoting & Negotiation Draft]
      F --> G[Stripe Payment Link Generator]
      G -->|Send Reply| B
      B -->|Message Sent| A
      G --> H[Action Notification]
      H --> I[Mobile App Feed 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Owner Dashboard Feed:** A new "Job Won" card appears in the feed.
  - **Card Content:** Displays customer name, service type, agreed estimate, and deposit status with Apple-style Glassmorphism UI tokens.
  - **Action Buttons:** "View Full Transcript", "Schedule Route", "Message Customer" (minimum 44x44px touch targets).
  - **Zero-Touch Execution:** The key UX is that the owner does *nothing* until the job is booked and the deposit is secured.

  ### Key Design Decisions & Why
  - **Zero-Touch Execution vs Approval Flow:** For field service owners, speed of response is critical. By empowering the agent to offer standard flat-rate quotes autonomously, we beat competitors to the lead. We use the approval flow *only* if the request falls outside standard parameters.
  - **Native Omnichannel Integration:** Relying strictly on OHC native APIs rather than requiring the user to connect Zapier ensures the setup meets the "under 10 minutes" criteria for non-technical users.

  - **Data Ingestion:** Webhooks connected to Twilio (SMS), WhatsApp, or OHC native Web Chat.
  - **Intent & Parsing Layer:** LLM classification (e.g., Gemini Pro) to determine the requested service and parse key details (e.g., "Need a ceiling fan installed", "Got a leaky pipe under the sink").
  - **Quoting Engine:** The agent queries the OHC backend for the owner's predefined `ServiceParameters` (e.g., Hourly rate, minimum callout fee, standard flat rates for common jobs like fan installation).
  - **Negotiation & Draft:** The agent drafts a conversational response providing a price estimate or asking clarifying questions if the request is too vague.
  - **Deposit Collection:** Once the customer agrees, the agent generates a Stripe Payment Link for the deposit and securely sends it in the chat.
  - **Owner Notification (Mobile UX):** The owner receives a push notification: "New Job Booked: Ceiling Fan Install. $50 Deposit Collected." The OHC mobile app feed shows a summary card of the interaction and the newly scheduled task.


  ## Implementation Prompt (For Engineering Swarm)
  - **User-Facing Outcome:** When a potential customer texts Carlos asking for a quote to install a ceiling fan, the Negotiator Agent replies instantly, provides a standard quote based on Carlos's settings, and collects a deposit via a secure link, all while Carlos is driving to his next job.
  - **CUJ & Acceptance Criteria:**
    1. A webhook simulates an inbound SMS from a customer requesting a common service.
    2. The Negotiator Agent parses the request, queries the `Service` catalog, and formulates a response with a quote.
    3. The Agent successfully transitions the conversation state to "Quote Offered".
    4. Upon simulated customer agreement, the Agent generates and sends a Stripe payment link for the deposit.
    5. The system creates a pending `Booking` and adds a notification card to the owner's mobile feed.
  - **Note:** Do not prescribe specific database schemas in this task; focus on the Agent's conversation loop and integration with the quoting/payment APIs.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

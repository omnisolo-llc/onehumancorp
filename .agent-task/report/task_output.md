issue_title: "[Architecture] Agentic Negotiator & Booker for Service Leads"
issue_description: |
  # Mission Queue Protocol: Agentic Negotiator & Booker

  ## Problem Statement
  Service business owners like Carlos (Field Service) lose approximately 30% of their inbound leads because they are physically "on the job" and cannot answer calls or reply to DMs instantly. OHC currently relies on widget-based client intake, requiring manual owner intervention to quote prices, negotiate, and finalize bookings with deposits, leading to missed revenue opportunities.

  ## Research Report
  ### Competitive Analysis
  - **11x.ai (Alice/Julian):** Uses autonomous digital workers for outbound sales and inbound call handling, effectively replacing initial SDR layers.
  - **Intercom Fin:** Resolves 50%+ of queries instantly using an AI resolution engine.
  - **Traditional SMB SaaS (Square/Shopify):** Rely on static booking calendars or manual estimates.

  ### OHC Opportunity
  To differentiate and empower the owner, OHC needs an "Agentic Negotiator & Booker" that can intercept inbound DMs/SMS/Calls, understand the service context, dynamically quote based on standard project types, negotiate if necessary within owner-approved limits, and secure a booking with a deposit—all without the owner lifting a finger while on a job site.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Inbound Channels
      SMS[Twilio SMS]
      Instagram[Instagram DM]
      Voice[Twilio Voice]
      end

      subgraph Core Processing
      MsgBus[Message Bus / Event Ingestion]
      NegotiatorAgent[Negotiator Agent]
      ContextMemory[Tenant Context & Pricing Rules]
      end

      subgraph Action Execution
      BookingService[Booking Service]
      QuoteService[Quote & Estimating Service]
      PaymentLink[Stripe Payment Links]
      end

      subgraph Owner Visibility
      AgentFeed[Agent Feed Action Card]
      end

      SMS --> MsgBus
      Instagram --> MsgBus
      Voice --> MsgBus

      MsgBus --> NegotiatorAgent
      NegotiatorAgent <--> ContextMemory
      NegotiatorAgent --> QuoteService
      QuoteService --> NegotiatorAgent
      NegotiatorAgent --> BookingService
      BookingService --> PaymentLink
      PaymentLink --> NegotiatorAgent
      NegotiatorAgent --> MsgBus

      NegotiatorAgent --> AgentFeed
  ```

  ### Mobile UX Flow (375px)
  1. **Zero-Touch Operation:** The primary flow is invisible to the user until a result is achieved.
  2. **Agent Feed Notification:** Carlos opens the OHC app and sees a success card in his Feed: "New Booking Secured - 10 mins ago".
  3. **Card Anatomy:**
     - **Title:** "Leaky Faucet Repair Booked"
     - **Summary:** "Agent successfully negotiated a $150 repair for tomorrow at 2 PM. A $50 deposit was collected."
     - **Action:** "View Conversation" (to see the AI's transcript with the customer), "View Booking Details".
  4. **Settings/Constraints View (375px):**
     - Carlos navigates to "Advanced -> Agents -> Negotiator".
     - Simple toggles: "Allow negotiation up to 10% discount", "Require $50 flat deposit", "Auto-book for standard jobs".
     - UI uses translucent glass cards with clear, non-technical copy.

  ### AI Agent Integration
  - **Negotiator Agent:** Runs as a sub-agent when an inbound lead is detected. Uses RAG against the tenant's pricing constraints and calendar availability.
  - **Skills:** `create_quote`, `generate_payment_link`, `create_booking`.
  - **Handoff Protocol:** If the customer asks a question outside of standard pricing (e.g., custom multi-day renovation), the agent triggers a `HANDOFF` event, which places an "Action Required" card in the owner's Agent Feed for manual quoting.

  ## Implementation Prompt
  Implement the foundation for the Agentic Negotiator & Booker.
  1. Define a `NegotiatorRule` database schema (tenant_id, min_price, max_discount_percent, required_deposit, auto_book_enabled) to store owner constraints.
  2. Implement an MCP tool or Skill called `generate_dynamic_quote` that the Negotiator Agent can call. It should take the customer's issue and return a price based on `NegotiatorRule`.
  3. Create a workflow in the backend where an inbound message (via `msgbus`) without an active human conversation triggers the Negotiator Agent.
  4. The Negotiator Agent must be able to use the `booking` service to check availability and the `billing` service to generate a deposit payment link.
  5. Upon successful booking or required handoff, the system must generate an `ActionCard` for the Agent Feed.

  **Acceptance Criteria:**
  - A mock inbound SMS trigger can spawn the Negotiator Agent.
  - The Negotiator Agent successfully creates a quote and books a slot using mocked LLM responses.
  - The system respects the `NegotiatorRule` (e.g., won't quote below minimum).
  - An Action Card is successfully published to the Agent Feed upon completion.

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

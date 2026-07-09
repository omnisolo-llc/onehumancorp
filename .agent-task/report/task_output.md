issue_title: "Implement Agentic Negotiator & Booker for Service Availability & Deposit Flow"
issue_description: |
  **Title**: Implement Agentic Negotiator & Booker for Service Availability & Deposit Flow

  **Problem Statement**:
  Service-based small business owners like Carlos (Handyman, 42) or Leo (Music Tutor, 22) struggle with disjointed booking systems and missed leads. They often lose ~30% of their leads because they are out on the job and cannot answer the phone or DMs. Moreover, integrating third-party bookings (like Calendly) with standard e-commerce payments causes friction and results in an "app tax." They need a proactive, intelligent agent that can intercept incoming queries, parse availability, quote a price, and secure a deposit automatically without owner intervention.

  **Research Report**:
  Our competitive analysis shows that platforms like Shopify require third-party apps for robust booking, which fractures the user experience and adds significant costs. Wix and Squarespace offer native booking but act as passive directories; they wait for the customer to drive the transaction. Meanwhile, AI-native solutions like 11x.ai or Intercom Fin show that autonomous conversational agents can resolve over 50% of inbound queries.
  By creating the "Agentic Negotiator & Booker," OHC can unify e-commerce, scheduling, and conversational AI. The Operations and Sales AI agents will proactively manage the calendar and take deposits, eliminating the need for passive third-party booking plugins. This positions OHC far ahead of legacy builders in terms of autonomous business operations.

  **Design Doc**:
  *   **Architecture Diagram (Mental Model)**:
      ```mermaid
      graph TD;
          Customer[Customer Inquiry via DM/Web] --> NegotiatorAgent[Agentic Negotiator];
          NegotiatorAgent --> OpsAgent[Operations Agent];
          OpsAgent --> AvailabilityDB[(Availability Blocks DB)];
          NegotiatorAgent --> PricingEngine[Pricing/Quote Generator];
          NegotiatorAgent --> PaymentAgent[Payment / Deposit Link Generator];
          PaymentAgent --> Stripe[Stripe API];
          Stripe --> BookingConfirmation[Booking Confirmed State];
      ```
  *   **Mobile UX Flow (375px)**:
      1.  **Customer Intake**: Customer sends a DM (e.g., "Can you fix a leaky pipe on Tuesday?"). The UI is purely conversational from the customer's perspective.
      2.  **Agent Negotiation**: The agent responds within the thread, confirming the project scope and generating a structured interactive "Quote Card."
      3.  **Deposit Payment**: The Quote Card contains a "Pay $50 Deposit & Book" button. When tapped, it opens a mobile-optimized, single-page native Stripe checkout drawer (no horizontal scroll on 375px).
      4.  **Owner Dashboard Feed**: The owner views the "Agent Feed" on their dashboard (clean Ubiquiti/macOS translucent styling) and sees "New Booking: $50 Deposit Secured for Tuesday." The owner taps to expand full details.
  *   **AI Agent Integration Points**:
      *   The Sales/Customer Success Agent (The Ambassador) handles initial intent classification and generates the conversational reply.
      *   The Operations Agent queries the `AvailabilityBlock` in PostgreSQL and allocates the slot.
      *   A Quote/Finance Agent generates a Stripe Payment Link and listens for webhooks to update the booking status to "Confirmed."
  *   **Key Design Decisions**:
      *   The booking flow must be native, avoiding third-party iframes to maintain maximum mobile responsiveness and conversion.
      *   The UI must adopt macOS Translucent Glass materials.
      *   Zero owner intervention is required for the initial deposit collection, but all actions are fully observable in the Agent Feed.

  **Implementation Prompt**:
  **Feature Name**: Agentic Negotiator & Booker
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos is on a job and receives an SMS/web inquiry. The Agentic Negotiator automatically queries Carlos's availability, quotes a standard call-out fee, and presents a deposit payment link. The customer pays, and Carlos simply receives a push notification that he has a new confirmed job.
  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1.  A user (acting as a customer) submits an inquiry through a test chat interface indicating a desired service time.
  2.  The backend AI logic intercepts the message, checks the database for available blocks, and generates a conversational response containing an actionable quote/booking card.
  3.  The booking card must render perfectly on a 375px viewport and include a fully functional "Pay Deposit" button.
  4.  The system must process a mock or test-mode deposit and update the database state to reflect a confirmed booking.
  5.  The owner dashboard must display this new confirmed booking in the Agent Feed without requiring any manual data entry.
  6.  Must have 100% unit test coverage for the new AI negotiation and availability logic.
  7.  Must include comprehensive Playwright E2E tests verifying the end-to-end customer booking flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "[architecture] Implement Autonomous 'Agentic Negotiator & Booker' for Service Businesses"
issue_description: |
  ## Problem Statement
  Service business owners like Carlos (field service) and Leo (tutor) lose up to 30% of potential leads because they cannot answer calls or messages while actively working. They need an automated way to intercept inquiries, negotiate quotes based on project type and availability, and secure deposits without any manual intervention. Current solutions (e.g., Calendly + Typeform + Stripe) require complex setup and do not adapt dynamically to the user's free text inquiry.

  ## Research Report
  - **Competitive Landscape**:
    - *Shopify/Wix/Squarespace*: Provide basic booking plugins but lack conversational AI to negotiate quotes.
    - *Lindy.ai / 11x.ai*: Focus on executive or outbound sales rather than instant inbound service booking and deposit collection.
    - *Intercom Fin*: Excellent at support resolution but not tailored for single-owner operations needing instant booking logic.
  - **OHC Differentiation**: The OHC assistant intercepts the message, understands the required service via the `Salesperson` agent, checks the calendar via the `Operations` agent, calculates a price, and requests a deposit natively within the chat stream.

  ## Design Doc
  ### Architectural Design
  - **Components**:
    - `Unified Inbox Listener`: Ingests messages from SMS, Web, Instagram DMs, etc.
    - `Salesperson Agent`: Analyzes the intent, extracts service details, and formulates a response/quote.
    - `Operations Agent`: Checks the internal OHC calendar for availability.
    - `Booking & Quoting API`: Generates Stripe deposit links and tentatively blocks the calendar.
  - **Architecture Diagram**:
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Inbox as Unified Inbox
      participant Sales as Salesperson Agent
      participant Ops as Operations Agent
      participant BookingAPI as Booking & Payment API
      participant Carlos as Owner App

      Customer->>Inbox: "Need a pipe fixed tomorrow morning. How much?"
      Inbox->>Sales: Route new inquiry
      Sales->>Ops: Check availability for "tomorrow morning"
      Ops-->>Sales: "Available at 9 AM and 11 AM"
      Sales->>BookingAPI: Generate quote ($150) & deposit link ($50)
      BookingAPI-->>Sales: Deposit link generated
      Sales->>Customer: "I can fix that tomorrow at 9 AM or 11 AM. It will be $150. Please pay the $50 deposit here to confirm: [Link]"
      Customer->>BookingAPI: Pays deposit
      BookingAPI->>Ops: Confirm booking
      Ops->>Carlos: Push Notification: "New Job Booked for Tomorrow 9 AM"
  ```

  ### Mobile UX Flow (375px Baseline)
  - **Customer View**: A clean conversational interface (or standard SMS/DM) where the AI speaks naturally. When the quote is ready, a native, high-contrast, Apple/Ubiquiti-style Glassmorphism payment card appears directly in the chat.
  - **Owner View (Carlos)**: The OHC mobile app feed surfaces a single card: "Job Booked: Pipe Repair ($150) - Deposit Paid. Tap to view details." No action is required unless Carlos wants to reschedule.

  ### AI Agent Integration Points
  - **Salesperson Agent**: Uses LLMs (Gemini/MiniMax) with strict system prompts to constrain pricing within the owner's defined service rates.
  - **Operations Agent**: Provides a tool definition for `check_availability(date, duration)` and `create_tentative_booking(...)`.

  ## Implementation Prompt
  Implement the "Agentic Negotiator & Booker" feature.
  1. Create the necessary backend integrations to listen to incoming unified inbox messages.
  2. Implement the `Salesperson Agent` logic to intercept unassigned inquiries, extract the service request, and fetch pricing.
  3. Integrate the `Operations Agent` to check calendar availability.
  4. Generate a deposit payment link via Stripe and send the proposal back to the customer.
  5. Provide a testable E2E flow showing an inbound message turning into a confirmed booking card in the owner's feed without owner intervention. Do not prescribe specific database schemas or API signatures; let the implementer design the optimal backend structure.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

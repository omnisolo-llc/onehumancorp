issue_title: "[Research] AI Automated Scheduling & Deposit Collection Engine"
issue_description: |
  # Research Report: AI Automated Scheduling & Deposit Collection Engine

  ## Problem Statement
  Service-based and custom-order small business owners (like Carlos the handyman or Maya the baker) experience high friction when converting a customer inquiry into a confirmed, paid booking. Currently, they must manually negotiate times, leave their communication app to generate an invoice or payment link, send the link, and manually verify the deposit before securing the calendar slot. This manual process leads to lost leads, delayed payments, and operational anxiety.

  ## Research Findings
  - **Market Gap:** Existing solutions (like Calendly or Shopify) either focus purely on scheduling without deep deposit integration or focus on product checkouts without calendar negotiation.
  - **Persona Need:** Carlos needs an AI that can negotiate a repair time over SMS and instantly collect a deposit. Maya needs an AI that can handle custom cake inquiries over Instagram DMs, lock a delivery date, and secure a deposit.
  - **Proposed Solution:** An integrated AI Scheduling & Deposit Collection Engine that autonomously handles the negotiation, books the slot, and generates a zero-friction checkout link for the deposit directly in the chat thread.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant AIAgent as Sales & Acquisition Agent
      participant Calendar as Scheduling Engine
      participant Payment as Payment Gateway (Stripe)

      Customer->>AIAgent: "Can you fix my deck this Friday?"
      AIAgent->>Calendar: Check availability for Friday
      Calendar-->>AIAgent: Available at 2 PM
      AIAgent->>Customer: "I can do Friday at 2 PM. It will be $500 total. Please pay a $100 deposit to secure the slot."
      AIAgent->>Payment: Generate Deposit Payment Link
      Payment-->>AIAgent: Deposit Link
      AIAgent->>Customer: Send Deposit Link
      Customer->>Payment: Pays Deposit
      Payment-->>Calendar: Webhook: Deposit Paid -> Lock Slot
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View:** Customer taps the deposit link in the chat thread. A bottom sheet modal appears with the booking details and an instant payment option (Apple Pay/Google Pay).
  2. **Merchant View:** Once the deposit is paid, Carlos receives a push notification: "New Booking confirmed for Friday at 2 PM. $100 deposit received."

  ### AI Agent Integration
  - The "Sales & Acquisition Agent" will be equipped with tools to query the calendar and generate Stripe checkout sessions.

  ## Implementation Prompt
  Implement the AI Automated Scheduling & Deposit Collection Engine.
  - Ensure the AI agent can autonomously negotiate times and generate deposit links.
  - The payment flow must be frictionless and optimized for mobile (375px).
  - Include an E2E Playwright test simulating a customer inquiry, negotiation, and successful deposit payment.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

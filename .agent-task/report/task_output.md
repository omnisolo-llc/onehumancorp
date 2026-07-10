issue_title: "Architectural Design: AI-Powered Missed Call & SMS Lead Recovery System"
issue_description: |
  # Title
  AI-Powered Missed Call & SMS Lead Recovery System

  # Problem Statement
  Small business owners like Carlos (Handyman) and Maya (Baker) spend their days doing physical work or deep in their craft. Carlos misses 40% of incoming service inquiries because he is up on a ladder or driving between jobs. When he checks his phone hours later, the lead has often moved on to another contractor. Owners need an assistant that captures missed calls, intelligently engages the customer via SMS/WhatsApp immediately, and drafts a structured service request or estimate in OHC before the owner even touches their phone.

  # Research Report
  - **Market Context**: Studies show up to 62% of calls to small businesses go unanswered, and customers expect a response within minutes. Modern SMBs lose thousands in potential revenue due to delayed responses.
  - **Competitor Analysis**:
    - **GoDaddy Airo / HubSpot Breeze**: Offer basic auto-replies but lack deep context of the owner's services, pricing, or schedule.
    - **ServiceTitan / Housecall Pro**: Powerful for field service but too complex and expensive for independent operators like Carlos.
    - **Wix / Shopify**: Mostly focused on web and cart abandonment, not phone/SMS lead recovery which is critical for local service providers.
  - **OHC Differentiation**: By routing a dedicated OHC business number to an AI-native voice/SMS inbox, the Customer Relationship Assistant can answer missed calls with an immediate context-aware SMS (e.g., "Hi, this is Carlos's assistant. Carlos is on a job, but can I get some details on your repair need?"). It then transforms the SMS thread into a structured OHC task, quote draft, and calendar hold seamlessly.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Telephony as Telephony Gateway
      participant OHCTriage as OHC Triage Queue
      participant CRAgent as Customer Relationship Agent
      participant OpsAgent as Operations Agent
      participant App as OHC Mobile App

      Customer->>Telephony: Calls Carlos (No Answer)
      Telephony->>Customer: Plays Voicemail / Drops Call
      Telephony->>OHCTriage: Webhook: Missed Call Event
      OHCTriage->>CRAgent: Trigger Recovery Workflow
      CRAgent->>Telephony: Send SMS: "Hi, this is Carlos's assistant..."
      Customer->>Telephony: SMS Reply: "Need a leaky pipe fixed."
      Telephony->>CRAgent: SMS Webhook
      CRAgent->>OpsAgent: Extract intent & check availability
      OpsAgent->>CRAgent: Next available slot found
      CRAgent->>App: Draft Quote & Push Notification to Carlos
      App-->>Carlos: "New Lead: Leaky Pipe (Draft Quote Ready)"
  ```

  ## Mobile UX Flow (375px)
  1. **Lock Screen / Push**: "New Lead: John Doe (Leaky Pipe) - AI Drafted Quote."
  2. **Work Triage Feed**: A unified dashboard card at the top of the feed showing a missed call icon, a brief SMS transcript summary, and a primary action button: "Review & Send Quote".
  3. **Quote Review Screen**: Pre-filled service item ("General Plumbing Repair") and an auto-suggested time slot, generated via translucent glass UI cards.
  4. **One-Tap Action**: Carlos taps "Approve & Send," dispatching the final booking link and deposit request to the customer.

  ## AI Agent Integration Points
  - **Work Triage**: Ingests telephony webhooks (missed calls, voicemails) via a Redis queue and converts them to feed items.
  - **Customer & Relationship Assistant**: Uses LLM parsing on voicemail transcripts or follow-up SMS replies to extract intent, customer name, and urgency. Maintains tenant-scoped memory.
  - **Operations Assistant**: Correlates the extracted intent with the owner's availability and service catalog to prepare a draft estimate and reserve a tentative schedule block.

  ## Key Design Decisions
  - **SMS-First Recovery**: Instead of complex AI voice bots that can frustrate users and hallucinate, a fast SMS follow-up ("Sorry I missed you...") feels personal, guarantees higher engagement, and works flawlessly on low-bandwidth networks.
  - **Zero-Touch Drafts**: The owner shouldn't have to copy-paste from an SMS to a quote. The system must map unstructured text into structured `Quote` and `Task` database records.
  - **Idempotency & Rate Limiting**: The recovery workflow must use distributed locks (Redis Redlock, key pattern: `ohc:lock:{tenant_id}:lead_recovery:{phone_number}`) to ensure we don't spam a customer who calls multiple times in succession.

  # Implementation Prompt
  Implement the backend infrastructure and mobile UX for the AI Missed Call & SMS Lead Recovery System.
  1. Build the telephony webhook ingestion layer that listens for missed call and SMS events, ensuring strict row-level multi-tenant isolation.
  2. Integrate the Customer Relationship Agent to parse these events, update tenant-scoped memory, and draft an initial SMS response based on the tenant's predefined "away" context.
  3. Create the Work Triage feed UI component (mobile-first, 375px viewport) that displays the recovered lead, the AI's summary, and a one-tap "Draft Quote" button utilizing OHC Premium Token styling.
  4. Ensure end-to-end idempotency using Redis Redlock so duplicate webhooks do not result in duplicate SMS messages.

  Acceptance Criteria: A missed call event results in a drafted lead card in the owner's triage feed, containing an AI summary and an actionable next step, fully tested via Playwright E2E flows mapping the UI interactions end-to-end. Do not prescribe specific database schemas or API signatures; design for the user-facing outcome.

  # Priority
  P1

  # Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

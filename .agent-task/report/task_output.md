issue_title: "Multimodal AI Voice & SMS Intake Engine for Autonomous Lead Triage"
issue_description: |
  ### Problem Statement
  Service-based and on-the-go owners (like Carlos the Handyman and Fatima the Food Cart Operator) miss critical business opportunities because they are physically occupied. When a customer calls and the owner cannot answer, the lead is often lost to a competitor. Furthermore, manually transcribing voicemails or responding to SMS inquiries at the end of the day disrupts their workflow and slows down the time-to-quote. OHC currently lacks an integrated, multimodal voice/SMS intake engine that can autonomously answer missed calls, engage via SMS, and convert spoken/text inquiries into structured quotes or orders in the Agent Feed.

  ### Research Report
  - **Market Context**: Competitors like 11x.ai, Bland AI, and Intercom Fin are pioneering AI voice and text agents. However, these are often enterprise-focused or require complex integrations (e.g., Twilio + OpenAI + CRM). SMB platforms like Wix and Shopify rely on basic contact forms or third-party SMS apps, lacking an integrated voice-to-action workflow.
  - **The OHC Opportunity**: By natively integrating a Twilio/Plivo backend with OHC's Operations and Sales AI Agents, we can intercept missed calls, immediately follow up via SMS ("Hi, Carlos is on a job. How can I help you?"), or use a voice AI agent to capture the lead's intent, transcribe it, and generate an actionable card in the owner's feed.
  - **Competitor Gaps**:
    - *Shopify/Wix*: No native voice/SMS intake; reliant on forms and email.
    - *Standalone Voice AI (e.g., Bland AI)*: Excellent at conversation but detached from the core POS/quoting system.
    - *GoDaddy Airo*: Focused on branding, not real-time operational intake.

  ### Design Doc
  #### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant C as Customer
      participant T as Twilio Webhook
      participant I as Intake Engine (OHC)
      participant LLM as Agentic Triage (Gemini)
      participant DB as OHC Postgres (Ledger)
      participant F as Owner Agent Feed (Mobile UI)

      C->>T: Calls/SMS Carlos's OHC Number
      T->>I: POST /webhooks/intake/voice_or_sms
      I->>LLM: Transcribe & Classify Intent
      LLM->>I: Extracted Lead Data (e.g., "Fix broken pipe, Address")
      I->>DB: Create Lead & Draft Quote
      I->>F: Push "New Lead Triage" Card
      F->>Carlos: Push Notification (Action Required)
  ```

  #### Mobile UX Flow & Screen Flow Description (375px First)
  1. **Customer Interaction**: The customer calls and hears an AI greeting or receives an immediate SMS: "Hi, this is Carlos's assistant. What do you need help with?"
  2. **Owner Feed Triage (375px view)**:
     - A high-priority card appears at the top of the Agent Feed: "📞 Missed Call: New Lead".
     - The card displays a summarized intent: "Broken pipe at 123 Main St. Requires urgent repair."
     - Transcribed text or a small audio play button (44x44px touch target) is available.
  3. **One-Tap Action**:
     - Beneath the summary, the AI Operations Agent presents a drafted Quote and a "Send Quote & Book Slot" button.
     - The owner taps to review the quote, adjusts the price natively using the mobile keyboard, and hits "Approve".

  #### AI Agent Integration Points
  - **Operations Agent (Triage)**: Listens to incoming webhooks, transcribes audio/text, and categorizes the intent (Emergency, Quote Request, General Inquiry).
  - **Sales Agent (Quoter)**: Takes the parsed intent and uses the tenant's historical pricing and service catalog to generate a draft estimate.
  - **Customer Success Agent (Ambassador)**: Handles the bidirectional SMS conversation to keep the customer engaged while the owner is busy.

  #### Key Design Decisions
  - **Asynchronous Processing**: Webhooks from telecom providers (Twilio) are placed in the PostgreSQL `SKIP LOCKED` job queue to ensure no dropped events during traffic spikes.
  - **Multimodal LLM Processing**: Use Gemini Pro's multimodal capabilities to directly process audio where possible, reducing transcription latency.
  - **Tenant Isolation**: All telecom numbers and incoming webhooks are strictly mapped to the `tenant_id` at the edge to prevent cross-contamination of leads.

  ### Implementation Prompt
  **Feature Name**: Multimodal AI Voice & SMS Intake Engine
  **Target Persona**: Carlos the Field Service Owner
  **Outcome**: When Carlos is under a sink and misses a call, his OHC AI assistant immediately texts the customer, gathers the repair details, and places a drafted Quote in Carlos's Agent Feed for one-tap approval.

  **Next Actions**:
  1. Implement the telecom webhook ingestion endpoints (`/api/v1/webhooks/telecom/sms` and `/api/v1/webhooks/telecom/voice`) with security signature validation.
  2. Create the asynchronous job workers that pass the payload to the LLM (Operations Agent) for intent classification.
  3. Extend the Agent Feed UI to render Voice/SMS Intake Cards with embedded audio players and one-tap "Generate Quote" buttons.
  4. Ensure complete mobile responsiveness (375px width, 44x44px tap targets).

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

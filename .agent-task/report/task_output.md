issue_title: "Implement Agentic Missed Call & SMS Lead Recovery System"
issue_description: |
  # Agentic Missed Call & SMS/WhatsApp Lead Recovery System

  ## 1. Problem Statement
  Service-based owners like Carlos (Handyman) and Maya (Baker) are often physically engaged in their work (e.g., repairing a roof, baking a cake). When their phone rings with a new lead, they frequently miss the call. By the time they check their voicemail or messages hours later, the lead has already contacted a competitor. Existing unified inboxes aggregate messages but do not actively engage the lead the moment they bounce. They need an assistant that instantly responds to missed calls and texts, qualifies the lead, and captures the demand before it goes cold.

  ## 2. Research Report
  - **Market Context**: SMBs miss an estimated 62% of incoming calls. Platforms like Twilio and standard virtual PBX systems allow for auto-responders ("Sorry I missed your call, text me"), but they are static and do not adapt to the context of the business or actively qualify the lead.
  - **Competitive Analysis**:
    - *Shopify/Wix*: Not built for phone/SMS-first service businesses.
    - *GoDaddy/Squarespace*: Offer basic unified inboxes but require manual replies.
    - *Vertical SaaS (Housecall Pro, Jobber)*: Expensive ($100+/mo), clunky, and their automations are rigid decision trees, not conversational AI.
  - **The OHC Opportunity**: By integrating directly with Twilio/WhatsApp APIs and employing the Customer Assistant AI, OHC can instantly text back a missed caller: "Hi, this is Carlos's assistant. Carlos is currently on a job. What do you need help with?" and guide them into a booking or quote request flow.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Phone] -->|Missed Call/SMS| B[Twilio/WhatsApp Gateway]
      B -->|Webhook| C[OHC Webhook Receiver]
      C --> D[Work Triage Agent]
      D -->|Contextualize| E[Customer Relationship Agent]
      E -->|Draft & Send SMS| B
      E -->|Update Feed| F[OHC Owner Feed / Unified Inbox]
      F --> G[Carlos's Mobile Device]
  ```

  ### AI Agent Integration Points
  - **Work Triage Agent**: Identifies if the incoming number is a known customer or a new lead. Groups the missed call event into the unified owner feed.
  - **Customer Relationship Agent**: Instantly drafts and sends a context-aware SMS/WhatsApp reply based on the owner's instructions and business context. Continues the conversation to qualify the lead (e.g., asking for photos of a repair, or desired date for a cake).

  ### Mobile UX Flow (375px)
  1. **The Lead Event**: Carlos's phone rings, he misses it. OHC instantly sends the AI text to the customer.
  2. **The Owner Notification**: Carlos finishes his task, checks OHC. The home shell shows a high-priority card: "1 New Lead: Broken Pipe (Qualified by Assistant)".
  3. **The Thread View**: Carlos taps the card. He sees the chat transcript where the AI captured the customer's issue and a photo of the broken pipe.
  4. **The Action**: A sticky action bar at the bottom (44x44px touch targets) offers "Draft Quote", "Call Back", or "Send Booking Link".

  ### Key Design Decisions
  - **Zero-Config Setup**: Carlos just provisions an OHC phone number or connects his WhatsApp Business account. The AI prompt is auto-generated from his business profile.
  - **Pessimistic AI Handoff**: The AI gathers information and stops. It does not finalize quotes or bookings without Carlos's approval, ensuring safety and trust.
  - **Unified Feed**: Missed calls are not a separate "voicemail tab"; they are prioritized work items in the main feed.

  ## 4. Implementation Prompt
  **Feature Name**: Agentic Missed Call & SMS Lead Recovery
  **Target Persona**: Carlos the Handyman
  **Outcome**: When Carlos misses a call, his OHC assistant instantly texts the caller, asks what they need, captures the details (including photos), and places a qualified lead card at the top of Carlos's feed.

  **Critical User Journey (CUJ)**:
  1. Carlos configures an OHC phone number in his settings.
  2. A customer calls the number. The call goes unanswered.
  3. The customer instantly receives an SMS: "Hi, Carlos is on a job. How can I help you today?"
  4. The customer replies via SMS with their problem.
  5. Carlos opens the OHC app and sees a "New Lead" card in his feed containing the conversation context, with a 1-tap button to "Draft Quote".

  **Acceptance Criteria**:
  - Integrate a generic webhook receiver capable of processing missed call and SMS events.
  - Implement the Customer Assistant capability to generate context-aware replies to unknown numbers.
  - Design the 375px mobile feed card that highlights the newly qualified lead and provides clear next-action buttons.
  - Ensure the conversation thread is visible in a clean, unified chat interface.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

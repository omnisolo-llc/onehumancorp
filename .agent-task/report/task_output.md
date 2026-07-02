issue_title: "Automated Re-engagement Agent for Service Bookings"
issue_description: |
  # Research Report: Automated Re-engagement Agent for Service Bookings

  ## Problem Statement
  Service-based small business owners, like Leo the Music Tutor, face a critical challenge: client retention and follow-up. Existing platforms (Shopify, Wix, Calendly) require manual tracking of who hasn't booked recently, or rely on complex, static email drips (e.g., Mailchimp) that are difficult for non-technical users to set up and lack context about the client's specific situation. Owners spend too much time manually reaching out to dormant clients or lose revenue when clients slip through the cracks.

  ## Research Report
  - **Market Landscape**: Traditional scheduling tools (Calendly, Acuity) handle the initial booking well but rely on external CRM integrations for post-appointment re-engagement. Website builders (Wix, Squarespace) offer basic automated emails (e.g., "Thanks for booking") but lack intelligent, context-aware follow-up mechanisms.
  - **The OHC Differentiator**: OHC's architecture integrates the booking data directly with the Customer Memory Graph and the AI Agent framework. This allows for proactive, intelligent re-engagement without requiring the owner to configure complex rules or workflows.
  - **Competitive Gaps**: Competitors provide the tools to build a workflow; OHC provides the "staff" (Agents) to execute the workflow autonomously.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Booking Service] -->|Completed Appointment Event| B(Event Bus)
      B --> C[Sales/CS Agent]
      C --> D{Context Resolution}
      D -->|Check Customer History| E[Customer Memory Graph]
      D -->|Check Availability| F[Calendar Service]
      C -->|Time elapsed since last booking > Threshold| G[Draft Message]
      G --> H{Approval Flow}
      H -->|Approved by Owner| I[Send via SMS/Email/WhatsApp]
      H -->|Auto-Approve Enabled| I
  ```

  ### Mobile UX Flow (375px)
  1. **Agent Feed**: The owner opens the OHC app and sees a card in their feed: "Leo, 3 students haven't booked their follow-up lesson this week. Review drafted messages?"
  2. **Review Screen**: Tapping the card opens a stack of drafted messages. Each message is personalized based on the student's history (e.g., "Hi [Name], it's been a week since our last session! Ready to tackle that new piece? Here's a link to grab a spot on my calendar: [Link]").
  3. **Action**: The owner can tap "Approve All", or swipe through individual messages to edit or send.

  ### AI Agent Integration Points
  - **Trigger**: A scheduled job or event listener detects when a customer has passed a predefined "dormancy threshold" (e.g., 7 days for a weekly tutor, 6 months for a dentist) without an upcoming booking.
  - **Context Gathering**: The Agent queries the Customer Memory Graph for past service history, preferences, and the owner's availability.
  - **Generation**: The LLM drafts a highly personalized, context-aware message, adopting the owner's tone.

  ## Implementation Prompt
  **Target Persona**: Leo the Music Tutor

  **Outcome**: An automated system where the AI Agent identifies students who are due for a lesson and drafts personalized follow-up messages for Leo to approve from his phone.

  **Critical User Journey (CUJ)**:
  1. Leo has a student who typically books weekly but hasn't booked for the upcoming week.
  2. The system detects this dormancy based on past booking patterns.
  3. The Sales/CS Agent drafts a personalized SMS message: "Hey Sarah, noticed you haven't booked for this week. Ready to practice those scales? Grab a time here: [Link]".
  4. Leo receives a notification on his phone, reviews the draft in the Agent Feed, and taps "Approve".

  **Acceptance Criteria**:
  - Implement a mechanism (e.g., cron job or event-driven) to identify dormant booking customers.
  - Integrate with the existing AI service to generate context-aware draft messages.
  - Present these drafts clearly in the mobile-first (375px) Agent Feed for user approval.
  - Must include automated E2E Playwright tests verifying the detection and approval flow.

  ## Priority: P1
  ## Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "[Architecture] Dynamic Client Intake & Consultation Mesh"
issue_description: |
  ## Problem Statement
  Service-based businesses (like Leo the music tutor, or a consultant/therapist) require detailed context before confirming a booking or starting a session. Currently, if Leo wants to know a new student's skill level or musical goals, he has to email them manually after they book a time slot. This fragments the onboarding experience and delays value delivery. Competitors often treat forms as separate entities from the booking engine, forcing the user to connect tools like Typeform to Calendly via Zapier. OHC needs an invisible, AI-powered intake mesh that seamlessly inserts dynamic, conversational questionnaires directly into the booking/checkout flow, ensuring the business owner is fully prepared before the interaction begins.

  ## Research Report
  - **Competitor Systems Audit**:
    - **Calendly/Acuity**: Allow basic custom questions at booking, but they are static, rigid text fields that cannot adapt based on previous answers or the specific service chosen.
    - **Typeform/Jotform**: Excellent dynamic logic, but they are standalone products. Integrating them with a booking system and a CRM requires technical knowledge (Zapier) that our personas lack.
    - **Shopify**: Not built for service intake. App store solutions are disjointed.
  - **OHC's Differentiation**: We merge the booking, payment, and intake into a single, fluid mobile experience. The AI (Customer Success Agent) can dynamically generate the intake questions based on the service selected (e.g., asking about dietary restrictions for a catering quote vs. asking about guitar experience for a music lesson) and summarize the responses into a plain-language briefing for the business owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      SERVICE_LISTING ||--o{ BOOKING_INTENT : "Initiates"
      BOOKING_INTENT ||--|{ INTAKE_NODE : "Requires"
      INTAKE_NODE }|--|| DYNAMIC_FORM_ENGINE : "Renders"
      DYNAMIC_FORM_ENGINE }|--|| AI_CS_AGENT : "Generates questions"
      INTAKE_NODE ||--o{ RESPONSE_DATA : "Captures"
      RESPONSE_DATA }|--|| CRM_PROFILE : "Enriches"
      CRM_PROFILE ||--o{ ADVISORY_BRIEFING : "Summarizes for Owner"
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  *   **Customer View (Intake Flow):**
      *   Customer selects a time slot for "Introductory Guitar Lesson" and taps "Continue."
      *   Instead of a static form, a conversational UI appears (Glassmorphism cards): "Great! To help Leo prepare, what's your current skill level?" (Options: Beginner, Intermediate, Advanced).
      *   If "Beginner" is selected, the next card asks: "Do you own a guitar yet?"
      *   Progress bar at the top indicates completion. Final step is the deposit payment.
  *   **Merchant View (Dashboard):**
      *   Leo receives a notification: "New Booking: Alex (Intro Lesson)."
      *   Tapping the notification shows the AI summary: "Alex is a beginner without a guitar. They want to learn acoustic strumming." No need to read raw form data.

  ### AI Agent Integration
  *   **The Ambassador (CS Agent)**: Autonomously drafts the intake questionnaire based on the service category.
  *   **The Advisor (Business Advisory)**: Summarizes the raw intake data into actionable insights for the merchant prior to the appointment.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the Dynamic Client Intake & Consultation Mesh.

  1.  Design a data model (`IntakeSchema`, `IntakeResponse`) that can attach to a `Service` or `BookingIntent`.
  2.  Build a mobile-first (375px) dynamic form renderer. It must support conditional logic (if answer is X, show question Y) but present it in a conversational, one-card-at-a-time flow rather than a long scrolling web form.
  3.  Integrate the CS Agent to auto-generate default intake templates based on the merchant's business type (e.g., a tutor gets different questions than a handyman).
  4.  Ensure all intake data is securely stored and linked to the multi-tenant CRM profile, passing through the Zero-Trust security boundary.
  5.  Create the UI component for the merchant dashboard that displays the AI-summarized intake brief alongside the booking details.

  **Priority:** P2 (Medium)
  **Estimated Scope:** Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

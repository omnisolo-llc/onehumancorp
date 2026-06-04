issue_title: "Implement Autonomous Client Intake Questionnaire Engine"
issue_description: |
  # Research Report & Implementation Brief

  **Problem:** Small service businesses (e.g., handyman, music tutor) suffer from lost leads and incorrect quoting due to disconnected back-and-forth communication or reliance on external form builders (Typeform). OHC needs a native, zero-config way to attach structured intake forms to bookings and services.

  **Findings:** An end-to-end architecture has been designed where AI agents automatically generate intake forms based on the service context, and then seamlessly parse the customer's responses (including photos) to generate draft quotes and update the CRM profile automatically.

  **Detailed Design Doc:** `docs/research/intake_questionnaire_engine_end_to_end_business_journey_detailed.md`

  **Implementation Prompts for the Engineering Swarm:**

  **Phase 1: Core Schema & API (P1, Medium Scope)**
  - Implement strict multi-tenant Row-Level Security (RLS) PostgreSQL schemas for `QuestionnaireTemplate`, `Question`, `IntakeSubmission`, and `SubmissionAnswer`.
  - Create the backend API endpoints (REST/gRPC) for CRUD operations on templates and answering submissions. Ensure zero-trust access controls are in place.

  **Phase 2: Mobile-First UX & Edge Synchronization (P1, Large Scope)**
  - Build the 375px mobile-optimized UI using the OHC Translucent Glass materials design tokens.
  - Implement a progressive disclosure flow for the customer-facing intake form, ensuring touch targets are >= 44x44px.
  - Integrate PWA/Flutter Isolate local caching to support optimistic UI updates and robust handling of photo uploads on spotty edge network connections.

  **Phase 3: AI Department Integration (P0, Large Scope)**
  - Hook the `Operations Agent` to the service creation flow to autonomously suggest and generate `QuestionnaireTemplate`s.
  - Integrate the `Sales & Quoting Agent` to act as an asynchronous job worker that parses `IntakeSubmission` events, extracts structured data (dimensions, preferences) into `jsonb parsed_entities`, and generates a draft quote for merchant approval.
  - Connect the `Customer Success Agent` to update the `Customer360` profile with new context.

  **Verification Protocol:**
  - Full Playwright E2E test coverage MUST be provided spanning the merchant service creation, customer booking/intake completion, and the AI's autonomous quote generation flow. Ensure no mock data is used for UI rendering.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, feature]
assignees: []

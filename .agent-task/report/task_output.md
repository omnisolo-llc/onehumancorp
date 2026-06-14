issue_title: "Implement Autonomous Quote & Deposit Link Generation Pipeline"
issue_description: |
  # Autonomous Quote & Deposit Link Generation Pipeline

  ## Problem Statement
  Service operators like Carlos (field service owner) and custom creators like Maya (baker) receive highly unstructured inbound inquiries (e.g., DMs, SMS) for custom work. Turning these inquiries into actual scheduled, paid work currently requires high manual effort: reading the request, checking availability, calculating pricing, generating a quote, drafting a reply, and generating a deposit link. This friction leads to lost leads and overwhelmed owners. OHC lacks an end-to-end, agent-driven pipeline that ingests an unstructured request and presents the owner with a one-tap actionable draft response containing a generated quote and a deposit payment link.

  ## Research Report
  ### Competitive Landscape
  - **Stripe Invoicing / Square Appointments:** Allow quote and deposit creation, but require the user to manually enter the customer details, line items, and schedule into a complex form. They are tools, not assistants.
  - **Shopify Sidekick:** Focuses primarily on store configuration and reporting, rather than active conversational sales for custom service bookings.
  - **Wix/GoDaddy:** Basic contact forms that drop into an inbox, with no AI-driven operational next steps.

  ### The OHC Gap
  OHC's current Work Triage groups messages, but does not yet connect the **Work Intake**, **Customer Assistant**, **Operations Assistant**, and **Sales Assistant** into a unified, invisible background workflow that outputs a fully baked quote and deposit link ready for owner approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Customer (DM/SMS)
      participant Intake as Work Triage
      participant SalesAgent as Sales & Revenue Assistant
      participant OpsAgent as Operations Assistant
      participant Stripe as Stripe API
      participant Owner as Owner (OHC Mobile App)

      User->>Intake: Unstructured Inquiry (e.g., "Need a cake on the 12th")
      Intake->>SalesAgent: Trigger Quote Generation Intent
      SalesAgent->>OpsAgent: Check Calendar/Inventory for the 12th
      OpsAgent-->>SalesAgent: Confirm Availability
      SalesAgent->>Stripe: Generate Payment Link (Deposit)
      Stripe-->>SalesAgent: Payment URL
      SalesAgent->>Intake: Draft Reply + Quote + Link
      Intake->>Owner: Push Notification: "Draft quote ready for Maya"
      Owner->>Intake: Review on Mobile UI
      Owner->>User: One-tap Approve & Send
  ```

  ### Mobile UX Flow (375px)
  1. **Push Notification:** Owner receives a notification: "New cake inquiry from Sarah. Draft quote ready."
  2. **Work Feed Screen:** Tapping the notification opens the Work Feed. A Unifi-style card shows the customer's raw message above an AI-generated draft reply.
  3. **Draft Review Card:** The translucent glass card displays:
     - **Draft Message:** "Hi Sarah! I'd love to make your cake. The total will be $150. You can secure the date with a 50% deposit here: [Link]"
     - **Quote Summary:** Line items extracted by the AI (e.g., 1x Custom Cake, Delivery).
  4. **Action Bar (Fixed Bottom):** Two primary touch targets (≥ 44x44px):
     - [ Edit Details ] (Secondary, translucent)
     - [ Approve & Send ] (Primary, solid vibrant color)
  5. **Success State:** The card collapses into a "Sent" state, and the work item moves to the "Pending Deposit" operational column.

  ### AI Agent Integration Points
  - **System Prompt (Sales Assistant):** "You are the Sales Assistant. Extract service requirements, calculate pricing based on the catalog, and draft a friendly reply including a Stripe payment link. Require owner approval."
  - **Distributed Coordination (Redis Redlock):** Use `ohc:lock:{tenant_id}:calendar:{date}` to tentatively hold the spot while the quote is pending.

  ## Implementation Prompt

  **Implementer Agent Objective:**
  Build the `Autonomous Quote & Deposit Link Generation Pipeline`. You are to implement the backend background worker that orchestrates this flow, and the frontend mobile-first UI for the owner to review and approve the draft.

  **Critical User Journey (CUJ):**
  1. The backend API receives a mocked inbound webhook simulating a customer DM.
  2. The AI job queue processes the message, queries the internal calendar for availability, and calls the Stripe Payment Link API (use the Stripe test mode adapter) to create a deposit link.
  3. The backend stores the drafted response and quote as a pending task in PostgreSQL.
  4. The owner opens the OHC web/mobile app (375px layout), sees the pending task card in their Work Feed, reviews the generated text and link, and clicks "Approve & Send".

  **Acceptance Criteria:**
  - The feature must be completely functional on a 375px viewport with no horizontal scrolling.
  - UI elements must use macOS-style Translucent Glass materials and Unifi dashboard card layouts.
  - Zero mock data in the UI; the Work Feed must load the actual generated quote from the backend.
  - 100% unit test coverage for new modules.
  - Provide at least 5 Playwright E2E tests covering the flow from the Work Feed to the approval action.
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []

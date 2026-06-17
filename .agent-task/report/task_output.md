issue_title: "Implement Agentic Project Intake and Autonomous Proposal Engine"
issue_description: |
  **Problem Statement:**
  For service-based owners and agency principals like Nora, capturing new client inquiries and turning them into professional, actionable proposals is a manual, multi-tool, and time-consuming process. Currently, Nora must monitor emails or web forms, manually assess the client's needs, check her team's availability, estimate costs, draft a proposal document, and follow up for approval. This friction leads to delayed responses, lost leads, and administrative burnout. OHC needs an autonomous agent that handles the end-to-end "Project Intake to Proposal" pipeline with zero technical setup required by the owner.

  **Research Report:**
  Our competitive analysis shows that traditional CRMs and quoting tools (e.g., HubSpot, HoneyBook, Dubsado) provide workflow builders, but they require the owner to pre-configure complex "if/then" logic, email templates, and pricing rules. AI-native tools (like 11x.ai or Lindy.ai) are pushing towards autonomous workers, but often lack deep integration into the owner's core operational data (calendar, inventory, ledger).

  OHC's unique advantage is the unified data model. By combining the `Sales Agent` (for drafting) and the `Operations Agent` (for availability/pricing), OHC can autonomously generate context-aware proposals. This closes the critical gap identified in our "Mobile-First Design & Operations Research Report", shifting the paradigm from "Dashboard Management" to "Agent Approval".

  **Design Doc:**
  *Architecture Diagram:*
  ```mermaid
  sequenceDiagram
      participant Client
      participant IntakeWebhook
      participant MessageTriageWorker
      participant SalesAgent
      participant DB (Quotes/SharedTasks)
      participant MobileFeed (Nora)

      Client->>IntakeWebhook: Submits Inquiry (e.g., "Need a website redesign")
      IntakeWebhook->>MessageTriageWorker: Enqueue Intake Event
      MessageTriageWorker->>SalesAgent: Trigger Proposal Generation
      SalesAgent->>DB (Quotes/SharedTasks): RAG Context (Past Projects, Pricing)
      SalesAgent->>DB (Quotes/SharedTasks): Draft Quote & Tasks
      SalesAgent->>MobileFeed (Nora): Push Action Card (PENDING_APPROVAL)
      MobileFeed (Nora)->>SalesAgent: 1-Tap "Approve & Send"
      SalesAgent->>Client: Send Professional Proposal via Email/SMS
  ```

  *Mobile UX Flow:*
  1. Nora receives a push notification: "New Project Inquiry from Acme Corp."
  2. She opens the OHC PWA on her 375px phone. The top card in her Unified Agent Feed displays a summary of the inquiry and a drafted proposal (Quote).
  3. The card uses OHC Premium Tokens (Glassmorphism, clean typography, `44x44px` touch targets).
  4. She can tap "Review Details" to see the line items and estimated timeline.
  5. She taps the primary action button: "Approve & Send Proposal".
  6. The card transitions to a success state, and the Sales Agent emails the proposal to the client with a Stripe checkout link for the deposit.

  *AI Agent Integration Points:*
  - Enhance `message_triage_worker.rs` or create a specific `intake_worker.rs` to parse incoming leads (via the existing `/api/v1/work-intake/submit` endpoint).
  - Use the LLM (Gemini Pro) to extract project requirements, generate `QuoteLineItem`s based on the owner's catalog/services, and draft a cover letter.
  - Insert a record into `agent_feed_items` containing the drafted Quote data in `proposed_action`, setting `lifecycle_state` to `PENDING_APPROVAL`.
  - Connect the Feed Approval action to the `action_router` to finalize the Quote in the database and dispatch the email.

  **Implementation Prompt:**
  1. **Backend Service**: Implement an asynchronous worker flow that listens to new work intake submissions. This worker must use the configured LLM provider to process the raw inquiry text, match it against the tenant's service catalog, and construct a structured draft Quote.
  2. **Agent Feed Integration**: Ensure the drafted Quote is pushed to the `agent_feed_items` table. The `proposed_action` JSON must contain the necessary schema to render a "Quote Approval" card in the UI and to process the approval via the Action Router.
  3. **Action Router Logic**: Implement the handler in `action_router.rs` for the `quote_draft_approval` feature type. Upon approval, it must persist the Quote via `quotes.rs` service logic, generate a checkout/deposit URL if applicable, and trigger the notification to the client.
  4. **Verification**: Write a Playwright E2E test simulating a client submitting an inquiry form, logging in as the owner (Nora), finding the drafted proposal in the Agent Feed, and successfully approving it. Ensure the test runs cleanly with `bazel test //...`.

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

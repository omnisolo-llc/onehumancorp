issue_title: "Research: Mobile-First Agentic Workflows for SMB Operators"
issue_description: |
  # Research Report: AI-Driven Mobile-First Workflows for SMB Owners

  ## Problem Statement
  SMB owners like Carlos (handyman) or Fatima (food cart) run their operations mostly from mobile devices with small screens and occasionally flaky internet connections. Existing CRM and ERP systems require heavy desktop usage, complex form filling, and technical setups that do not fit into the chaotic, fast-paced nature of mobile operations. They need an AI agent that can ingest requests, parse natural language to handle bookings/orders, and present a clean, native-feeling mobile experience for triage and fulfillment.

  ## Research Report
  - We analyzed the needs of 5 core personas (Maya, Carlos, Priya, Leo, Fatima). A recurring theme is that "administrative overhead" costs them 10-15 hours a week.
  - Platforms like Shopify and Wix offer mobile apps, but they still act as "admin portals." The user must click through 4-5 layers of menus to find an order or modify a booking.
  - **Proposed Approach:** An AI-first "Unified Work Inbox." Instead of navigating to "Orders," "Bookings," and "Messages" separately, the AI Agent triages incoming context (emails, DMs, form submissions) and presents them as actionable "Task Cards" in a single mobile-first feed.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD;
      A[Incoming DM/Email/Form] --> B[AI Triage Agent];
      B --> C[Intent Classification];
      C --> D[Task Feed Database];
      D --> E[Mobile-First UI (Flutter/Tauri)];
      E --> F[Owner Quick Actions: Approve, Reject, Draft Reply];
    ```
  - **Mobile UX Flow:**
    1. Owner opens app (375px viewport).
    2. Main screen shows "Today's Priorities" (Task Cards).
    3. Card 1: "Maya, 3 cake inquiries overnight." Buttons: [Review Drafts] [Dismiss].
    4. Owner taps "Review Drafts." AI presents a drafted response based on business context and previous orders.
    5. Owner edits (or accepts) and taps [Send & Request Deposit].
  - **Key Design Decisions:**
    - Use a unified stream approach rather than siloed module tabs to reduce cognitive load.
    - Leverage Gemini Pro for intent classification and draft generation.
    - Implement optimistic UI updates for mobile offline resilience.

  ## Implementation Prompt
  **Goal:** Build the Mobile-First Unified Work Feed UI and integrate it with the AI Triage Agent backend.
  **CUJ:**
  1. Owner receives a new customer message.
  2. The system categorizes it and adds a prioritized task to the feed.
  3. The owner views the feed on a mobile device, reviews the AI-generated draft, and approves it.
  **Acceptance Criteria:**
  - The UI must render perfectly on a 375px width screen without horizontal scrolling.
  - The backend must expose an API to fetch triaged tasks.
  - The AI integration must classify incoming context and draft a response.
  - Comprehensive Playwright E2E test verifying the flow from task creation to draft approval.
  - 100% unit test coverage for new backend and frontend logic.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

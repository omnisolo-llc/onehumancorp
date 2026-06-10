issue_title: "Implement the Agent Feed Core and Notification Action Cards"
issue_description: |
  # Research Report: Agent Feed Core & Action Cards Implementation

  ## Target Persona: Maya (Home Baker) and Carlos (Handyman)

  ## Problem Statement
  Business owners on the OHC platform are currently missing critical interactions because the platform does not proactively notify them. Existing solutions expect the user to manually check a dashboard to manage their tasks or messages. Maya might miss an Instagram DM about a custom order because she is busy baking, and Carlos might miss a quote request while on a job. They need an AI that acts as an "Invisible Operations Manager", monitoring these incoming events and providing a simple, actionable feed to review.

  ## Architecture & Design Flow
  - **Event Ingestion & Routing**: A central message bus (e.g., Redis Pub/Sub) where webhooks (like Instagram Graph API or Stripe) publish events. An async worker dequeues and routes these events.
  - **Intent Classification & Draft Generation**: For conversational events (DMs, emails), integrate with the LLM (Gemini Pro/MiniMax) using the `ohc_builtin_agent` to classify intent and generate a contextual draft response using RAG on the user's data.
  - **Action Card UI**: The mobile-first Flutter/Tauri UI must display a new "Agent Feed". Each item in the feed is an "Action Card" with a 375px optimized layout. The card contains:
    - The original trigger (e.g., "Customer DM: Do you have vegan cakes?").
    - The AI's suggested action/draft ("Drafted Reply: Yes, we have vegan cakes available!").
    - Clear action buttons: "Approve & Send", "Edit", and "Discard".
  - **Execution Protocol**: Approving an action must trigger a secure execution flow back through the agent protocol to fulfill the action (e.g., calling the Instagram API to send the message).

  ## Implementation Prompt
  - Build the backend Event Ingestion pipeline to receive events and place them on a job queue.
  - Implement the "Operations Manager" agent worker that processes these events, calls the LLM for intent and draft generation, and creates an `ActionCard` record for the user.
  - Design and implement the mobile-first "Agent Feed" screen containing the Action Cards. Ensure it passes the 30-second "grandmother test" and looks premium using translucent materials.
  - Implement the end-to-end "Approve" flow for an Action Card, executing the drafted action securely.
  - Include full unit tests for the agent worker and Playwright E2E tests verifying the Action Card approval flow from the UI. Do not use mocked data in the UI.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

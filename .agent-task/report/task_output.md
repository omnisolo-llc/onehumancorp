issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  ## Problem Statement
  Business owners like Fatima (food cart) and Maya (baker) need to manage their operations entirely on their phones. Currently, managing complex tasks like fulfilling orders, updating inventory, and responding to customer inquiries requires navigating through traditional dashboards that are ill-suited for 375px mobile environments. The lack of a mobile-first, proactive management interface creates friction and slows down daily operations, violating the OHC premise of "open OHC and immediately know what needs attention today."

  ## Research Report
  Our competitive analysis indicates that traditional e-commerce platforms (Shopify, Wix) treat mobile apps as supplementary dashboards and require desktop access for full administrative capabilities. By contrast, "Link-in-Bio" tools have seen massive adoption by operating entirely within the mobile context, but they lack robust business and operational features.

  Deep dives into our internal documents (`agent_feed_deep_dive.md` and `ohc_smb_mobile_first_design_research.md`) highlight a critical architectural gap: a mobile-first, unified agentic "Approval" interface. Instead of forcing owners to seek out work inside complex menus, the system should push actionable "Agent Proposals" (e.g., drafted emails, inventory alerts, order fulfillment prompts) directly to the user's primary view for immediate triage and execution.

  ## Design Doc
  **Architecture Diagram**
  ```mermaid
  graph TD
    A[Event Sources: Webhooks, Schedule, DB] --> B(Job Queue / Event Bus)
    B --> C{Agent Feed Router}
    C --> D[Customer Agent]
    C --> E[Operations Agent]
    C --> F[Marketing Agent]
    D --> G(Action Card Generator)
    E --> G
    F --> G
    G --> H[Unified Agent Feed UI - Mobile]
    H --> I(User Action: Approve/Edit/Discard)
  ```

  **Mobile UX Flow (375px MVP)**
  1.  **Home Screen**: The main view upon launching the app is a vertical feed of prioritized Action Cards.
  2.  **Card Layout**: Each card uses OHC Premium Tokens (translucent materials, clean typography) and contains:
      -   Agent Icon/Type indicator.
      -   Concise summary of the proposed action (e.g., "Drafted response to Instagram DM for Maya").
      -   Large, touch-friendly primary action button (e.g., "Approve & Send" with min 44x44px target).
      -   Secondary actions (e.g., "Edit", "Discard").
  3.  **Interaction**: Tapping a card expands it for more details (e.g., full text of the drafted response).
  4.  **Completion**: Tapping "Approve" triggers the corresponding backend API call to execute the action, removes the card from the feed, and provides a clear success state.

  **AI Agent Integration Points**
  -   The feed relies on the existing AI backend capabilities to analyze business events and generate the actionable card payloads.
  -   The frontend requires a unified endpoint or subscription (e.g., Server-Sent Events or WebSockets) to fetch pending cards in real-time.

  ## Implementation Prompt
  Implement the "Unified Agent Feed" mobile UI inside the Flutter app as described in the Design Doc.

  **User-Facing Outcome**: Upon opening the OHC app on a mobile device, the owner sees a prioritized feed of Action Cards proposing the day's next steps (e.g., "3 new orders to fulfill", "Drafted promo email"). They can execute these actions with a single tap.

  **Critical User Journey (CUJ)**:
  1. Owner opens the app on a simulated 375px mobile screen.
  2. The unified feed displays pending Action Cards.
  3. The owner taps "Approve" on an Operations Agent card proposing to fulfill 3 outstanding orders.
  4. The card is marked as approved, transitions out of the feed, and an execution confirmation is shown.

  **Acceptance Criteria**:
  -   The UI must be built in Flutter and strictly adhere to a 375px mobile layout without horizontal scrolling.
  -   All interactive elements must have a minimum touch target size of 44x44px.
  -   The UI must connect to the backend API to fetch and render actionable feed items.
  -   Playwright E2E tests must verify the display of cards and the complete approval interaction flow, adhering to zero-mock UI rules.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

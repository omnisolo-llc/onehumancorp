issue_title: "Implement Proactive Agent Feed UI/UX for Mobile Work Triage"
issue_description: |
  # Research Report: Proactive Agent Feed for Mobile Work Triage

  ## Title
  Implement Proactive Agent Feed UI/UX for Mobile Work Triage

  ## Problem Statement
  Small business owners (like Maya the Baker or Carlos the Handyman) currently have to hunt through dashboards, inboxes, and separate tools to figure out what needs their attention today. They suffer from "dashboard fatigue" and miss critical leads or operational tasks. Existing tools are reactive—the owner must query the system. OHC needs to become proactive, presenting a unified, prioritized "Work Feed" that acts as a true work assistant, surfacing drafted replies, new bookings, and actionable insights on a mobile-first (375px) screen.

  ## Research Report
  Our competitive analysis reveals that while platforms like Shopify (Sidekick) and HubSpot (Breeze) are introducing AI, they often function as complex advisors or conversational bots rather than a unified operational feed. The "Agent Feed Deep Dive" and "Mobile First Agentic Workflows" research clearly point to an "Invisible AI Automation" model:
  - **Shopify/Wix:** Rely on disparate apps for different functions (chat, inventory, bookings), forcing the user to context switch constantly. AI is a chatbot, not a proactive workflow engine.
  - **The Gap:** SMBs need a central nervous system. When a customer DMs on Instagram, an order comes in, and inventory runs low, these should not be separate notifications in separate apps. They should be prioritized "Action Cards" in a single feed.
  - **The Solution:** The Agent Feed. A central timeline where various OHC AI "departments" (Operations, Customer Success, Sales) push drafted actions for owner approval. This transforms the platform from software the owner administers into an assistant the owner manages.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      EventBus[Central Event Bus / Queue]
      Webhook1[Instagram DM] --> EventBus
      Webhook2[Stripe Payment] --> EventBus
      InternalState[Low Inventory Alert] --> EventBus

      EventBus --> AgentRouter[Agent Dispatcher]

      AgentRouter --> AmbassadorAgent[The Ambassador - CS]
      AgentRouter --> OperationsAgent[The Manager - Ops]

      AmbassadorAgent --> DraftReply[Draft DM Reply]
      OperationsAgent --> ActionItem[Flag Reorder Task]

      DraftReply --> FeedDB[(Agent Feed Database)]
      ActionItem --> FeedDB

      FeedDB --> MobileUI[OHC Mobile App Feed 375px]

      MobileUI -.->|Owner Approves Action| ExecutionLayer[Execution API]
  ```

  ### Mobile UX Flow (375px first)
  1.  **Home Screen (The Feed):** The immediate landing experience after login is not a complex dashboard of graphs, but a vertical, scrollable feed of "Action Cards", sorted by urgency.
  2.  **Action Card Anatomy (Translucent Glass & UniFi Style):**
      -   **Header:** Identifies the AI Agent department (e.g., "The Ambassador" with a subtle avatar/icon) and urgency status token.
      -   **Context:** Brief summary of the event (e.g., "Maya, you have a new inquiry about vegan cakes from @customer on Instagram.")
      -   **Proposed Action (The Draft):** The AI-generated draft response or proposed operation (e.g., "Draft reply: 'Hi! Yes, we have 3 vegan cakes available for Saturday. Want to reserve one?'").
      -   **Action Buttons (Primary):** Clear, touch-friendly buttons (min 44x44px) like "Approve & Send", "Edit", or "Dismiss".
  3.  **Interaction:** Tapping "Approve" executes the action immediately (optimistic UI update), collapsing the card. Tapping "Edit" opens a native mobile keyboard view to tweak the draft before sending.
  4.  **Empty State:** When all tasks are clear, the feed shows a positive, reassuring empty state ("All caught up, Maya! Here's a quick summary of yesterday's sales...").

  ### AI Agent Integration
  -   The backend must support an event-driven architecture where specific AI agents subscribe to topics.
  -   Agents generate structured JSON payloads representing the "Action Card" (title, body, proposed_action_payload), which is stored in the `FeedDB`.
  -   The Frontend fetches this feed via a unified API endpoint, rendering the appropriate card types dynamically.

  ## Implementation Prompt
  Implement the user-facing "Agent Feed" mobile-first UI component. This is the new landing experience for the OHC app.
  1.  **CUJ:** A non-technical owner logs into OHC on their smartphone. They land on the Agent Feed and see a prioritized list of Action Cards requiring their attention (e.g., a drafted reply to a customer inquiry). They can tap "Approve" on a card to execute the action or "Edit" to modify the draft.
  2.  **Design System:** Strictly adhere to the macOS-style Translucent Glass materials combined with clean Ubiquiti UniFi modular dashboard card layouts. Ensure all touch targets are at least 44x44px and the layout is perfectly optimized for a 375px viewport with zero horizontal scrolling.
  3.  **Technical:** Create the necessary React/Flutter components to render a feed of dynamic action cards. Define a clear, extensible data interface for an "ActionCard" that different backend agents can populate. Include Playwright E2E tests verifying the feed rendering, the approval interaction, and the empty state behavior using realistic (non-mocked) seed data representing different owner personas.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

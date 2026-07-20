issue_title: "Implement Agent Feed Mobile UI"
issue_description: |
  # Research Report: Agent Feed Mobile UI Implementation

  ## Problem Statement
  Business owners currently rely on complex dashboards requiring proactive checking to manage their operations. The existing tools lack a mobile-first, proactive "Agent Feed" that pushes actionable insights directly to the owner's device, simplifying management and enabling "zero-click" automations.

  ## Design Doc
  The goal is to implement a unified Agent Feed, optimizing the business management experience for a 375px mobile screen.

  ### Mobile UX Flow (375px)
  1.  **Feed Layout**: A vertical feed of "Agent Action Cards." Each card represents a pending action, notification, or suggested workflow from an AI agent (e.g., Marketing, Operations, Customer Service).
  2.  **Card Components**:
      *   **Header**: Agent type identifier (icon/color).
      *   **Body**: Brief, actionable text describing the proposed action (e.g., "Drafted a reply to a DM from customer X").
      *   **Actions**: Prominent touch targets (min 44x44px) for "Approve," "Edit," or "Dismiss."
  3.  **Interaction**: Tapping an action triggers the relevant agent workflow (e.g., sending the drafted message) and removes the card from the feed.

  ### AI Integration
  *   **Agent Outputs**: The feed consumes outputs from various agents (Marketing, Ops, Advisory) via a unified event bus or API.
  *   **Intent Resolution**: The feed displays pre-resolved intents with concrete actions, abstracting the underlying LLM complexity from the user.

  ## Implementation Prompt
  **Feature Name**: Mobile-First Agent Feed
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya can open the OHC app and immediately see a prioritized list of actions suggested by her AI agents, approving them with a single tap.

  **Next Actions**:
  1.  Create a mobile-friendly (375px width optimized) layout for the Agent Feed using OHC Premium Tokens (Glassmorphism, typography).
  2.  Implement the `AgentActionCard` component with minimum 44x44px touch targets.
  3.  Integrate the feed with the existing agent output API to display real or simulated agent actions.
  4.  Ensure the feed provides clear visual feedback upon approving or dismissing an action.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

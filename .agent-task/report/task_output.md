issue_title: "Implement Mobile-First Unified Agent Feed UI"
issue_description: |
  # Mission Queue Protocol: Mobile-First Unified Agent Feed UI

  ## Problem Statement
  Small business owners and operators (e.g., Maya the baker, Carlos the handyman) using OneHumanCorp currently experience friction when managing complex business operations. Traditional admin dashboards are designed for desktop environments, requiring horizontal scrolling, complex forms, and multiple steps to complete actions. When these owners are on the go, they need a simple, intuitive, and mobile-first way to manage tasks, communicate with customers, and approve agent actions. The lack of a streamlined mobile interface limits the effectiveness of OHC's AI agents, as users cannot easily review and approve automated work from a 375px phone screen.

  ## Research Report
  Based on our analysis of competitor platforms and the `ohc_smb_mobile_first_design_research.md` document:
  - **The Mobile Management Gap**: Legacy platforms (Shopify, Wix) treat mobile apps as supplementary tools for viewing stats, but require a desktop for complex setup and management.
  - **The "Companion App" Model**: While good for checking revenue or fulfilling orders, companion apps fail when it comes to designing store updates, configuring discounts, or managing complex agent workflows.
  - **The OHC Vision**: OHC must differentiate by offering a fully functional mobile experience where complex tasks are managed via a **Chat & Approval UI** powered by Agents.
  - **Agent Feed Deep Dive**: The Agent Feed is the central nervous system for business owners, replacing static dashboards with a proactive feed of actionable cards (e.g., drafted messages, suggested promotions, urgent tasks).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Frontend "Flutter App (Mobile-First 375px)"
          UI[Unified Agent Feed UI]
          Card[Agent Action Card Component]
          ApprovalBtn[Approve / Reject Controls]
      end

      subgraph Backend "Go + Bazel Backend"
          FeedAPI[Agent Feed gRPC/REST API]
          EventBus[Event Pub/Sub]
      end

      subgraph AI "AI Agent Departments"
          Ops[Operations Agent]
          Marketing[Marketing Agent]
          CS[Customer Service Agent]
      end

      subgraph Storage
          DB[(PostgreSQL)]
      end

      UI --> FeedAPI
      FeedAPI --> DB
      FeedAPI --> EventBus

      EventBus --> Ops
      EventBus --> Marketing
      EventBus --> CS

      Ops -.-> |Proposes Action| DB
      Marketing -.-> |Drafts Content| DB
      CS -.-> |Drafts Reply| DB
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1.  **Home Screen (Unified Feed):** Upon opening the app, the user sees a vertical scrollable list of "Agent Proposals" and "Urgent Items". No complex navigation menus are visible by default.
  2.  **Action Card:** Each card in the feed represents a specific agent task or proposal.
      -   **Header:** Agent icon/color (e.g., Marketing, Ops) and a short title (e.g., "Drafted Instagram Post").
      -   **Body:** The context or drafted content (e.g., "Here is your generated Instagram post for the new cake.").
      -   **Actions:** Large, touch-friendly buttons (minimum 44x44px). Primary action is "Approve & Execute", secondary is "Edit/Review", tertiary is "Discard".
  3.  **Interaction Flow:**
      -   User taps "Approve": The action is executed immediately, and the card is dismissed with a success animation.
      -   User taps "Edit": The card expands or navigates to a focused edit screen using native mobile keyboards.

  ### Mobile UX Flow Constraints
  -   **Layout:** strictly adheres to a 375px width (no horizontal scrolling).
  -   **Touch Targets:** All interactive elements must be at least 44x44px.
  -   **Design System:** Utilize OHC Premium Tokens, including macOS-style Translucent Glass materials and clean Ubiquiti UniFi modular dashboard card layouts.
  -   **Simplicity:** Pass the "grandmother test." Hide complex developer terms behind "Advanced Settings."

  ### AI Agent Integration Points
  -   The feed aggregates outputs from various AI departments (Operations, Customer Service, Marketing, Sales).
  -   The backend must support an API endpoint to fetch the personalized, prioritized feed of pending approvals and alerts for the specific tenant.
  -   Agent workflows must be designed to pause and wait for user approval via this feed when necessary.

  ## Implementation Prompt
  **Objective**: Build a mobile-first (375px) "Unified Agent Feed" in the Flutter frontend that replaces the traditional complex admin dashboard. This feed presents actionable cards from all OHC Agents.

  **User-Facing Outcome**: When the owner opens the OHC app, they see a clean, prioritized vertical feed of "Agent Proposals" and "Urgent Items." They can quickly approve drafted emails, review generated content, and manage operations with single taps.

  **Critical User Journey (CUJ)**:
  1.  User opens the app and logs in.
  2.  The home screen displays the Unified Agent Feed with at least 3 simulated or real action cards from different agent types (e.g., a drafted customer reply, an order fulfillment suggestion, a marketing post draft).
  3.  The user views a card with drafted content.
  4.  The user taps the "Approve" button (which must be a minimum of 44x44px).
  5.  The card successfully processes the approval (mocked or real backend call) and provides clear visual feedback (e.g., card dismissal, success toast).

  **Acceptance Criteria**:
  -   Implement the feed layout strictly adhering to a 375px viewport with no horizontal scrolling.
  -   Create a reusable `AgentActionCard` component that supports different agent types, content bodies, and action buttons.
  -   All interactive elements must have a minimum touch target size of 44x44px.
  -   Apply OHC Premium Tokens (Glassmorphism, specific typography, spacing).
  -   Ensure the UI passes accessibility and responsive layout checks.
  -   Implement Playwright E2E tests covering the interaction flow (viewing the feed, clicking approve, verifying state change).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
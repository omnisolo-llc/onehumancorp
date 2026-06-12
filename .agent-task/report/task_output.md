issue_title: "Implement Work Triage & Unified Agent Feed Mobile UI"
issue_description: |
  # Mission Queue Protocol: Work Triage & Unified Agent Feed Mobile UI

  ## Problem Statement
  SMB Owners (like Maya and Fatima) are currently overwhelmed by disjointed communication and task management across different apps and tools. While OHC integrates these backend systems, the user interface still feels like a complex, legacy web application forced onto a small screen. There is no central, intelligent, prioritized stream telling the owner "Here is what you need to do right now." They must proactively check dashboards, messages, and schedules, slowing them down and causing them to miss critical business opportunities. The current mobile interface fails the "375px first" non-negotiable requirement for complex operations.

  ## Research Report
  Our competitive analysis shows that legacy platforms (Shopify, Wix) treat mobile apps as a secondary "companion" view, mainly for checking stats or basic fulfillment. They fundamentally fail at enabling complex business setup and active, intelligent operations on a 375px mobile screen. Modern Link-in-Bio tools succeed via simplicity but lack deep business capabilities.

  To bridge this gap and execute OHC's unique "Invisible AI Automation" differentiator, we must move away from static dashboards and complex navigation menus towards a **Unified Agent Feed**.

  Based on the findings in `docs/business/market_research/ohc_smb_mobile_first_design_research.md` and `docs/business/market_research/agent_feed_deep_dive.md`, the Agent Feed uses an "Approval UI" paradigm. Agents classify incoming events (Work Triage), generate drafted responses or proposed operational actions, and present them as simple Action Cards requiring minimal cognitive load to approve or edit.

  ## Design Doc

  ### High-Level Architecture

  ```mermaid
  graph TD
      A[Event Sources: DMs, Orders, Tasks] --> B(Event Pipeline)
      B --> C{Work Triage Agent}
      C -->|Intent & Context Resolution| D[Action Card Generator]
      D --> E[Unified Agent Feed UI]
      E -->|User Taps 'Approve'| F[Execution Agent / API]
  ```

  ### Mobile UX Flow (375px First)
  1. **Launch**: User opens the OHC mobile app. The home screen *is* the Unified Agent Feed.
  2. **Feed Layout**: A vertical stack of "Action Cards". No complex hamburger menus or multi-level navigation required for primary tasks.
  3. **Action Card Anatomy**:
      - **Agent Indicator**: Icon/subtle color indicating the department (e.g., Customer Success, Operations, Marketing).
      - **Context Summary**: Brief text explaining *why* this card is here (e.g., "New custom cake inquiry from @maya_bakes").
      - **Proposed Action/Draft**: The AI-generated suggestion (e.g., "Draft reply: 'Yes, we can do vegan! Here is the deposit link.'").
      - **Primary CTA**: Large (min 44x44px), thumb-friendly "Approve" button.
      - **Secondary CTA**: "Edit" or "Dismiss".
  4. **Execution**: Tapping "Approve" executes the action instantly and animates the card out of the feed, providing a satisfying sense of clearing work.

  ### AI Agent Integration
  - The Work Triage system will orchestrate incoming webhooks and state changes.
  - LLM classification routes events to specialized agents (Customer Relationship, Operations, Sales).
  - Agents utilize RAG against the tenant's context to generate highly accurate drafts presented in the UI.

  ### Visual & Styling Decisions
  - **OHC Premium Tokens**: Cards must use the macOS-style translucent glass styling against the app background.
  - **Clear Hierarchy**: Use typography to distinguish the "Why" (context) from the "What" (proposed action).
  - **Touch-Friendly**: All interactable elements must adhere strictly to the > 44px touch target rule.

  ## Implementation Prompt

  **Objective**: Build the Mobile-First (375px) Unified Agent Feed and Action Card components for the "Work Triage" capability.

  **Critical User Journey (CUJ)**:
  1. The user (Owner Persona) navigates to the Work Triage / Home view on a 375px screen.
  2. The view displays a vertical list of pending Action Cards (e.g., a drafted reply to a customer inquiry, a suggested marketing email).
  3. The user reviews a card and taps the primary "Approve" button.
  4. The card visually confirms the action and is removed from the feed.

  **Acceptance Criteria**:
  - The UI component must be fully responsive, starting perfectly optimized for a 375px width. Horizontal scrolling is strictly prohibited.
  - Implement a reusable `ActionCard` component that accepts props for Agent Type, Context, Draft Content, and Action Callbacks.
  - Apply the defined OHC translucent glass design tokens for the card styling.
  - Ensure all buttons and interactive areas have a minimum 44x44px touch target.
  - Add robust automated Playwright E2E tests simulating an owner approving an item in the triage feed on a mobile viewport. The test must verify the card exists, the button can be clicked, and the card is subsequently dismissed.
  - The UI must not use mock hardcoded data; it should consume data from the relevant backend API or a verified seed data script for the E2E test.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_title: "Implement the Unified Agent Feed (Mobile MVP)"
issue_description: |
  # Unified Agent Feed (Mobile MVP) Implementation

  ## Problem Statement
  The legacy dashboard approach requires business owners to navigate complex menus and seek out information. This is counter to the OHC vision of an "assistant-first" experience. Owners (like Maya, Carlos, and Fatima) need a central hub that proactively surfaces what needs their attention today—whether that's a new order, a pending customer DM, or a suggested marketing action—and allows them to act on it immediately from their phone.

  ## Research Report
  - **Market Context:** Traditional platforms (Shopify, Wix) rely on complex mobile apps that often require switching to a desktop for advanced tasks. Link-in-bio tools simplify but lack depth.
  - **The OHC Differentiator:** OHC replaces the complex admin dashboard with a vertical feed of actionable cards from various AI agents (Operations, Marketing, Advisory, etc.). This "Approval UI" paradigm allows complex actions to be executed with a single tap.
  - **Reference Docs:**
    - `docs/business/market_research/agent_feed_deep_dive.md`
    - `docs/business/market_research/ohc_smb_mobile_first_design_research.md`

  ## Design Doc
  ### Architecture
  The Agent Feed serves as the primary UX layer, aggregating events and proposals from the backend AI Job Queue and Agent Departments.

  ```mermaid
  graph TD
      A[Backend Event Pipeline] -->|Publishes Events| B(Intent & Context Resolution Layer)
      B -->|Generates Drafts/Proposals| C(Agent Feed API)
      C -->|Delivers Action Cards| D[Flutter / PWA Frontend]
      D -->|User Action: Approve/Edit/Discard| E[Backend Execution/Agent Handoff]
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Screen (The Feed):** The primary view upon opening the app. A vertical, scrollable list of "Action Cards".
  2. **Card Anatomy:**
     - **Header:** Agent Type (e.g., 🛠️ Operations, 📢 Marketing) and timestamp.
     - **Body:** Concise summary of the event/proposal (e.g., "Maya, you have 3 new custom cake inquiries.")
     - **Actions:** Large (minimum 44x44px touch targets) buttons representing the next step (e.g., "Draft Replies", "View Orders", "Approve Discount").
  3. **Interaction:** Tapping a primary action either executes the task immediately (if fully automated) or expands the card/transitions to a detail view for review (e.g., reviewing an AI-drafted email before sending).

  ### AI Agent Integration
  The feed is populated by different "Agent Departments":
  - **Operations Agent**: Triggers on new orders, inventory alerts.
  - **Marketing Agent**: Triggers on scheduled campaigns, social media interactions.
  - **Advisory Agent**: Triggers on analytics insights, weekly summaries.
  Each card must specify which agent generated it to build trust with the owner.

  ### Design System & Visuals
  - Adhere strictly to the OHC Premium Token library.
  - Use clear Apple/Ubiquiti-style hierarchy.
  - Employ restrained translucent materials (Glassmorphism) for cards to provide depth against the background.
  - Ensure strong spacing and readable typography.

  ## Implementation Prompt
  **Objective:** Implement the frontend (Flutter / PWA) for the Unified Agent Feed, serving as the new home screen.

  **Critical User Journey (CUJ):**
  1. The user logs into the OHC app.
  2. They are immediately presented with the "Agent Feed" screen instead of a traditional dashboard.
  3. The feed displays a prioritized list of Action Cards (e.g., a pending message to review, a daily summary, a low-inventory alert).
  4. The user interacts with a card (e.g., tapping "Approve" on a drafted message), and the UI updates to reflect the action taken (e.g., card dismissed, loading state shown).

  **Acceptance Criteria:**
  - The feed is the default view upon login.
  - The layout is perfectly optimized for a 375px width screen (no horizontal scrolling, 44x44px minimum touch targets).
  - Implements the OHC translucent glass design tokens.
  - Supports at least three distinct card types (e.g., Notification, Approval Request, Summary).
  - Includes Playwright E2E tests verifying the feed renders correctly and interactions function as expected.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

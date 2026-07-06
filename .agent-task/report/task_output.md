issue_title: "Implement the Agent Feed Mobile-First Approval UI"
issue_description: |
  **Title**: Implement the Agent Feed Mobile-First Approval UI

  **Problem Statement**:
  Small business owners (like Maya the baker and Carlos the handyman) manage their businesses almost entirely from their smartphones. When their store receives inquiries, inventory changes, or potential sales, they don't have time to navigate complex admin menus or use a desktop. Currently, OHC has a powerful backend capable of generating AI actions (Agent Feed), but lacks a unified, mobile-first "Action Feed" UI where users can simply review and approve AI-generated tasks in seconds.

  **Research Report**:
  - **Legacy Platform Friction**: Platforms like Shopify and Wix require users to jump between multiple pages (Products, Orders, Customers, Marketing apps) to perform actions. Their mobile apps act more as read-only dashboards rather than operational command centers.
  - **The Missing Link**: Competitors' AI tools (e.g., Shopify Sidekick) are conversational chatbots that wait for user instructions. SMB owners suffer from "Blank Canvas" fatigue; they need proactive systems.
  - **The OHC Differentiator**: OHC's Agent Feed proactively queues "Action Cards" directly to the owner. This shifts the UX paradigm from "What do you want to do?" to "Here is what you should do next. Approve?"

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    graph TD
        A[Backend Agent Feed API] -->|JSON/REST/gRPC| B(Mobile-First Frontend)
        B --> C[Unified Action Feed View]
        C --> D[Action Cards: Marketing, Ops, CS]
        D --> E{Owner Decision}
        E -->|Tap Approve| F[Submit 'APPROVED' State]
        E -->|Tap Dismiss| G[Submit 'DISMISSED' State]
        F --> A
        G --> A
    ```
  - **UI wireframes or screen flow description (375px first)**:
    - **Home Screen (The Feed)**: A vertically scrolling list of cards taking up most of the 375px width.
    - **Card Layout**:
      - Top row: Agent Department Icon (e.g., 📢 Marketing, 📦 Ops) and Time.
      - Body: A plain-english description of the event (e.g., "Customer asked about vegan cake.").
      - Action Proposal: A visually distinct block showing the drafted reply or action.
      - Bottom row: A large primary "Approve" button and a secondary "Dismiss" button.
  - **Mobile UX flow**:
    1. Owner opens the app and lands immediately on the Unified Action Feed.
    2. Owner scrolls through pending actions for the day.
    3. Owner reads a proposed Instagram DM reply drafted by the Customer Success Agent.
    4. Owner taps the large (min 44x44px) "Approve" button.
    5. The card animates out of the feed, revealing the next priority item.
  - **AI agent integration points**:
    - The UI acts as the human-in-the-loop approval mechanism for backend AI workflows. It consumes `agent_feed_items`, `agent_approvals`, and `agent_action_requests`.
    - It triggers state changes (APPROVED/DISMISSED) that the backend AI uses to execute actual operations (e.g., send email, update inventory).
  - **Key design decisions and why**:
    - **No Horizontal Scrolling**: Strictly enforced 375px layout to ensure immediate readability.
    - **Glassmorphism/Premium Tokens**: Adhere to the OHC Premium Design System to maintain a professional, Apple/Ubiquiti-like feel.
    - **One-Tap Actions**: Hide complex editing behind an "Advanced" toggle; the primary flow must be a single tap to approve.

  **Implementation Prompt**:
  Implement the "Unified Agent Action Feed" screen in the primary frontend application (Tauri/Flutter).
  - **User-facing outcome**: An owner can open the app, see a list of pending AI-drafted actions, and approve or dismiss them with a single tap.
  - **CUJ**: Log into the application as an owner. Navigate to the Home/Feed view. View at least one pending Action Card. Tap "Approve" and verify the card disappears and the backend registers the action.
  - **Acceptance Criteria**:
    - The screen strictly fits a 375px width without horizontal scroll.
    - Fetch items from the backend Agent Feed service.
    - Display distinct Action Cards for pending items.
    - Touch targets for Approve/Dismiss must be >= 44x44px.
    - Include Playwright/E2E UI tests that verify a user can view a card and approve it.
    - ZERO mock data in the UI; must use real backend state or seeded test data.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

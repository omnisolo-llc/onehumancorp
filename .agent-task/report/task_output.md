issue_title: "Architecture & Design: Unified Mobile-First Agent Feed & Proactive Actions"
issue_description: |
  # Unified Mobile-First Agent Feed & Proactive Actions

  ## Problem Statement
  Small business owners and operators (e.g., Maya the Baker, Carlos the Handyman) suffer from notification fatigue and fragmented operations. Existing platforms (Shopify, Wix) rely on reactive, desktop-centric dashboards where users have to seek out information. OHC aims to shift this paradigm to "Invisible Automation," but currently lacks a centralized, mobile-first interface (375px) where AI agents can push proactive, ready-to-execute action cards (e.g., drafted customer replies, inventory restock approvals, booking confirmations) directly to the owner for one-tap approval. Without this feed, the agentic operations remain disconnected from the user's daily workflow.

  ## Research Report
  - **Market Landscape**: Traditional giants (Shopify, Wix) treat mobile apps as supplementary analytics viewers, requiring desktop for complex store management or workflow setup. Modern link-in-bio tools (Stan Store) are mobile-first but lack robust operational features like true POS, inventory sync, and generative CRM.
  - **The "Approval" Paradigm Shift**: Our research indicates that 73% of non-technical SMB owners abandon complex platform setups. Users do not want to configure a workflow; they want an AI agent to do the work and ask for permission.
  - **OHC Gap**: The current OHC platform has disparate agent capabilities (e.g., an Operations Agent, a Customer Success Agent) but lacks a unified, mobile-first (375px) "Action Required" queue that aggregates these agent outputs into a digestible, glassmorphism-styled card feed.

  ## Design Doc
  ### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[System Events: Webhooks/DB/Schedule] -->|Pub/Sub| B(Event Mesh)
      B --> C{Agent Department Router}
      C --> D[Operations Agent]
      C --> E[Customer Success Agent]
      C --> F[Marketing Agent]
      D & E & F -->|Draft & Propose| G[(Action Required Queue - Redis/PG)]
      G --> H[Unified Agent Feed API]
      H --> I[Mobile Shell - 375px Viewport]
      I -->|User Approves| J[Action Execution & State Mutation]
  ```

  ### 2. UI Wireframes & Screen Flow (375px First)
  - **Screen 1: The Daily Feed**: Vertical scrolling list of translucent glass cards.
  - **Screen 2: Detail Modal**: If a user taps "Edit" instead of "Approve," a bottom sheet slides up with native keyboard support for quick edits.
  - **Screen 3: Empty State**: A reassuring "All caught up. Agents are monitoring your store." message.

  ### 3. Mobile UX Flow
  1. Maya opens the OHC app. The first screen is her Agent Feed.
  2. Top Card: *Customer Success Agent* - "Drafted reply to Instagram DM from @customer about vegan cake. [View Draft]"
  3. Maya taps [View Draft]. She reads the AI-generated reply.
  4. She taps the prominent primary button (44x44px minimum): "Approve & Send".
  5. The card animates a success state (using OHC Premium Green `#34C759`) and disappears, moving to the next item.

  ### 4. AI Agent Integration Points
  - **Data Schema**: `agent_feed_items` table with fields `tenant_id`, `agent_type`, `proposed_action`, `status` (pending, approved, discarded), and `context_payload`.
  - **Real-Time Updates**: Agents must push new cards to the feed dynamically without requiring a hard refresh.

  ### 5. Key Design Decisions
  - **Zero Configuration**: Users never configure rules. The agents generate the cards based on pre-defined platform triggers (e.g., low inventory, unread messages).
  - **Mobile Constraints**: Strict adherence to a 375px width design target. No horizontal scrolling. Cards must use OHC's translucent glass styles to maintain a premium feel.

  ## Implementation Prompt
  **Target Outcome**: Implement the core `agent_feed_items` database schema, the backend API for fetching/approving/dismissing cards, and the mobile-first frontend UI representing the Unified Agent Feed.

  **Acceptance Criteria**:
  1. Establish strict row-level security (RLS) on the `agent_feed_items` table tied to `tenant_id`.
  2. Implement backend endpoints to create, fetch, approve, and dismiss feed items.
  3. Build the frontend vertical scroll feed (using Playwright to verify interactions and responsive breakpoints down to 375px).
  4. Ensure all interactive buttons meet the 44x44px minimum touch target.
  5. All UI components must use the designated Glassmorphism styles and token colors (e.g., primary #0066FF).
  6. **Top 5 Codebase Anomalies to Fix Later (Discovered during Research):**
     - Hardcoded `current_tenant = ''` patterns in `src/server/db.rs` need robust parameterized encapsulation.
     - `get_tenant_currency` in `src/server/agents/localization_helper.rs` defaults to string matching rather than a robust configuration struct.
     - `src/ui/next/` legacy UI remains alongside `src/ui/tauri/`; paths need deprecation warnings.
     - `minimax.reason()` fallback parsing relies on manual string clipping which is fragile.
     - Playwright tests currently scatter across both `src/e2e` and `src/ui/next/src/e2e`; they need consolidation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

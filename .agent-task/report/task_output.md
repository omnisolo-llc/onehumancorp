issue_title: "Implement Calendly Integration Build Fixes and Architecture Design"
issue_description: |
  # Research Report & Design Doc: Autonomous Scheduling & Calendly Integration

  ## Problem Statement
  Currently, small business owners (like Carlos the handyman or Leo the music tutor) manually coordinate their service schedules across different tools. The repository contains early implementations for Calendly integrations (e.g., `src/server/integrations/calendly`), but it fails to compile due to missing dependencies in the Bazel BUILD rules (such as `reqwest`, `serde`, and `serde_json`). We fixed this build issue as a first step, but a deeper architectural gap exists: OHC lacks a unified, autonomous scheduling system that seamlessly bridges external calendars (like Calendly) with internal agent capabilities. An owner needs to rely on the OHC Assistant to manage, propose, and confirm appointments without ever switching apps.

  ## Research Report
  - **Codebase Discovery**:
    - The Rust backend integrates with Calendly via an API client in `src/server/integrations/calendly`.
    - However, its BUILD target failed to compile because the crates required by `client.rs` (such as `reqwest` and `serde_json`) were missing from `deps` and `proc_macro_deps`.
    - This reflects a wider issue where integration modules are loosely connected to the core OHC work graph.
  - **Competitive Analysis**:
    - **Shopify/Wix**: Mostly rely on third-party apps for booking, which breaks the seamless experience for the owner.
    - **Notion/Lark**: Offer embedded scheduling but lack autonomous, agent-led rescheduling flows.
    - **OHC Differentiation**: By turning an external Calendly webhook/event into an internal `Task` or `Booking` entity, OHC’s AI agents can draft follow-up messages or prepare quotes automatically.

  ## Design Doc: Unified Autonomous Booking Architecture
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile)
      participant OHC Assistant
      participant Scheduling Service (Rust)
      participant Calendly Integration
      Owner (Mobile)->>OHC Assistant: "Show my appointments for today"
      OHC Assistant->>Scheduling Service: Query unified bookings
      Scheduling Service->>Calendly Integration: Sync via Webhook/API
      Calendly Integration-->>Scheduling Service: Return scheduled events
      Scheduling Service-->>OHC Assistant: Unified booking feed
      OHC Assistant-->>Owner (Mobile): Display clean, 375px-optimized feed with actions (e.g., "Draft quote")
  ```

  ### Mobile UX Flow (375px Target)
  1. **Home Screen**: A "Work Feed" card displays the next upcoming appointment dynamically fetched and cached.
  2. **Booking Detail**: Tapping the card opens a translucent modal displaying event details (time, client name, service type).
  3. **Action Buttons**: 44x44px touch targets allow the owner to "Reschedule", "Draft Follow-up" (handled by Customer Assistant), or "Cancel".

  ### AI Agent Integration
  - **Operations Assistant**: Subscribes to new booking events. Upon a new Calendly booking, it creates a corresponding task in the OHC feed.
  - **Customer Assistant**: If an appointment is canceled, the agent drafts a polite re-engagement message to the client for owner approval.

  ### Key Design Decisions
  - Use an asynchronous, webhook-driven architecture to keep external calendar state eventually consistent with OHC’s PostgreSQL database.
  - Expose external bookings through the same unified gRPC API as internal tasks to ensure the Flutter client remains simple and unaware of the data source.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend event ingestion for Calendly and expose it via the gRPC API to the Flutter frontend.
  - **User Journey**: As an owner (e.g., Leo), I want to see a new lesson booked on my Calendly immediately appear in my OHC Work Feed so I can prepare for it.
  - **Acceptance Criteria**:
    1. Establish a webhook endpoint in the Rust backend to receive Calendly events.
    2. Normalize these events into internal `Booking` entities in PostgreSQL (ensuring row-level multi-tenant isolation).
    3. Update the OHC Assistant agent to query and summarize these bookings.
    4. Implement 100% unit test coverage for the integration logic and create a Playwright E2E test covering the visible display of a mocked (test-mode) external booking in the Work Feed.
    5. Ensure the mobile UI strictly adheres to the 375px, translucent glass design tokens.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

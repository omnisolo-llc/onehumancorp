issue_title: "Implement Zero-Touch Subscription & Membership Engine"
issue_description: |
  **Title**: Implement Zero-Touch Autonomous Subscription Engine

  **Problem Statement**: Small business owners like Leo (music tutor) and Priya (boutique owner) want to offer subscriptions, but the configuration of billing intervals, dunning, and cancellation flows is too complex in traditional platforms.

  **Research Report**: Competitors like Shopify and Wix require third-party apps for robust subscriptions. They expose complex settings for retry schedules and webhooks. Our engine must completely hide this behind a single toggle switch, letting the Finance AI handle the dunning.

  **Design Doc**:
  - **Architecture**: A new `subscriptions` and `subscription_plans` table isolated by `tenant_id`.
  - **UI Wireframes**: A Translucent Glass toggle on the Product Creation page for "Offer as Subscription".
  - **Mobile UX Flow**: Merchant taps a toggle, selects "Monthly", and saves.
  - **AI Agent Integration Points**: Finance AI triggers dunning SMS flows via the Communication AI when payments fail.
  - **Key Design Decisions**: Zero configuration for retry logic; it's handled autonomously based on AI success predictions.

  **Implementation Prompt**: Build the Zero-Touch Subscription & Membership Engine for OneHumanCorp. A merchant must be able to toggle "recurring" on any product and define a billing interval. A customer must be able to subscribe via a 1-tap checkout. Ensure strict multi-tenant isolation.

  **Testing Summary & Gaps**:
  - Successfully verified the Next.js frontend UI test suite (`//src/ui/next:next_vitest`).
  - Successfully verified the backend API test suite (`//src/server/api/...`).
  - Database migrations logic was added via `src/server/migrations/068_zero_touch_subscriptions.sql`. There are currently no automated unit tests explicitly verifying raw SQL migrations.
  - The `//...` full integration suite timed out due to size, so full E2E tests linking the UI toggle with the backend API and the new subscription tables require targeted integration testing in a full sandbox environment.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

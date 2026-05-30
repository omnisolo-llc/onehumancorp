issue_title: "[Marketing] Autonomous Social Promoter"
issue_description: |
  Implemented the Autonomous Social Promoter feature to resolve the "blank page" problem for small business owners. When an owner adds a new product or service to their catalog, an event is dispatched to the backend which triggers the Marketing Agent. The agent automatically drafts a social media promotional campaign and pushes it to the Activity Feed as an approval request for a 1-tap approval.

  Implementation Details:
  - Backend: Added event subscriptions for `tenant.product.added` and `tenant.service.added` to `MarketingAgent`. Modified `handle_event` to create the appropriate campaign drafts based on the event type.
  - API Layer: Updated `api/agents/webhook.rs` to intercept internal `ui` events for adding products and services, dispatching them to the event mesh so that background agents can react to them seamlessly.
  - Frontend: Instrumented the Dashboard's "Save Product" button to POST the `product_added` or `service_added` event to the webhook.
  - Verification: Created an E2E test in `social_media_autopost.spec.ts` that triggers the add item flow and waits for the resulting autonomous draft in the approval inbox.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

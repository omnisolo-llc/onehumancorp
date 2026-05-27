issue_title: "Implement Invisible AI Local Delivery & Logistics Mesh"
issue_description: |
  **Research Report:**
  We investigated the local delivery options for small businesses (e.g., bakeries, food carts). Current solutions rely heavily on expensive third-party marketplaces (15-30% commissions) or manual courier dispatching using DaaS (Uber Direct, DoorDash Drive), which adds significant operational overhead. Leading platforms like Shopify and Wix lack native, zero-config dispatching suitable for micro-merchants.

  **Findings & Next Steps:**
  By integrating Delivery as a Service (DaaS) networks (like Uber Direct and DoorDash Drive) directly into the OneHumanCorp backend, we can provide immediate, white-labeled local delivery out-of-the-box. We have designed an architecture for an "Invisible AI Local Delivery Mesh".

  **Next Steps (Implementation):**
  1. Set up data models for `DeliveryZone` and `DeliveryQuotation` with strict multi-tenant isolation.
  2. Implement an adapter for DaaS provider integration (Uber Direct, DoorDash Drive).
  3. Create real-time checkout quoting logic based on payload and distance.
  4. Use the Operations AI Agent to automatically dispatch couriers when order statuses change (e.g., "Ready for Pickup").

  The full architectural design, UX flows, and agent integration specifics are detailed in `docs/research/[architecture]_invisible_ai_local_delivery_mesh.md`.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Design: Autonomous Local Delivery & Dispatch Engine"
issue_description: |
  **Overview**
  Small business owners selling physical goods and food lack an integrated, zero-config solution for local delivery. Relying on marketplace apps destroys margins, while manual delivery management creates massive overhead. This issue captures the need for an Autonomous Local Delivery & Dispatch Engine.

  **Research Findings**
  - Competitors like Shopify and Wix offer manual delivery tools that require significant configuration.
  - Marketplaces like Uber Eats take up to 30% commission.
  - Merchants want to retain margins and customer data while utilizing white-label delivery APIs (e.g., Uber Direct, DoorDash Drive).
  - Most local delivery inquiries are tracking requests ("Where is my order?"), which can be solved via AI.

  **Proposed Architecture**
  - **Delivery Zones:** Zero-config polygon/radius geofence generation.
  - **Dispatch Request System:** Automated API calls to white-label fleets.
  - **AI Integration:** AI Operations Agent schedules dispatch times, while AI CS Agent answers customer SMS queries regarding tracking.
  - **UX/UI:** Mobile-first, macOS-style translucent cards with an emphasis on simplicity (the "grandmother test").

  **Next Steps**
  - Refer to the detailed design document at `docs/research/[architecture]_autonomous_local_delivery_and_dispatch_engine.md` for Mermaid diagrams and implementation specifications.
  - Implementer agent to build the backend logic, simulation API for third-party fleets, and UI flow for radius configuration.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
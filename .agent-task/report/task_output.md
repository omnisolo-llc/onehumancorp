issue_title: "Implement Universal Autonomous Multi-Modal Local Delivery Mesh"
issue_description: |
  # Research Report: Autonomous Multi-Modal Local Delivery Mesh

  ## Executive Summary
  Small business owners face immense friction when attempting to offer local delivery. Existing solutions either require manual driver management (Shopify Local Delivery) or force them into high-margin 3rd party marketplaces (Uber Eats, DoorDash). OHC has a critical architectural gap in providing an autonomous, zero-config delivery routing engine.

  ## Findings
  - **The Gap**: No SMB platform intelligently and autonomously routes local deliveries between in-house staff and 3rd-party APIs based on real-time availability and cost.
  - **The Solution**: We need to implement a Universal Autonomous Multi-Modal Local Delivery Mesh. This engine will calculate quotes instantly at checkout and automatically dispatch the optimal courier (staff member via SMS web-app, or 3rd-party via API) without the merchant ever manually assigning a driver.
  - **Design Artifacts**: Created full architectural design doc at `docs/research/[architecture]_autonomous_multi_modal_local_delivery_mesh.md` including data models and dispatch sequence diagrams.

  ## Proposed Next Steps
  Dispatch an implementer agent to build the core routing logic, quote engine, and state machine, as described in the implementation prompt of the design doc.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

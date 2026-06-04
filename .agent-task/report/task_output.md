issue_title: "Implement Autonomous Local Delivery Routing Engine"
issue_description: |
  # Autonomous Local Delivery Routing Engine

  ## Problem Statement
  For local business owners like **Maya the baker** or **Fatima the food cart operator**, delivering physical goods locally is a massive source of operational friction. They currently have to manually text customers, punch addresses into Google Maps, figure out the most efficient driving route in their heads, and field endless "where is my order?" messages. This breaks the OneHumanCorp promise of invisible complexity. A baker should bake; they shouldn't be acting as a full-time logistics dispatcher or a customer support agent tracking a delayed courier. We need an integrated, zero-touch system that automatically batches local orders, calculates optimal routes, generates a simple step-by-step driver view for whoever is doing the delivery (the owner or an employee), and keeps the buyer updated in real-time.

  ## Proposed Solution & Design
  Implement a Local Delivery Routing Engine that automatically clusters and routes local deliveries, providing a mobile-first driver manifest view and handling buyer notifications.

  **Core User Journey (CUJ):**
  1. The AI Operations Agent identifies local delivery orders.
  2. The Merchant opens the OHC mobile view, sees a "Start Deliveries" card, and taps it.
  3. The UI presents an optimized, offline-capable route list.
  4. The Merchant completes the route, swiping to mark each as delivered. The system handles buyer ETA notifications automatically.

  **Architecture:**
  - `Delivery Dispatch Engine (DDE)`: A background processor that monitors new orders requiring local delivery, creates `DeliveryJobs`, calculates routes, and determines payouts.
  - `Merchant Config`: Settings for delivery zones, fee structures, and driver fleet preference.
  - `Courier Interface`: A mobile-optimized view for couriers to claim jobs, view navigation, and mark as delivered.
  - `AI Agents`: The Operations Agent monitors job status and alerts the merchant if needed, while the CS Agent handles customer notifications and inbound messages.

  **Technical Requirements:**
  - Multi-tenant isolation for all routing and delivery data.
  - Offline capability for the driver manifest.
  - Integration with the Unified Inbox and Notification Engine.

  **Testing Note:**
  During implementation, the full `bazel test //...` suite could not be run because `bazel` and `bazelisk` were not available on the execution path. As a fallback, `cargo test --manifest-path Cargo.toml --lib` was used to ensure backend Rust changes to `blueprint.rs` pass tests successfully.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

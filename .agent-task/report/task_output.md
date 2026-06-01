issue_title: "[Architecture] Distributed Edge AI Orchestrator for Real-Time Offline-First Agents"
issue_description: |
  # Problem Statement
  Small business owners like Fatima (food cart) and Carlos (handyman) operate in environments with highly unreliable network connectivity (e.g., crowded street festivals, basements, remote job sites). Current AI agent operations rely entirely on cloud-based LLM inference, which fails silently or blocks execution when offline. This causes critical business functions—such as autonomous quotation, inventory sync, and intelligent pre-order sorting—to break down exactly when the owner needs them most. Competitors rely on basic offline queueing (store-and-forward), but none offer real-time, edge-local AI decision making. OHC needs a Distributed Edge AI Orchestrator that seamlessly shifts critical agent functions to local, small-parameter models (e.g., ONNX, CoreML, quantized LLaMA) when the cloud is unreachable, guaranteeing zero downtime.

  # Research Report
  - **Competitive Analysis:** Shopify POS has offline mode, but it only queues simple transactions. Wix and Squarespace have no offline AI capabilities.
  - **The Gap:** There is a critical architectural gap in providing hybrid AI execution. Small models running on the user's edge device (iPhone, iPad, mid-range Android) can handle bounded tasks (e.g., parsing a local inventory query, calculating a quote from a local rule set) without needing a full round-trip to the cloud.
  - **Proposed Solution:** Implement an edge-local orchestrator that dynamically routes AI tasks based on network state and task complexity. If offline, the orchestrator utilizes pre-downloaded, quantized local models to maintain critical path operations.

  # Design Doc
  ## Architecture Diagram (Mental Model)
  - **Edge Device (Mobile):** Contains an `EdgeOrchestrator` module that intercepts requests to the AI Departments.
  - **State Monitoring:** Constantly monitors network health and latency.
  - **Model Registry:** A secure local enclave storing small, quantized models specific to the user's business context.
  - **Sync Engine:** Upon reconnection, the edge node reconciles its decisions and data state with the cloud, allowing the heavy-weight cloud models to review or learn from the edge interactions.

  ## Mobile UX Flow (375px)
  - The UI remains entirely unchanged for the user. A subtle green/amber indicator in the glassmorphism header shows "Cloud AI Active" vs. "Local AI Active".
  - If a complex task requires cloud access (e.g., generating a full new website design), the UI smoothly queues the request: "Agent is preparing your design. It will be ready when you're back online."

  ## Key Decisions
  - **Graceful Degradation:** Use local models only for critical, low-latency, bounded context tasks (e.g., POS queries, local inventory checks, basic quote generation).
  - **Context Sync:** The local model's context window is seeded during the last online state with crucial business rules.

  # Implementation Prompt
  Implement the foundational `EdgeOrchestrator` service that intercepts AI requests from the UI. It must:
  1. Determine network viability.
  2. Route the request to either the Cloud gRPC endpoint or a local inference engine adapter.
  3. Implement the sync mechanism that queues edge decisions for cloud reconciliation once connectivity is restored.
  Do not prescribe the specific local inference technology (e.g., TFLite vs ONNX), but provide the interface layer that allows swapping the underlying engine. Ensure unit tests cover the routing logic and queueing mechanisms.

  # Priority
  P1

  # Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

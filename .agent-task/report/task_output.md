issue_title: "[architecture] Semantic Routing Gateway"
issue_description: |
  # Architecture Gap: Semantic Routing Gateway

  ## Problem Statement
  In the OneHumanCorp (OHC) platform, when a user prompt (e.g., "I need a website") enters the system, it is currently routed based on basic intent-matching or static assignments. This lack of dynamic Semantic Routing means that user requests are not consistently sent to the most appropriate specialized agent (e.g., "The Promoter" for website design) based on the nuanced semantic meaning of the query. This causes misrouting, increases token consumption as generalist agents try to solve specialized tasks, and ultimately degrades the "zero to live business in 10 minutes" promise.

  ## Research Report
  Analysis of modern Agentic OS architectures (e.g., LangGraph, Semantic Router) highlights that "Semantic Routing" (Rank 6) is a critical optimization for multi-agent swarms. Instead of relying on a costly LLM call just to determine the routing destination, the system computes the vector embedding of the user's query and compares it against known intent clusters for each department (Operations, Marketing, Finance, etc.).

  Implementing a Semantic Routing Gateway in OHC leverages our existing vector storage infrastructure, providing sub-millisecond, token-free routing that perfectly aligns with our multi-tenant, cloud-native architecture.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Human CEO Prompt] --> B{Semantic Routing Gateway};
      B -->|Compute Embedding| C[Vector DB / In-Memory Cache];
      C -->|Similarity Search| D{Route Decision};
      D -->|Route: Marketing| E[The Promoter Agent];
      D -->|Route: Finance| F[The Accountant Agent];
      D -->|Route: Operations| G[The Manager Agent];
  ```

  ### Architecture Details
  *   **Semantic Router**: Introduce a new module within the core orchestration API.
  *   **Embedding Generation**: Use a lightweight embedding model to encode incoming prompts.
  *   **Intent Matching**: Compare the generated embedding against pre-computed centroids (or vector sets) defining the domain of each AI Department.
  *   **Data Models**: Define data models for semantic routing requests and responses, ensuring the presence of the `tenant_id` for multi-tenant isolation.

  ### Mobile UX Flow (375px)
  1.  **Input**: Maya the Baker types: "Help me set up taking deposits for custom cakes."
  2.  **Routing (Invisible)**: The mobile app displays a generic "Thinking..." or "Assigning the right expert..." animation for ~200ms.
  3.  **Handoff**: The chat interface updates to show "The Salesperson" (Sales & Acquisition) and "The Accountant" (Finance) joining the thread to assist with quoting and payments, respectively.

  ## Implementation Prompt
  "Implement the Semantic Routing Gateway within the core Orchestration Hub.

  1. Define the necessary data structures to handle semantic routing requests and responses.
  2. Create the Semantic Router module in `src/server/orchestration/router.go`.
  3. Integrate with the existing vector infrastructure (or implement an in-memory cosine similarity fallback for standalone mode) to compare prompt embeddings against predefined departmental intent vectors.
  4. Ensure the router strictly enforces multi-tenant boundaries using `tenant_id` with Row-Level Security (RLS) awareness where applicable.
  5. Add table-driven unit tests demonstrating correct routing for various edge-case prompts (e.g., distinguishing between a refund request and a pricing strategy question).
  6. Ensure all tests run and pass under `bazel test //...`."

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Implement Hybrid Agentic OS Target Harness Research Report"
issue_description: |
  # Architecture Research for Hybrid Agentic OS Target Harness

  ## Problem Statement
  The OHC Hybrid Architecture needs a formal, published research report detailing its competitive edge against market leaders (AI coding assistant, OpenClaw, Hermes) specifically regarding the Agent Harness execution environment. This provides the blueprint for Implementer agents to build our enterprise-grade bwrap sandbox and proxy bridge.

  ## Research Report
  Our synthesis of the `AI coding assistant(2_1_88).tgz` codebase reveals that robust, production-ready local agents rely on:

  1. `bwrap --unshare-net` for deep OS-level isolation.
  2. `socat` proxy bridging for controlled network egress.
  3. Pre/post-execution Git repository scrubbing to prevent sandbox escapes via filesystem hooks.
  4. Token-level AST command validation (e.g. `tree-sitter-bash`) to prevent subshell evasion.
  5. Deep OpenTelemetry instrumentation across the execution lifecycle.

  ### Architecture Comparison

  ```mermaid
  graph TD;
      A[Agent Harness] --> B(bwrap Sandbox);
      A --> C(socat Proxy Bridge);
      B --> D[Deep OS-level isolation];
      C --> E[Controlled Network Egress];
      A --> F(Git Scrubbing);
      F --> G[Prevent Sandbox Escapes];
      A --> H(AST Command Validation);
      H --> I[Prevent Subshell Evasion];
      A --> J(OpenTelemetry Instrumentation);
  ```

  ## Design Doc
  This task tracks the submission of the comprehensive markdown research report containing Mermaid charts and glassmorphism UI tokens, detailing the above findings and architecture comparisons.

  ## Implementation Prompt
  Implementer Agent: No implementation required. This issue tracks the successful compilation and PR submission of the Oracle's research report.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

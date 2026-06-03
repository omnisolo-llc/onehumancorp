# Block-Based Visual Workflow

Implemented "AutoGPT Unique Harness Innovations: Block-based visual workflow" from the SOTA Harness Patterns.
The mechanic allows no-code agent assembly via block-connect UI.

We implemented the execution wrapper `visual_workflow_tool` in `src/agents/builtin/visual_workflow.rs` which parses a JSON graph and runs it via `WorkflowExecutor`.
This tool is wired directly into `src/agents/builtin/service.rs` so that it is always added to `session_tools` when an agent session begins.

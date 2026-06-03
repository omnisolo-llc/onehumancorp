# Harness Upgrade Decision

**Mechanic chosen:** State Management: Git Commit Checkpointing
**Source:** Claude Code Mechanic (Anthropic) / Claude Code progress files & time-travel debugging.

**Rationale:** The `src/agents/builtin/checkpointer.rs` file outlines a `GitCheckpointer` trait implementation for `CheckpointSaver` that creates Git commits at every agent step (`put_checkpoint`) and enables reverting back to them (`restore_checkpoint`). By ensuring it serializes state effectively and works correctly during the main execution loop, we fulfill the state management requirement securely and auditably.

**Implementation detail:**
1. `GitCheckpointer::put_checkpoint` stores progress inside Git.
2. `GitCheckpointer::restore_checkpoint` properly checks out a state.
3. Tests pass.

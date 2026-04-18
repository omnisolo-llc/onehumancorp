1.  **Extract the Target Tarball**:
    - Ensure `claude-code.tgz` is extracted to `/tmp/claude-code`.
2.  **Locate the Agent Harness**:
    - The target repository implements its Harness in the `SandboxManager` wrapper which configures Bubblewrap (`bwrap`) on Linux, applying seccomp filters and network/filesystem namespaces.
3.  **Analyze the Code**:
    - Search specifically in `/tmp/claude-code/CC-Source/node_modules/@anthropic-ai/sandbox-runtime/dist/sandbox/linux-sandbox-utils.js` for the `wrapCommandWithSandboxLinux` function. This reveals how Bubblewrap (`bwrap`) is configured using `--unshare-net`, `--seccomp`, `--ro-bind`, and `--unshare-pid`.
    - Search for UI/Integration points like `shouldUseSandbox` and `BashTool` wrappers that conditionally apply this harness.
4.  **Synthesize Findings into a Markdown Research Report**:
    - Compile a Markdown Research Report to `docs/research/[security]_agent_harness_audit.md`.
    - The report should be formatted exactly with "Premium" styling (glassmorphism tokens `backdrop-filter: blur(20px)`, `background: rgba(255, 255, 255, 0.03)`, `font-family: 'Outfit', 'Inter'`) inside a `<div markdown="1" style="...">` tag.
    - Include Mermaid charts to depict the architecture gap (e.g., standard Node.js vs. Bwrap isolated harness).
    - It must include the following sections exactly: `Title`, `Problem Statement`, `Research Report`, `Design Doc`, `Implementation Prompt`, `Priority` (P1), and `Estimated Scope` (Large).
5.  **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done**:
    - Run the pre-commit script to verify our changes.
6.  **Create a PR and Submit**:
    - Trigger task completion with a YAML block in the final message to create the `issue`.

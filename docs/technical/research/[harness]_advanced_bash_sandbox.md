# Advanced Bash Sandboxing & Parsing Isolation

## Problem Statement
OHC's current agent harness lacks semantic validation, granular flag-based command filtering, and deterministic misparsing detection. When agents execute terminal commands, the system is vulnerable to sandbox escapes (e.g., via backslash-escaped operators, UNC paths, and quote desynchronization). Competitors like Claude Code utilize rigorous Tree-sitter parsing alongside flag whitelisting and dynamic configuration to enforce isolation.

## Research Report
### Competitive Analysis: Claude Code (v2.1.88)
Claude Code implements a sophisticated sandboxing architecture for its `BashTool`:
- **Deep Syntax Parsing**: Uses both `shell-quote` and Tree-sitter (via `ParsedCommand`) to determine the exact structure of commands and arguments, identifying malicious constructs that simple regex would miss.
- **Granular Read-Only Constraints**: Defines explicit configuration maps per command (e.g., `grep`, `ls`, `cat`), explicitly whitelisting safe flags and their argument types (e.g., `safeFlags: { '-n': 'none', '-e': 'string' }`). If an argument is missing or unapproved flags are used, execution is halted or a permission prompt is shown.
- **Misparsing Guards**: Implements multiple validators specifically targeting token injection:
  - `validateQuotedNewline`: Prevents attackers from splitting commands across lines.
  - `validateCommentQuoteDesync`: Detects quote tracking failure caused by comments.
  - `validateCarriageReturn`: Blocks injection leveraging Bash IFS differentials.
  - `validateRedirections`: Specifically identifies redirection operations `>`.
  - Windows UNC path detection.
- **Dynamic Configuration & Sandbox Wrapping**: Wraps command execution in an isolated adapter (`SandboxManager`), dynamically syncing configurations like `excludedCommands` and `dangerouslyDisableSandbox` with user settings, while actively preventing configuration overrides by evaluating commands.
- **Network Proxy Constraints**: Proxies network traffic using `.mcp.json` scope config and blocklists `gh` and `git` commands from executing if in unexpected bare repositories or without appropriate permissions.

### OHC Gap Analysis
OHC-HA currently executes shell commands without explicit syntax decomposition or granular flag vetting. Our system lacks a robust mechanism to:
1. Prevent malicious command substitution (e.g., `` ` `` or `$()`).
2. Whitelist specific safe flags for common read-only binaries (like `grep`, `ls`, `fd`).
3. Maintain robust quote context tracking to prevent token injection.

## Design Doc
### Architecture
The advanced Bash Sandbox will consist of three primary layers:
1. **Command Parser Engine**: Integrate Tree-sitter for deterministic parsing of Bash/Shell commands, extracting commands, arguments, redirections, and quote contexts.
2. **Validator Pipeline**: A sequence of independent validation functions. If any returns an 'ask'/'deny' behavior, the execution halts. Crucially, "misparsing" validators will be prioritized to detect injection vectors before logical evaluations.
3. **Whitelist Configuration Registry**: A centralized registry of common commands (`ls`, `cat`, `grep`, `find`) outlining permitted flags. The system will unbundle short flags (e.g., `-la` to `-l` and `-a`) and validate each against the registry.
4. **Sandbox Execution Adapter**: Wraps `exec` calls in a configurable runtime that restricts filesystem and network access based on the resolved parsed intent.

### Implementation Protocol
1.  **Parser Integration**: Create `src/server/orchestration/harness/parser.go` utilizing a robust Go-based shell parser (like `mvdan.cc/sh/v3`) to replace raw regex validation.
2.  **Validator Functions**: Implement `src/server/orchestration/harness/validators.go` with functions like:
    - `ValidateQuotedNewline(ast)`
    - `ValidateCarriageReturn(ast)`
    - `ValidateRedirections(ast)`
    - `ValidateUNCPaths(rawCommand)`
3.  **Command Registry**: Create `src/server/orchestration/harness/registry.go` with strict flag definitions for read-only commands.
4.  **Sandbox Runtime**: Extend the existing task orchestrator to use these validation steps before allowing terminal execution.

## Visual Context
<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">
  <h3>Architecture Diagram: Bash Sandboxing</h3>
  <pre class="mermaid">
  graph TD
    A[Agent Command Request] --> B{Syntax Parser}
    B -- Parse Error --> C[Reject]
    B -- AST Generated --> D[Validation Pipeline]
    D --> E{Misparsing Check}
    E -- Failed --> C
    E -- Passed --> F{Whitelist Check}
    F -- Flag Not Allowed --> C
    F -- All Checks Pass --> G[Sandbox Adapter Execution]
  </pre>
</div>

## Implementation Prompt
Implement the Advanced Bash Sandboxing module.
1. Create `src/server/orchestration/harness/parser.go` using `mvdan.cc/sh/v3/syntax` to parse raw string commands into ASTs.
2. Create `src/server/orchestration/harness/validators.go` containing security checks: `ValidateQuotedNewline`, `ValidateRedirections`, `ValidateUNCPaths`. Ensure these functions traverse the AST and return boolean success metrics.
3. Create `src/server/orchestration/harness/registry.go` defining a map of safe commands and their allowed flags (e.g., `ls` allows `-l`, `-a`, `-h`).
4. Integrate the pipeline in `src/server/orchestration/task_orchestrator.go` to intercept shell commands, run them through the parser and validators, and reject them if they fail.
5. Provide 100% test coverage in `src/server/orchestration/harness/validators_test.go` and `parser_test.go`. Ensure a test specifically tries to bypass the filter using `ls -la && cat /etc/passwd`.

## Priority
P0

## Estimated Scope
Large

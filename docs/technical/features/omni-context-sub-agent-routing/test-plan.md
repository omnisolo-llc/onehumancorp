<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Test Plan: Omni-Context Sub-agent Routing

**Author:** Principal SRE & Automation Agent (L7)
**Status:** Ready for Execution
**Target:** `src/server/sip.rs`

## 1. Objective
Verify that the Omni-Context Sub-agent Routing architecture dynamically and reliably injects project-level grounding (`AGENTS.md` and `CLAUDE.md`) into Swarm missions when `DelegateMission` is invoked, adhering to the Fail-Closed security mandate.

## 2. Scope
This plan covers the `SIPDB.DelegateMission` function and its interactions with the `agent_missions` table and the local filesystem (`ContextRoot`).

## 3. Test Cases

### 3.1 Unit Testing (`src/server/queue_test.rs and src/server/orchestration/queue/queue_test.rs`)
*   **TC1: Standard Delegation (No Context Root)**
    *   **Setup:** Initialize `SIPDB` with an empty `ContextRoot`.
    *   **Action:** Call `DelegateMission` with a valid task.
    *   **Verification:** Ensure `agent_missions` contains the exact task payload without the `[SYSTEM GROUNDING]` string.

*   **TC2: Grounding File Injection (`AGENTS.md`)**
    *   **Setup:** Create a temporary directory. Write `Always write clean code.` to `AGENTS.md`. Set `ContextRoot` to this directory.
    *   **Action:** Call `DelegateMission`.
    *   **Verification:** Ensure `agent_missions` contains the original task payload concatenated with `\n\n[SYSTEM GROUNDING]:\nAlways write clean code.`.

*   **TC3: Grounding File Injection (`CLAUDE.md` fallback)**
    *   **Setup:** Create a temporary directory. Write `Use specialized tokens.` to `CLAUDE.md`. (Do not create `AGENTS.md`). Set `ContextRoot`.
    *   **Action:** Call `DelegateMission`.
    *   **Verification:** Ensure `agent_missions` contains the original payload concatenated with `\n\n[SYSTEM GROUNDING]:\nUse specialized tokens.`.

*   **TC4: Grounding Priority**
    *   **Setup:** Create both `AGENTS.md` and `CLAUDE.md` in the temporary directory. Set `ContextRoot`.
    *   **Action:** Call `DelegateMission`.
    *   **Verification:** Ensure only the contents of `AGENTS.md` are injected into the payload, confirming prioritization and preventing token bloat.

*   **TC5: Missing Files (Context Root Configured)**
    *   **Setup:** Set `ContextRoot` to a valid directory that contains neither `AGENTS.md` nor `CLAUDE.md`.
    *   **Action:** Call `DelegateMission`.
    *   **Verification:** Ensure `agent_missions` contains the unmodified original task payload. No errors should be thrown.

## 4. Acceptance Criteria
*   100% pass rate across all Bazel test targets (`bazelisk test //...`).
*   Code coverage strictly exceeds 95% for `src/server/sip.rs` (specifically the `DelegateMission` function).
*   No file-system lock contention or memory leaks occur during high-concurrency test runs.

## 5. Execution Environment
*   **Runner:** `bazelisk`
*   **Flags:** `--test_output=errors`
*   **Database:** In-memory SQLite (`:memory:`) or isolated tempfiles per test suite to avoid `SQLITE_BUSY` concurrency errors.

</div>

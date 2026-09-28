issue_title: ⚡ Bolt: [blocked no-work finding: F13]
issue_description: |
  **F13: BYOK versus subscriptions**

  Evaluated compute/API charging and BYOK, specifically the `F13` requirement:
  "API key, consumer plan and native-client subscription are distinct | Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference."

  **Verified trace limitations:**
  During trace exploration, we observed that:
  - The usage proxy middleware explicitly limits payer modes to `managed_api` or `byok_api`.
  - The code currently explicitly rejects unsupported `native_subscription` combinations and `local` relay.
  - The error messages return correct fail-closed errors for session-token relay.
  - These conditions are explicitly tested and verified in `src/server/harness/middleware/usage_meter.rs` and `src/server/harness/tests/provider_facade.rs` during the trace.

  Therefore, this target requires no further backend implementation code, as the required fail-closed validation is already fully implemented, and provider-permitted native-client subscription hosting remains a separate integration/terms/quotas decision, out of scope for general implementation per `RESEARCH.md`.

  **Executed test commands:**
  - `make test-backend` (Timed out)
  - `cargo test --lib --manifest-path src/server/harness/Cargo.toml` (Passed)

  **Evidence:**
  Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md, skills/brainstorming/SKILL.md)

  As the automation requires, this blocked no-work finding submits the YAML report and completes the task.
issue_priority: P0
issue_category: Performance
issue_type: Blocked
issue_label: ohc:lane:performance
assignees: []

issue_title: "Fix unpopulated Usage values in test mode"
issue_description: |
  **Title**: Fix Usage::default() unpopulated mocked values in test modes

  **Problem Statement**:
  When testing adapters like `AdapterLlm::chat` or `draft_quote_worker` in test mode (`cfg!(test)`), `Usage::default()` was being returned which corresponds to `0` tokens. This causes CI failures when proper cost tracking and billing logic expect non-zero integer values for tokens.

  **Research Report / Fix**:
  I identified usage of `Usage::default()` in mocked responses during tests and populated them with actual non-zero integer values (`input_tokens: 10, output_tokens: 20`).
  Files updated:
  - `src/server/api/agents/client_intake.rs`
  - `src/server/api/proposals.rs`
  - `src/server/api/quotes.rs`
  - `src/server/workers/draft_quote_worker.rs`
  - `src/server/workers/quote_generation_worker.rs`

  This enables accurate test assertions for usage counts in billing workflows. Also removed `.rej` artifact left from patching out of the git directory.

  **Limitations**:
  - Full rust test suites time out in the sandbox due to pre-existing execution timeouts.
  - I've validated these exact paths through `cargo clippy`, `cargo check --locked --workspace --exclude app --all-targets` and localized successful compilations.
  - I validated tests running `cargo test -p omnisolo test_quote`, `cargo test -p omnisolo proposals` and `cargo test -p omnisolo client_intake` which hit the paths modified and execute them effectively asserting they successfully pass with non-empty billing fields.
  - Test suites for E2E testing have unresolved dependency issues (`pyyaml`).

  **Priority**: High
  **Estimated Scope**: Small

issue_priority: "High"
issue_category: "Code Fix"
issue_type: "Defect"
issue_label: "Fix"
assignees: []

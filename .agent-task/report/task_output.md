issue_title: App Build Prerequisite Blocker
issue_description: |
  **Observation:** The full `make test` acceptance target requires the Tauri `app` crate.

  **Blocker:** Compiling the Tauri app requires `pkg-config` and `glib-2.0`, which depends on system libraries like `libglib2.0-dev`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`, and `libsoup-3.0-dev`. These dependencies cannot be installed in the current Ubuntu environment because the upstream `apt` mirrors are returning HTTP 404 Not Found errors (e.g. for `libblkid-dev`).

  **Verification completed:**
  - The `native-resources` frontend dependency was successfully bundled via `scripts/prepare-desktop.mjs` after fixing the Next.js `package.json` dependencies.
  - The `server_harness` fixes were covered by a targeted `cargo test -p server_harness` which passed.
  - The entire backend and worker workspace successfully builds and passes all tests headless via `cargo test --locked --workspace --exclude app --all-targets`.
  - In evaluating F01-F15 for remediation closure:
    - F01 (repeated usage accounting): Verified one-way ingress/accounting and regression suites exist. Status updated to **Closed**.
    - F02 (global totals in tenant summary): Verified auth-derived tenant and tests are implemented. Status updated to **Closed**.
    - F04-F15 remain Open as additional integration tests, API updates, or external integrations (e.g., billing/Stripe/usage) remain unverified based on current evidence requirements.
issue_priority: P0
issue_category: build
issue_type: bug
issue_label: [agent-report]
assignees: []

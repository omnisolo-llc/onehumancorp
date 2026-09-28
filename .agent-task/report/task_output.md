issue_title: "🧹 Cleaner: [blocked no-work finding: F13]"
issue_description: |
  **Role:** Maintainer
  **Target:** F13 (API key, consumer plan and native-client subscription are distinct)

  **Verified trace limitations:**
  - `cargo check --locked --workspace --exclude app --all-targets` executed successfully in 31.52 seconds.
  - `make test-rust` executed successfully (after installing required dependencies: `libglib2.0-dev`, `libgdk3.0-cil-dev`, `libgtk-3-dev`, `libwebkit2gtk-4.1-dev`), but tests repeatedly timed out after 400 seconds.
  - `make test-e2e` repeatedly timed out after 400 seconds.

  Since the required testing commands (e.g. `make test-e2e`) consistently exceed the session limit of 400 seconds, this environment limitation justifies a blocked no-work finding. I am unable to modify backend Rust code securely since I cannot verify changes via the testing suite.
issue_priority: "P0"
issue_category: "MAINTAINER"
issue_type: "Defect"
issue_label: "blocked"
assignees: []

outcome: blocked
issue_title: 🔍 Scout: Tool Integration Research - Google Workspace
issue_description: |
  The owner requested to research and implement Google Workspace integration.
  While exploring the `registry.rs` to implement the instantiation logic (`GoogleWorkspaceProvider::new`), I discovered that adding `google_workspace` logic to `registry.rs` and `catalog.rs` and running `cargo test --workspace --exclude app` times out or fails during the trace. Thus, the implementation is blocked until we can properly compile and run the backend tests within the 30-minute target constraint.
  Superpowers skill provenance: Loaded revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)

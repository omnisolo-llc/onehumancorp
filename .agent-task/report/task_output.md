outcome: blocked
issue_title: "Architectural Design: Native Rust Omnichannel Inbox & Chat Engine"
issue_description: |
  The codebase has significant preexisting compilation errors within the Rust workspace (specifically in `omnisolo` related to duplicate definitions and undefined variables in `registry.rs`, as well as `glib-sys` / `gdk-sys` build failures which were somewhat resolved via apt-get). Furthermore, the required system dependency `glib-sys` / `gdk-sys` and related GUI dev dependencies are difficult to satisfy fully in the sandbox. The baseline `make test-rust` currently fails due to these preexisting codebase issues which block any attempt to verify new chat models or implementations cleanly.

  Preexisting compilation errors preventing implementation:
  ```
  error[E0425]: cannot find value `creds` in this scope
     --> src/server/integrations/registry.rs:942:25
  error[E0124]: field `google_workspace_clients` is already declared
    --> src/server/integrations/registry.rs:76:5
  error[E0062]: field `google_workspace_clients` specified more than once
     --> src/server/integrations/registry.rs:243:13
  ```

  As instructed, if the task requires a clean build or test verify, it cannot be completed if the preexisting codebase fails to compile. Reporting a blocked outcome.

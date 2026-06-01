# Known Build Issues

When checking the `main` branch, a large number (>900) of Rust compilation errors are present.

1.  **Missing Crates:** Code heavily references `ohc_builtin_agent`, `server_ohc`, and `server_auth` which are not defined in the `Cargo.toml` dependencies and lack their own `Cargo.toml` workspace member definitions.
2.  **Unresolved Types in `src/server/lib.rs`:** There are many `cannot find type X in this scope` and `cannot find struct, variant or union type X in this scope` errors in `src/server/lib.rs`, specifically relating to `HubService` implementation details (`PublishMessageResponse`, `AgentCapabilities`, `MeshEvent`, `EventStreamRequest`, `TeammateMeshEvent`, `InviteRequest`, `StartOnboardingRequest`, etc.).
3.  **Missing `Cargo.toml` Workspaces:** Bazel build files exist (e.g. `src/agents/builtin/BUILD.bazel`) indicating Bazel builds successfully resolve these, but `cargo check` fails due to a lack of `Cargo.toml` mappings. The Bazel build itself currently fails due to configuration/visibility errors.

Feature work (#19635) is proceeding in an isolated manner to bypass these global build failures.

4. **Integration Isolation:** `OperationsAgent` was successfully augmented to handle `tenant.fulfillment.failed` and trigger re-dispatch routes. The actual `execute` body is mocked as the broader tool execution framework has dependency issues as detailed above.

5. **Test State:** Global testing frameworks (`cargo test`, `bazel test //...`) also fail compilation due to the broken `server_ohc` / `ohc_builtin_agent` cross-dependencies that existed on the `main` branch prior to this feature implementation. Feature-specific test scaffolds have been placed in `src/server/services/fulfillment/service_test.rs` and `e2e/tests/fulfillment_mesh.spec.ts`.

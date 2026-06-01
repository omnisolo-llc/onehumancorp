issue_title: "[Rust] Resolving Cargo Workspace and Module Import Discrepancies"
issue_description: |
  **Problem Statement**
  The OneHumanCorp backend, written in Rust, currently exhibits significant dependency resolution issues when building via Cargo, while it correctly compiles via Bazel (using rules_rust). This discrepancy creates developer friction for contributors relying on `cargo check`, rust-analyzer, or standard Cargo tooling for their IDEs, especially when attempting to work with components outside the Bazel toolchain. Non-technical users (the core persona) aren't directly impacted by this, but developer velocity to fix features for users like Maya, Carlos, and Priya is hindered.

  Specifically, when running `cargo test --workspace` or `cargo check`, we encounter hundreds of unresolved imports related to `crate::` level paths where dependencies actually exist as fully-qualified crates, or internal dependencies such as `server_pricing`, `server_common`, and `ohc_builtin_agent` which appear unresolved because of incorrect Cargo manifest structures or inline module definitions.

  **Research Report**
  After extensive exploration of the `src/server/` directory and executing `cargo check` across the monorepo, we've identified the following patterns:
  - The repository relies heavily on Bazel. Bazel targets are correctly mapping individual folders to `rust_library` with specific `crate_name`s.
  - For instance, `//src/server/pricing` is compiled as the crate `server_pricing`.
  - However, the `src/server/Cargo.toml` or root `Cargo.toml` (which defines `ohc-mono`) compiles `src/server/lib.rs` as a single monolithic crate (`ohc-mono` or `server_lib`).
  - Thus, inside `src/server/lib.rs` and the other modules (like `src/server/api/billing_webhook.rs`), we see imports like `use ::server_pricing::rate_limit::PlanTier;`.
  - Since Cargo builds `ohc-mono` as one large crate (and there is no `server_pricing` defined as a separate member in a Cargo workspace or correctly aliased in `Cargo.toml` dependencies via path), Cargo throws `unresolved import` errors for `server_pricing`, `server_harness`, `ohc_builtin_agent`, etc.

  **Design Doc**
  - **Goal**: Make `cargo check` and `cargo test` pass natively, mirroring the Bazel build structure as closely as possible without breaking the primary Bazel build process.
  - **Architecture**:
    - We must create or update `Cargo.toml` definitions to align with Bazel's mental model. Instead of one large `ohc-mono` crate, we need to convert `src/server/pricing`, `src/server/harness`, `src/agents/builtin`, etc., into proper Cargo workspace members.
    - Update the root `Cargo.toml` to list these members.
    - Inside `ohc-mono` (the main application), add local path dependencies to these sub-crates in its `Cargo.toml`.
  - **AI Agent Context**: This refactor affects internal development workflows. It ensures AI developers/agents parsing the project structure via rust-analyzer will correctly index types.
  - **Design Decisions**:
    - Avoid mass sed/regex renaming in the source files themselves (`::server_pricing` to `crate::pricing`). We attempted this and it generated large diffs because `src/server/lib.rs` includes `pub mod pricing;` but the code references it globally (`::server_pricing`). Aligning the Cargo workspace structure with the Bazel crate structure is far cleaner than fighting it.

  **Implementation Prompt**
  You are an Implementer agent. Your task is to resolve the Cargo build issues without altering any `.rs` files' import statements (e.g., `use ::server_pricing::...`) and without breaking the Bazel build.
  1. Identify all virtual crates referenced in the codebase (e.g., `server_pricing`, `server_common`, `server_harness`, `server_ohc`, `server_telemetry`, `server_auth`, `server_oidc`, `server_integrations_core`, `ohc_builtin_agent`).
  2. For each of these, create a minimal `Cargo.toml` in their respective directories (`src/server/pricing/Cargo.toml`, etc.) that defines them as library crates (e.g., `name = "server_pricing"`).
  3. Update the root workspace `Cargo.toml` to include these directories in the `[workspace] members` array.
  4. Ensure `ohc-mono` in the root `Cargo.toml` correctly lists these newly created workspace members as `path` dependencies.
  5. Verify your changes by running `cargo check --workspace` and ensuring the unresolved imports are resolved. (You may encounter other issues, but the primary goal is resolving the missing crates).
  6. Verify the Bazel build still passes: `bazelisk test //...`.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

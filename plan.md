1.  **Implement NATS Integration Module:**
    *   Create `src/server/integrations/nats/provider.go`.
    *   Define `NatsIntegration` struct that implements `IntegrationProvider`.
    *   Implement `Metadata()` to return NATS metadata (Id, Name, Category="Event Mesh", etc.).
    *   Implement `WizardSteps()` to return the configuration fields (NATS URL, Credentials File Path).
    *   Create `src/server/integrations/nats/provider_test.go` with 100% coverage tests for `Metadata()` and `WizardSteps()`.
    *   Create `src/server/integrations/nats/BUILD.bazel`.

2.  **Register NATS Integration in Catalog:**
    *   Modify `src/server/integrations/catalog.go` to import `github.com/onehumancorp/mono/src/server/integrations/nats`.
    *   Add `&nats.NatsIntegration{}` to the `Catalog` variable.
    *   Update `src/server/integrations/BUILD.bazel` to include `//src/server/integrations/nats` in its dependencies.
    *   Run `bazelisk test //src/server/integrations/...` to verify.

3.  **Complete pre-commit steps:**
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
    *   Call `pre_commit_instructions`.

4.  **Submit the code:**
    *   Submit using the `submit` tool with a descriptive commit message.

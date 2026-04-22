1. **Implement Local Stateful Execution Proxy Integration**
    - Create `srcs/server/integrations/local_execution_proxy/proxy.go`.
    - Define `LocalStatefulExecutionProxyIntegration` which implements the `Integration` interface (`Metadata` and `WizardSteps`).
    - Define `ReverseTunnelClient` for mTLS connection (placeholder for full implementation).
    - Define `LocalExecutionMCPTool` for tool execution (placeholder for full implementation).
2. **Implement Tests for Local Stateful Execution Proxy Integration**
    - Create `srcs/server/integrations/local_execution_proxy/proxy_test.go`.
    - Add tests to cover `Metadata`, `WizardSteps`, `ReverseTunnelClient.Connect` and `LocalExecutionMCPTool.ExecuteCommand`.
3. **Register the Integration**
    - Modify `srcs/server/integrations/catalog.go` to import the new package and add `&local_execution_proxy.LocalStatefulExecutionProxyIntegration{}` to `Catalog`.
4. **Update BUILD.bazel Files**
    - Create `srcs/server/integrations/local_execution_proxy/BUILD.bazel`.
    - Modify `srcs/server/integrations/BUILD.bazel` to include `//srcs/server/integrations/local_execution_proxy` as a dependency.
5. **Run Tests**
    - Use `run_in_bash_session` to execute `bazelisk test //srcs/server/integrations/...` to ensure all tests pass.
6. **Pre-commit Steps**
    - Complete pre-commit steps to make sure proper testing, verifications, reviews and reflections are done.
7. **Final Output**
    - Output `issue_id: 5840` as requested.

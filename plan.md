1. **Explore & Analyze**: Understand the NATS integration requirement from the `docs/research/[backend]_nats_hybrid_event_mesh.md`.
2. **Implement**:
   - Create `NatsIntegration` provider struct in `src/server/integrations/nats/nats.go`. (Done)
   - Register it in `src/server/integrations/catalog.go`. (Done)
   - Register `IntegrationTypeNats` in `src/server/integrations/registry.go`. (Done)
3. **Verify**: Run `bazel test //src/server/integrations/...` to ensure all tests pass and the module integrates properly without cyclic dependencies. (Done)
4. **Pre-commit**: Check pre-commit steps using `pre_commit_instructions`.
5. **Submit**: Submit the PR with title `🚀 NATS: Hybrid Event Mesh Integration`.

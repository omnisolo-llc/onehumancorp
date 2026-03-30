# Test Plan: Capability Plugin Mesh & Aesthetics

1.  **Architectural Linting:** Verify DAG checks on dynamically registered capabilities.
2.  **State Persistence:** Ensure `capability_manifest` entries are correctly persisted to the `swarm_memory` table.
3.  **UI Verification:** Use Playwright to verify that the core Dashboard UI applies the required `backdrop-filter` and `background` CSS tokens, generating visual proof.

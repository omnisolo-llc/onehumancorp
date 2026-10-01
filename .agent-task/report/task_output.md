issue_title: "Blocked no-work finding: evaluate resource-based charging/customer-funded inference"
issue_description: |
  The owner requested to evaluate resource-based charging/customer-funded inference and address F05 (telemetry not invoice-grade). However, the existing infrastructure does not provide real workload cost accounting, measured deployment cost, representative workload distribution, or reconciled provider invoice. Building a generic ERP without this data contradicts the current evidence-first mandate.

  - We have reviewed the current `docs/research/business_capability_and_usage_economics_audit.md` and `docs/research/native_migration_and_remediation.md`.
  - The audit clearly points out that F05 requires "Platform compute/idle/support allocation, provider-invoice reconciliation and payment collection are not completed by this ledger."
  - Further, the audit instructs: "evaluate resource-based charging/customer-funded inference. Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research. Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection."
  - Building this architecture requires the missing owner workflow evidence and actual metrics before new code is warranted, marking this a blocked task as we cannot manufacture these prerequisites.

  **Verification Limitations:**
  - The test command `MALLOC_ARENA_MAX=2 cargo test -p server_harness --test provider_facade` failed (`facade_rejects_byok_api_if_tenant_key_absent_or_revoked`). This is an unrelated infrastructure issue where the `ConnectionVault` returns a `SERVICE_UNAVAILABLE` (503) error because `OMNISOLO_CONNECTION_KEYS` is absent or malformed in the test environment. The test failure is recorded as a trace limitation and the test suite is not claimed to pass.

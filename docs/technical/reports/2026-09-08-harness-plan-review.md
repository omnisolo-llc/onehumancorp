# Harness plan review and remediation — 2026-09-08

## Scope and status

Reviewed the latest committed implementation against the cross-harness model
routing plan (2026-08-23) and universal harness services plan (2026-08-30).
The repository contains other, older product and UI plans; this review does not
establish their completion. Existing unrelated documentation changes were preserved.

**The universal harness services plan is not complete.** Passing deterministic
tests establish the fixes below, not all of the design's acceptance criteria.

## Implemented corrections

- Local-service admission now checks registry descriptors, capability subsets,
  configuration digests, and consistent scope associations. Authorization rejects
  nil binding IDs and mismatched project, workspace, task, and attempt identities.
- The provider facade binds reasoning settings for both supported inference
  paths, rejects mismatches, disables upstream redirects, hides configured URLs
  in Debug, and cancels pending requests and response streams when revoked.
- Native OmniSolo runs retain validated service references in capsule exports.
  The native bridge propagates bindings at creation, execution, and capsule
  import, and uses the request's attempt ID when initializing a run.
  Invalid replacement bundles do not overwrite previously validated references.
- The live runner requires explicit opt-in, rejects empty and unknown selections,
  distinguishes incomplete native coverage, and prevents a selected shim group
  from bypassing native coverage. It retains structured test evidence instead of
  manufacturing successful usage, reasoning, and service-probe fields.
- Live evidence validation checks assistant text rather than echoed user input,
  requires positive numeric token usage, and rejects failed/cancelled events.
  Pi translation and the actual OpenHands downgrade field are recorded.
- Live service fixtures supply the required project identity. Failed test
  diagnostics remain visible on stderr with the upstream provider key redacted.

Each correction has a regression that failed before its implementation.

## Remaining implementation gaps

| Priority | Gap | Source / next required work |
| --- | --- | --- |
| High | Provider routes remain worker-wide. | `src/server/harness_worker/lib.rs` creates one facade from worker defaults. Move route creation and revocation to attempt admission/lifecycle, including request-selected models and cancellation/deletion. |
| High | Service bindings are descriptors, not live service authority. | `LocalServiceRegistry` has no issued-binding lease/generation/revocation store or operation gateway. Connect the scoped contract to existing memory, MCP, workspace, artifact, browser, cache, and integration implementations. |
| High | Namespace consistency is not membership authorization. | The gRPC request lacks a trusted project/workspace authorization context. Checking caller-provided bindings against one another cannot prove that the tenant may access that project/workspace. |
| High | The four shim names do not run the named CLIs. | `openai_compatible_shim.py` uses `HARNESS_ENTRYPOINTS` as an allowlist, then calls the provider directly. Implement pinned CLI invocation, bounded output, native failures, and process cleanup for each of the four entries. |
| High | Cross-harness service acceptance is unproven. | Existing conformance checks compare binding metadata. Add real writer/reader probes through adapters and configured backends, including tenant denial, MCP authorization, workspace snapshots, and browser lease isolation. |
| Medium | Facade policy remains incomplete. | Enforce admitted output limits, attempt/correlation identity, and provider-specific payload policy. Successful upstream bodies are still streamed without content redaction. |
| Medium | Capsule handoff has no fresh binding issuance. | Portable references survive export, but target admission must rebind them and reject stale generations/configuration without transferring live authority. |
| Medium | Deployment inventory overstates service readiness. | Reconcile compatibility claims with actual gateway operations and measured live evidence after the integrations above exist. |

## Verification

Executed on the combined remediation tree:

```text
cargo test -p server_harness -p omnisolo_harness_worker --tests
24 test suites: 428 passed, 0 failed, 3 ignored

python3 -m unittest discover -s scripts/tests -p 'test_live_harness_matrix.py'
7 passed

python3 -m unittest deploy.tests.openai_compatible_shim_test deploy.tests.kimi_acp_bridge_test
8 passed

bash deploy/tests/harness_worker_deployment_contract_test.sh
passed

bash -n scripts/test-live-harness-matrix.sh
passed
```

The ignored tests require live Codex authentication, an explicitly enabled real
harness worker, or the pinned OpenHarness SDK checkout. The live flags were unset
and the base pinned worker image was absent. No live provider matrix or image
build was performed; Docker daemon availability alone does not establish live
compatibility. Rust test output is saved at
`/tmp/harness-plan-completion-tests.log` for this workspace session.

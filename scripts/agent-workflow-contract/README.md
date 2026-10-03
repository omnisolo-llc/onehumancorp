# Agent workflow HTTP contracts

This bounded harness compiles complete, byte-exact production workflow and hire
HTTP handler bodies, request/response types and workflow envelope logic. The
production bearer middleware and auth Store create and authenticate two real
local tenant users. It does not compile the full server or certify deployment.

The actual admission, dispatch and lifecycle code runs with terminal text inference recorded instead of contacting a provider. Current signed membership and token revocation are checked through the production auth Store. The small Hub
fixture compiles the actual lookup and tenant-bound status update methods while preserving registration/list behavior required by these HTTP contracts;
it is not evidence of Redis, telemetry, or full Hub integration. No paid agent,
provider, workflow command, marketplace request, or live customer effect runs.

The tests must preserve genuine two-tenant HTTP effects and fail on leaked reads,
dropped task input, or an unacknowledged registration. Input manifests bind the
exact production source, local dependencies, and tests. Do not count this as
full make lint/test or provider verification.

Run from the repository root with the pinned Rust toolchain and cached locked
dependencies:

```sh
python3 scripts/focused_ci_gate.py agent-workflow-contract
```

The required PostgreSQL security CI job also runs this offline gate. The wrapper
requires at least 57 executed tests, rejects failures/skips/filters, and retains
the log plus source manifest. `make test` checks the mandatory CI registration
through the native-contract discovery guard; it does not claim that discovery is
runtime acceptance.

The configured tenant text adapter and real LLM client modules are compiled. Its
unit tests record the terminal LlmClient request, proving exact model/text and an
empty tool list without contacting a provider. The full server and genuine
provider execution still require their separate gates.

The raw RPC boundary tests compile the complete production JSON-RPC HTTP handler,
router constructor, authenticated proxy and exact main mount/outer middleware.
The terminal AppServer only records dispatch; provider/agent/tool paths panic.
A test-owned loopback connection closes without a response to exercise actual
proxy transport loss. These tests prove that anonymous/staff requests and an
unscoped global runtime cannot dispatch, not durable task-store isolation.
The original mounted anonymous/staff/global-fallback failures are preserved as
baseline evidence. The raw legacy endpoint is explicitly unavailable until a
tenant-bound runtime exists; the supported tenant text workflow API remains.

Three actual proxy regressions distinguish signed browser authentication from a configured agent runtime returning401/403, preserve current caller token validity and prohibit default self-proxying into the retired raw endpoint. An absent explicit runtime is503 before any request; agent authorization rejection is502/no-store. These are truthful error contracts and do not certify a functional legacy agent operation.

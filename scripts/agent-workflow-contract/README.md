# Agent workflow HTTP contracts

This bounded harness compiles complete, byte-exact production workflow and hire
HTTP handler bodies, request/response types and workflow envelope logic. The
production bearer middleware and auth Store create and authenticate two real
local tenant users. It does not compile the full server or certify deployment.

The terminal swarm dispatch is an explicit recording boundary. The small Hub
fixture preserves registration/list behavior required by these HTTP contracts;
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
requires at least nine executed tests, rejects failures/skips/filters, and retains
the log plus source manifest. `make test` checks the mandatory CI registration
through the native-contract discovery guard; it does not claim that discovery is
runtime acceptance.

# Explicit isolated Rust test execution

The ordinary workspace run keeps opt-in tests ignored because their external
prerequisites are not implicit. CI additionally runs fourteen exact test names
with `scripts/ignored_rust_gate.py run`: Redis disconnect, PostgreSQL/MySQL
migration replay, PostgreSQL mesh reconnect, three real pinned OpenCode process
tests, the real pinned OpenHarness SDK test, and six restored campaign cases.
The campaign's three ordinary cases are discovered by the normal native run.

Each selected case must emit its own successful named result and exactly one
passed test with zero failures or ignored cases. Filtering unrelated cases is
intentional for these exact selectors and never certifies those other tests.
Zero discovery, a wrong selector, process failure, missing prerequisites or
changed Rust inputs fail the gate. Per-case logs, the source manifest and the
aggregate result are retained as a required CI artifact.

CI owns disposable PostgreSQL17, MySQL8.4 and Redis7 containers with dynamic
loopback ports. `OHC_IGNORED_SERVICE_ISOLATION=1` is an explicit declaration of
that ownership; do not use it with an existing application/customer service.
The runner rejects remote service hosts and non-test database names. Campaign
fixtures create their own schemas and restricted roles inside this database.
The Redis regression deliberately kills this disposable instance's pubsub
clients, and the mesh regression terminates this database's LISTEN connections.

OpenCode is the existing npm-locked1.18.15 binary. The SDK checkout is the
existing pinned [AgentBoardTT OpenHarness revision](https://github.com/AgentBoardTT/openharness/tree/85c54682a209ca7c3fc8b1ab2e820b6724dc3028),
with no saved checkout credentials. These process/SDK tests use synthetic keys,
loopback provider replies or local fixture modules. The two OpenCode lifecycle
tests point to loopback even though they do not issue a prompt. Test child
environments do not inherit live-provider credentials or opt-ins.

The genuinely live Codex and harness-matrix tests are **not selected**. They
remain unexecuted until a bounded provider/account configuration, data scope and
cost authorization are established. A local fixture does not turn either live
test into a pass. The existing twenty-nine catalog/sync/tax bodies also execute
in separate source-bound PostgreSQL gates; report those executions separately
from the default workspace's ignored count and this fourteen-case inventory.

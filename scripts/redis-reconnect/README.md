# Redis queue and cache connection recovery

This lane compiles exact original `Job`, `TaskQueue`, and `RedisTaskQueue` source
items through the shared Rust parser, and the whole production cache module.
It preserves the existing protobuf dependency and wire payload. A build-time
source manifest records source and selection hashes.

Run `bash scripts/redis-reconnect/run.sh` with Redis server/CLI installed; the
script owns, PID-verifies, and cleans up its loopback process. Supplied service
URLs are rejected, including shared CI services. First run
`bash scripts/redis-reconnect/fetch.sh` to fetch the repository-locked dependencies.
The required CI command is `python3 scripts/focused_ci_gate.py redis-reconnect`;
it rejects fewer than 19 tests, failures, ignored tests, and filters. Source and
selection hashes are verified after execution against unchanged input snapshots. The
real service tests serialize connection termination and use unique keys; they
never flush shared data. They issue `CLIENT KILL` and must not target a shared
or production Redis service.

Separate loopback RESP fault fixtures model receipt loss and unavailable
connections, count mutation attempts, and retain the original failure outcome.
Recovery probes repeat only safe PING/GET commands. Enqueue, pop and
acknowledgement are not retried after an unknown outcome.

The adapter retains distinct command and blocking queue connections,
limits cache connection/response waits to 250ms each, queue connection/command
waits to 500ms each, and the blocking response wait to 1500ms (above the existing
one-second BLPOP). Pending reconnection wait and response wait are distinct;
these per-phase bounds are not a claimed end-to-end operation deadline.
The existing five-second aggregate startup deadline remains. A mutex serializes
BLPOP admission so uncancelled concurrent callers receive their own response
window; post-pop HSET uses the command socket. Cancellation releases admission
but cannot retract a submitted BLPOP, so a later caller can still queue behind
that command. Cancellation and response loss can leave a pop's outcome unknown.
No queue command is replayed.

A response timeout alone does not trigger replacement in redis 0.27.6. A
still-open, indefinitely unresponsive socket therefore continues to return
bounded failures until the transport closes; dropped sockets recover on later
calls. These are distinct properties in the tests.

This does not repair queue claim atomicity, role routing, durability, lease or
scheduling semantics. Pub/Sub subscriptions need explicit resubscription and
cannot recover missed invalidations merely by reconnecting. The invalidator
service explicitly reconnects and subscribes again, with a 500ms combined
connection/subscription budget and a one-second retry delay.
The subscription transport fixture compiles its complete original source,
including both existing unit tests. Unrelated SQL, HTTP and product regeneration
dependencies are inert and panic if invoked. The reconnect test publishes no
events: it observes NUMSUB, kills the subscription socket and observes NUMSUB
again. It does not certify external purges or distributed cache freshness.
Cache TTL/SWR, coalescing and tags are unchanged.

Redis 0.27.6 is already installed, licensed BSD-3-Clause, with MSRV 1.70. The
standalone server_utils crate must explicitly enable connection-manager.
The [pinned upstream implementation](https://github.com/redis-rs/redis-rs/blob/redis-0.27.6/redis/src/aio/connection_manager.rs)
returns the failed operation's error and reconnects for future operations. Its
connection-attempt retry policy does not replay commands. Full workspace
acceptance and dependency advisory reconciliation remain separate gates.

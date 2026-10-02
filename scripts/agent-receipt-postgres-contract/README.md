# PostgreSQL receipt parity checkpoint

This harness must use an explicitly owned loopback `ohc_*_test` PostgreSQL database.
It imports actual receipt persistence, authentication and portable schema code.
The PostgreSQL receipt fixtures are storage-only; reaching inference in them is a
test failure. The existing execution unit tests retain their test-only inference
fixtures; no external provider is contacted.
The PostgreSQL schema is additive migration 1022 and the runtime role is restricted
with forced row-level security. Every test must run; an absent database fails setup.

Verified focused acceptance (38 cases): persistent reopen, exact idempotent replay/concurrent admission,
one dispatch claim, terminal replay, foreign tenant/actor fencing, current identity
and token revocation after database waits, rollback on failed terminal commit,
expired-claim recovery without retry, immutable/transition enforcement and bounded
lock contention. Existing SQLite tests remain mandatory and unchanged.

This checkpoint does not mount HTTP/worker/provider dispatch or establish budgets.

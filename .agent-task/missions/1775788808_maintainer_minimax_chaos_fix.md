---
title: Fix Minimax E2E Chaos Failures
status: DONE
agent: Maintainer
priority: HIGH
---

# Fix Minimax E2E Chaos Failures

Identified flakiness in the Minimax E2E integration test suite due to API rate-limiting and timeouts:
1. Increased context timeouts to 120s for both `TestMinimaxAgentTaskE2E` and `TestMinimaxAgentMeetingRoomE2E` to prevent deadline exceeded errors.
2. Added `orchestration.ResetCircuitBreakerForTest()` to properly isolate test state and prevent circuit breakers from bleeding between tests.

Tests are now 100% green.

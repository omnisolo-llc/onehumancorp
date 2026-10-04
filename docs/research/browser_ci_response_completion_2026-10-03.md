# Browser response-completion remediation, 2026-10-03

## Observed source and CI evidence

Published head: `e90530e30d5d153f6ab1b5400c776bd7433c309c`. CI tested its merge snapshot `6b9cedb6123e1973547a9e44bfa42e5a64fb47ac` in [run 37149714079](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079).

Across all 12 completed Playwright jobs: **1,772 passed, 9 failed, 22 skipped**, 1,803 discovered. Shards 1–12 respectively reported pass/fail/skip: 147/0/4, 151/0/0, 151/0/0, 146/4/0, 150/0/0, 147/1/2, 141/2/7, 149/0/1, 144/0/6, 150/0/0, 150/0/0, 146/2/2. The 22 skipped cases include the existing 21 provider/runtime positive gates plus the separately configured recorded-text-analysis acceptance case. They remain OPEN and unrun, not certified by missing-runtime behavior.

- [Shard 4](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079/job/111281959371): four persisted-mutation isolation cases (`/unified-feed`, `/dashboard/unified-feed`, `/feed`, `/action-center`) reached actual HTTP 200 and then hung at `response.finished()`. The HTML step records confirm none reached the subsequent reload. Context finalization masked the original wait in the terminal error.
- [Shard 6](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079/job/111281959328): proposal unavailable-model response reached HTTP 502, but its browser body was unavailable to `response.text()`.
- [Shard 7](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079/job/111281959339): Agent Protocol creation reached HTTP 503 but `response.json()` did not complete; scaling's exact error-text assertion matched both the real alert and Next's empty route announcer.
- [Shard 12](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079/job/111281959329): Agent Protocol history likewise stalled reading the 503 body; the crawl recorded Goose's real missing-runtime 503 as an unexpected failure.
- [Click coverage](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37149714079/job/111286174572) downloaded all 12 receipts and correctly refused certification because the run did not finish successfully. This was not a missing-artifact diagnosis.

## Repairs and bounded evidence

The affected finite-response UI paths discarded unread bodies after receiving their headers. They now consume the actual body before disposing of each response. No browser request is intercepted, replaced or replayed; all existing HTTP status, response-body, persisted-state and post-reload assertions remain. Agent Protocol still retains input and holds unknown creation outcomes. Confirmed 401/403 rejections remain known rejections even when their body delivery fails. Checkpoint/step selection epochs and in-flight locks are unchanged.

The Goose UI distinguishes unverified/unavailable history from a successfully read empty list, removes obsolete extension controls when refreshing, checks the response status and validates extension records before offering execution. The smoke classifier recognizes only same-origin GET `/api/v1/agents/goose` with no query/fragment, HTTP 503 and the exact single-field missing-runtime error. Other origins, endpoints, methods, statuses, extra body fields and different errors remain failures. The browser crawl additionally requires visible unavailable state, no empty-history claim and no executable extension. This extends the existing SONA-specific missing-runtime treatment; it does not claim runtime configuration or a completed operation.

Scaling and checkpoint-history alerts are scoped to their required visible message while retaining exact text assertions. No test discovery, runtime gate or assertion was disabled.

## Verification

Pinned Node `22.22.1` was used for primary checks.

- Response-lifecycle regression RED: six failures, 25 passes; the six failures demonstrated unread real `Response` bodies. GREEN: 31/31.
- Goose regression RED: six failures, 19 passes, with the malformed-list case also producing the existing `extensions.map` exception. GREEN with hosted-voice/classifier regressions: 44/44.
- Auth rejection/body-failure edge regressions were observed failing before repair and then passed, preserving known rejection and stable user-visible wording.
- Final focused affected suites: **80/80 tests**, seven files, 6.83 seconds.
- Final-source complete Next suite: `npm --prefix src/ui/next test -- --maxWorkers=4` passed **3,415/3,415 tests across 497 files**, 176.15 seconds, exit 0. Source hashes were unchanged throughout that run.
- Root script suite: `npm run test:scripts` passed **950/950**, zero skips, 51.70 seconds, exit 0.
- `npm run typecheck:web`, `npm run typecheck:e2e` and `npm run lint:node` passed on final source; `git diff --check` passed.
- Complete browser discovery (`PLAYWRIGHT_TEST_DIR=./src`, Playwright `--list`) remains **1,803 tests in 461 files**.
- An earlier full-unit run was interrupted after it picked up newly added auth-body regressions during their RED stage. It is not used as final-source certification; the complete result above is the subsequent unchanged-source run.
- The source/test fingerprint for the browser patch, excluding this evidence document, is SHA-256 `9bb0e839df82e9b9f02927f52bb9204d5916c9b12bd9cfc1b92fc2b6ee8c7d11` over sorted path-to-SHA256 JSON.

**Remaining verification:** these are unit/source-level repairs pending newly built real-stack CI. The isolated worktree has no final-source native binaries or production web artifact. Local Chromium launch was attempted both normally and through the available escalation but was blocked by platform socket permissions. No local browser pass is claimed. Full `make lint`/`make test`, final click coverage and all configured positive provider/runtime/text-analysis cases remain governed by their actual results and prerequisites.

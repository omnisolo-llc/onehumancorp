# Recorded location data and escalation boundaries

The location dashboard must display actual tenant-scoped staff, task and shift-summary records. Empty successful reads remain empty. Authentication, storage, transport and malformed-response failures remain explicit errors; none establish Alice, sample inventory work, invented attendance or a customer complaint.

The current repair preserves task titles, real summaries and manual draft edits. Cancellation and newer requests fence late draft responses. Failed or unconfirmed escalation delivery preserves the draft and recorded summary. A future genuine delivery receipt must be durable and owner/tenant-bound before the page can acknowledge delivery. The existing unavailable delivery endpoint is an open capability, not a completed feature.

The three mounted staff read handlers return storage errors instead of successful empty arrays. Additive migration1027 brings the unchanged historical staff/timecard schema into the active native PostgreSQL migration path, then forces owner row-level security. Earlier SQL files and recorded rows are preserved.

## Deliberate demo retirement

The mounted staff simulate-event and generate-summary routes inserted simulated events or a fixed fabricated AI summary. Searches across all tracked application, CLI, docs and MCP/tool sources found no maintained callers or advertised contract; current staff-manager source tests explicitly exclude both operations. The two handlers/routes are retired. Real staff creation, task, timecard, summary, shift and escalation read routes remain; existing records are not deleted.

## Verification boundaries and remaining work

- Focused UI/proxy regressions cover real records, verified emptiness, actual errors, cancellation and retained manual edits.
- The staff-read native harness links the unchanged three production handlers and their actual tenant helper to real SQLite/PostgreSQL pools. Claims injection and its small database container are explicit test boundaries; it is not complete-server authentication proof.
- The native suite requires a disposable loopback PostgreSQL database and source fingerprints. Full-server and actual browser acceptance remain separate requirements.
- Staff creation still needs a genuine durable invitation contract instead of an unstored invite token. Timecard synchronization still needs transaction/error/idempotency repair. Those are open follow-ups, not certified by the read-only tests.
- Genuine escalation drafting/delivery, atomic authorization and provider admission remain unfinished; no live provider or customer call is used in these checks.

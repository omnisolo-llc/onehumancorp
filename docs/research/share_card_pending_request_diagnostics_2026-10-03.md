# Share-card startup request evidence

## Exact hosted failure

On source `cda9b2e68ffaad0728ede398bd4b618359c41a0d`, run
[37161136150](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37161136150),
[shard 3](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37161136150/job/111316304075)
finished 150 passed / 1 failed. The failure was the comprehensive `/share-card`
click audit, before discovery, at `dashboard_audit_fixture.ts`'s five-second
request-settling assertion. [Shard 4](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37161136150/job/111316304064)
also failed the share-card Back/Skip isolation case at the same boundary.

The shard-3 artifact shows the full document redirect reached onboarding, its
initial draft/state response waits returned, and Upload Image was visible.
The fixture then observed a nonempty set of same-origin fetch/XHR requests for
five seconds. Its screenshot showed the conversational setup view. The receipt
contains no discovered targets or click observations. This is distinct from the
previous document-replacement failure.

The response waits returning does **not** establish successful body completion:
the existing initial-read loop does not inspect `response.finished()`'s returned
Error, and onboarding catches individual restoration/parse failures. Neither
artifact contains a network trace, HAR, or pending-request URL. The specific
stranded request and root cause therefore remain **unidentified**.

## Bounded diagnostic checkpoint

The fixture now records the existing same-origin fetch/XHR lifecycle in a helper.
It preserves exactly the previous inclusion rule and retires requests only on
actual `requestfinished` or `requestfailed` events. Receiving headers never
satisfies the settling requirement. Every pending request contributes to the
count even when the diagnostic report is truncated.

On the unchanged failure boundary, the error contains at most twenty pending
and twenty recently settled records, plus the total and omitted pending counts.
Records include a sequence ID, method, sanitized path, initiating document path,
resource type, start time/age, response-header arrival, status, MIME type and
completion/failure state. A boolean identifies Next RSC query presence.

No query values, fragments, URL credentials, request headers, response headers
other than the validated MIME type, cookies, request bodies or response bodies
are retained. Only explicitly known static route paths and prefixes are retained. Every
unknown route or suffix is redacted, including short opaque reset/device tokens
and ordinary-looking user identifiers. Failure text is limited
to Chromium's `net::ERR_*` codes or a generic transport failure. The observer
never reads or cancels a body. Raw tracing is not enabled because it would
persist authentication-bearing request material; this checkpoint supplies the
narrow lifecycle evidence needed at the failing assertion instead.

The five-second deadline, owner authentication, database isolation, required
initial reads, rendered readiness and exact click-inventory assertions remain
unchanged. This is diagnostic instrumentation, **not a claimed production fix**.

## Source tracing and next decision

The first share-card document mounts the application shell before calling
`location.replace`. Reachable requests include help, videos, tooltips, session
identity and Next Link prefetches; onboarding then mounts those again and reads
its draft/state. No intentional streaming API starts at this boundary.

The source has possible abandoned-body paths for unsuccessful tooltip, help,
video and identity replies, and stale onboarding responses. Next prefetch also
has unsuccessful-response paths. None is attributed to this failure without the
missing request evidence. A normal canceled request would already retire from
the pending set. The next failed hosted attempt must identify the request,
initial document, header/body phase and terminal events before selecting a
production repair. A passing attempt alone does not identify the previous cause.

Local Chromium still aborts during startup because the execution environment
rejects its singleton socket with `Operation not permitted`; no local browser
pass is claimed. No central checkout, publication, timeout, response policy or
production onboarding behavior was changed by this checkpoint.

## Verification

Diagnostic RED: six new evidence assertions failed while two existing lifecycle
semantics checks passed against the extracted original tracker. A second RED run added four short opaque-token/path-identifier cases, which
failed against heuristic redaction. After switching to explicit static route
templates, all twelve diagnostics tests and 27 existing navigation/onboarding
fixture checks passed (39 total). Changed-file ESLint and a serialized E2E TypeScript check (Node 22.22.1,
1.5 GiB heap cap) passed. Browser discovery remains 1,808 cases in 462 files.
The code fingerprint was unchanged through final checks. Real-browser
verification remains pending a new hosted run.

An attempted full Next suite and root script suite were stopped with exit 130
during shared memory pressure and are incomplete. Earlier aggregate TypeScript
and ESLint processes were killed with exit 137 before the successful bounded
checks. The incomplete Next log contains six unclassified failures:

- website-builder/owner.safety: restores a storefront draft only on returning
  to its verified owner; holds an empty local builder when remote restore is network
- agents/page.execution: an owner switch clears private input and fences the
  late response body; a stored acknowledgement is never used as a fabricated
  fresh acceptance receipt
- widget-clipboard-feedback: interactive demo stays pending until copying succeeds
- assistant/page: malformed successful tour mixed valid and invalid steps is
  unavailable rather than absent

Observed durations were 11–129 seconds under pressure. These were not reproduced
in a healthy execution and are not classified as source regressions; the exact
published UI source has a passing hosted Node job. No full-suite local success
is claimed, and those cases were not changed or suppressed.

For the preceding approval-cache repair, this same hosted cda9 run independently
passed all ten real decision-handler tests in
[Native Rust job 111314950475](https://github.com/omnisolo-llc/onehumancorp/actions/runs/37161136150/job/111314950475)
and the strengthened action-center persistence browser case in shard 4. That
verified predecessor result does not certify this new diagnostic checkpoint.

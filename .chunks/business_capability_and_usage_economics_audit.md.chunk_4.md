ls

`services/billing/service.rs:49-85` uses global cost/token/agent snapshots while labeling the response with the requested organization. The service is registered in `lib.rs:9828,9856`. Authentication at the service boundary does not turn global totals into tenant-specific totals. Before exposing or billing from this response, test two organizations with different usage and verify no aggregate crosses their boundary.

### C. A budget monitor is not a hard spending reservation

`pricing/budget.rs:49-84` increments spend first and reports `false` after exceeding the limit; its test describes a soft limit. This helper alone does not atomically reserve funds before a provider request. It does not establish the behavior of every other limiter. Customer-funded usage needs tested concurrent reserve/settle/release behavior, bounded in-flight exposure, cancellation and restart recovery. Round at settlement with sufficient sub-cent precision, not by discarding each tiny request.

### D. Usage attribution differs across model paths

`services/billing/auditor.rs:8-16` lacks provider/model, provider request ID, payer/auth mode and rate-card revision. Its totals are process-local maps using a shared cost config. Conversely, `harness/middleware/inference.rs:27-36` already returns richer provider usage and binding data, and Codex has native usage events. Preserve those assets.

The proposal LLM adapter invokes a model but returns `Usage::default()` (`api/proposals.rs:90-115`). The separate local-model path returns only text (`minimax.rs:561-650`). The proxy forwards streamed bytes without settlement in the inspected function (`provider_facade.rs:341-409`). Do not infer that every model call is metered because one runtime reports usage. Inventory each call path, including embeddings, research, background summaries and verification models.

DB telemetry is useful observability, but counters or manually reported costs are not necessarily billable facts. `/billing/report-cost` accep
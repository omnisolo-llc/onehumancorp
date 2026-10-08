ost dashboard, checkout/portal/cancellation and reporting routes exist (`api/billing_api.rs:79-108`). DB telemetry and cost aggregation coexist with process counters. | This is not yet demonstrated to be an authoritative, reconciled usage-billing system. Avoid the inaccurate claim that there is no cost persistence anywhere. |
| Desktop and web assets | Tauri config uses `frontendDist: ./next_out`; `src/ui/tauri/BUILD.bazel:24-48` consumes exported dashboard/builder HTML. | Tauri is the desktop shell, but exported Next assets remain build inputs. The earlier instruction to disregard Next solely because it is called legacy was too strong. Trace actual loaded assets before UI changes. |
| Journey tests | `src/e2e/full_journey_e2e.spec.ts:1-7` delegates to `currentAppSmoke`; the previously inspected autonomous-ops test does likewise. | Those named tests alone do not prove inquiry, accepted work, provider payment and reconciliation. Other tests exist; their coverage must be mapped, not presumed absent. |

## 3. Findings that block trustworthy metered billing

### A. Usage accounting can feed itself

`src/server/hub.rs:61-73` creates an unbounded telemetry channel, installs its sender on the CostAuditor, and calls `record_event` whenever the receiver gets an event. `services/billing/auditor.rs:199-201` sends that event to the same channel again. Nonzero-token events therefore have a source-visible feedback path:

```text
record_event -> telemetry queue -> Hub receiver -> record_event -> telemetry queue ...
```

This can repeatedly account for the same usage and emit/write telemetry. It was not reproduced in a running server during this review. It is not appropriate to call this exponential queue growth; the problem is unbounded repeated processing per input event. Separate ingestion, accounting and metric export; require a stable event identity and a test showing one accepted event is accounted once and the queue drains.

### B. Organization cost summary reads global tota
# OneHumanCorp: capability, owner-needs and usage-economics audit

**Review date:** 2026-09-18 (America/Los_Angeles). **Revision:** 2026-09-18-usage-audit.

**Purpose:** establish what the product actually contains, what owners describe needing, how current AI business products overlap, and what metered compute/API or customer-funded inference would require. This is an evidence review, not a new implementation roadmap or a published price list.

The previous $99 subscription, 300-step allowance, $299 setup, fixed pilot conversion/margin thresholds and exclusive web/design/marketing segment are **suspended hypotheses**, not accepted requirements. Preserve useful earlier findings and OHC target IDs, but do not dispatch work from those assumptions. The user now asks us to evaluate compute/API charging and BYOK, including provider-permitted subscription access, before committing to a plan.

## 1. Scope and evidence discipline

Source baseline: `f8e9d8dd5c099f417df0c32f6131e9b465e5fb20`, branch `fix/bazel-modernization-and-cleanup`. Existing documentation changes were preserved. This review inspected selected critical implementation paths, route registration, billing, provider access, business workflows, UI packaging and test definitions. It did not inspect every source file or exercise the full running product. Line numbers below refer to that source baseline.

Use these distinct labels: **source present**, **mounted route/caller traced**, **unit/integration tested**, **provider-sandbox verified**, **real owner verified**. A class name, screenshot, generated link or smoke-test title cannot establish the last three. No new interviews, owner identity verification, payment commitments, provider charges or production measurements were obtained.

The attempted existing check was:

```sh
bazel test //src/server/pricing:server_pricing_unit_test --test_filter=budget --test_output=errors --jobs=4
```

It **timed out after 180 seconds during Bazel analysis**, before test results were available. The invocation was cancelled. This is neither a passing test nor evidence that a test assertion failed. Findings below are static source findings unless explicitly stated otherwise. No runtime fixes were made during this review.

## 2. Existing capabilities and actual gaps

| Area | What exists and evidence | Gap or unverified part; reuse implication |
|---|---|---|
| Business setup | `src/server/api/onboarding/mod.rs:26-54` defines intake/chat, saved state, draft, launch and setup-health routes with owner/admin authentication. Mounted in `src/server/lib.rs:9201`. | Setup endpoints exist; successful provider connection and an owner-ready operating business are not established. Reuse the state flow rather than inventing another wizard. |
| Customer proposals and project scope | `api/proposals.rs:16-40,120-140` has tenant/customer records, cent amounts, deposits, scope, milestones, draft/intake and approval routes; mounted at `lib.rs:9263`. | `/intake` ignores the inquiry when constructing fixed scope and price (`proposals.rs:815-845`). A milestone schema is not verified service fulfillment. |
| Calendars and Google tools | `integrations/google_calendar/client.rs:120-180` makes authenticated free/busy and event requests. `integrations/google_workspace/provider.rs:48-120` exposes Drive, Sheets and Gmail operations through its client. | Existing connector code is a real reuse asset. End-user authorization, secure credential lifecycle, refresh/revocation and complete UI-to-provider flows need separate proof. |
| Customer connection UI/API | `api/tool_integrations.rs:121-170,240-266`, mounted at `lib.rs:8592`. | Connect deliberately returns HTTP 501 because verification/encrypted storage are unavailable. Listed integrations have `usable: false`. This surface is incomplete but fails honestly; do not replace the failure with a fake connected badge. Other directly configured integration paths must be checked separately. |
| Invoicing and customer payments | Invoice persistence and line items exist. HTTP creation calls `InvoiceServiceImpl::create_invoice` (`api/invoice.rs:613-655`), mounted at `lib.rs:9260`. Proposal approval calls `StripeClient::create_checkout_session` (`proposals.rs:397-426`). | Invoice creation separately generates a checkout-looking UUID URL without creating a provider session (`invoice.rs:45-48`). Real Stripe plumbing and placeholders coexist; consolidate and verify rather than claiming all payments are fake or all are ready. |
| Commerce and fulfillment | Builder, catalog, checkout, cart, storefront and fulfillment routers are mounted (`lib.rs:9192-9229`). `api/fulfillment.rs:16-34,83-109` includes shipping/pickup states, provider references and persisted tracking updates. | The codebase is not exclusively a digital-agency app. Reservation, shipping exceptions, stock, payment and owner workflows require live proof. A broad commerce rewrite is not justified by directory names. |
| Agent coordination and approvals | Department/event/memory coordination exists (`orchestration/departments/orchestrator.rs:148-220`); approvals/activity/ledger routes exist (`api/agents/approvals.rs:130-168`). | Several simulation routes remain in the same API. Verify that real execution, persistent approval, cancellation and result evidence flow through the actual production path; do not count simulated feed items as customer outcomes. |
| Provider runtime | Scoped/revocable proxy, output bounds and secret redaction exist (`harness/middleware/provider_facade.rs:295-409`). Responses client carries model, response ID, usage and binding information (`inference.rs:27-36`). Codex codec captures usage notifications (`codex_app_server.rs:554-558`). | Do not build another generic provider abstraction first. Extend the existing paths with reliable accounting and clearly defined payer/auth modes. Subscription login, authorized embedding and customer billing are not proved by an adapter. |
| Cost and plan UI | `/billing/my-plan`, cost dashboard, checkout/portal/cancellation and reporting routes exist (`api/billing_api.rs:79-108`). DB telemetry and cost aggregation coexist with process counters. | This is not yet demonstrated to be an authoritative, reconciled usage-billing system. Avoid the inaccurate claim that there is no cost persistence anywhere. |
| Desktop and web assets | Tauri config uses `frontendDist: ./next_out`; `src/ui/tauri/BUILD.bazel:24-48` consumes exported dashboard/builder HTML. | Tauri is the desktop shell, but exported Next assets remain build inputs. The earlier instruction to disregard Next solely because it is called legacy was too strong. Trace actual loaded assets before UI changes. |
| Journey tests | `src/e2e/full_journey_e2e.spec.ts:1-7` delegates to `currentAppSmoke`; the previously inspected autonomous-ops test does likewise. | Those named tests alone do not prove inquiry, accepted work, provider payment and reconciliation. Other tests exist; their coverage must be mapped, not presumed absent. |

## 3. Findings that block trustworthy metered billing

### A. Usage accounting can feed itself

`src/server/hub.rs:61-73` creates an unbounded telemetry channel, installs its sender on the CostAuditor, and calls `record_event` whenever the receiver gets an event. `services/billing/auditor.rs:199-201` sends that event to the same channel again. Nonzero-token events therefore have a source-visible feedback path:

```text
record_event -> telemetry queue -> Hub receiver -> record_event -> telemetry queue ...
```

This can repeatedly account for the same usage and emit/write telemetry. It was not reproduced in a running server during this review. It is not appropriate to call this exponential queue growth; the problem is unbounded repeated processing per input event. Separate ingestion, accounting and metric export; require a stable event identity and a test showing one accepted event is accounted once and the queue drains.

### B. Organization cost summary reads global totals

`services/billing/service.rs:49-85` uses global cost/token/agent snapshots while labeling the response with the requested organization. The service is registered in `lib.rs:9828,9856`. Authentication at the service boundary does not turn global totals into tenant-specific totals. Before exposing or billing from this response, test two organizations with different usage and verify no aggregate crosses their boundary.

### C. A budget monitor is not a hard spending reservation

`pricing/budget.rs:49-84` increments spend first and reports `false` after exceeding the limit; its test describes a soft limit. This helper alone does not atomically reserve funds before a provider request. It does not establish the behavior of every other limiter. Customer-funded usage needs tested concurrent reserve/settle/release behavior, bounded in-flight exposure, cancellation and restart recovery. Round at settlement with sufficient sub-cent precision, not by discarding each tiny request.

### D. Usage attribution differs across model paths

`services/billing/auditor.rs:8-16` lacks provider/model, provider request ID, payer/auth mode and rate-card revision. Its totals are process-local maps using a shared cost config. Conversely, `harness/middleware/inference.rs:27-36` already returns richer provider usage and binding data, and Codex has native usage events. Preserve those assets.

The proposal LLM adapter invokes a model but returns `Usage::default()` (`api/proposals.rs:90-115`). The separate local-model path returns only text (`minimax.rs:561-650`). The proxy forwards streamed bytes without settlement in the inspected function (`provider_facade.rs:341-409`). Do not infer that every model call is metered because one runtime reports usage. Inventory each call path, including embeddings, research, background summaries and verification models.

DB telemetry is useful observability, but counters or manually reported costs are not necessarily billable facts. `/billing/report-cost` accepts metric reports (`billing_api.rs:117-171`); it must not become the billing authority without trusted producer identity, replay protection and reconciliation. Missing usage means unknown/pending, not free and not permission to guess a debit.

### E. Commercial output can still be a placeholder

The mounted proposal intake writes a fixed $5,000 proposal and $2,500 deposit with website-redesign scope, independent of the submitted inquiry (`proposals.rs:825-845`). The invoice path persists an invented checkout URL. These are concrete gaps in existing features, not reasons to commission a new proposal or payment subsystem. Earlier research also identified reminder logic that logs drafting before implementing it; verify each completion claim against the actual artifact/provider state.

**Consequences:** current cost dashboards should not be treated as invoice-grade meters, and the presence of a business screen should not be advertised as an autonomously completed process. None of these defects was repaired by this document.

## 4. What online owner accounts reveal

This is purposive qualitative reading, not a random survey, authenticated interview study or measurement of which need is statistically most common. Forum publication times were not reliably established from the returned relative timestamps; record access date rather than invent exact dates. Examples, monetary amounts and business identities are self-reported. Comments are distinguished from original posts. Vendor customer stories are curated.

| Source | Reported situation | Our inference to validate against OHC |
|---|---|---|
| [Owner story O1](https://www.reddit.com/r/smallbusiness/comments/1w33g2r/customer_ordered_a_product_paid_deposit_but/) | A container-conversion business received a deposit, finished the unit and could not collect the balance; it occupied shop space while cash was needed. | Track contractual/payment state, correct contact, reminders, delivery holds and cash exceptions. Drafting another invoice is not the whole job. Do not adopt commenters' legal advice as a product rule. |
| [Owner story O2](https://www.reddit.com/r/smallbusiness/comments/1wcdbr1/how_do_you_handle_email_marketing_for_a_small/) | A solo ecommerce operator lacked time and budget for email marketing. Replies favored manageable routines and relevant repeat-purchase messages. | Prefer low-maintenance recurring work with clear outcomes over campaigns that need daily supervision. The comments are not measured conversion evidence. |
| [Owner story O3](https://www.reddit.com/r/smallbusiness/comments/1ujiq5a/anyone_else_tired_of_marketing_advice_that/) | The author described being overwhelmed by advice to simultaneously produce frequent videos, funnels, ads and SEO without a team. | Reduce the owner's workload; do not translate every business goal into a larger agent-generated to-do list. |
| [Owner story O4](https://www.reddit.com/r/smallbusiness/comments/1wcxrvr/should_i_open_a_retail_store/) | A candle maker was weighing a retail lease against ecommerce/production needs and discussed product materials and selling prices. | Source-linked scenarios must distinguish material margin from profit after labor, premises, fees and overhead. Do not repeat a self-described profit percentage as audited profit. |
| [Owner story O5](https://www.reddit.com/r/smallbusiness/comments/1w7nlh1/local_print_business_retiring_offered_me_the/) | A potential print-shop buyer asked about financial and operational diligence. A commenter highlighted repeat orders whose details may live in the retiring owner's memory. | Durable customer/job/specification history and migration matter. The tacit-knowledge example is a commenter's risk scenario, not a verified fact about that shop. |
| [Owner story O6](https://www.reddit.com/r/smallbusiness/comments/1wbtu07/did_business_and_revenue_fall_of_a_cliff_in_august/) | An agency described leads that were not converting into bookings; other industries in the replies reported different experiences. | Measure qualification and conversion separately from lead volume; do not infer an economy-wide trend or respond automatically with more marketing output. |
| [Anthropic workshop account O7](https://claude.com/blog/what-1-000-small-business-owners-taught-us-about-ai), 2026-09-10 | A painting contractor challenged an AI estimate that used floor area rather than the required wall measurements. | Auditable inputs and deterministic domain calculations matter more than fluent confidence. The vendor's workshop sample skewed to 5–50-person firms, not solo owners; it does not validate our first buyer. |
| [Google owner gallery O8](https://workspace.google.com/ai/customers/) | Google's written vignettes feature Delgado Guitars handling inquiries, The Woobles examining inventory run rates and The Village Store examining supply costs. | Needs extend beyond digital services. These are vendor-selected descriptions, not independently verified results or videos watched in this review. |

Another parcel-loss thread was examined but contained authenticity disputes; it was not used as core evidence. No forum posts or outreach were made.

**Provisional synthesis:** collect cash and avoid lost business; connect the data owners already have; carry out repeatable administration without creating supervision work; preserve customer context; and make consequential calculations/actions reviewable. These are candidate problem clusters, not a validated market ranking. The current evidence does not justify making web/design agencies the only eligible segment.

## 5. What ChatGPT, Gemini and Claude already do

Product observations checked 2026-09-18; vendor pages describe offered behavior, not independently measured superiority. Current overlap is much greater than “chatbots answer, OHC acts.”

| Current primary source | Capability observation | Implication for OHC |
|---|---|---|
| [ChatGPT Work](https://openai.com/chatgpt-work/) | Uses files, connected tools and desktop apps to create work products; supports plugins and recurring tasks. | Documents, cross-tool actions and scheduling are competitive baseline capabilities. Compare a real OHC workflow against Work rather than old chat-only behavior. |
| [Gemini in Workspace](https://workspace.google.com/solutions/ai/) and [Workspace Studio](https://workspace.google.com/studio/) | Assistance within the productivity suite, source-grounded research and no-code flows with connectors/custom extensions. | Owners already working in Gmail/Docs/Sheets may favor improving their existing workspace. Another generic assistant or flow builder needs a concrete advantage. |
| [Claude SMB update](https://claude.com/blog/claude-for-small-business-launches-new-workflows-integrations-and-training-programs), **2026-09-15** | Announces 43 workflows and 27 new integrations, including commerce/finance tools; covers reporting, inquiries and proposals. Its Mothership Coffee example describes fragmented operational data brought together for reporting. | The earlier May-only comparison is stale. Source-linked reporting and lead-to-proposal workflows overlap directly with OHC's proposed value. This is vendor evidence, not proof of OHC demand. |
| [Cowork/Claude interface update](https://claude.com/blog/cowork-is-now-claude), **2026-09-16** | Announces convergence of chat and task execution plus continuing/scheduled work. Initial rollout is Pro/Max, with other plan timing differentiated. | Do not claim all plans have identical availability, or that competitors cannot persist work beyond a conversation. |

**Hypothesis worth testing:** OHC is an owner-controlled business operations workspace and execution environment that keeps durable records and business rules, works with the owner's chosen permitted AI provider, verifies effects and makes total costs understandable. Provider choice alone is not established willingness to pay. A provider-native plugin/connector to OHC and an OHC-controlled runtime are alternative distribution patterns, not two new products we must immediately build.

## 6. BYOK, subscription access and who pays

Do not use “BYOK” to imply that a consumer subscription supplies a general API credential. Technical support, commercial permission, privacy terms, quotas and owner authorization must all be checked for the exact mode.

| Mode under evaluation | AI provider payer | OHC charging basis / unresolved question |
|---|---|---|
| Managed API | OHC's contracted API account | OHC compute/resources plus explicitly priced API consumption, subject to the provider's commercial terms. Do not assume permission to resell bare access or pool consumer subscriptions. |
| Customer API key / cloud account | Customer directly | OHC hosting, storage, tools and other own costs; do not rebill customer-paid inference as OHC consumption. Inventory any embeddings/judges/background calls that still use an OHC-funded key. |
| Provider-native client with an eligible subscription | Customer subscription/credits | Potentially OHC execution-environment resources, only within provider-permitted integration/auth patterns. Quota is not unlimited autonomous capacity and a token is not transferable general API access. |
| Customer machine / local model | Customer hardware and applicable provider | OHC only charges for actual optional cloud services it provides. An owner-machine workflow cannot promise always-on hosted execution while that machine is offline. |

### Provider-specific evidence

**OpenAI:** [Business overview](https://help.openai.com/en/articles/8792828-chatgpt-business-overview) distinguishes Business from separately billed API usage. [Codex authentication](https://learn.chatgpt.com/docs/auth) supports ChatGPT subscription or API-key login; recommends API keys for programmatic CLI work; and documents Enterprise access tokens for trusted private Codex automation, not general API calls. This establishes different access modes, not approval for arbitrary hosted subscription brokering.

**Anthropic:** [Claude Code legal/auth guidance](https://code.claude.com/docs/en/legal-and-compliance) permits hosting the **unmodified** Claude Code binary under stated conditions, including users authenticating and paying directly. It prohibits a third-party Claude.ai login/token relay and end-user usage resale/intermediation in that arrangement. An OHC-owned Agent SDK/API application needs API/cloud authentication. These are distinct architectures; confirm the proposed deployment against the full terms rather than treating consumer credentials as BYOK API keys.

**Google:** [Gemini CLI quotas](https://geminicli.com/docs/resources/quota-and-pricing/) distinguishes eligible Google AI/Code Assist login allowances from API-key and Vertex billing. A generic Workspace subscription is not automatically a Gemini API entitlement. [Gemini API billing](https://ai.google.dev/gemini-api/docs/billing) ties API keys to projects and Cloud Billing; token classes and cache storage can have separate costs. Verify current plan eligibility and embedding terms, and never silently switch an exhausted subscription session to a paid OHC API key.

This is product due diligence, not a legal opinion. No supported native-client subscription mode was demonstrated end-to-end in OHC. The existing Codex adapter and API/OAuth credential types are reuse assets, not that demonstration.

## 7. Cost accounting before a price card

The current source does not supply a measured deployment cost, representative workload distribution or reconciled provider invoice. Do not replace the former invented monthly budgets with invented per-token or per-compute prices.

A proposed internal cost model is:

```text
OHC serving cost = OHC-funded model/tool usage
                 + allocated CPU, memory and GPU resources
                 + databases, storage, queues, backups and logs
                 + browser/runtime capacity and network egress
                 + paid external tools and communication
                 + payment collection, support and incident handling
                 + attributable idle/shared capacity
```

Customer-paid BYOK inference is tracked for owner visibility but excluded from OHC's provider expense and from an OHC inference debit. Keep OHC revenue, customer business revenue and money the customer pays directly to a provider separate. Also separate variable contribution from fixed engineering/admin/acquisition cost and company profit.

Define compute precisely: active CPU-seconds, provisioned memory-time, GPU-time or reserved sandbox-time are different resources. A worker waiting on an API may use little CPU but still reserve memory and a browser. Avoid double-charging overlapping bundles; disclose whether waiting/reserved time is billable. Compare shared workers and idle suspension using actual workloads, without weakening tenant isolation.

A 20% markup is not a 20% margin: an illustrative $10 cost sold at $12 yields $2, or **16.7% of revenue before other costs**. Small compute bills may not fund support. A minimum prepaid balance, explicit reserved-capacity charge or separately priced support could address this, but no particular fee is selected here. Test owner preference and unit economics rather than quietly reinstating the old subscription.

### Minimum evidence required for usage billing

Capture durable, deduplicated events with tenant/project/task/attempt and provider request IDs; payer and auth mode; provider/model; actual input/output/cache or tool quantities; resource unit and measured interval; rate-card version; estimated/reserved/settled/refunded state; and external reconciliation reference. API balances, subscription quota and customer cash are not interchangeable units.

Use integer subunits or decimal arithmetic with sufficient precision, immutable adjustments rather than silent history edits, trusted producers and tenant-specific reads. Reserve a conservative ceiling before starting new paid work, bound concurrent work, settle actual usage, and reconcile incomplete streams or unknown outcomes. A local cap can bound OHC-originated work, not all other usage of the customer's shared API account. Provider reporting can lag; do not advertise exact global remaining funds from local counters alone.

Retries and unsuccessful work can still consume resources. Decide and disclose the distinction between customer-requested attempts, provider-billed usage and retries caused by OHC defects. Record all economic cost even when OHC credits/refunds the customer's bill. A failure must not disappear from the cost denominator or become an invented successful outcome.

The owner should see **estimated task cost and maximum authorized spend**, then an itemized OHC bill, separate customer-direct provider usage, and clear unknown or pending reconciliation. Internal token complexity can be hidden behind plain-language totals without hiding the actual charge basis.

## 8. Evidence needed before a concrete implementation plan

These are decision prerequisites, not an approved feature backlog:

| Evidence question | Reuse / gap to investigate | Decision unlocked |
|---|---|---|
| Can one imported business produce a source-linked operational brief? | Existing Google clients, customer/finance data and agent feed; connection lifecycle still incomplete. | Whether connected reporting reduces effort beyond current provider-native tools. |
| Can an inquiry become an accurate proposal and a usable payment request? | Existing proposals, invoices, calendars and Stripe client; replace or disable placeholder behavior with evidence. | Whether customer-to-payment execution is a viable first workflow and for which owner type. |
| Can a recurring follow-up stop on payment, cancellation or revoked authority? | Existing approvals, queue/agent primitives and receivables code; actual delivery/recovery still needs proof. | Whether ongoing automation saves net owner time rather than requiring supervision. |
| Can the same authorized workload be costed in managed API and customer-funded modes? | Existing proxy, native usage events and cost UI; meter feedback, tenancy, attribution and reservation defects block confidence. | Sustainable resource rates and a credible BYOK offering. |

For each representative workload measure owner setup/review/correction time; observed success/failure; provider requests and token classes; active/reserved resources; cold starts, waits and retries; stored/network data; support effort; and an actual invoice reconciliation. Report workload size, deployment mode, payer, sample count and p50/p95 rather than extrapolating from one demo. Use provider sandbox/test boundaries where available; do not spend real money or run live customer actions without authorization.

Collect a small, permissioned set of recent owner workflows across candidate segments before choosing a segment. Compare each against both its existing manual/SaaS process and the current AI business tools. Exact willingness to pay, usage tolerance, privacy preference and desired autonomy remain unknown. Public stories help select questions, not answer those commercial questions conclusively.

**Current decision:** repair the evidence foundation and evaluate resource-based charging/customer-funded inference. Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research. Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection.

## Source register

All external sources linked inline were accessed **2026-09-18**. O1–O6 are anonymous/self-reported discussion threads with unverified publication timestamps, not interviews. O7 is Anthropic's dated workshop account; O8 is Google's undated written customer gallery. Product announcements have publication dates stated above; live provider help/terms and quota pages can change independently of announcements. No testimonial savings, profit uplift, installation counts or claimed market share were adopted as OHC evidence. No browsing of customer credentials or private owner documents was performed.

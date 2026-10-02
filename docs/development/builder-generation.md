# Builder generation: bounded operator-owned text drafts

The builder and Brand Studio use the existing `ConfiguredTextAnalysis` provider
and `WorkflowExecution` admission path. There is no template success fallback.

An operator must explicitly bind the process-wide provider to exactly one tenant
using `OMNISOLO_BUILDER_TENANT_ID`. A request must also present the current signed
owner/admin bearer for that tenant. The first requester never establishes this
binding. Other tenants cannot spend this global credential. Configure the
existing `OMNISOLO_LLM_PROVIDER`, `OMNISOLO_LLM_MODEL` and applicable endpoint/key;
the existing `OMNISOLO_MAX_TOKENS` limit remains 1–4096 (default 2048). The existing
admission enforces current membership, token expiry/revocation, four concurrent
requests, a 90-second deadline and one provider attempt. This is an explicitly
operator-funded self-hosted path; it has no durable monetary reservation,
usage settlement or managed multi-tenant customer entitlement. It must not be
advertised as metered managed generation.

## What is implemented

- Supplied-text website drafts, restricted to proposed Hero/Text blocks and
  title/description SEO suggestions. No inferred product records, prices,
  shipping, tax, testimonials or appointment availability are accepted.
- Supplied-text brand, voice, copywriting and campaign suggestions from the real
  configured provider. A successful Brand Studio response is persisted first and
  returns the actual owned record UUID. Database failure is an explicit error.
- Server-authored `generation` provenance retains the exact signed `tenant_id`, provider/model, time and
  `input_source: supplied_text`. `kind: model_draft` and
  `business_facts_verified: false` mean suggestions requiring review, not
  verification that generated prose is true.
- Empty logo, catalog and photograph arrays reflect unavailable generation or
  source data. Prompt text is not a photograph. Nothing is published or scheduled
  by generation. No nonexistent export formats or image previews are invented.

URLs and filenames do not supply content. Requests containing these inputs return
422 `source_fetch_unavailable`; no URL or asset is fetched. The GEO route returns
503 `visibility_measurement_unavailable` because its old keyword demonstration
was not an observed AI-search visibility measurement. Missing provider/binding is
503. An unconfirmed provider response is 502 `generation_outcome_unknown` without
retry. Malformed/unsupported model JSON is 502 `invalid_generation_response`.
Legacy persisted toolboxes without generation provenance remain in storage and
are excluded from owned reads rather than presented as new provider results.

## Storage authority and identity boundaries

Builder's historical RLS scope is a mapped UUID. This mapping can alias a legacy
string tenant and a different tenant named by its UUIDv5. Every toolbox GET/list
and the legacy publication reader therefore also requires the exact raw signed
tenant stored in server-authored provenance; the provider cannot supply that
field. UUID RLS alone is not the raw-tenant isolation guarantee.

Brand persistence acquires a PostgreSQL transaction, revalidates the current
bearer, and reuses publication's canonical user/role row locks. Those membership
locks remain held through the write. After INSERT (which may block), the exact
bearer, expiry and current authority are rechecked before commit. Any observed
revocation rolls back. Shared-PostgreSQL membership changes serialize against
the held row locks. A separate credential Store, or a token-revocation write that
does not participate in these locks, cannot provide an atomic cross-resource
revocation fence; this implementation does not claim one. Missing canonical
membership tables or unavailable persistence are explicit failures.

The strict publication JSON parser is also reused for model output: duplicate
decoded keys (including escaped spellings), excessive depth, unsafe numeric
literals and malformed JSON cannot silently collapse into accepted drafts.

## Evidence and outstanding verification

Run `OHC_BUILDER_GENERATION_TEST_DATABASE_URL=postgres://.../ohc_builder_generation_test
bash scripts/builder-generation-contract/run.sh` against an explicitly owned
loopback disposable PostgreSQL database. The source-bound harness imports the
production generation, authentication/admission, provider adapter and persistence
code. It uses actual isolated HTTP protocol fixtures and a restricted PostgreSQL
role with RLS, not production model substitutes or paid provider calls. Registry
dependencies must match the repository lockfile, source fingerprints must remain
unchanged during tests, and absent database prerequisites fail the gate.

This proves the local contract, not live model quality or paid-provider readiness.
Full server compilation, repository `make lint`/`make test`, current frontend
adaptation and hosted CI remain required. Live provider/model behavior, safe URL
fetching, actual media production, verified catalog grounding and managed cost
accounting are separate capabilities and remain unverified/unimplemented here.

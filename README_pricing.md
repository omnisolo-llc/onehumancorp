# Dynamic Pricing and Instant Quotes
This documents the setup for the instant quote UI, which fetches dynamic pricing rules from the edge cache and instantly evaluates them in <50ms without hitting the LLM.

- UI: `src/ui/next/src/app/instant-quote/page.tsx`
- Edge Rules API: `src/server/api/pricing_rules.rs` (mounted at `/api/v1/pricing/rules`)
- Triage Approval API: `/api/agents/approvals/simulate-quote-draft`

Run Playwright tests: `bazelisk test //src/e2e:playwright --local_test_jobs=1`

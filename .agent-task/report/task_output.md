issue_title: '✍️ Scribe: Research on Proposal and Payment Request Viability'
issue_description: |
  Title: Research on Proposal and Payment Request Viability
  Problem Statement: Assess whether an inquiry can become an accurate proposal and a usable payment request using existing implementations (proposals, invoices, calendars, and Stripe client).
  Research Report: As per the audit of the codebase:
  - F07 (fabricated proposal terms): The intake process now preserves inquiry and validates owner-supplied line items/deposit with checked integer arithmetic. Missing pricing creates `NEEDS_PRICING`; optional unselected items are excluded from committed totals. Real-stack persistence/isolation cases added, though complete browser execution remains outstanding.
  - F08 (fabricated checkout links): Invoice creation returns a draft without invented Stripe IDs/URLs. Real Stripe session client validates returned evidence and stable operation identity. Mercado Pago placeholder returns unavailable instead of a generated URL. Full provider sandbox replay/payment event reconciliation, persisted provider receipts across every workflow and all business transitions remain outstanding.
  Design Doc:
  - Architecture: Not applicable, this is a research document. No architectural changes proposed.
  - UI wireframes: Not applicable.
  - Mobile UX flow: Not applicable.
  - AI agent integration points: Not applicable.
  Implementation Prompt: No immediate implementation requested. First, collect a small, permissioned set of recent owner workflows across candidate segments to verify if customer-to-payment execution is a viable first workflow and for which owner type. Do not spend real money or run live customer actions without authorization.
  Priority: High
  Scope: Complete full provider sandbox replay/payment event reconciliation, persist provider receipts across every workflow and all business transitions, and ensure complete browser execution for proposal generation.
  Source Dates: 2026-09-18
  Study Populations: Existing web/design/marketing service professionals workflows within the OmniSolo codebase.
  Uncertainties: Whether customer-to-payment execution is a viable first workflow and for which owner type. Exact willingness to pay, usage tolerance, privacy preference, and desired autonomy remain unknown.
  Metric Definitions: Owner setup/review/correction time; observed success/failure; active/reserved resources; cold starts, waits and retries; stored/network data; support effort; and actual invoice reconciliation.
  Skill Provenance: Loaded skills `using-superpowers`, `brainstorming`, `writing-plans`, and `executing-plans` from `https://github.com/obra/superpowers.git` at revision `b0a1f28b`.
issue_priority: P1
issue_category: Research
issue_type: documentation
issue_label: agent-report
assignees: []

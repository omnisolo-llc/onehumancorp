issue_title: "Scribe Documentation Report: Capability, Usage Economics, and Native Migration"
issue_description: |
  # Documentation Report: Capability, Usage Economics, and Native Migration Audit

  ## Skill Provenance
  - **Superpowers Revision:** 5bf4e78011075bcfc0dc295f0724994cd123ee71
  - **Skills Used:** `using-superpowers` (skills/using-superpowers/SKILL.md) and `brainstorming` (skills/brainstorming/SKILL.md) adapted for standalone research.

  ## Source Dates
  - **Audit Date:** 2026-09-18 (Revision: 2026-09-18-usage-audit)
  - **Native Build Evidence Date:** 2026-09-19

  ## Study Populations
  - **Focus:** Solo independent service owners (based on forums, Claude/Google customer vignettes)
  - **Examples Observed:** Container conversion businesses, solo ecommerce/candle makers, retiring print-shop owners, painting contractors, and guitar workshops. These represent service, retail, and physical-operation workflows, rather than an exclusive digital-service market.

  ## Uncertainties
  - Which service niche experiences the strongest repeated pain?
  - Which specific channel and payment integrations will owners reliably connect?
  - What is the acceptable monthly cost and what are the acceptable bounds for agent autonomy?
  - Can users trust the outcome evidence of agent execution?
  - Does the usage telemetry accurately prevent unbounded repeated processing, as identified in finding F01?

  ## Metric Definitions
  - **Serving Cost Target:** ≤ $29.70/month (30% of an illustrative $99/mo subscription target) after onboarding. Includes model/tool compute ($10-$24), incremental infra ($5-$8), operations labor ($8-$17), and collection cost ($3).
  - **CI Performance Target:** Maximum 30 minutes for the complete Linux required gate; stretch target of 15 minutes with warm caches; diagnostic 10-minute core-build target (backend, Next, Tauri).

  ## Scope & Current Capability Evidence
  The documentation must cover how an owner accomplishes verified business work with the current product, distinguishing supported behavior from planned features:
  - **Business Setup:** Onboarding state flows exist, but verified owner-ready operating businesses are still needed.
  - **Customer Proposals:** Tenant/customer records, deposits, and approvals exist. However, intake currently writes fixed scope independent of inquiries.
  - **Calendars & Tools:** Authenticated Google Calendar/Workspace requests exist, but full secure credential lifecycles need separate proof.
  - **Payments:** Invoicing logic is present, but complete Stripe/MercadoPago workflows with real provider sessions are not fully reconciled.
  - **Agent Coordination:** Department/event coordination exists, but simulated feed items must be replaced with real execution, persistent approval, and result evidence.
  - **Provider Runtime:** Scoped proxy and secret redaction exist, but require reliable accounting and clear payer modes before full usage billing is enabled.
issue_priority: "P1"
issue_category: "documentation"
issue_type: "research_report"
issue_label: "agent-report"
assignees: []

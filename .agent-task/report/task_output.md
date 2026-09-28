issue_title: "[Research] Customer-to-payment execution viable workflow"
issue_description: |
  **Revision:** 2026-09-18-usage-audit

  **Uncertainty Addressed:** Can an inquiry become an accurate proposal and a usable payment request? Whether customer-to-payment execution is a viable first workflow and for which owner type.

  **Source Evidence:**
  - `docs/research/business_capability_and_usage_economics_audit.md` highlights the need to replace placeholder URLs in proposals/invoicing.
  - The audit indicates that invoice URLs were fabricated in `F08` (closed) and proposals had fabricated scope in `F07` (closed).
  - The native build verification confirms that for F07: "Intake preserves inquiry and validates owner-supplied line items/deposit with checked integer arithmetic... Real-stack persistence/isolation cases added; complete browser execution remains outstanding."
  - For F08: "Invoice creation returns a draft without invented Stripe IDs/URLs. Real Stripe session client validates returned evidence and stable operation identity... Full provider sandbox replay/payment event reconciliation, persisted provider receipts across every workflow and all business transitions remain outstanding."

  **Conclusion / Metrics / Scope:**
  - The core paths for proposal generation and invoice persistence without fabricated Stripe IDs are implemented.
  - To prove this loop fully, we must execute the full browser journey (outstanding for F07) and perform provider sandbox replay/payment event reconciliation (outstanding for F08).
  - A digital-service solo owner (e.g., web/design/marketing) remains the best segment for this loop because the deliverables and milestones are digital and bounded, avoiding complex physical supply chains.
  - Further work should focus on testing the provider sandbox integration for Stripe (or Mercado Pago) rather than creating new API endpoints.

issue_priority: "P1"
issue_category: "research"
issue_type: "audit"
issue_label: "ohc:journey:J1"
assignees: ["scribe"]

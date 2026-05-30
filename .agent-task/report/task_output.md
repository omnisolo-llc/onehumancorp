issue_title: "Instant Localized Invoicing Ledger Architecture Design"
issue_description: |
  # Instant Localized Invoicing Ledger

  ## Problem Statement
  Small business owners need to quickly issue professional, accurate, and tax-compliant invoices. Current platforms lack native invoicing, forcing users to rely on complex accounting software or generic tools without proper localization. OHC needs a system to instantly generate localized invoices that integrate directly with a unified ledger.

  ## Research Report
  *   **Shopify:** Basic invoicing; complex B2B/localized invoicing requires expensive third-party apps.
  *   **Wix/Squarespace:** Rudimentary invoicing; manual setup for local tax jurisdictions.
  *   **Stripe Invoicing:** Powerful but developer-focused and complex for non-technical users.
  *   **OHC Differentiation:** The Finance & Payments agent handles invoicing invisibly. It observes an order, determines the customer's location, applies local tax rules, and generates a localized invoice with a payment link, seamlessly recorded into the multi-tenant ledger.

  ## Design Doc
  See `docs/research/[architecture]_instant_localized_invoicing_ledger.md` for full Architecture Diagram, UI Wireframes & 375px Baseline, Mobile UX Flow, AI Agent Integration Points, and Key Design Decisions.

  ## Implementation Prompt
  Build the architecture and UI for the "Instant Localized Invoicing Ledger" to let users generate and send localized invoices. The system should automatically calculate correct local tax, generate a beautiful preview, and securely record the transaction in the ledger.

  ## Acceptance Criteria:
  *   Mobile Parity (375px viewport, Translucent Glass aesthetics)
  *   Accurate local tax calculation based on mocked locations
  *   Isolated ledger integration (corresponding database entry)
  *   Strict multi-tenant boundaries for financial records

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

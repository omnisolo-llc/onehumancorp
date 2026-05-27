issue_title: "Implement Invisible Autonomous Bookkeeping and Tax Ledger"
issue_description: |
  # Problem Statement
  For non-technical small business owners like Maya (the baker) and Carlos (the handyman), managing finances is a dreaded, error-prone task. They need a system that tracks every cent invisibly, automatically categorizes expenses, sets aside estimated taxes, and generates compliant reports without them ever needing to touch a spreadsheet or know what a ledger is.

  # Research Report
  We lack an integrated, zero-touch financial ledger. OHC must provide a system where the AI acts as a virtual CFO. Every transaction is instantly reconciled, categorized, and recorded in a multi-tenant, immutable ledger.

  # Design Doc
  See `docs/research/[architecture]_invisible_autonomous_bookkeeping_and_tax_ledger.md` for the full architecture diagram, mobile UX flow, AI agent integration points, and key design decisions (Immutable Ledger, Zero-Trust Multi-Tenancy, Offline-First Snap).

  # Implementation Prompt
  Deploy an invisible bookkeeping engine that automatically tracks income, categorizes expenses, and estimates tax liability in real-time. Dashboard should display simple "Money In", "Money Out", and "Tax to Save" metrics.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
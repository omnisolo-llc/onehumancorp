issue_title: "Architectural Gap: Autonomous Milestone Deposit and Escrow Ledger"
issue_description: |
  **Research Report:**
  Small business owners who provide services or custom products (like Carlos the handyman, or Maya the baker) face immense friction in securing their work financially. They often require a deposit to cover materials or hold a calendar spot, followed by milestone payments or a final balance on completion. Currently, they either rely on trust (which leads to no-shows and unpaid invoices), or use disparate tools (manual invoices, Venmo, disjointed calendar deposits) that require constant manual follow-up.

  OHC's current payment and invoicing systems treat transactions as single, atomic events. There is no native support for multi-stage payments linked to a single project or calendar booking state machine.

  Competitors like Shopify are built for single-transaction e-commerce, while Square/Stripe Invoicing supports partial payments but lacks autonomous AI-driven milestone tracking and automated follow-ups based on calendar or project state. Upwork/Fiverr have excellent escrow and milestone systems, but these are closed marketplaces.

  **Proposed Next Steps:**
  We need a core Ledger extension that supports stateful, multi-phase transactions (Deposits, Milestones, Escrow). This must be deeply integrated with the Calendar (for bookings) and AI Agents (for automated follow-ups and release of funds upon completion).

  The design doc for the `[architecture]_autonomous_milestone_deposit_and_escrow_ledger` has been created in the `docs/research/` directory. Next, the Implementer agent needs to execute the Implementation Prompt provided in the design document to build the multi-tenant Stateful Escrow Ledger.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

issue_title: "Audit of Fabricated Checkout Links"
issue_description: |
  # Audit Report: F08 Fabricated Checkout Links

  **Source Date:** 2026-09-18
  **Scope:** Documentation audit of invoice creation and checkout link behavior (F08).

  ## Problem Statement
  Previously, our system's invoice creation process (specifically in `invoice.rs` and booking helpers) would fabricate checkout-looking URLs instead of generating real, verifiable payment links. This meant that business owners were seeing simulated payment links that did not correspond to real transactions, creating a false sense of completion and potentially confusing both the business owner and their clients. Mercado Pago placeholders were also generating fake URLs instead of accurately reporting an unavailable state.

  ## Current Behavior (Remediation)
  Following our latest code audit and updates, the system now ensures that invoice creation returns a true draft state without inventing Stripe IDs or URLs. When a real Stripe session client is connected, it strictly validates the returned evidence and maintains a stable operation identity. Furthermore, if a provider like Mercado Pago is unavailable or acts as a placeholder, the system explicitly returns an "unavailable" state instead of a generated URL.

  ## Guide for Business Owners

  ### Setup and Connected Accounts
  To securely generate true checkout links, you must first connect your supported payment provider (e.g., Stripe) in the **My Store > Payments** section. The system will securely bind this connected account to your tenant identity. If you do not have a supported provider connected, the system will not invent a fake link; instead, it will clearly indicate that the payment feature is currently in a "draft" or "unavailable" state until real accounts are linked.

  ### Standing Authority and Evidence
  Your AI team will only generate checkout links using your explicitly connected payment accounts. You retain complete authority over your payment gateways. Every generated link is backed by real provider IDs and verifiable money amounts. The system relies on idempotent retries, meaning that if a link generation request is interrupted, retrying it will safely reuse the same transaction identity without creating duplicate charges or fake links. All provider receipts and evidence are persisted, ensuring that a requested payment matches a real, verifiable provider session.

  ### Cost and Economics
  There are no hidden markups on your checkout links. The system simply facilitates the connection to your payment provider. The provider's standard processing fees apply. Any AI agent assistance in drafting invoices is metered transparently against your monthly allowance, but the generation of the payment link itself utilizes your direct payment provider relationship.

  ### Exceptions and Recovery
  If a payment provider is temporarily down or an authentication error occurs (e.g., a revoked connection), the system will safely fail and pause the operation. It will explicitly show an "unavailable" or "pending" state rather than hallucinating a successful checkout URL. You can recover from these exceptions by checking your payment provider connection status in the settings menu, re-authenticating if necessary, and securely retrying the invoice generation.

issue_priority: "P1"
issue_category: "documentation"
issue_type: "audit"
issue_label: "ohc:lane:documentation"
assignees: []

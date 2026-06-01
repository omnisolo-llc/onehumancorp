issue_title: "[Architecture] Implement Unified Tap-to-Pay POS Architecture"
issue_description: |
  # Problem Statement
  Priya (Boutique Owner), Carlos (Handyman), and Fatima (Food Cart) need a reliable way to process in-person payments directly from their phones. Currently, OHC lacks a unified Tap-to-Pay (POS) architecture integrated with Stripe Terminal. Without this, non-technical owners are forced to juggle external card readers and manually reconcile in-person sales with online storefront inventory.

  # Research Report
  Leading platforms like Shopify and Square provide seamless POS experiences. Shopify's POS relies heavily on dedicated hardware or complex app integrations. OHC has an opportunity to leapfrog by leveraging Stripe's Tap to Pay on iPhone and Android, meaning Maya, Carlos, or Priya only need their smartphone to accept contactless payments. This reduces friction to zero—no extra dongles or Bluetooth pairing required for basic operations.

  # Proposed Architecture & Next Steps
  We need to design a system that:
  1. Integrates Stripe Terminal SDK into the Flutter client.
  2. Creates a backend `TerminalSession` entity with strict row-level security for tenant isolation.
  3. Uses the "Finance & Payments" AI agent to automatically reconcile offline POS payments with online inventory in real-time.

  Please see the detailed research document added to `docs/technical/architecture/research/` for Mermaid.js diagrams, mobile-first UX specifications, and the exact implementation prompt.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

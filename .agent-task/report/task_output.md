issue_title: "Implement Omnichannel Tap-to-Pay Terminal SDK for Mobile-First Offline POS"
issue_description: |
  **Research Report & Findings:**

  Small business owners—especially those operating in mobile, pop-up, or field service environments like Carlos (handyman) and Fatima (food cart)—frequently operate in areas with poor or no internet connectivity. Relying solely on cloud-dependent payment gateways means lost sales when the network drops. They need a resilient, zero-hardware solution that turns their existing mobile device into an offline-capable tap-to-pay terminal.

  Competitor systems (like Shopify POS or Stripe Terminal) often require purchasing physical card readers or struggle to synchronize transactions captured completely offline without complex manual reconciliations. OHC needs a native, mobile-first Tap-to-Pay SDK that seamlessly captures payments offline and synchronizes transparently with the multi-tenant ledger once connectivity is restored, abstracting away the complexities of local caching and eventual consistency from the merchant.

  **Target Persona Validation**
  * **Carlos (Handyman, 42):** Needs to take payments in basements or remote job sites with zero cell service. Wants a single app on his Android phone to quote, bill, and tap-to-pay.
  * **Priya (Boutique Owner, 35):** Takes her inventory to local pop-up markets. Needs to quickly process sales via Tap to Pay on her phone without worrying about a bulky card reader or a dropped 5G connection causing a double charge.

  **Proposed Next Steps:**
  * Implement the TapToPaySDK integration within the Tauri mobile client (`src/ui/tauri/`).
  * Build the corresponding cloud reconciliation service (`src/server/services/billing/`).
  * Utilize the existing standalone local SQLite DB (managed by PowerSync) for offline transaction durability.
  * Use the NATS JetStream event mesh to replicate transaction events to the cloud.

  See the full architectural design document at `docs/research/[architecture]_omnichannel_tap_to_pay_terminal_sdk.md`.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
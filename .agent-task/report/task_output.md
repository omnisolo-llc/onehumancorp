issue_title: "Implement Autonomous Reputation & Referral Engine"
issue_description: |
  # Research Report: Autonomous Reputation & Referral Engine

  ## Problem Statement
  Small business owners rely on word-of-mouth and local reputation to grow, but asking for reviews or managing referral programs introduces high friction. Competitor platforms rely on expensive, complex, desktop-first third-party apps like Loox or Smile.io. OHC needs an invisible, autonomous "Growth Partner" that proactively solicits verified reviews at the exact right moment (e.g., post-service) and credits referrers automatically without merchant intervention.

  ## Proposed Architecture & Design Decisions
  1. **Event-Mesh Integration:** The engine must subscribe to the `ServiceCompleted` and `OrderDelivered` events on the NATS Hybrid Event Mesh. Our research confirmed these events are currently fired and tracked in `src/server/services/growth/event_listener.rs`.
  2. **Context-Aware Outreach:** The AI Customer Success Department evaluates transaction context and dispatches a 1-tap rating request via the Omnichannel Comm Engine (SMS/WhatsApp).
  3. **Ledger-Backed Referrals:** We identified `ohc_universal_ledger` (in `src/server/db/migrations/015_job_queue_and_ledger.sql`) as the correct append-only data store for referral credits. When a referred friend checks out, the credit allocation is immutably logged to the ledger.
  4. **Multi-Tenancy & Data Model:** The system requires new tables `reputation_profiles`, `reviews`, and `referral_codes` with strict Row Level Security (RLS) bound to `tenant_id`.

  ## Next Steps for Implementation
  - Create database migration to establish `reputation_profiles` and `reviews`.
  - Update `src/proto/hub.proto` to add `SubmitReviewRequest`, `GetReputationRequest`, and corresponding responses under `service GrowthService`.
  - Extend `MyGrowthService` in `src/server/services/growth/service.rs` to handle these RPCs.
  - Enhance the NATS subscribers in `event_listener.rs` to generate an SMS review prompt.
  - Integrate referral generation via `referral_api::generate_referral_link` when reviews are >= 4 stars.
  - Implement the referral credit issuance appending an `event_type` of 'ReferralConversion' into `ohc_universal_ledger`.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []

# Research Report: Offline Tap-to-Pay Architecture

**Problem Statement:**
Non-technical business owners (like Carlos the handyman and Fatima the food cart operator) often operate in areas with poor or zero internet connectivity (basements, crowded street corners). Currently, OneHumanCorp (OHC) requires an active internet connection to process payments. If the connection drops, they cannot accept payments, directly causing lost revenue and customer frustration.

**Research Findings:**
- Competitors like Square POS offer robust offline mode capabilities where transactions are queued locally and synced when connectivity is restored.
- Stripe Terminal supports offline mode, allowing Tap-to-Pay on iPhone/Android to function without immediate internet access, securely storing encrypted payment data.
- OHC's current architecture lacks a local CRDT-based queue for transactions and relies entirely on synchronous API calls to `src/server/integrations/stripe/routing.rs` and the Stripe API.

**Design Document:**
- Introduce an Offline Transaction Queue in the Flutter PWA/Mobile App using SQLite/Isar.
- Implement a CRDT (Conflict-free Replicated Data Type) sync engine (`src/server/services/sync/offline_pos.rs`) that reconciles offline transactions once the device reconnects.
- Utilize Stripe Terminal's SDK for offline card data capture.
- AI Operations Department coordinates background reconciliation and alerts the owner if any offline transaction fails upon sync (e.g., declined card).

**Implementation Prompt:**
Implement the offline Tap-to-Pay architecture. Create the `offline_pos.rs` sync handler in the backend to process batched offline transactions. Update the Flutter client to queue transactions locally when `navigator.onLine` is false. The user should see a "Payment Saved Offline" notification, and a background sync should automatically occur when connectivity is restored.

**Priority:** P0
**Estimated Scope:** Large

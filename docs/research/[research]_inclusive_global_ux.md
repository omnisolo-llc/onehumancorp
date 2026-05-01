# Issue Brief: Inclusive Global UX (Multilingual & Offline-First)

## Problem Statement
Founders like Fatima (Food Cart Operator) are often overlooked because existing platforms are English-heavy and require a constant, high-speed connection. To be truly "for everyone," OHC must work in the user's native language and remain functional in low-bandwidth or offline environments.

## Research Report
- **Competitor Audit:**
    - **Shopify/Wix:** Support multiple languages but the *management dashboard* is often poorly translated or English-only in advanced settings.
    - **Offline Support:** Almost non-existent. If the internet drops, the POS or dashboard usually breaks.
- **SMB Pain Point:** "I can't run my business when the market's WiFi goes down" and "The app uses too much data" (Source: Trustpilot/Global SMB reviews).
- **Leapfrog Advantage:** OHC is built with Rust and Slint, enabling high-performance, native-like behavior with a small footprint. By implementing an "Offline-First" event log, OHC ensures that a sale made in a food cart is never lost.

## Design Doc
### High-Level Architecture
- **AI-Driven Translation:** Use Gemini Pro to provide high-fidelity, context-aware translations of the entire dashboard, not just the storefront.
- **Durable Local State:** Every UI action is written to a local SQLite "Outbox" before being synced to the Hub. This allows full functionality without a network.
- **Low-Data Mode:** Automatic image compression to WebP at the source and aggressive caching of static assets.

### Mobile UX Flow (375px First)
1. **Onboarding:** Language is detected or chosen immediately. Fatima sees "Marhaba" (Arabic) or "Hello" (English).
2. **Operations:** Fatima records an order while at the cart. The app shows a "Syncing..." icon but allows her to finish the sale.
3. **Sync:** Once she gets a signal, the data is pushed to the Hub invisibly.

## Implementation Prompt
Implement a localization framework using `Fluent` or a similar Rust-compatible system, integrated with the AI backend for dynamic translation. Create a local "Event Outbox" in SQLite to capture all dashboard writes (orders, inventory changes) while offline, and implement a `SyncManager` that handles background reconciliation when connectivity is restored.

## Priority
P1

## Estimated Scope
Medium

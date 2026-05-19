# Architecture Brief: Offline-First In-Person Tap-to-Pay POS

## Title
[Architecture] Offline-First In-Person Tap-to-Pay POS Platform

## Problem Statement
Small business owners like Priya (boutique owner) and Fatima (food cart operator) operate in environments with flaky internet connections (e.g., bustling street markets, deep inside concrete retail stores). They need to process in-person sales seamlessly, track inventory, and sync analytics back to the cloud. Current competitors like Shopify POS and Square provide solutions, but they often struggle when completely offline, leaving merchants unable to ring up customers. The OHC platform must support an "Offline-First" Point-of-Sale (POS) system that allows tap-to-pay functionality, reads the local product catalog, and gracefully syncs data when connectivity is restored, without showing technical errors to the user.

## Research Report
- **Competitor Systems Audit**:
  - **Shopify POS & Square**: Often require a constant connection for high-tier catalog sync and reliable card processing. They fall back to "offline mode" with heavy warnings and limitations.
  - **Modern Local-First Frameworks**: Expo SQLite, CRDTs (Conflict-free Replicated Data Types), and tools like PowerSync or ElectricSQL represent the bleeding edge of offline-first mobile sync.
  - **Stripe Terminal Mobile SDK**: Stripe provides robust SDKs (React Native, iOS, Android) that support offline/store-and-forward payments (if configured/allowed) or graceful queuing.
- **Identified Gap**: OHC lacks a unified, resilient local-first architecture for the mobile app that can operate a POS checkout entirely from local SQLite, queue the payments/inventory updates, and sync them back to the OHC-SIP database upon reconnection. The current data structures in OHC do not explicitly outline this edge-caching and sync boundary.
- **Value Proposition**: Priya and Fatima never miss a sale due to a "No Internet Connection" spinner.

## Design Doc

### Key Design Decisions
1.  **Local-First Database (SQLite + CRDT)**: The mobile app will use a local SQLite database to store the full product catalog and customer list for the specific tenant. Syncing with the cloud uses CRDTs or a background sync engine (like PowerSync) to resolve conflicts automatically.
2.  **Background Sync Queue**: All mutations (sales, inventory changes) are written locally first, immediately updating the UI. A background worker periodically attempts to flush the event queue to the cloud via the API Gateway.
3.  **Stripe Terminal Integration**: Utilize the Stripe Terminal React Native / Mobile SDK to interface with card readers (e.g., Stripe Reader M2 or BBPOS WisePad 3) via Bluetooth, abstracting away the complex EMV certification logic.

### AI Department Coordination
- **The Operations Agent**: Monitors the sync queue. If an offline transaction is delayed by more than 24 hours, it alerts the user with a plain-language notification ("You have 5 sales waiting to sync. Please connect to Wi-Fi."). It also handles inventory reconciliation once sync completes.

### Architecture Diagram (Mermaid.js)
```mermaid
graph TD
    subgraph Mobile Device (Offline-First)
        UI[Mobile POS UI] --> LocalDB[(Local SQLite)]
        UI --> StripeSDK[Stripe Terminal SDK]
        StripeSDK -.-> BluetoothReader[Card Reader]
        LocalDB --> SyncQueue[Background Sync Queue]
    end

    subgraph Cloud Infrastructure (OHC-SIP)
        API[API Gateway]
        API --> ConflictResolver[CRDT / Sync Engine]
        ConflictResolver --> CloudDB[(Primary OHC DB)]
        StripeWebhooks[Stripe Webhooks] --> WebhookHandler
        WebhookHandler --> CloudDB
    end

    SyncQueue -- Network Restored --> API
```

### Mobile UX Flow (375px First)
- **Checkout View**: A clean, high-contrast numeric keypad and a scrollable, image-rich catalog (Glassmorphism cards).
- **Status Indicator**: A small, unobtrusive cloud icon in the header. Green = Synced, Gray = Offline. Tapping it explains "Working offline. Sales will save automatically when connected."
- **Tap-to-Pay Modal**: When "Charge $45.00" is tapped, a clean modal appears prompting the user to "Tap card on reader," integrating directly with the Stripe SDK's UI constraints but matching the OHC visual tokens (Outfit font, 8px/16px curves).

### Security & Multi-Tenant Rules
- The local SQLite database must be encrypted at rest using OS-level keystores (e.g., iOS Keychain, Android Keystore).
- Tenant isolation is strictly enforced during sync; the Sync Queue attaches the derived `organization_id` from the secure auth token, never trusting client-side payload IDs.

## Implementation Prompt
Implement the Offline-First Mobile POS foundation. Set up the local SQLite database schema for the product catalog and offline transaction queue. Integrate the Stripe Terminal Mobile SDK for handling card present payments. Implement a background sync worker that polls the local queue and pushes transactions to the main OHC backend when network connectivity is available. On the UI side, build the 375px-optimized mobile POS checkout screen with the local catalog grid and a clear, non-technical offline status indicator. Ensure all UI components use the OHC premium design tokens (Translucent Glass materials, Outfit font). Do NOT prescribe the exact API endpoints or SQL DDL statements for the cloud sync.

## Priority
P0

## Estimated Scope
Large

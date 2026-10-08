# Virtual Wallet & Card Issuing Engine Design

## Problem
Small business owners face cash flow issues due to 2-5 day payment hold times.
OneHumanCorp needs to bypass this by issuing a native Virtual Wallet and Business Debit Card to provide true instant liquidity when a transaction clears.

## Architecture

We need to add Wallet and Virtual Card domain models to the backend, integrate with the existing Ledger, and expose an API for the UI. The UI will show a Wallet Dashboard and Virtual Card Reveal Flow.

### 1. Backend Data Models (`src/server/domain/wallet.rs` and `src/server/domain/virtual_card.rs`)

**Wallet:**
- `id`: String (UUID)
- `tenant_id`: String
- `available_balance_cents`: AtomicI64
- `currency`: String

**VirtualCard:**
- `id`: String (UUID)
- `wallet_id`: String
- `status`: String (Active, Frozen)
- `tokenized_pan`: String (Mocked for now, last 4 exposed)

### 2. Service Layer (`src/server/services/capital/wallet_service.rs`)
- Manages wallets and virtual cards per tenant.
- Integrates with `LedgerManager` so that when a payment is processed, the `merchant_cut` updates the wallet's `available_balance_cents`.

### 3. API endpoints
- GET `/api/wallet`
- GET `/api/wallet/card`

### 4. Frontend UI (`src/ui/next/src/app/wallet/page.tsx` and components)
- **Wallet Dashboard**: Shows balance, recent transactions (mocked or integrated with ledger).
- **Virtual Card Reveal**: Shows a digital card, masked PAN, and a "Reveal" button. Follows Apple/UniFi design tokens (16px rounded corners for cards, glassmorphism, accent colors).

## Trade-offs and Rulings
- **Security:** We'll mock the actual PAN/CVV generation for now since we're not integrating a real banking API yet.
- **Data Persistence:** The current Ledger is in-memory (`HashMap` and `AtomicI64`). We'll follow this pattern for the Wallet for now, to ensure consistency and avoid over-engineering with DB migrations if the ledger is currently in-memory.
- **Frontend Framework:** The issue mentions Tauri desktop/mobile app (`src/ui/tauri`), but the Next.js app in `src/ui/next` is the actual UI packaged by Tauri. We'll build it in Next.js.

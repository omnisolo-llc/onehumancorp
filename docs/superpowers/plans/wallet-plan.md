# Wallet Implementation Plan

## Task 1: Wallet and Virtual Card Domain Models
- Create `src/server/domain/wallet.rs` with `Wallet` struct.
- Create `src/server/domain/virtual_card.rs` with `VirtualCard` struct.
- Update `src/server/domain/mod.rs` to expose `wallet` and `virtual_card`.

## Task 2: Wallet Service
- Create `src/server/services/capital/wallet_service.rs` to manage Wallets and Virtual Cards.
- Make it integrate with the existing `LedgerManager`.

## Task 3: Backend API Endpoints
- Since the backend exposes APIs via tonic gRPC and there's a Next.js frontend, we should verify if we need a new tonic endpoint or if Next.js does server actions directly against the database or via gRPC. Based on typical OHC architecture, Next.js calls gRPC or direct queries. We will just add the service to `src/server/services/capital/mod.rs` and make it accessible. Wait, the Next.js ui usually uses API routes or server actions.

## Task 4: UI Dashboard
- Create `src/ui/next/src/app/wallet/page.tsx` for the Wallet Dashboard and Virtual Card Reveal.
- Ensure the UI follows Apple/UniFi design tokens.
- Add mock data fetching in the page to display the wallet and card.

## Task 5: Playwright Tests
- Create `src/ui/next/tests/wallet.spec.ts` (or equivalent location) with 5 Playwright E2E tests covering the CUJ (Navigate to Wallet, View Balance, View Card, Reveal Card, Handle missing card).

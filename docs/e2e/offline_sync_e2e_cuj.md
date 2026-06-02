# Offline-First Tap-to-Pay Omnichannel Inventory Sync Mesh E2E CUJ

## Persona
Priya, Boutique Owner (35, semi-technical). Runs an omnichannel clothing store.

## Business Context
Priya is managing her storefront while making in-person sales at a pop-up market. She has limited network connectivity. She needs to ensure that when she processes a Tap-to-Pay transaction on her mobile phone, the offline transaction is locally processed and properly synced with the online inventory as soon as connectivity is restored, ensuring online customers cannot purchase sold-out items.

## Text Flow Chart

### Step 1: Initial Setup
- **Action**: Navigate to Home Page, Login as Priya.
- **Expected State**: Dashboard loads.
- **Action**: Navigate to "Products" section. Create a new product "Silk Scarf" with inventory count 5.
- **Expected State**: Product created, UI displays 5 in stock.

### Step 2: Simulate Offline Point-of-Sale Checkout
- **Action**: Use the mobile app POS layout. Start a checkout for 1 "Silk Scarf".
- **Action**: Simulate going offline (e.g., via Playwright's `browserContext.setOffline(true)`).
- **Action**: Process Tap-to-Pay payment.
- **Expected State**: UI optimistically shows "Paid. Syncing pending...". The local cart is cleared, and an offline event queue holds the `inventory_deduction` mutation.

### Step 3: Network Restoration & Sync
- **Action**: Simulate restoring network connection (`browserContext.setOffline(false)`).
- **Expected State**: The offline sync queue triggers a sync request to the API backend (`/api/offline-sync`).

### Step 4: Verification
- **Action**: Navigate back to "Products" section or Online Storefront view.
- **Expected State**: The "Silk Scarf" inventory correctly displays 4 instead of 5.

### Step 5: Sold Out Scenario
- **Action**: Buy the remaining 4 items via the offline/online process.
- **Expected State**: Inventory drops to 0, and the item is marked as "Sold Out".

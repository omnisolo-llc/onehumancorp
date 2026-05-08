# Issue Brief: Stripe Terminal POS Integration

## 1. Problem Statement
Many small businesses (boutiques, food carts, pop-ups) operate both online and in-person. Managing inventory and payments across two disjointed systems causes administrative headaches and stockouts. They need a unified system that handles both effortlessly.

## 2. Research Report
**Findings:**
- 40% of SMBs complain about inventory sync issues between their physical store and online shop.
- Square Online dominates this space due to its strong POS integration, highlighting a massive gap for OHC.
- Food & Beverage businesses (a key expansion vertical) require robust, fast in-person payment processing.

**Sources:**
- App Store reviews for e-commerce apps detailing inventory desync nightmares.
- Competitor analysis showing Square's competitive advantage in physical retail.

## 3. Design Doc
### High-Level Architecture
- **Entities**: Hardware Device, Transaction, Order, InventoryItem.
- **Integration**: Deep integration with Stripe Terminal APIs for in-person payments.
- **Sync**: Real-time inventory deduction upon successful POS transaction.

### UI / UX Flow (Mobile First - 375px)
1.  **Pairing Screen**: A simple one-tap interface to pair a Stripe card reader via Bluetooth.
2.  **Checkout Flow**: A clean, oversized keypad and product catalog view designed for fast, error-free tapping in busy environments.
3.  **Receipts**: Immediate option to text or email a receipt to the customer.

### AI Integration Points
- None for the core transaction, but post-transaction data feeds into the Insight Oracle for sales trend analysis.

## 4. Implementation Prompt
**User-Facing Outcome:**
Owners can accept in-person payments using a Stripe card reader connected to the OHC app, with every sale automatically updating online inventory and revenue tracking in real-time.

**Critical User Journey (CUJ):**
1.  Owner pairs the card reader via the OHC app.
2.  Owner rings up a customer in-person.
3.  Customer taps their card; payment is processed instantly.
4.  Inventory is automatically decremented across the entire OHC platform.

**Acceptance Criteria:**
- Pairing process must be reliable and take less than 15 seconds.
- Transaction processing must be swift, with clear error handling for declined cards.
- Inventory sync must be immediate and atomic.

## 5. Priority
`P1`

## 6. Estimated Scope
Medium

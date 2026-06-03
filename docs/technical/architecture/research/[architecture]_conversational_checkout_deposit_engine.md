<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Design Doc: Conversational Checkout & Instant Deposit Engine

**Author(s):** Principal Product Architect (L8)
**Status:** Proposed
**Last Updated:** 2024-06-02

## 1. Problem Statement
**The Pain Point:** Maya (The Home Baker) and Carlos (The Freelance Handyman) close 80% of their business via social media DMs (Instagram, WhatsApp) or text messages. When a customer says "I want the vegan cake for Saturday," Maya currently has to leave the app, generate a Stripe link, copy it, paste it back, and then manually verify if the deposit was paid before adding it to her calendar. This friction causes massive drop-offs.
**The Goal:** Enable the "Sales & Acquisition" AI Agent to autonomously generate a secure, localized zero-click checkout card directly inside the DM thread. The moment the deposit is paid, the system must instantly lock the inventory/capacity and notify Maya.

## 2. Research Report
- **Competitor Systems Audit:**
  - **Shopify Inbox:** Allows sending product links in chat, but still redirects to a full browser checkout flow. Doesn't support service deposits natively.
  - **Stripe Payment Links:** Fast, but lacks deep bidirectional sync with the merchant's live calendar/inventory without custom webhooks.
  - **Meta WhatsApp Business:** Native payments exist (UPI in India, Pix in Brazil), but they are heavily fragmented and not synced with an omnichannel unified inventory mesh.
- **Identify Gaps:** OHC needs a universal engine that bridges the DM thread, localized payment gateways (Mercado Pago, Stripe), and the `Unified Capacity Mesh`. The AI must construct a transaction state, hold a soft lock on the inventory, and release the checkout card.

## 3. Design Doc

### 3.1 Architecture Diagram
```mermaid
graph TD
    A[Customer Instagram/WhatsApp DM] -->|Message: "I want to book Tuesday"| B(Omnichannel AI Inbox);
    B -->|Intent: Booking| C[Sales & Acquisition AI Agent];
    C -->|Request Capacity Hold| D[Unified Capacity Mesh];
    D -- Soft Lock Granted (15 mins) --> C;
    C -->|Generate Session| E[Conversational Checkout Engine];
    E -->|Create Intent| F[Payment Gateway: Stripe / Mercado Pago];
    E -->|Render Interactive Card| B;
    B -->|Send DM with Deep Link| A;
    F -- Webhook: Deposit Paid --> G[Ledger & Reconciliation];
    G -->|Commit Inventory| D;
    G -->|Notify Operations| H[Operations Agent];
```

### 3.2 Data Model & Invariants
- **CheckoutSession:** `id`, `tenant_id`, `customer_id`, `type (deposit/full)`, `amount`, `status (pending/paid/expired)`, `inventory_lock_id`.
- **Tenant Isolation:** Enforced via RLS in Postgres. All Webhook processing requires tenant context verification.
- **Invariants:**
  - A Soft Lock on inventory/capacity expires strictly after 15 minutes.
  - The conversational card must deep-link to a native OS payment sheet (Apple Pay / Google Pay / Pix) where supported, falling back to a minimal WebP-optimized webview.

### 3.3 Mobile-First & Performance Targets
- **375px Flow:** The customer taps the checkout bubble in the DM. A half-sheet modal slides up displaying the quote, deposit amount, and an "Apple Pay" or "Pix" button. No keyboard entry required.
- **Performance:** Checkout sheet must render in < 200ms.

## 4. Implementation Prompt
**For Implementer Agent:**
Implement the Conversational Checkout & Instant Deposit Engine.
- **User-Facing Outcome:** The AI Sales Agent can reply to a customer in WhatsApp/IG with a dynamic checkout bubble. The customer taps it to pay a deposit instantly via Apple Pay/Google Pay/Mercado Pago, which auto-secures their booking.
- **Acceptance Criteria:**
  - Create the backend data models for `ConversationalCheckoutSession` and soft-locks.
  - Integrate with the AI Inbox to trigger checkout generation based on intent.
  - Provide an E2E test verifying a mock DM flow: Customer requests quote -> AI sends checkout link -> Mock payment webhook -> Inventory is permanently locked.
  - Ensure strict tenant isolation.
- **Priority:** P0
- **Estimated Scope:** Large

</div>

# Onboarding State Machine & User Journey

This document outlines the multi-step onboarding process for OneHumanCorp (OHC), designed to take a user from sign-up to a live, AI-managed business in under 10 minutes.

## 🗺️ The 10-Minute Journey

### Step 1: Vision & Intent
- **Goal:** Capture the core business idea.
- **Actions:**
    - Choose business type (Online Store, Service, Restaurant, etc.).
    - Opt for "Instant Build" (AI-driven) or "Manual Setup".
- **AI Role:** Tailors the subsequent steps based on the selected type.

### Step 2: Identity
- **Goal:** Establish the brand name.
- **Actions:**
    - Provide business name (e.g., "Maya's Cakes").
- **AI Role:** Live preview of the brand name applied to templates in real-time.

### Step 3: Offerings
- **Goal:** Define what is being sold.
- **Actions:**
    - Select selling categories (Physical, Services, Subscriptions).
    - Add the first product/service name.
- **AI Role:** Auto-generates a compelling product description from the name alone. Detects currency based on user locale.

### Step 4: Monetization
- **Goal:** Configure payment preferences.
- **Actions:**
    - Select between Online Only or Hybrid (Online + In-person).
- **Backend Role:** Configures internal payment routing (Stripe/MercadoPago).

### Step 5: Professional Presence
- **Goal:** Choose the look and feel.
- **Actions:**
    - Select a website template (Modern, Bold, etc.).
- **UI Role:** Shows a mini-preview of the actual business name in the template styles.

### Step 6: Ownership
- **Goal:** Create the admin account.
- **Actions:**
    - Email, Password, or SSO (Google/Apple).
- **Security:** Automatic SPIFFE/SPIRE identity provisioning for the new tenant.

### Step 7: Presence
- **Goal:** Finalize the URL.
- **Actions:**
    - Auto-assign free subdomain (`*.ohc.app`) or connect custom domain.

### Step 8: Launch
- **Goal:** Go live with celebration.
- **Actions:**
    - Click "Publish".
- **Experience:** Confetti animation, auto-copy link to clipboard.

---

## 💾 State Persistence & Cross-Device Resume

Onboarding state is persisted at **every step**.

- **Table:** `onboarding_state`
- **Key:** `(tenant_id, organization_id)`
- **Data:** `current_step` (Integer), `state_json` (JSONB)

If a user starts on mobile (e.g., at a coffee shop) and finishes on a laptop at home, they land exactly where they left off. The `loadOnboardingState` function in the frontend automatically fetches the last known state and redirects to the appropriate wizard step.

## 🤖 AI Agent Provisioning

Upon completion, the `OnboardingAgent` seeds the following specialized AI Teammates:
1. **The Manager (Operations)**
2. **The Promoter (Marketing)**
3. **The Salesperson (Sales)**
4. **The Ambassador (Customer Success)**
5. **The Accountant (Finance)**
6. **The Protector (Legal)**
7. **The Advisor (Advisory)**

Each agent is automatically subscribed to relevant business events (e.g., `tenant.order.placed`, `tenant.payment.success`).

## ✅ Post-Onboarding Checklist

A "Welcome Checklist" ensures users know their next high-value actions:
- [x] Business live
- [ ] Add 3 more products
- [ ] Connect Instagram
- [ ] Share your link with a friend

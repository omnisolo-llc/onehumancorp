# Issue Brief: Omnichannel Loyalty & Rewards Engine

## Title
Omnichannel Loyalty & Rewards Engine: Invisible Offline-to-Online Customer Retention

## Problem Statement
Small business owners—like Priya (boutique owner) and Fatima (food cart operator)—struggle to retain customers across physical and digital touchpoints. Current loyalty programs are fundamentally broken for SMBs. They either require expensive secondary hardware (dedicated POS tablets for phone number entry), forcing customers to download yet another app, or managing physical punch cards that get lost. If a customer buys a dress from Priya in-store and then shops online via Instagram the next week, the two transactions are rarely linked without manual, error-prone data entry. They need an invisible system that tracks customer lifetime value (LTV) natively across *all* channels—in-person Tap-to-Pay, online checkout, and Instagram DMs—and automatically deploys rewards without the merchant lifting a finger.

## Research Report
Current SMB loyalty platforms are high-friction for both the merchant and the consumer:
- **Shopify POS / LoyaltyLion:** Excellent for e-commerce but requires the customer to explicitly create an account or provide an email. The offline POS integration requires staff to actively ask for contact details during a busy checkout, slowing down the line.
- **Square Loyalty:** Works well offline but is siloed from the merchant's other digital tools if they aren't fully embedded in the Square ecosystem (e.g., selling via social DMs).
- **FiveStars / Toast:** Requires bulky terminal hardware and significant monthly SaaS fees, making it inaccessible for mobile-first operators like Fatima.

**Opportunity:**
OneHumanCorp can achieve market dominance by utilizing the *Teammate Mesh* and *Buyer Identity Engine* to create an "Invisible Loyalty" experience. When a customer uses Tap-to-Pay on Priya's phone, the tokenized payment instrument is linked to their global OHC buyer profile. When they later buy online, the system instantly recognizes them. The *AI CRM Agent* monitors their points balance and automatically sends an SMS (or WhatsApp) reward—"Hi, you've earned a free cupcake on your next visit!"—completely autonomously. No apps, no punch cards, no hardware.

## Design Doc

### 1. Architecture Diagram
```mermaid
graph TD
    A[Customer Transaction] --> B{Channel?}

    B -->|In-Person Tap-to-Pay| C[Mobile POS Engine]
    B -->|Online Checkout| D[E-Commerce Engine]
    B -->|Social Commerce| E[AI Inbox Agent]

    C --> F[Identity Resolution Engine]
    D --> F
    E --> F

    F -->|Tokenized Identity| G[Centralized Customer Ledger]
    G --> H[Loyalty Event Trigger]

    H -->|Threshold Reached| I[Autonomous CRM Agent]
    I -->|Generates Reward| J[Discount Code / Store Credit]
    J --> K[Customer SMS/WhatsApp Notification]
    K --> L[Merchant Activity Feed "Reward Sent"]
```

### 2. UI Wireframes & Mobile UX Flow (375px First)

**Screen 1: The "Customer Profile" Card (For Priya)**
- **UI:** A macOS Translucent Glass card within the OHC app. Clean UniFi-style layout showing the customer's name, total LTV, and a circular progress ring for their loyalty status.
- **Interaction:** A toggle switch: "Auto-Send Rewards (AI Managed)". When ON, the merchant never has to manually issue a discount. A feed below shows recent AI actions: *“Sent 10% off code for Birthday”*.

**Screen 2: The Customer Experience (SMS/WhatsApp)**
- **UI:** No app download required. A plain-text or WhatsApp rich message: *"Thanks for visiting Priya's Boutique! You just unlocked $5 off your next purchase. Tap here to save to Apple Wallet."*
- **Interaction:** The link generates a dynamic Apple Wallet/Google Pay pass utilizing the secure mobile delivery engine.

### 3. AI Agent Integration Points
- **CRM / Customer Success Agent:** Automatically calculates point accruals based on complex logic (e.g., double points on slow Tuesdays) without Priya needing to configure complex rules. It drafts and sends the reward notifications.
- **Operations Agent:** Flags VIP customers in the merchant's daily briefing ("Your top customer, Sarah, is visiting today. Recommend the new summer collection.").

### 4. Key Design Decisions
- **Zero-Friction Enrollment:** Loyalty tracking is tied to the tokenized payment method (Zero-Trust security) or phone number. Explicit sign-ups are bypassed wherever legally permissible via global OHC buyer networks.
- **Hardware-Free:** Relies entirely on the merchant's mobile device (NFC Tap-to-Pay) and existing digital channels.
- **Autonomous Execution:** Merchants opt-in to AI-managed rewards. They do not design campaigns; the AI determines the optimal reward to drive a second purchase.
- **Design System:** Strict adherence to 44x44px touch targets on mobile. All settings hidden behind an "Advanced Settings" switch to pass the grandmother test.

## Implementation Prompt
**For Implementer Agent:**
Design and implement the core data models and service boundaries for the `OmnichannelLoyaltyEngine`.
- Define entities for `CustomerIdentity`, `LoyaltyLedger`, `RewardTier`, and `IssuedReward`.
- Ensure strict multi-tenant isolation; Customer PII and ledger balances must be strictly scoped to the `organization_id` using Zero-Trust policies.
- Expose an event-driven interface (via NATS hybrid event mesh) that listens to successful transactions from `PosEngine` and `CheckoutEngine`.
- Define the Webhook/Event schema for the `CRMAgent` to consume `RewardThresholdMet` events and dispatch notifications.
- Do not prescribe specific UI frameworks, but ensure the API responses are optimized for minimal payload size on mobile edge-caching.

## Priority
P1

## Estimated Scope
Large

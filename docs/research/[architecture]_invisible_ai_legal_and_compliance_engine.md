# [Architecture] Invisible AI Legal & Compliance Engine

## Problem Statement
Small business owners like **Carlos (handyman)**, **Maya (custom baker)**, and **Leo (music tutor)** face significant legal and compliance risks but cannot afford lawyers. When Carlos takes on a major electrical repair, he needs a liability waiver. When Maya takes a $500 custom wedding cake deposit, she needs a clear cancellation contract to prevent chargebacks. Leo needs his students to agree to a 24-hour cancellation policy. Currently, they rely on handshake agreements or copy-pasting generic templates from the internet, exposing them to disputes and legal liability. They need a system that invisibly generates legally binding, localized contracts and policies, seamlessly integrates e-signatures into the checkout or booking flow, and tracks compliance deadlines (like health permits for Fatima) without requiring any legal expertise.

## Research Report
**Competitive Systems Audit:**
- **DocuSign / HelloSign:** Powerful e-signature platforms, but they are standalone tools. The business owner has to manually draft the document, upload it, and email it to the client. It adds friction to the sales process.
- **Shopify / Wix Legal Generators:** Provide generic templates for Privacy Policies and Terms of Service, but do not offer dynamic, context-aware contracts for services, custom orders, or bookings.
- **LegalZoom:** Expensive, one-off document creation. Not integrated into the daily flow of business operations.

**Gaps Identified:**
There is no platform that treats legal protection as an automated, invisible layer of the checkout/booking process. OHC needs a "Protector" engine that dynamically drafts contracts based on the specific items in a cart or the nature of a booking, presents them for a seamless "tap-to-agree" or mobile e-signature, and immutably stores the record to defend against disputes.

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    subgraph Mobile Device (375px)
        App[OHC Mobile App] --> CheckoutUI[Checkout / Booking UI];
        CheckoutUI --> ConsentUI[Glassmorphism E-Signature Modal];
    end

    App -- "Initiate Transaction" --> Gateway[OHC API Gateway];

    Gateway --> LegalEngine[Invisible Legal Engine];
    LegalEngine --> Ledger[(Universal Immutable Ledger)];

    Gateway --> Agents[AI Agent Swarm];

    subgraph Agent Departments
        Agents --> LegalAgent[The Protector: Drafts Contracts & Policies];
        Agents --> FinanceAgent[The Treasurer: Links Consent to Payment];
        Agents --> OpsAgent[The Manager: Blocks Ops until Signed];
    end

    LegalEngine -- "Audit Trail" --> Ledger
```

### Mobile UX Flow (375px First)
1. **Trigger (Business Owner):** Carlos is setting up a new service ("High-Voltage Repair") on his OHC app. He toggles a switch: "Requires Liability Waiver."
2. **Drafting (Invisible):** The Legal Agent (The Protector) autonomously drafts a localized waiver based on Carlos's business address and the service description. Carlos doesn't have to read legal jargon; he just sees a badge saying "Protected by OHC Legal."
3. **Checkout (Customer):** A customer books the repair. During the mobile checkout, right before the Apple Pay / Tap-to-Pay step, a clean bottom sheet slides up. It presents a plain-language summary: "By booking, you agree that Carlos Handyman Services is not liable for pre-existing electrical faults."
4. **E-Signature:** The customer signs with their finger or taps "I Agree" natively in the UI. No redirecting to a clunky third-party PDF viewer.
5. **Storage:** The cryptographic hash of the agreement, the timestamp, and the customer's IP/Identity are stored immutably in the OHC Ledger, ready to be used by the Dispute Resolution Engine if a chargeback occurs.

### AI Agent Integration Points
- **The Protector (Legal AI):** Monitors the business's catalog and location. It dynamically generates required policies (GDPR banners for EU, CCPA for California) and service contracts. It also scans Fatima's profile and alerts her 30 days before her local food cart permit expires.
- **The Treasurer (Finance AI):** Will not capture the final payment intent or release funds from escrow until The Protector verifies that the required contract has been cryptographically signed.
- **The Ambassador (Customer Success AI):** If a customer requests a refund that violates the signed cancellation policy, the agent automatically and politely replies with a reference to the agreed-upon terms, saving the owner from an awkward conversation.

### Key Design Decisions & Security
- **Zero-Friction E-Signatures:** Traditional PDFs are terrible on mobile. The Legal Engine renders contracts as responsive, native UI components that pass the "grandmother test."
- **Immutable Audit Trail:** Consent records must be stored with cryptographic proofs (SPIFFE identity + timestamp + document hash) to ensure they hold up in dispute mediation (e.g., Stripe Chargebacks).
- **Multi-Tenant Isolation:** Legal documents contain sensitive PII. The architecture guarantees row-level and object-storage isolation so tenant A's contracts can never be accessed by tenant B.

## Implementation Prompt
Implement the Invisible AI Legal & Compliance Engine.
- **User-Facing Outcome:** Business owners can attach auto-generated legal requirements to their products or services with a single tap. Customers sign these dynamically generated agreements via a frictionless, mobile-native UI during checkout.
- **CUJ:** Carlos adds a "Requires Liability Waiver" toggle to his repair service. A customer books the service on their phone, is presented with a native "tap-to-agree" summary of the AI-generated waiver, and completes the booking. The signed agreement is stored immutably on Carlos's ledger.
- **Acceptance Criteria:**
  - Ensure the e-signature/consent UI is mobile-first, adhering to the 375px baseline and OHC Glassmorphism design system (no PDF viewers).
  - The Legal AI Agent must be able to generate plain-language summaries and full legal texts based on product/service metadata.
  - Signed agreements must be hashed and stored immutably with multi-tenant isolation.
  - The checkout process must conditionally block payment capture until the required legal consent is registered.

## Priority
P1

## Estimated Scope
Large

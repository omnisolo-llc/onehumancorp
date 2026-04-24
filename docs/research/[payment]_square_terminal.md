# Scout: Payment Processing (Square Terminal)

## Title
Unified In-Person & Online Payments 💳 (Square Terminal API Integration)

## Problem Statement
Small business owners like Priya (the boutique owner) and Carlos (the handyman) operate in a hybrid world. They need to accept payments both online (via their storefront) and in-person (via a card reader or phone tap). Currently, managing two separate payment systems (like Stripe for web and a different POS for physical) leads to fragmented inventory, inconsistent financial reporting, and double entry. OHC needs a way to bridge this gap seamlessly.

## Research Report
- **Goal**: Evaluate Square Terminal API and Mobile Payments SDK as the primary in-person payment solution for OHC.
- **Features evaluated**:
  - **Terminal API**: Programmatically trigger payments on physical Square Terminal hardware.
  - **Tap to Pay**: Enable iPhone and Android devices to accept contactless payments directly.
  - **Unified Catalog**: Sync OHC products to Square's catalog for consistent pricing across all channels.
  - **Offline Payments**: Ability to process payments in low-connectivity areas (e.g., Carlos at a remote job site).
- **Benefits for OHC users (Non-technical)**:
  - Professional physical checkout experience for customers.
  - "One truth" for all sales: whether sold on the web or in the shop, it's all in the OHC dashboard.
  - No need for expensive hardware; they can start with "Tap to Pay" on their existing phone.
- **Integration Risks**:
  - Hardware pairing (Bluetooth/Network) can be tricky for non-technical users.
  - Square's API is extensive and requires careful mapping to OHC's multi-tenant architecture.
- **Pricing**: Standard processing fees per transaction; no monthly software fees for basic API usage.
- **Cloud vs Standalone**: Native support for Cloud mode. For Standalone, the Mobile Payments SDK can run directly within the OHC mobile app, communicating with local SQLite for transaction records.

### Persona Pain Point Summary
| Persona | Pain Point | Solution via Square Integration |
|---------|------------|---------------------------------|
| **Priya (Boutique)** | Sells a dress in-store, but it's still showing as "In Stock" on her OHC website. | Square Catalog sync ensures that an in-person sale instantly updates her OHC inventory. |
| **Carlos (Handyman)**| Has to wait for customers to "pay the invoice later" after he finishes a job. | Carlos uses "Tap to Pay" on his phone to collect payment immediately upon completion. |

## Design Doc
- **Component**: `PhysicalPaymentService`
- **Responsibilities**:
  - Integrate Square Mobile Payments SDK for Flutter.
  - Provision Square "Locations" for each OHC tenant.
  - Handle the "Terminal API" flow to push order totals to a physical card reader.
  - Reconcile physical transactions with OHC's Finance department records.
- **User Experience**:
  - A "Take Payment" button in the OHC Mobile App.
  - The phone screen prompts the customer to "Tap or Insert Card."

## Implementation Prompt
"Implement the Square Terminal API and Mobile Payments SDK integration in `src/server/integrations/square/`. Create a Flutter bridge in `src/app/lib/services/payment/` that allows the OHC app to initiate a physical payment. Ensure that successful transactions are recorded in the OHC database with the metadata 'channel: in-person'. Acceptance criteria: A user can select an order in the OHC app, tap their phone (using Tap to Pay), and see the order marked as 'Paid' in their dashboard."

## Priority
P0

## Estimated Scope
Large

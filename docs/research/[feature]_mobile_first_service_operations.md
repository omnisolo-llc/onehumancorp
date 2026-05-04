# Issue Brief: Mobile-First Service Operations

## Problem Statement
Leo (music tutor) and Carlos (handyman) need to manage their entire business from a phone, but current tools for quoting and booking are "desktop-first" and high-friction for mobile use.

## Research Report
- **Competitive Landscape:** Square Appointments is good but siloed. Shopify is overkill for pure service businesses.
- **User Pain:** Carlos loses leads because he can't send a professional quote while standing on a ladder.
- **Mobile Benchmark:** All touch targets must be ≥ 44px; flow must be completed in < 3 taps.

### Comparative Table: Service Operations
| Feature | OHC | Square | Shopify |
| :--- | :--- | :--- | :--- |
| **Quoting** | AI-Drafted (1-Tap) | Manual Entry | Requires Add-on |
| **UX Target** | 375px Native | Hybrid | Desktop-First |
| **Payment Link** | Inline with Quote | Separate Flow | Checkout Only |

```mermaid
graph LR
    A[Inquiry] --> B{Manager Agent}
    B --> C[Draft Quote]
    C --> D[Owner Approval]
    D --> E[PDF + SMS Link]
    E --> F[Client Deposit]
```

## Design Doc
### High-Level Architecture
- **Ops Agent (The Manager):** Handles the "Quote -> Book -> Pay" state machine.
- **1-Tap Quoting:** Owner selects "Service Type" + "Complexity" -> AI generates a beautiful PDF quote sent via SMS/WhatsApp.
- **Unified Calendar:** Merges personal Google/iCal with business bookings to prevent double-booking.

## Implementation Prompt
Create a "Quick Quote" flow in the mobile Slint UI. A user should be able to select a service, and the "Salesperson" agent should draft a professional estimate based on past similar jobs. Upon owner approval, it is sent to the customer with an "Instant Deposit" link.

## Priority
P0

## Estimated Scope
Medium

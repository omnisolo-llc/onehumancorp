# Title: Autonomous AI Quoting and Deposit Engine

## Problem Statement
Service providers and custom product creators (like handymen, bakers) frequently complain on Reddit about the manual back-and-forth required to finalize a quote, take a deposit, and schedule a delivery. This friction leads to missed sales and lost time. Current platforms either lack this capability or require expensive, clunky 3rd-party apps.

## Research Report
- **Findings:** Based on analysis of user reviews on Trustpilot and Reddit discussions (e.g., r/smallbusiness), solopreneurs need a system that acts as a salesperson, not just a static form.
- **Competitive Comparison:** Shopify requires multiple paid apps (e.g., Globo Request a Quote + deposit app) to achieve this. Wix Bookings is rigid and doesn't handle custom, free-text quote negotiations well.
- **Sources:** Trustpilot (Shopify reviews), Reddit (r/sweatystartup, r/smallbusiness), Competitor sites (Shopify App Store).

## Design Doc
- **Core Entities:** `QuoteRequest`, `Quote`, `PaymentIntent`.
- **UI Flow (Mobile First - 375px):**
  1. Customer submits a natural language request on the storefront ("I need a vegan chocolate cake for 20 people on Saturday").
  2. Owner receives a push notification: "Agent drafted a quote for a Vegan Cake. Review?"
  3. Owner taps notification. Sees a beautiful glassmorphism card (OHC Premium Token library) with the AI-suggested price, line items, and required deposit amount.
  4. Owner taps "Approve & Send".
  5. Customer receives an SMS/Email with a 1-tap Apple Pay/Google Pay checkout link.
- **AI Agent Integration:** The "Salesperson" Agent parses the free-text customer request, queries the business's `memory` (past quotes, inventory, pricing strategy), and structures the `Quote`.

## Implementation Prompt
Implement the Autonomous Quoting Engine. The system must allow a customer to submit a free-text request. The Salesperson AI must process this request, generate a structured quote with a deposit requirement, and present it to the owner in a mobile-optimized (375px) UI for 1-tap approval. Upon approval, it must generate a Stripe payment link. The UI must use the OHC Premium Token library (Glassmorphism, Outfit font).

## Priority
P0

## Estimated Scope
Large

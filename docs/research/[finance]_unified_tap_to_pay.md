# Issue Brief: Unified Tap-to-Pay with Proactive Agent Inventory Sync

## Title
Unified Mobile Tap-to-Pay & Proactive Inventory Sync

## Problem Statement
Small business owners (like Priya the boutique owner) who sell both in-person and online are forced to use disjointed systems (e.g., Square for in-person, Shopify for online), leading to inventory discrepancies, double-selling, and manual reconciliation fatigue. They need a single, simple mobile interface to accept in-person payments that automatically syncs with their online storefront via AI agents without needing separate POS hardware.

## Research Report
- Evaluated Shopify POS, Square POS, and Wix.
- Found that app-switching and manual inventory updates are a primary driver of SMB churn.
- Tap-to-Pay on existing hardware (iPhone/Android) eliminates a significant adoption barrier (hardware cost).
- Our user research (r/smallbusiness, Trustpilot) confirms "out of stock online" refunds are a major source of customer dissatisfaction (45% of omni-channel friction).

## Design Doc
- **Integration:** Embed Stripe Terminal Tap-to-Pay SDK within the OHC mobile app.
- **UX Flow (375px):** "Take Payment" floating action button -> Enter Amount / Select Product -> Customer Taps Phone -> Success. All touch targets ≥ 44x44px.
- **AI Agent Hook:** The `PaymentProcessed` event must fire a message to the KAIROS Event Mesh. "The Manager" (Operations) picks this up, deducts the inventory universally, and if stock drops to 0, proactively flags it as "Sold Out" on the public website.
- **Advisory Hook:** "The Advisor" uses this data to recommend re-orders in the weekly briefing.

## Implementation Prompt
Implement the "In-Person Payment" flow for the OHC mobile app using the Stripe Terminal Tap-to-Pay SDK. Create a new `PaymentProcessed` event in the core event mesh. Ensure "The Manager" agent is subscribed to this event to autonomously decrement inventory and update the online storefront status without user intervention. Provide a 375px mobile-first UI for entering the transaction amount or selecting a product from the catalog.

## Priority
P1

## Estimated Scope
Medium

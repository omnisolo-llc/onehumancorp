# Title: Integrate Stripe for Global Online and In-Person Payments

## Problem Statement
Service providers like Carlos and retail owners like Maya need a reliable way to get paid. Navigating traditional merchant accounts is complex, and failing to offer convenient payment methods (like Apple Pay, credit cards, or local alternatives) results in lost sales. They need an easy way to generate invoices, accept online payments, and take in-person payments.

## Research Report
**Tool Evaluated:** Stripe
**Ease of Use:** Extremely high for the end-consumer. For the business owner, the onboarding is streamlined via Stripe Connect.
**Key Features:** Payment Links, Invoicing, Terminal (for physical hardware), support for 135+ currencies and dozens of local payment methods.
**Pricing:** Standard pay-as-you-go model (typically 2.9% + 30¢ per successful card charge), which is standard and acceptable for most SMBs avoiding monthly fees.
**Reputation:** The industry gold standard for payment processing. Highly reliable (99.999% uptime).
**Environments:** Works perfectly in Cloud mode. Standalone mode can utilize Stripe Terminal SDKs for local, physical hardware transactions while syncing data to the cloud.

## Design Doc
**Trigger:** User clicks "Get Paid" in the OHC dashboard.
**Action:** User enters an amount and description. OHC generates a Stripe Payment Link.
**User Experience:** The owner sees a "Payments" tab. They can quickly generate a link to text to a client, or they can tap a button on their phone/tablet to initiate a "Tap to Pay" transaction for in-person sales.

## Implementation Prompt
Integrate Stripe via Stripe Connect to handle payment processing for OHC users. Implement a feature allowing users to generate shareable Payment Links for specific amounts. Additionally, build a simple "Invoice" view where users can track which payment links have been fulfilled. Keep the interface focused on "Create Payment Link" and "Track Earnings" without exposing the underlying API complexity.

## Priority
P0

## Estimated Scope
Large
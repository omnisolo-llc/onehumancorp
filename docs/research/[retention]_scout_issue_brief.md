# Issue Brief: Automated SMS Abandoned Cart Recovery System

## Problem Statement
Small businesses lose a massive percentage of potential revenue to abandoned checkout flows. While larger platforms emphasize email recovery, SMS has a 98% open rate compared to email's 20%. Solopreneurs rarely set up SMS campaigns because it requires integrating and configuring complex third-party tools like Klaviyo.

## Research Report
The friction of setting up a third-party SMS marketing tool, configuring webhooks, and writing templates is entirely too high for a solopreneur. Embedding SMS recovery directly into the core platform with sensible, pre-configured defaults (e.g., automatically sending a 10% discount text exactly 1 hour after abandonment) provides instant, measurable ROI without any configuration required from the user.

## Design Doc
**High-Level Architecture & Entities:**
- Background worker tracking `Cart` object age and state.
- Integration with an SMS gateway (e.g., Twilio).
- `ComplianceRecord`: Logic to handle opt-outs (STOP requests) to ensure legal compliance.

**Mobile UX Flow:**
1. **User Action:** Customer adds an item to cart, enters their phone number during the first step of checkout, but fails to complete payment.
2. **System Trigger:** One hour passes automatically.
3. **Delivery:** System automatically dispatches SMS: "Hi! You left something behind at Maya's Bakery. Complete your order here for 10% off: [secure link]".
4. **Conversion:** Customer clicks link, restores session, and completes checkout.

**AI Agent Integration Points:**
- AI optimizes the timing and copy of the SMS based on customer behavior and demographic data.

## Implementation Prompt
Implement a robust background worker system that continually polls for abandoned carts and automatically dispatches SMS recovery messages based on pre-configured templates. Ensure strict architectural compliance with SMS marketing regulations (handling STOP requests natively).

**Critical User Journey (CUJ):**
1. Cart session is created and left in 'abandoned' state.
2. Background job identifies the cart after threshold time elapses.
3. System formats message payload and dispatches to SMS gateway.
4. System logs the recovery attempt.

**Acceptance Criteria:**
- A cart abandoned for a specified mock duration correctly triggers an SMS payload to a mock notification gateway.
- The system must implement robust idempotency (never send the same recovery text twice for the same cart).
- The system must successfully process a mock 'STOP' webhook and prevent future messages to that number.

## Priority
P2

## Estimated Scope
Medium

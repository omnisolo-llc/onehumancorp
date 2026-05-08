**Title**: Integrate Twilio for OHC

## Problem Statement
My customers often don't check their emails. I need to send them appointment reminders and order updates via text message so they actually see them.

## Research Report
**Tool Evaluated:** Twilio

**Findings:** Twilio is the industry standard for programmable SMS. It provides reliable global delivery, handles opt-outs automatically, and has extensive documentation. Registration for A2P 10DLC (US) can be a hurdle for small businesses but Twilio Trust Hub assists with this.

**Pricing:** Pay-as-you-go, approx $0.0079 per SMS in the US + monthly phone number fee.

**Cloud vs Standalone Mode:** Works in both modes via standard API calls.

## Design Doc
OHC will provision a Twilio subaccount and phone number for the business owner. Critical alerts (appointment reminders, order shipped) are sent as SMS via the Twilio API.

## Implementation Prompt
Implement a Twilio SMS integration that automatically sends appointment reminders and order confirmations to customers. The business owner should simply toggle 'Enable SMS Notifications' without managing Twilio accounts directly.

## Priority
P0

## Estimated Scope
Medium

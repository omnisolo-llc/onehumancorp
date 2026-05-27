# [SMS & Notifications] Twilio Integration

## Title
Integrate Twilio for Global SMS Alerts & Customer Notifications

## Problem Statement
Fatima the Food Cart Operator doesn't have a reliable internet connection at her cart and relies on SMS text messages to know when a pre-order arrives. She needs reliable notifications to avoid missing orders or push notifications.

## Research Report
- **Strategy**: Direct API integration with Twilio
- **Target Persona**: Fatima (Food Cart Operator)
- **Advantages**: Twilio is the industry standard for SMS messaging globally. Incredibly reliable, programmable, and cheap per-message cost.
- **Risks**: A2P 10DLC compliance in the US requires business registration, potentially tough for informal businesses.
- **Pricing**: Pay-as-you-go (~$0.0079 per SMS in US).
- **Compatibility**: Cloud (Centralized OHC Twilio account); Standalone (User provides API key).

## Design Doc
- Users can enable "SMS Notifications" in the "Operations" settings.
- When an order is placed, the OHC backend triggers a Twilio API call to text the business owner.
- Additionally, "The Ambassador" can send order confirmation texts to customers who prefer SMS over email.

## Implementation Prompt
Add Twilio integration to dispatch SMS order notifications to the business owner and provide SMS-based order updates to end customers.

## Priority
P0

## Estimated Scope
Small

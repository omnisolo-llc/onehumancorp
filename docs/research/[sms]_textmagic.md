# [SMS] TextMagic for Global Reach

## Title
🔍 Scout: Integrate TextMagic for Global Pay-As-You-Go SMS

## Problem Statement
Fatima (Food Cart Operator) needs to tell her customers "Your order is ready!" She doesn't want a complex monthly subscription. She wants a simple "top up and send" model that works globally without jumping through technical hoops.

## Research Report
- **Tool**: TextMagic
- **Target Persona**: Fatima (Food Cart Operator), Carlos (Handyman).
- **Value Proposition**: TextMagic is designed for simplicity. It is an approachable choice for owners who aren't technical experts.
- **Key Advantages**:
  - **No Monthly Fees**: Pure pay-as-you-go model.
  - **Global Two-Way SMS**: Customers can reply to alerts.
  - **Extreme Simplicity**: Designed for people who don't want technical complexity.
  - **High Reliability**: Multiple ways to deliver messages.
- **Risks**: SMS rates vary by region.
- **Pricing**: Transparent pay-as-you-go pricing.
- **Compatibility**: Works perfectly in both Cloud and Standalone modes.

## Design Doc
- **User Experience**:
  - The owner goes to "Notifications" and connects TextMagic.
  - They buy a small amount of credit.
  - In the dashboard, they tap "Notify Customer" when an order is ready.
  - TextMagic delivers the SMS.
  - Customer replies appear in the OHC inbox.
- **Visuals**: A simple balance indicator on the dashboard.

## Implementation Prompt
Implement the TextMagic integration to support outbound and inbound SMS notifications. Create a simple top-up interface. Enable automated SMS alerts for common events. Ensure that inbound customer replies are captured and displayed within the OHC communication inbox.

## Priority
P2

## Estimated Scope
Medium

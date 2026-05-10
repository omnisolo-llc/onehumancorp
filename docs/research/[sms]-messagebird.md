# Title: Reliable SMS Notifications via MessageBird
## Problem Statement
Many customers, particularly in certain demographics or regions, prefer text messages over emails for critical updates like order confirmations or appointment reminders. Business owners need a simple way to send these texts.

## Research Report
**Tool Evaluated:** MessageBird (Bird)
- **Ease of Use:** High, excellent developer API and straightforward dashboard.
- **Pricing:** Pay-as-you-go, very competitive international rates.
- **Reputation:** Strong global player in omnichannel communications.
- **Advantages:** Excellent global carrier coverage, high delivery reliability, easy opt-out compliance management.
- **Risks:** Regulatory compliance for SMS (like A2P 10DLC in the US) can be confusing for owners.
- **Environment:** Fully supported in Cloud and Standalone.

## Design Doc
MessageBird will be used as the backend provider for SMS notifications. Business owners will just see a toggle to "Send SMS Updates" in OHC. OHC will handle the routing, formatting, and compliance metadata required by MessageBird to ensure delivery.

## Implementation Prompt
Integrate MessageBird to enable automated SMS notifications for key events (like order shipped or appointment confirmed). Abstract the technical setup so the business owner only has to enable a setting and write a simple message template.

## Priority
P1

## Estimated Scope
Medium

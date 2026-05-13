# [Video Conferencing] Microsoft Teams Integration

## Title
Microsoft Teams Integration for B2B Consultations

## Problem Statement
Marcus the Consultant conducts most of his meetings with corporate clients who mandate Microsoft Teams for security reasons. Currently, he has to manually create Teams links and paste them into OHC calendar invites.

## Research Report
- **Strategy**: Integration with Microsoft Graph API to create Teams meetings.
- **Advantages**: Critical for B2B users. Integrates tightly with the Outlook Calendar sync.
- **Risks**: Requires Microsoft 365 Business or Enterprise accounts to create online meetings via API.
- **Pricing**: Included with Microsoft 365 subscriptions.
- **Ease of Use**: Seamless if the user already uses the Microsoft ecosystem.
- **Compatibility**: Cloud and Standalone.

## Design Doc
- User authorizes Microsoft Teams via OAuth.
- When a customer booked a consultation, OHC automatically generates a unique Teams meeting link via the Graph API.
- The link is embedded in the confirmation email and the OHC calendar event.

## Implementation Prompt
Integrate the Microsoft Graph API to dynamically generate Microsoft Teams meeting links for scheduled consultations. Attach these links to calendar invites and confirmation notifications.

## Priority
P1

## Estimated Scope
Medium

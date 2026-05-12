# Issue Brief: Zoom Meeting Generation

## Title
Implement Zoom Meeting Generation for Small Business Owners

## Problem Statement
Everyone knows Zoom. Some clients specifically ask for a Zoom link because it is familiar and they already have the app installed.

## Research Report
The integration automatically creates a Zoom meeting when a virtual service is booked.

**Persona Impact:** The business owner doesn't have to manually open the Zoom app, click 'Schedule', copy the link, and paste it into an email. OHC handles all of it automatically in the background.

**Advantages:** Universal brand trust. Extremely reliable video quality.

**Risks:** The customer is forced to leave the OHC ecosystem and open a third-party app, breaking the seamless brand experience.

**Pricing Estimate:** Requires the business owner to have their own paid Zoom account if meetings last longer than 40 minutes.

**Environment:** Cloud and Standalone supported.

## Design Doc
1.  **Zoom Login:** A simple 'Connect Zoom' button in the settings.
2.  **Link Injection:** Automatically add the generated Zoom join URL to the automated email confirmations sent to the customer.

## Implementation Prompt
Integrate Zoom to automate the creation of meeting links for virtual appointments, saving the user from manual copy-pasting.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
If the user has a free Zoom account, the OHC booking system must automatically prevent them from accepting group class bookings that exceed Zoom's 40-minute limit, preventing awkward disconnections mid-session.

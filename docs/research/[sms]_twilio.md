# Issue Brief: Automated SMS Reminders

## Title
Implement Automated SMS Reminders for Small Business Owners

## Problem Statement
A dog groomer suffers from clients forgetting their appointments. 'No-shows' cost her hundreds of dollars a week.

## Research Report
Twilio allows sending automated text messages globally.

**Persona Impact:** The groomer sets a rule in OHC: 'Send a reminder 24 hours before the appointment'. The client receives a text. The groomer's no-show rate drops to near zero, directly increasing her income.

**Advantages:** SMS has a near 100% open rate, vastly superior to email for urgent reminders.

**Risks:** The US government requires strict business registration (A2P 10DLC) to send business texts. If this process is confusing, the groomer will give up.

**Pricing Estimate:** A few cents per message. Very high ROI for preventing missed appointments.

**Environment:** Works in both Cloud and Standalone modes.

## Design Doc
1.  **Registration Wizard:** A very simple, plain-English form in OHC that asks for the business's tax ID and address to handle regulatory registration in the background.
2.  **Automated Triggers:** Simple toggles to enable '24 Hour Reminder' texts.

## Implementation Prompt
Integrate automated SMS capabilities so businesses can drastically reduce appointment no-shows. Focus entirely on making the required legal registration process invisible for the user.

## Priority
P0

## Estimated Scope
Large

### Unique Considerations
The integration must support two-way SMS. If the client replies 'I need to cancel' to the reminder text, that message must appear in the OHC Unified Inbox so the groomer can re-book the slot immediately.

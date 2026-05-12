# Issue Brief: Loops Simple Campaigns

## Title
Implement Loops Simple Campaigns for Small Business Owners

## Problem Statement
Users find Mailchimp too complex and cluttered for simply sending a quick 'We are closed today' update to their customers.

## Research Report
Loops provides a very clean, straightforward interface for sending email updates.

**Persona Impact:** A much gentler learning curve. The user can quickly draft a plain-text update and blast it to their customer base without navigating complex campaign builders.

**Advantages:** Clean, modern, and very fast to use.

**Risks:** Less brand recognition among traditional small businesses.

**Pricing Estimate:** Very generous free tier.

**Environment:** Supported in both Cloud and Standalone deployments.

## Design Doc
1.  **Integration Panel:** A simple toggle in the OHC settings to 'Sync Contacts to Loops'.
2.  **Status Indicator:** A clear visual cue showing '500 Customers Synced'.

## Implementation Prompt
Integrate Loops to provide users with a simpler, cleaner alternative for sending mass email updates to their synchronized customer list.

## Priority
P2

## Estimated Scope
Medium

### Unique Considerations
Loops relies heavily on custom events. The OHC integration should expose a way for the business owner to trigger a Loops sequence when an invoice is marked as 'Overdue', completely automating their collections process.

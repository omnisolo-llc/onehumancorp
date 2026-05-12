# Issue Brief: Mailchimp Newsletter Sync

## Title
Implement Mailchimp Newsletter Sync for Small Business Owners

## Problem Statement
A local retail shop collects email addresses. To send a promo, the owner has to manually export a CSV from OHC and import it into Mailchimp, a tedious chore they often skip.

## Research Report
Mailchimp is a popular tool for designing and sending mass email newsletters.

**Persona Impact:** The shop owner just logs into Mailchimp and clicks 'Send'. Their entire OHC customer list is already there, perfectly synced and up-to-date.

**Advantages:** Mailchimp is highly recognized. Users know how to use it to design beautiful emails.

**Risks:** The user must manage two separate platforms.

**Pricing Estimate:** Free tier available for small lists, making it very accessible.

**Environment:** Works in both Cloud and Standalone modes.

## Design Doc
1.  **Connect Account:** A simple 1-click OAuth connection.
2.  **Background Sync:** A silent, automatic process that ensures every new customer added to OHC is instantly added to the Mailchimp audience list.

## Implementation Prompt
Build a silent, automatic contact sync to Mailchimp so business owners can seamlessly send marketing newsletters to their OHC customer base without manual data entry.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
The sync must handle Mailchimp's strict 'Cleaned' or 'Unsubscribed' statuses. If a customer unsubscribes from a newsletter in Mailchimp, OHC must respect that status locally to prevent accidental spamming from other parts of the platform.

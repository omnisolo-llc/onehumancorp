# Email Marketing: Mailchimp

## Problem Statement
Business owners want to send monthly newsletters or promotions to their customer list, but they don't know how to export contacts, format HTML emails, or track who opened what. They just want a "Send blast to all customers" button.

## Research Report
Mailchimp offers a robust API for audience management and campaign creation.
- *Ease of Use*: Good, though their API has a learning curve.
- *Pricing*: Free tier up to 500 contacts, which is sufficient for many new small businesses.
- *Reputation*: Very strong, reliable delivery rates, strict spam compliance.

## Design Doc
- *Trigger*: User hits "Sync Contacts" or "Create Campaign" in the OHC Marketing tab.
- *Action*: OHC syncs the local customer database to a Mailchimp Audience. For campaigns, OHC can trigger a draft campaign creation via API.
- *User Interface*: A "Marketing" tab showing total synced contacts and recent campaign stats (open rate). A button to "Design Email in Mailchimp".

## Implementation Prompt
Build a Mailchimp integration that automatically syncs the OHC customer list to a Mailchimp Audience. Whenever a new customer is added in OHC, they should be added to Mailchimp. Display basic metrics (total subscribers) in the OHC Marketing dashboard.

## Priority
P1

## Estimated Scope
Medium

## Environment Support
Cloud, Standalone.

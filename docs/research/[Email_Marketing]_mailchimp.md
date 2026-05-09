# Mailchimp Integration for Customer Campaigns

## Title
Sync Customer Lists with Mailchimp for Email Campaigns

## Problem Statement
Small business owners often maintain a list of customers in one system and have to manually export and import CSV files into their email marketing tool to send newsletters or promotions. This manual process is tedious, prone to errors, and means marketing lists are always out of date. They need a seamless way to keep their customer database synced with their email marketing platform automatically.

## Research Report
Mailchimp is a major player in the email marketing space, offering tools to create beautiful emails, automate campaigns, and track performance. Research indicates its pricing strategy is structured across plans: Free, Essentials, Standard, and Premium. While it starts at $0, costs scale based on contact count (e.g., $13-$135/month for typical small businesses). Recent price hikes and hidden fees for duplicate/unsubscribed contacts are noted risks.

Despite pricing concerns for scaling businesses, it remains one of the most popular tools for beginners due to its drag-and-drop editor and ease of use. Integrating Mailchimp allows OHC to leverage a powerful external tool for campaign creation rather than building a complex editor from scratch. The API is robust and supports both Cloud and Standalone integrations well.

## Design Doc
The user will connect their Mailchimp account via API key or OAuth in the OHC settings. Once linked, the user can select an OHC customer segment (e.g., "All Active Clients") and map it to a specific Mailchimp Audience. OHC will keep this audience synchronized in the background. If a new customer is added to OHC, they are automatically pushed to Mailchimp. If a customer unsubscribes via a Mailchimp email, that status is synced back to OHC to ensure compliance.

## Implementation Prompt
Build a one-way or two-way sync between OHC customer lists and Mailchimp Audiences. Allow the user to connect their account and select which OHC tags/segments correspond to which Mailchimp lists. Ensure that unsubscribe events from Mailchimp are reflected in the OHC customer profile to prevent future unwanted communications.

## Priority
P2

## Estimated Scope
Medium

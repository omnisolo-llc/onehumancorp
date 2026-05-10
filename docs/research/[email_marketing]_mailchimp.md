# Title: Automated Customer Re-engagement Email Campaigns

## Problem Statement
Boutique owners like Priya want to email past customers when new stock arrives, but managing exports, lists, and external campaign tools is too complex. They need an automated way to email customers directly from OHC.

## Research Report
- **Tool Evaluated**: Mailchimp
- **Persona Value**: High for marketing and retention.
- **Advantages**: Market leader, robust API, excellent tagging and segmentation, high deliverability.
- **Risks**: Strict anti-spam policies might penalize users if bad lists are imported.
- **Pricing**: Free tier (up to 500 contacts). Essentials starts at $13/mo.
- **Cloud vs Standalone**: Cloud (OAuth). Standalone (API Key).

## Design Doc
- **Integration Trigger**: Customer makes a purchase in OHC.
- **Action**: Customer is auto-added to the Mailchimp audience with relevant tags. Marketing agent suggests campaigns based on events (e.g., new stock).
- **User Interface**: Marketing dashboard showing suggested AI drafts, open rates, and clicks.

## Implementation Prompt
Build an integration that syncs OHC customers to a Mailchimp audience automatically after purchase. Allow the AI Marketing agent to create and send email campaigns via the Mailchimp API based on user approval.
- **Acceptance Criteria**: Purchase auto-syncs customer to Mailchimp. User can approve an AI-drafted campaign and send it via OHC.

## Priority
P1

## Estimated Scope
Medium

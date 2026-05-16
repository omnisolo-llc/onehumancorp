# Title: Customer Email Campaign Integration

## Problem Statement
Small business owners struggle to re-engage past customers and announce new products or promotions. They often rely on manual, single emails (which don't scale) or find enterprise email marketing tools too complex and expensive. They need a straightforward way to send beautiful, bulk emails to their existing customer base without worrying about spam compliance.

## Research Report
**Tool Analyzed**: Mailchimp
**Ease of Use**: Very high. Mailchimp is famous for its intuitive drag-and-drop email builder and friendly onboarding process, designed specifically for small businesses.
**Reputation**: One of the oldest and most trusted names in SMB email marketing. High deliverability rates.
**Pricing**: Free tier up to 500 contacts and 1,000 sends/month. Paid plans start at $13/month. Very accessible.
**Environment**: Cloud only. Standalone mode would still require connecting to the Cloud API to actually dispatch the emails and handle bounce/spam tracking.
**AI Integration**: OHC AI agents could automatically draft newsletter content based on recent inventory additions or business milestones, presenting the draft to the owner for one-click approval.

## Design Doc
**Integration Trigger**: The user clicks "Create Campaign" from their Customer/CRM list in OHC.
**Actions Taken**:
- The selected customer list is synced from OHC to a Mailchimp Audience.
- The user is provided an embedded or redirected Mailchimp template editor to design the email.
- Once sent, OHC fetches high-level analytics (open rate, click rate) via webhook/API and displays them.
**User View**: A new "Campaigns" tab in OHC. The user selects an audience segment, writes/designs the email (potentially assisted by AI), and clicks "Send." Later, they see simple metrics like "45% opened this email."

## Implementation Prompt
Integrate Mailchimp for sending email broadcasts. Allow the user to authenticate their Mailchimp account. Create a flow where the user can select a segment of their OHC customers and sync them to a Mailchimp list. Provide a UI to draft a simple text/image email and dispatch it to that list. Finally, display a dashboard widget showing the open rate and click rate for the most recently sent campaign.

## Priority
P2

## Estimated Scope
Large

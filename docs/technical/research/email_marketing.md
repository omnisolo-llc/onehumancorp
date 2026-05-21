# [Email Marketing] Automated Campaign Campaigns

## Problem Statement
Boutique owners like Priya want to notify their existing customers when new stock arrives or a sale starts. Using a separate tool like Mailchimp is complicated, expensive, and requires manually exporting/importing customer lists. They need a built-in way to send beautiful emails to their OHC customer list.

## Research Report
- **Target Tools**: Amazon SES or SendGrid (under the hood for OHC).
- **Competitive Analysis**: Shopify Email provides basic functionality. Dedicated tools (Mailchimp, Klaviyo) are too complex for our personas.
- **Ease of Use**: OHC abstracts the complexity. The "Promoter" AI helps draft the email based on a prompt ("Tell my customers about the summer sale").
- **Pricing**: SES is extremely cost-effective ($0.10 per 1000 emails). We can offer a generous free tier for OHC users.
- **Reputation**: High deliverability when properly configured.
- **Advantages and Risks**: High ROI and keeps users inside the platform. Main risk is users sending spam and ruining the OHC shared domain reputation.
- **Cloud vs Standalone**: Cloud implementation is straightforward (shared SES). Standalone would require users to provide their own SMTP credentials, which is too technical.

## Design Doc
- **Integration Flow**: In the "Marketing & Advertising" department, users can select "Send an Email Announcement."
- **Actions**: The system uses the existing customer list. The AI can draft the subject and body. The system handles unsubscribe links and bounce tracking automatically.
- **User Experience**: A simple interface to draft a message (or have AI draft it), pick an audience (e.g., "All past customers"), and click send. No managing of API keys or domain verification for the user; OHC handles sending from a verified shared domain or the user's connected domain.

## Implementation Prompt
Build a native email marketing feature that allows users to send broadcast emails to their customer list directly from the OHC app. The feature should integrate with the existing customer database, allow for AI-assisted drafting of email content, and automatically handle unsubscribe requests and deliverability tracking. Ensure the user interface is completely free of technical email jargon.

## Priority
P1

## Estimated Scope
Medium

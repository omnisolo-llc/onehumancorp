# Title: Send Professional Newsletters and Promotions with Mailchimp

## Problem Statement
Small business owners have lists of customer emails but don't know how to use them effectively. Sending bulk emails from a standard Gmail or Outlook account looks unprofessional, lacks tracking, and often ends up in spam folders. They need a simple way to send attractive updates or promotions to their customers without needing design skills or worrying about spam compliance.

## Research Report
Mailchimp is one of the most established email marketing platforms for small businesses.
- **Ease of Use**: Excellent drag-and-drop template builder. Non-technical users can create beautiful emails easily. List management is straightforward.
- **Pricing**: The free tier is generous (up to 500 contacts and 1,000 sends/month), making it accessible for very small or new businesses. Paid plans scale with the business.
- **Reputation**: Very high sender reputation, meaning emails are less likely to hit spam. Robust compliance features (unsubscribe links, CAN-SPAM adherence) are built-in automatically.
- **Comparison**: While tools like Resend or SendGrid are great for transactional emails and developers, Mailchimp is far superior for the actual business owner who needs to design a visual newsletter.
- **Cloud vs Standalone**: API integrations for pushing customer data to Mailchimp work perfectly in both Cloud and Standalone modes as OHC is making outbound requests.

## Design Doc
- **Triggers & Actions**: When a new customer is added in OHC (e.g., they make a purchase or book an appointment), their email is synced to a Mailchimp list. The user logs into OHC to see basic campaign stats (open rates) pulled from Mailchimp.
- **User Experience**: In OHC, a "Marketing" tab provides a simple toggle: "Sync Customers to Newsletter List". When enabled, every new customer email is sent to Mailchimp. OHC provides a button "Create Email Campaign" which opens the Mailchimp designer. A dashboard widget shows "Last Email Performance: 24% Opened".

## Implementation Prompt
Integrate customer email syncing and marketing analytics.
- **User-Facing Outcome**: The business owner's customer list in OHC automatically stays in sync with their newsletter list. They can see how many people opened their last email directly on the OHC dashboard.
- **Acceptance Criteria**:
  - A toggle exists to "Sync Customers to Newsletter".
  - Adding or updating a customer in OHC automatically updates the synced list in the email tool.
  - The OHC dashboard displays basic metrics (open rate, click rate) for the most recent sent campaign.

## Priority
P2

## Estimated Scope
Medium

# Simple Email Marketing Integration

**Problem Statement:**
Small business owners want to send updates, promotions, or newsletters to their customer base but find traditional tools like Mailchimp too complex or expensive. They just want a simple way to say "Email all my customers who bought X last month" without dealing with complex list segmentation, HTML builders, or steep subscription fees.

**Research Report:**
- **Evaluated Tools:** SendGrid, Mailgun, Amazon SES.
- **Ease of Use:** As an infrastructure provider, these are invisible to the user. OHC will provide a simple WYSIWYG editor.
- **Pricing:** Very cheap per email (e.g., SES is $0.10 per 1000 emails), making it highly affordable to bundle or pass through.
- **Reputation:** SendGrid and SES have excellent deliverability rates if domain authentication is handled correctly.
- **Cloud vs Standalone:** Requires Cloud infrastructure for reliable sending and DKIM/SPF management. Standalone mode might require users to bring their own SMTP credentials or route through a central OHC relay.

**Design Doc:**
- **Trigger:** Business owner selects "Send Announcement" from their customer list view.
- **Action:** They write a plain-text or simple rich-text email. OHC handles the heavy lifting of batch sending, unsubscribe links, and bounce management via the provider API.
- **User Interface:** A minimalist email composer. A "To" field that defaults to "All Customers" with easy dropdowns for simple filters (e.g., "Customers this month").

**Implementation Prompt:**
Implement a simple email campaign feature that allows the business owner to compose a message and send it to their entire customer list or specific subgroups. The interface must provide a basic text editor. The system must automatically handle appending mandatory "Unsubscribe" links and updating the customer's opt-out status if they click it.

**Priority:** P2
**Estimated Scope:** Medium

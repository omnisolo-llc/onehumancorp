# [Email Marketing] Simple Customer Campaigns with Brevo

**Title**: Implement Brevo for Easy Email Marketing Campaigns

**Problem Statement**:
Small business owners want to send updates, promotions, and newsletters to their customer lists but find tools like Mailchimp overwhelming and overly complex. They need a straightforward way to email their contacts without navigating complicated segmentation or automation rules.

**Research Report**:
- **Evaluated Tools**: Brevo (formerly Sendinblue), Mailchimp, Resend.
- **Findings**: Brevo offers a solid balance of simplicity, features, and an excellent transactional/marketing split. Mailchimp is powerful but has become increasingly bloated and expensive for simple use cases. Resend is very developer-focused, making it harder for non-technical users to build templates without coding.
- **Ease of Use**: Brevo has a user-friendly drag-and-drop template builder that is suitable for beginners.
- **Pricing**: Brevo offers a generous free tier (300 emails/day) and affordable paid plans that charge per email volume rather than contact count, which is ideal for small businesses with growing lists.
- **Cloud vs Standalone**: In Cloud mode, we can manage API keys per tenant. In Standalone mode, users can bring their own Brevo API key to send emails directly from their local machine via the API.

**Design Doc**:
- **Trigger**: The user clicks "Send Email Blast" in the OHC "Customers" or "Marketing" tab.
- **Action**: OHC uses the Brevo API to create a campaign, attach a selected list of contacts, and dispatch the email.
- **User View**: A simple form where the user types a Subject, types their message (or selects a pre-made template), selects "All Customers", and clicks "Send Now". Basic stats (Open Rate, Click Rate) appear on the same page later.

**Implementation Prompt**:
Create a feature that allows users to send bulk emails to their contact list. Provide a simple interface to draft an email (Subject, Body) and send it to all stored customer email addresses. The integration should handle sending the emails reliably and tracking basic metrics like open rates, which should be displayed in a simple history view.

**Priority**: P2
**Estimated Scope**: Medium

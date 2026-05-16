# Simple Email Campaigns via Resend Integration

**Title**: Simple Email Campaigns via Resend Integration
**Problem Statement**: Small business owners need an easy way to send newsletters, promotions, or updates to their customer list without learning complex marketing platforms like Mailchimp.

**Research Report**:
- Resend is a developer-friendly email platform with a strong focus on deliverability and simplicity.
- **Ease of Use**: OHC can abstract the complexity, allowing the user to simply write an email in a rich text editor and click "Send to all customers."
- **Pricing**: Generous free tier (up to 3,000 emails/month), which covers most SMB needs.
- **Reputation**: Excellent deliverability and modern API.
- **Cloud vs Standalone**: Works seamlessly in Cloud mode. Standalone mode might require users to input their own API key.
- **Key Advantages**: High deliverability, simple abstraction for end-users.
- **Key Risks**: Strict spam compliance rules could lead to account suspensions if users abuse the system.

**Design Doc**:
- Users access the "Marketing" tab and select "New Campaign."
- They write their message using a simple editor and select their target audience (e.g., "All Customers" or "Recent Buyers").
- OHC handles the distribution via Resend, displaying basic open-rate analytics back to the user in a digestible format.

**Implementation Prompt**: Integrate Resend to allow business owners to compose and send email campaigns directly to their customer lists, and display basic analytics (open rates, clicks) within the dashboard.

**Priority**: P1
**Estimated Scope**: Medium

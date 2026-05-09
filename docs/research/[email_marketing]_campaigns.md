## [Email Marketing] Issue Brief: Native Email Campaigns

**Title**: Scout 🔍: Automated Email Marketing Campaigns via SendGrid/SES
**Problem Statement**: Small businesses want to keep in touch with past customers (e.g., announcing new products or sales) but find external tools like Mailchimp too complicated and disjointed from their customer data. They need a simple, integrated way to send marketing emails.
**Research Report**:
- **Tools Evaluated**: SendGrid, Amazon SES, Mailchimp, Resend.
- **Evaluation**: Instead of forcing users to use a complex third-party tool like Mailchimp, we should build a native email campaign manager powered by a reliable transactional email API like SendGrid or SES.
- **Ease of Use**: Extremely high. The user creates campaigns directly in OHC, utilizing their existing customer list. The AI Marketing Agent can even draft the emails.
- **Pricing**: SendGrid/SES costs will be absorbed by OHC platform fees or billed transparently based on volume.
- **Cloud vs. Standalone**: Easy in Cloud mode (centralized API keys). For Standalone, we can either proxy through OHC servers or require the user to provide their own SMTP/SendGrid credentials.
**Design Doc**:
- Customer data is automatically tagged and segmented within OHC based on purchase history.
- The user navigates to the "Marketing" tab to draft an email campaign (assisted by AI).
- OHC handles list management and unsubscribe links internally.
- Emails are dispatched via the centralized SendGrid/SES integration.
**Implementation Prompt**: Build a native email campaign tool. Implement a rich text editor (or AI drafter) for email content. Integrate SendGrid or AWS SES for reliable delivery. Implement unsubscribe tracking and bounce handling to maintain sender reputation.
**Priority**: P1
**Estimated Scope**: Large

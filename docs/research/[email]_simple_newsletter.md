# Simple Newsletter to Existing Customers

**Problem Statement:** John (a local gym owner) wants to send a monthly update to all his members, but finds tools like Mailchimp too complicated and expensive. He just wants to select his OHC customer list and send a nice-looking email.

**Research Report:** Mailchimp is powerful but has a steep learning curve. Resend provides a developer-friendly API with high deliverability, while SendGrid is a legacy option. For the small business owner, we can use Resend under the hood to send simple text/HTML emails without them needing to manage a separate email marketing platform.

**Design Doc:** In the Customers tab, the user clicks "Send Email to All". A simple rich-text editor opens. The user types their message and clicks send. OHC uses a provider like Resend to batch-send the emails to the selected customer list.

**Implementation Prompt:** Build a simple email composer UI in the OHC dashboard. Add a "Send Campaign" button that takes the selected customer list and dispatches the email via our email provider, handling unsubscribes automatically.

**Priority:** P2

**Estimated Scope:** Medium

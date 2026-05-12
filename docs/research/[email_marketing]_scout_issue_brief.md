# Email Marketing Research Brief

## Title
Integrated Email Campaign Management

## Problem Statement
Small business owners need to re-engage past customers with promotions, newsletters, and updates. While they collect customer emails, using external tools like Mailchimp requires exporting/importing CSVs, which is tedious and error-prone. They need a simple way to send beautiful emails directly to their customer list without managing separate databases.

## Research Report
### Market Context
Email marketing is highly competitive. Solutions range from simple (Substack) to complex (ActiveCampaign). The primary challenge is deliverability—ensuring emails don't end up in spam folders.

### Tool Evaluations

#### 1. Mailchimp
- **Ease of Use:** High, excellent drag-and-drop editor.
- **Pricing:** Free up to 500 contacts, then gets expensive quickly.
- **Capabilities:** Advanced automations, great analytics, strict compliance enforcement.
- **Reputation:** The 800lb gorilla, but small businesses increasingly resent its pricing model.

#### 2. SendGrid / Mailgun (Transactional APIs)
- **Ease of Use:** Low for end-users, high for developers.
- **Pricing:** Very cheap (e.g., thousands of emails for a few dollars).
- **Capabilities:** Pure delivery infrastructure. No native UI for designing campaigns.
- **Reputation:** Excellent deliverability, requires a custom frontend.

#### 3. Klaviyo
- **Ease of Use:** Moderate. Heavily optimized for e-commerce.
- **Pricing:** High.
- **Capabilities:** Deep Shopify/WooCommerce integrations, SMS capabilities.
- **Reputation:** Best in class for retail, overkill for service businesses.

### Recommended Direction
Build a simple campaign editor within OHC that uses a reliable API (like SendGrid or AWS SES) on the backend. This gives the business owner a seamless experience while keeping sending costs near zero.

## Design Doc
### Trigger & Action
1. **Trigger:** Business owner selects a segment of customers and drafts an email campaign.
2. **Action:** OHC compiles the list, renders the email template, and queues the emails for delivery via the transactional email API.
3. **User View:** A "Campaigns" tab where users can draft emails, select recipients, and view open/click rates.

### Environment Support
- **Cloud Mode:** OHC manages the SMTP infrastructure or API keys.
- **Standalone Mode:** User must provide their own SMTP credentials (e.g., their Gmail or a custom SMTP server) to send campaigns.

## Implementation Prompt
Create an "Email Campaigns" feature tied to the CRM.
- Allow the user to select multiple contacts from their customer list.
- Provide a rich text editor to draft the email subject and body.
- Implement an integration with a dummy/sandbox SMTP server to handle delivery.
- Provide a simple dashboard showing the status of the campaign (Sent, Opened, Failed).
- Must include a mandatory "Unsubscribe" link at the bottom of every email.
- Acceptance criteria include successfully sending an email to a list of 5 contacts and tracking the "sent" status.

## Priority
P2 (Medium)

## Estimated Scope
Large

### Extended Email Marketing Analysis
#### Deliverability & Spam Regulations
Email sending is fraught with compliance risks, such as CAN-SPAM in the US and GDPR in the EU. Small business owners often lack awareness of these laws. The integration must enforce the inclusion of physical business addresses and one-click unsubscribe links. Additionally, bounce handling must be robust, automatically removing hard bounces from lists to protect sender reputation.

#### Rendering Reliability
Email clients (Outlook, Gmail, Apple Mail) render HTML inconsistently. The text editor should output safe, standardized HTML structures. We should avoid complex CSS that might break in older desktop email clients.

#### Analytics and Tracking
To provide value, the business owner must see the impact of their campaigns. The system must inject tracking pixels and rewrite URLs to monitor open rates and click-through rates, while ensuring that Apple's Mail Privacy Protection (MPP) features are taken into account (which artificially inflate open rates).

#### Opt-In Management
Double opt-in mechanisms should be available as an option. The customer CRM must clearly delineate between contacts who have subscribed to marketing emails and those who have only interacted transactionally (e.g., received an invoice).

### User Persona Match
- **Fatima (Boutique Owner):** High value. She runs seasonal promotions and needs to alert her customer base to new arrivals.
- **Carlos (Consultant):** Low value. His communication is primarily 1-on-1, and he rarely sends bulk newsletters.

### Conclusion
By embedding email marketing directly into the CRM, OHC eliminates the friction of list syncing. This empowers business owners to cultivate their customer relationships seamlessly, without paying for expensive third-party subscriptions.

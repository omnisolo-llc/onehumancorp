# Scout 🔍: Tool Integration Research Report

## [Social Media] Issue Brief

**Title**: Scout 🔍: Integrate WhatsApp Business API for Unified Inbox
**Problem Statement**:
Many small business owners, especially outside the US, rely heavily on WhatsApp to communicate with customers. Managing these messages manually is tedious and can lead to missed opportunities.
**Research Report**:
- **Tool**: WhatsApp Business API
- **Evaluation**: The WhatsApp Business API allows businesses to automate and manage their interactions. By integrating it, OHC's 'Customer Success' agent can handle common inquiries.
- **Ease of Use**: Business owners simply authenticate with their Facebook/WhatsApp credentials.
- **Pricing**: Priced per conversation. Usually free for the first 1000 conversations.
- **Cloud vs. Standalone**: Works well in Cloud mode. In Standalone, the user would need to configure their own Meta app.
**Design Doc**:
- The user links their WhatsApp account in the 'Social Inbox' tab.
- Webhooks receive incoming messages.
- The AI Agent generates replies based on the business context.
- Replies are sent back via the API.
**Implementation Prompt**:
Implement the WhatsApp Business integration. Create a UI for connecting the account. Set up webhooks to receive and route messages to the AI agent, and send responses back.
**Priority**: P1
**Estimated Scope**: Medium


## [Calendar] Issue Brief

**Title**: Scout 🔍: Integrate Calendly for Easy Scheduling
**Problem Statement**:
Scheduling meetings, consultations, or classes involves a lot of back-and-forth emails. Business owners need a simple way to let clients book time without conflict.
**Research Report**:
- **Tool**: Calendly API
- **Evaluation**: Calendly handles calendar sync (Google, Outlook) and timezone conversions. Integrating it allows OHC to embed scheduling directly into the business's storefront or chatbot.
- **Ease of Use**: Users just connect their Calendly account via OAuth. No manual calendar setup required.
- **Pricing**: Has a free tier. Paid tiers offer more features and integrations.
- **Cloud vs. Standalone**: Works in both Cloud and Standalone modes via OAuth.
**Design Doc**:
- User connects their Calendly account.
- OHC fetches available event types.
- A booking widget is embedded in the storefront or shared via AI agents.
**Implementation Prompt**:
Integrate the Calendly API. Provide a way for users to link their account via OAuth. Display their available event types and allow embedding the Calendly booking widget on their site.
**Priority**: P1
**Estimated Scope**: Small


## [Email Marketing] Issue Brief

**Title**: Scout 🔍: Integrate Mailchimp for Customer Engagement
**Problem Statement**:
Business owners want to send newsletters and promotions but struggle to manually export their customer list from OHC and import it into an email tool.
**Research Report**:
- **Tool**: Mailchimp Marketing API
- **Evaluation**: Mailchimp is a popular email marketing tool. Integrating it allows automatic syncing of the OHC customer list, ensuring marketing campaigns always reach the right audience.
- **Ease of Use**: Simple OAuth connection. OHC handles the background syncing.
- **Pricing**: Generous free tier. Paid plans based on the number of contacts.
- **Cloud vs. Standalone**: Works in both modes via OAuth.
**Design Doc**:
- User connects their Mailchimp account.
- OHC automatically syncs new customers to a designated Mailchimp audience.
- The 'Marketing' agent can suggest campaign ideas based on customer data.
**Implementation Prompt**:
Implement Mailchimp API integration. Allow OAuth connection. Set up a background sync to push new OHC customers into a specified Mailchimp audience.
**Priority**: P2
**Estimated Scope**: Medium


## [Payment] Issue Brief

**Title**: Scout 🔍: Integrate Paytm for the Indian Market
**Problem Statement**:
Stripe is not always the best option in all regions. Small businesses in India prefer localized payment gateways like Paytm for better acceptance rates and lower fees.
**Research Report**:
- **Tool**: Paytm Payment Gateway
- **Evaluation**: Paytm is widely used in India. Supporting it expands OHC's reach and provides a familiar checkout experience for Indian customers.
- **Ease of Use**: Business owners enter their Paytm API credentials in the settings.
- **Pricing**: Transaction-based fees, generally competitive in the Indian market.
- **Cloud vs. Standalone**: Works in both Cloud and Standalone modes by configuring API keys.
**Design Doc**:
- User configures Paytm credentials in the 'Payments' settings.
- During checkout, customers in India see Paytm as a payment option.
- OHC handles the payment redirect and webhook verification.
**Implementation Prompt**:
Add Paytm as a payment gateway option. Provide fields for the user to input their Merchant ID and Secret Key. Implement the checkout flow and payment verification webhook.
**Priority**: P2
**Estimated Scope**: Medium


## [Shipping] Issue Brief

**Title**: Scout 🔍: Integrate EasyPost for Streamlined Shipping
**Problem Statement**:
E-commerce businesses spend too much time calculating shipping rates, printing labels, and tracking packages manually across different carriers.
**Research Report**:
- **Tool**: EasyPost API
- **Evaluation**: EasyPost provides a unified API for multiple carriers (USPS, UPS, FedEx, etc.). Integrating it allows OHC to offer real-time rates and automated label generation.
- **Ease of Use**: Users enter their EasyPost API key or connect their carrier accounts through the EasyPost dashboard.
- **Pricing**: Priced per label generated. Some carriers have negotiated rates available.
- **Cloud vs. Standalone**: Works in both modes.
**Design Doc**:
- User sets up EasyPost API keys.
- During checkout, EasyPost calculates real-time shipping rates based on package dimensions.
- The 'Operations' agent can automatically generate labels when orders are fulfilled.
**Implementation Prompt**:
Integrate the EasyPost API. Implement real-time rate calculation during checkout. Provide a UI for business owners to generate and print shipping labels directly from the order details page.
**Priority**: P1
**Estimated Scope**: Large


## [Sms] Issue Brief

**Title**: Scout 🔍: Integrate AWS SNS for Reliable SMS Notifications
**Problem Statement**:
Not all customers check their email regularly. For critical updates like order confirmations or appointment reminders, SMS is more effective, especially for non-English speakers.
**Research Report**:
- **Tool**: AWS Simple Notification Service (SNS)
- **Evaluation**: AWS SNS provides a reliable and scalable way to send SMS messages globally. It can be used by OHC agents to send important alerts.
- **Ease of Use**: Requires AWS setup, but OHC can abstract this in Cloud mode.
- **Pricing**: Pay-as-you-go based on the destination country.
- **Cloud vs. Standalone**: Easy in Cloud mode. Standalone users would need their own AWS account.
**Design Doc**:
- OHC configures an SNS topic or uses direct SMS publishing.
- The 'Customer Success' agent triggers SMS messages for specific events (e.g., appointment reminders).
- Users can configure message templates in the settings.
**Implementation Prompt**:
Implement AWS SNS for sending SMS messages. Create a service to handle sending SMS. Allow configuration of message templates and triggers (e.g., new order, appointment reminder).
**Priority**: P2
**Estimated Scope**: Small


## [Video] Issue Brief

**Title**: Scout 🔍: Integrate Zoom for Automated Meeting Links
**Problem Statement**:
Consultants and tutors using OHC need to manually create Zoom links for every booking and send them to clients, which is prone to errors and forgotten links.
**Research Report**:
- **Tool**: Zoom API
- **Evaluation**: The Zoom API allows automatic creation of meetings. Integrating it with the scheduling feature ensures every booking automatically gets a unique Zoom link.
- **Ease of Use**: Users connect their Zoom account via OAuth.
- **Pricing**: Requires a Zoom Pro account for API access.
- **Cloud vs. Standalone**: Works via OAuth in both modes.
**Design Doc**:
- User connects their Zoom account.
- When a new appointment is scheduled, OHC calls the Zoom API to create a meeting.
- The Zoom link is saved to the appointment and automatically emailed/SMSed to the customer.
**Implementation Prompt**:
Integrate the Zoom API. Provide OAuth connection flow. Automatically generate a Zoom meeting when a new appointment is booked, and include the link in the confirmation notifications.
**Priority**: P1
**Estimated Scope**: Medium

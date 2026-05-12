**Title**: SMS & Notifications Integration via Twilio

**Problem Statement**:
Small business owners often have customers who prefer or require SMS communication over email (especially crucial for low-English-proficiency users or areas with low smartphone penetration). Sending order updates or appointment reminders manually via a personal phone is unprofessional and doesn't scale. They need automated, reliable SMS notifications.

**Research Report**:
Twilio is the industry standard for programmable SMS and voice.
- **Ease of Use for Non-Technical Users**: Setting up Twilio directly is highly technical (buying numbers, configuring A2P 10DLC compliance). For OHC to pass the "grandmother test", OHC must act as the platform provider, abstracting away the Twilio account management. Users should just toggle "Enable SMS Notifications".
- **Features**: Global carrier coverage, extremely reliable delivery, handles opt-out compliance (STOP messages) automatically, supports Alphanumeric Sender IDs (in supported countries).
- **Reputation & Reliability**: The most reliable and widely used CPaaS provider globally.
- **Pricing**: Pay-as-you-go. Roughly $0.0079 per SMS in the US, but varies wildly internationally. OHC will need to implement a credit or billing system to pass these costs to the business owner to prevent abuse.
- **Cloud vs Standalone**: Works identically in both. Standalone users could potentially input their own Twilio credentials in an "Advanced" mode if they want direct billing.

**Design Doc**:
- **Trigger**: Specific system events occur (e.g., Order Confirmed, Appointment Reminder 24h before).
- **Action**: OHC backend calls the Twilio Programmable Messaging API to dispatch a templated SMS to the customer's phone number.
- **User View**: The business owner sees an "SMS Notifications" toggle in settings. Customers receive professional, branded text messages.
- **Architecture**: Implement the Twilio Node.js SDK. Define message templates. Crucially, implement phone number formatting (E.164) and validation before sending. OHC Cloud must handle A2P 10DLC registration complexities on behalf of the tenants.

**Implementation Prompt**:
Integrate Twilio Programmable Messaging to send automated SMS notifications for key lifecycle events (e.g., order confirmations, appointment reminders). Abstract the configuration so the business owner only needs to enable the feature. Ensure all outbound phone numbers are properly validated and formatted. Implement basic cost-tracking so SMS usage can be billed to the tenant.

**Priority**: P2 (medium)
**Estimated Scope**: Medium

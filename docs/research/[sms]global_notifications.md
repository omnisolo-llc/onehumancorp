### Title
`[sms]global_notifications`: Implement SMS Notifications via Twilio

### Problem Statement
Email open rates can be low. For urgent updates (like appointment reminders or order pickups), SMS is far more effective. Business owners, especially those serving demographics with lower tech literacy, need a reliable way to send text messages directly to their customers.

### Research Report
- **Tool**: Twilio API
- **Pros**: Global reach, unmatched reliability, comprehensive API for SMS and Voice.
- **Cons**: Can be expensive at scale. Strict compliance rules (A2P 10DLC) require complex registration for US numbers.
- **Reputation**: The industry standard for programmatic SMS.
- **Pricing**: Pay-as-you-go (approx. $0.0079 per SMS in the US, higher internationally).
- **Ease of Use for Non-Technical Users**: Users just type a message or configure a template. The platform handles carrier routing.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: System events (e.g., 24 hours before an appointment) or manual user action trigger a notification.
- **Action**: The OHC API server formats the message and sends it via the Twilio API.
- **User View**: A toggle in settings for "Enable SMS Reminders" and a simple text input for custom messages.

### Implementation Prompt
Implement automated SMS notifications using Twilio. Focus on high-value transactional messages like appointment reminders and order readiness alerts. The integration must handle the complexities of phone number formatting (E.164) and provide clear error messages if a delivery fails. Ensure there is a robust opt-out (STOP) handling mechanism to maintain compliance.

### Priority
P1

### Estimated Scope
Medium

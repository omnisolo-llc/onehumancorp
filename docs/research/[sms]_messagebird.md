# Scout: Tool Integration Research [Q2]

## [SMS] Issue Brief: MessageBird Integration

**Title**: Global SMS Notifications via MessageBird

**Problem Statement**:
In many regions where OHC operates, business owners and their customers may have limited or intermittent data connectivity. A baker like Maya might miss an app notification for a new order if her internet is spotty. Reliable SMS alerts for new orders and automated SMS appointment reminders for customers ensure that the business runs smoothly regardless of data availability.

**Research Report**:
- **Tool**: MessageBird (now Bird) SMS API.
- **Evaluation**: A leading global communications platform with high-reliability SMS delivery across almost every country.
- **Ease of Use**: Very high. Invisible to the end customer and a simple toggle for the merchant.
- **Pricing**: Pay-as-you-go per message (rates vary by country).
- **Reputation**: Excellent global reach and deliverability.
- **Cloud vs. Standalone**: Cloud mode can use a managed OHC pool; Standalone mode allows the user to provide their own Bird API key.

**Design Doc**:
- User enables "SMS Alerts" in their Notification settings.
- When a new order is paid or an appointment is booked, OHC triggers an async job to send an SMS via MessageBird.
- OHC handles international phone number formatting (E.164) automatically.
- The owner receives an SMS: "New order from Carlos for $45! Check your OHC app."
- Customers receive a reminder 1 hour before an appointment.

**Implementation Prompt**:
Integrate the MessageBird (Bird) SMS API for outbound notifications. Implement order alerts for merchants and appointment reminders for customers. Ensure robust handling of global phone number formats and deliverability tracking.
- **Acceptance Criteria**: Merchant receives an SMS for new orders. Customers receive SMS reminders for bookings. Settings panel allows toggling SMS on/off.
- **Priority**: P2
- **Estimated Scope**: Medium

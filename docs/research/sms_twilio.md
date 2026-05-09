# Integrate Twilio for SMS Notifications
## Problem Statement
Many customers, particularly in certain demographics or regions, do not have smartphones or regular internet access. They rely on basic SMS for appointment reminders and order updates. Business owners need a reliable way to reach them without manually typing texts from their personal phones.
## Research Report
Twilio is the market leader for programmable SMS.
- **Ease of Use**: Seamless for the business owner. They just see "SMS Sent" on the customer's profile.
- **Pricing**: Very transparent, per-segment pricing. Very affordable.
- **Reputation**: High deliverability, strong compliance tools (opt-out handling).
## Design Doc
OHC will use an SMS provider to send automatic text messages for key events (e.g., appointment confirmed). The user interface will show a simple toggle for "Send SMS Reminders" and allow basic template customization.
## Implementation Prompt
Add an SMS settings page where the user can enable "SMS Notifications". Provide default, plain-language templates for common events (like "Appointment Tomorrow"). Ensure the system automatically respects "STOP" replies to comply with regulations. For Standalone mode, provide a simple field to input their own provider credentials.
## Priority
P1
## Estimated Scope
Medium

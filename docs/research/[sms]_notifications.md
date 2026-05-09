## [SMS] Issue Brief: Automated SMS Notifications

**Title**: Scout 🔍: Twilio Integration for SMS Order Alerts
**Problem Statement**: Many small business owners (e.g., food cart operators) work in noisy, fast-paced environments where app notifications or emails are easily missed. They need reliable SMS alerts for new orders to ensure they start preparing them immediately.
**Research Report**:
- **Tools Evaluated**: Twilio, Plivo, MessageBird.
- **Evaluation**: Twilio is the industry standard for programmable SMS, offering high reliability and global reach.
- **Ease of Use**: The user simply toggles "Enable SMS Alerts" and verifies their phone number.
- **Pricing**: Twilio charges per message sent. OHC will need to cover this cost or pass it to the user.
- **Cloud vs. Standalone**: Cloud mode uses OHC's master Twilio account. Standalone mode would require users to provide their own Twilio credentials.
**Design Doc**:
- A settings panel allows users to opt-in to SMS notifications and verify their mobile number.
- When a critical event occurs (e.g., "New Order Paid"), OHC triggers an asynchronous job.
- The job calls the Twilio API to send a short, concise SMS to the user.
**Implementation Prompt**: Integrate the Twilio API for outbound SMS. Build a preferences UI for users to enable SMS alerts for specific events (like new orders). Ensure phone numbers are stored and formatted correctly (E.164).
**Priority**: P2
**Estimated Scope**: Medium

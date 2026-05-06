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

**Title**: Twilio Integration for Global SMS Notifications

**Problem Statement**:
Business owners and their customers (especially those with lower English proficiency or in regions with poor internet) rely heavily on SMS. They need automated SMS reminders for appointments, order updates, and marketing to reduce no-shows and increase engagement.

**Research Report**:
- **Tool**: Twilio (SMS & Notifications).
- **Ease of Use**: The integration itself abstracts the complexity. The business owner just sees "Send SMS" toggles.
- **Pricing**: Pay-as-you-go. Roughly $0.0079 per message in the US, varies globally. Additional costs for phone number rental (e.g., $1.15/month).
- **Reputation**: The industry standard for programmable SMS. Highly reliable with global carrier coverage.
- **Compatibility**: Requires API key configuration. Works in both Cloud and Standalone modes.

**Design Doc**:
- **Trigger**: System events (e.g., appointment created, order shipped) or manual broadcast.
- **Action**: OHC sends a request to the Twilio API to dispatch the SMS.
- **User Interface**: A settings page to configure automated SMS templates (e.g., "Remind customer 24h before appointment").
- **Integration Flow**: User enters Twilio Account SID, Auth Token, and sender phone number in Settings.

**Implementation Prompt**:
Integrate Twilio for outbound SMS capabilities. Add settings for business owners to enter their Twilio credentials. Implement automated SMS triggers for key events like appointment reminders and order confirmations. Ensure opt-out compliance handling is documented or handled by the provider.

**Priority**: P1
**Estimated Scope**: Medium

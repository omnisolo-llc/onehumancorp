# Global SMS Notifications for Critical Updates via Twilio

**Title**: Global SMS Notifications for Critical Updates via Twilio
**Problem Statement**: Non-technical or low-English-proficiency users, as well as their customers, often prefer SMS over email for critical updates like order confirmations or appointment reminders.

**Research Report**:
- Twilio is the industry standard for programmatic SMS delivery globally.
- **Ease of Use**: The business owner simply toggles "Enable SMS Notifications" in their settings. OHC handles the routing.
- **Pricing**: Pay-per-message model; very affordable for standard notification volumes.
- **Reputation**: Extremely high reliability and global reach.
- **Cloud vs Standalone**: Cloud mode can pool usage, while Standalone mode requires the user to plug in their own Twilio credentials.
- **Key Advantages**: Near-instant delivery, high open rates compared to email.
- **Key Risks**: Strict A2P 10DLC compliance rules in the US can make onboarding complex if not abstracted properly.

**Design Doc**:
- In "Settings > Notifications," users can toggle SMS alerts for new orders or booking reminders.
- Customers providing their phone numbers at checkout automatically receive SMS updates.
- OHC abstracts the Twilio integration, ensuring messages are sent strictly for transactional purposes to comply with carrier regulations.

**Implementation Prompt**: Implement a Twilio integration that allows business owners to seamlessly send automated, transactional SMS updates (e.g., order confirmed, appointment tomorrow) to their customers.

**Priority**: P1
**Estimated Scope**: Medium

# SMS & Notifications: MessageBird (Bird)

**Problem Statement:** Non-technical business owners need to send appointment reminders or urgent updates via text, especially to customers with low English proficiency or those who don't check email.

**Research Report:** MessageBird (now Bird) offers global omnichannel communication APIs, often competing with Twilio but with more focus on marketing and CRM.
- Ease of Use: API-driven, requires OHC to build the UI wrapper. They also have "Inbox" products.
- Pricing: Competitive global SMS rates.
- Reputation: Strong international presence.
- Cloud vs. Standalone: Cloud-based API.

**Design Doc:**
- OHC provides a "Send SMS" interface linked to customer profiles.
- Automated triggers (e.g., appointment tomorrow) send templated SMS via MessageBird API.
- UI wireframes or screen flow description (375px first): Simple text input box on the customer profile page to send a quick message.
- Mobile UX flow: Native-feeling SMS compose view within the OHC app.

**Implementation Prompt:** Integrate MessageBird for transactional SMS capabilities (order confirmations, appointment reminders).
- Acceptance Criteria: SMS messages are delivered reliably. Delivery receipts are logged in OHC. Opt-outs (STOP) are handled automatically.

**Priority:** P2
**Estimated Scope:** Medium

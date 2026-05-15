## 6. SMS & Notifications: MessageBird

**Title:** Integrate MessageBird for Global SMS Communications
**Problem Statement:** Email open rates are dropping. Businesses need a reliable way to send urgent updates (like delivery notifications or appointment reminders) via SMS, especially for customers with low English proficiency who prefer texting.
**Research Report:**
- **Tool evaluated:** MessageBird (Bird)
- **What problem it solves for which persona:** Provides high-visibility notifications for service and local delivery businesses.
- **Ease of Use:** Developer-focused, but once integrated, invisible to the user.
- **Pricing:** Pay-per-message, varies heavily by country.
- **Reputation:** Strong competitor to Twilio, especially good in Europe and Asia.
- **Advantages & Risks:**
  - *Advantages:* Excellent global coverage, robust API.
  - *Risks:* Strict telecom regulations (like A2P 10DLC in the US) make onboarding a business a massive headache.
- **Cloud/Standalone Mode:** Cloud is simple via API. Standalone works if the business provides their own API key.
**Design Doc:**
- **Trigger:** An appointment is booked or an order is dispatched.
- **Action:** OHC triggers an SMS payload via the API to the customer's phone number.
- **User View:** A toggle in OHC settings: "Send SMS reminders to customers."
**Implementation Prompt:**
Implement a notification service interface that supports SMS routing. Provide a settings UI for the business owner to enable/disable SMS notifications for specific events (e.g., 'Order Shipped').
**Priority:** P1
**Estimated Scope:** Medium

### Title
Integrate Twilio for Global SMS Notifications and Alerts

### Problem Statement
Food cart operators, service professionals, and other business owners who are highly mobile rely on their phones to run their operations. They (and their customers) often miss email notifications. They need reliable, instantaneous SMS alerts for critical events like new pre-orders, booking confirmations, or last-minute schedule changes, especially in areas with poor data connectivity or for users with limited technical proficiency.

### Research Report
**Tool Evaluated:** Twilio
**Overview:** Founded in 2008, Twilio is a massive American cloud communications company providing programmable communication tools for making calls and sending texts via web service APIs. It is a public company (NYSE: TWLO) with over $5 billion in revenue (2025).
**Key Features & Advantages:**
- Industry-standard API for programmatic SMS, MMS, and Voice.
- Global carrier coverage, ensuring delivery reliability worldwide, which is critical for non-English speakers or users on low-end devices globally.
- Features like two-factor authentication (via Authy, acquired 2015) and email (via SendGrid, acquired 2018).
- Highly scalable, running on AWS infrastructure.
**Risks:** Twilio has faced security incidents, notably a social engineering/phishing breach in 2022 that exposed some customer data, and a 2024 breach of Authy phone numbers. Engineering must ensure API keys are securely managed.
**Ease of Use:** For the end-user (business owner/customer), it is completely invisible and effortless (they just receive a text). For the platform, the API is famously developer-friendly.
**Pricing:** Pay-as-you-go per message/call. Very cost-effective for automated alerts.
**Deployment:** Cloud service via REST APIs.

### Design Doc
**Integration Trigger:** SMS notifications are configured globally by the OHC platform for transactional alerts, or enabled per-tenant in the "Customer Success" settings (e.g., "Send SMS reminders to clients").
**Action:** OHC backend uses the Twilio API to dispatch SMS messages triggered by specific state changes in the PostgreSQL database (e.g., `Order Status -> Ready for Pickup`).
**User Experience:**
- **Business Owner:** Fatima (food cart) gets an instant text: "New order: 2x Chicken over Rice. Reply 1 to confirm."
- **Customer:** Receives a text: "Your appointment with Carlos Handyman is tomorrow at 2 PM. Reply CANCEL to reschedule."

### Implementation Prompt
Implement a robust SMS notification service using the Twilio API to handle outbound transactional messaging for the platform.

**Acceptance Criteria:**
1. Create a generalized Twilio client service in the Go/Rust backend capable of sending SMS given a phone number and message body.
2. Implement an asynchronous job queue worker (using the existing `SKIP LOCKED` Postgres pattern) to process SMS dispatch to prevent blocking the main API thread.
3. Handle Twilio webhook callbacks to process delivery statuses (e.g., failed delivery) and simple inbound replies (like "1" or "CANCEL") routed back to the appropriate tenant's inbox.
4. Ensure all phone numbers are validated and formatted to E.164 standard before dispatch.

### Priority
P0

### Estimated Scope
Large

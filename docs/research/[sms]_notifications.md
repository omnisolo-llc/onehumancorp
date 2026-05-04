# [sms] Global SMS Notifications & Marketing

## Title
Implement Global SMS Notifications & Marketing

## Problem Statement
While email is common, SMS has a much higher open rate and immediacy. For users like Fatima (Food Cart Operator) or Carlos (Freelance Handyman), customers expect instant SMS updates about their order readiness or appointment arrival times. Furthermore, SMS marketing is a highly effective way to drive repeat business. They need an integrated way to send automated SMS notifications and targeted marketing blasts without managing a separate Twilio account.

## Research Report
### Market Evaluation
- **Twilio**: The industry standard for programmable SMS.
    - *Ease of use (for OHC)*: Excellent APIs, comprehensive global coverage.
    - *Pricing*: Pay-as-you-go, but A2P 10DLC compliance fees in the US add overhead.
    - *Cloud vs. Standalone*: Highly effective in Cloud where OHC manages subaccounts and compliance. Extremely difficult for Standalone users who would need to navigate A2P 10DLC registration independently.
- **MessageBird / Plivo / Sinch**: Strong alternatives to Twilio, sometimes better pricing outside the US.
    - *Ease of use (for OHC)*: Similar API structures to Twilio.
    - *Cloud vs. Standalone*: Same complexities as Twilio regarding user-managed accounts in Standalone mode.
- **AWS SNS (Simple Notification Service)**:
    - *Pros*: Already in the AWS ecosystem (if OHC uses AWS), simple for basic notifications.
    - *Cons*: Less feature-rich for conversational SMS or marketing campaigns compared to Twilio.

### Integration Risks & Considerations
- **A2P 10DLC Compliance (US)**: US carriers require strict registration for Application-to-Person messaging to prevent spam. Guiding non-technical users through brand and campaign registration is complex and time-consuming. OHC might need a dedicated sub-account strategy or a unified OHC toll-free number to abstract this.
- **Global Costs**: SMS pricing varies wildly by country. Sending SMS to certain regions can be prohibitively expensive, requiring careful cost control and tiering within OHC.
- **Opt-In/Opt-Out Management**: Strict adherence to TCPA and global spam laws is required. OHC must automatically handle "STOP" replies.

## Design Doc
### User Experience
1. **Enablement**: In the "Customer Success" tab, the user enables SMS notifications. They are assigned an OHC-managed phone number (or a shared short code/toll-free number, depending on compliance strategy).
2. **Automated Alerts**: When a customer places an order, the "Operations" agent automatically sends an SMS confirmation. When an appointment is approaching, an SMS reminder is sent.
3. **SMS Marketing**: Similar to email, the user can ask the "Promoter" agent to draft a short SMS blast (e.g., "Flash sale today! 20% off all cakes. Reply BUY to order.").
4. **Two-Way Chat (Future)**: Customers can reply to the SMS, and it routes into the Unified Inbox for the AI or user to respond.

### System Flow
- OHC integrates an SMS gateway provider (like Twilio).
- Backend services trigger SMS sending via the provider's API for specific events (order created, booking reminder).
- Webhooks handle delivery receipts and inbound messages.
- The system automatically intercepts "STOP" and "UNSUBSCRIBE" messages, updating the customer profile to block future marketing SMS (while potentially still allowing transactional SMS, depending on local laws).

## Implementation Prompt
Implement an SMS notification and marketing system using a provider like Twilio. Create workflows for transactional notifications (order updates, booking reminders) managed by the "Operations" agent, and promotional campaigns drafted by the "Marketing" agent. The system MUST abstract the complexity of carrier compliance (like A2P 10DLC) from the end-user as much as possible. Implement robust opt-out handling via webhooks. Do not prescribe specific database schemas or API endpoints; focus on the reliability of the notification pipeline and the UX of drafting an SMS campaign.

## Priority
P1

## Estimated Scope
Large
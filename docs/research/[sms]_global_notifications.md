# 💬 SMS & Notifications: Global Delivery

## Title
Global SMS Notifications Integration

## Problem Statement
For non-technical or limited-English proficiency users like Fatima (The Food Cart Operator), SMS is the most reliable way to receive order alerts. Push notifications can be missed, and emails are often ignored. We need a robust SMS delivery system to send critical alerts (new orders) and customer notifications (order ready for pickup).

## Research Report
- **Goal**: Evaluate global SMS delivery providers for reliability and cost-effectiveness.
- **Tools Evaluated**:
    - **Twilio**: The industry standard. Highly reliable, excellent global coverage, but can be expensive.
    - **MessageBird**: Good pricing, especially in Europe and Asia. Strong omnichannel API.
    - **Vonage (formerly Nexmo)**: Solid alternative, competitive pricing.
    - **Amazon SNS**: Cost-effective for simple SMS, but lacks advanced routing and conversational features.
- **Recommendation**: Integrate with **Twilio** for initial Cloud mode deployment due to its unmatched reliability and developer experience. Ensure the integration is abstracted so we can swap providers if costs become prohibitive.
- **User Impact**: Fatima receives an immediate SMS: "New order: 2x Falafel Wrap. Pickup in 15 mins." The customer receives an SMS when the food is ready. This requires zero app interaction from Fatima while she is busy cooking.

## Design Doc
- **Component**: `NotificationAgent`
- **Responsibilities**:
    - Abstract SMS sending via a generic interface.
    - Handle phone number validation and formatting (E.164).
    - Queue SMS messages for reliable delivery with retries.
    - Process delivery status webhooks.
- **Integration Point**: The `OrderService` will trigger events that the `NotificationAgent` consumes to dispatch SMS alerts.

## Implementation Prompt
Implement the Global SMS integration. Create a service that sends SMS messages via the chosen provider (e.g., Twilio). Ensure the service handles phone number formatting and validation. Implement a queuing mechanism to retry failed messages. Add webhook handlers to track message delivery status (Sent, Delivered, Failed) and log them for observability.

## Priority
P0

## Estimated Scope
Small

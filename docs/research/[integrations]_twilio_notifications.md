# Issue Brief: SMS Notifications via Twilio

## Title
Mission-Critical SMS & WhatsApp Notifications

## Problem Statement
"I don't check my email often, and I missed a pickup notification." For users like Fatima (Halal Food Cart) or Carlos (Handyman), SMS is the only reliable way to communicate. They need to get a text the second an order is placed or a booking is confirmed.

## Research Report
- **Tool**: Twilio Messaging API.
- **Ease of Use**: High for developers; reliable for users.
- **Persona Fit**:
    - **Fatima (Food Cart)**: Receives an SMS: "New Pre-order: 2 Gyro Platters at 12:30 PM."
    - **Carlos (Handyman)**: Customer gets an SMS reminder 1 hour before he arrives.
- **Cloud vs. Standalone**:
    - **Cloud**: Mandatory for carrier connectivity.
    - **Standalone**: Can simulate SMS via local notifications or browser-based push.
- **Pricing**: Pay-as-you-go (approx. $0.0079 per SMS). Essential for "Mission Critical" communication.
- **Competitive Analysis**: Twilio is the gold standard for reliability and global coverage.

## Design Doc
- **Integration**: "The Ambassador" (Customer Success Agent) uses Twilio to send "Outbound Notifications."
- **User Experience**:
    - User enables "SMS Alerts" in settings.
    - OHC provisions a local number (via Twilio).
    - AI Agent handles the "SMS Conversation" if a customer replies ("I'll be 10 mins late!").

## Implementation Prompt
Integrate the Twilio Messaging API for outbound SMS and WhatsApp notifications. Focus on "High-Urgency" events like order placements and appointment reminders. Ensure "The Ambassador" agent can draft responses to incoming SMS messages. Support global phone number formats.

## Priority
P0 (Critical for Fatima and Carlos)

## Estimated Scope
Small

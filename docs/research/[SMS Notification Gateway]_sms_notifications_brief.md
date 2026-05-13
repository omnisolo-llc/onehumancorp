# SMS Notification Gateway

## Problem Statement
Email open rates are low, and non-technical customers (like Fatima) rely heavily on SMS for updates, appointment reminders, and promotions.

## Research Report

**Market Context:**


**Evaluated Tools:**

#### In-Depth Evaluation: Twilio
**Market Position**: The industry leader in programmable communications (SMS, Voice).
**Pricing**: Pay-per-message. Complex regulatory costs (A2P 10DLC registration fees in the US).
**Integration Approach**: Twilio provides the raw pipes. OHC must build the logic for opt-ins, opt-outs (handling STOP replies via webhooks), and rate limiting. The regulatory burden (trusthub registration) is a major UX hurdle for small businesses; OHC may need to act as an ISV to abstract this.
**Persona Impact**: Essential for businesses where customers don't check email. An SMS reminder reduces no-shows dramatically.

#### In-Depth Evaluation: MessageBird
**Market Position**: Strong European presence, aggressive competitor to Twilio. Now known as Bird.
**Pricing**: Competitive per-message rates.
**Integration Approach**: Similar technical hurdles to Twilio regarding webhooks and regulatory compliance.

#### In-Depth Evaluation: Sinch
**Market Position**: Enterprise-focused but viable. Massive global carrier network.
**Pricing**: Often negotiable at volume.
**Integration Approach**: Robust API, suitable as an alternative routing option if Twilio fails.

## Design Doc
Integrate an SMS gateway API (like Twilio or MessageBird). OHC's notification service will allow routing alerts (appointment reminders, order updates) to SMS based on customer preference. Include opt-out handling via webhooks.

## Implementation Prompt
Add an SMS notification preferences section to the customer profile. Implement automated SMS reminders for appointments and a feature for owners to send quick SMS broadcasts to opted-in customers.

## Priority
P1

## Estimated Scope
Medium

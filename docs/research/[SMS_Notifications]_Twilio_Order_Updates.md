## 6. SMS & Notifications
**Title**: Integrate Twilio for SMS Order Updates
**Problem Statement**: Customers often miss email notifications. Small business owners (like food delivery or repair services) need to send instant text message updates (e.g., "Your repair is done") to reduce no-shows and incoming "is it ready yet?" calls.
**Research Report**:
- **Tool**: Twilio Programmable SMS API
- **Problem it solves for which persona**: Helps service and local retail businesses keep customers informed instantly.
- **Ease of Use**: Owner enables SMS in OHC; OHC provisions a number via Twilio behind the scenes.
- **Pricing**: ~$0.0079 per message in the US, varies globally. Often requires A2P 10DLC registration fees in the US.
- **Key Advantages**: Most robust telecom API, global reach.
- **Integration Risks**: Strict regulatory compliance (A2P 10DLC in the US) can make onboarding small businesses complicated; toll fraud risks.
- **Environment**: Cloud (OHC manages Twilio account) and Standalone (User brings their own Twilio SID/Auth Token).
**Design Doc**:
- **Trigger**: Order status changes to "Ready for Pickup".
- **Action**: OHC sends an SMS via Twilio to the customer's phone number.
- **User Interface**: Owner can toggle "Send SMS Updates" on order statuses.
**Implementation Prompt**: Integrate Twilio's SMS API to send automated, customizable text messages to customers when their order status changes. Implement phone number validation and handle opt-out (STOP) webhooks securely.
**Priority**: P1
**Estimated Scope**: Medium

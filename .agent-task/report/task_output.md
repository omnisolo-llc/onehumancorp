# Research Report: SMS & Notifications Tool Integration

## Phase 1: Research

**Category:** SMS & Notifications
**Tool:** Twilio
**Problem Space:** Non-English speaking users like Fatima (the food cart operator) and service providers like Carlos (freelance handyman) need reliable, instant notifications. Emails often get missed or ignored. For users with low English proficiency or in certain demographics, SMS is the primary and most reliable communication channel.
**Alternatives Evaluated:** Vonage, Plivo, MessageBird, AWS SNS.
**Why Twilio:** Unmatched global carrier coverage, highly reliable API, extensive documentation, robust compliance (A2P 10DLC for US), and straightforward pricing for low-volume users. While slightly more expensive per message than Plivo or MessageBird, its reliability and out-of-the-box international support make it the gold standard.

## Phase 2: Evaluation

**What problem it solves:**
- Gives business owners instant alerts when a new order/booking is placed.
- Allows sending automated order confirmations and reminders directly to customers' phones (e.g., "Your cake from Maya is ready for pickup!").
- Critical for accessibility and diverse demographics where smartphone/app usage might be lower, but basic SMS is universal.

**How it benefits OHC users:**
- **Zero Configuration:** The user simply sees an "Enable SMS Notifications" toggle in their Settings or "Customer Success" department. They do not need to create a Twilio account or manage API keys.
- **Reliability:** Fatima gets a ding on her phone immediately when a new food order comes in, even on a slow data connection or low-end device.
- **Professionalism:** Customers receive professional, branded SMS messages, increasing trust.

**Integration Risks & Considerations:**
- **Cost Management:** SMS costs real money per message. OHC needs a way to meter usage or bundle it into premium tiers to avoid runaway costs.
- **Compliance:** Strict A2P 10DLC regulations in the US require business registration to send messages, which could complicate the "zero configuration" promise. Opt-out (STOP) handling must be rock solid.
- **Spam:** Risk of bad actors using OHC to send spam SMS.

**Pricing Estimate:**
- ~$0.0079 per inbound/outbound message (US). International rates vary significantly.

**Cloud vs. Standalone Mode:**
- **Cloud Mode:** Works perfectly. OHC manages the primary Twilio account and bills tenants based on usage.
- **Standalone Mode:** The user would need to provide their own Twilio API credentials (SID and Auth Token) since they are hosting their own instance, violating the "Zero Configuration" slightly, but standard for self-hosted apps.

## Phase 3: Issue Brief

**Title:** [SMS & Notifications] Integrate Twilio for Universal SMS Alerts
**Problem Statement:** Business owners (especially in service or food sectors) and their customers often miss email notifications. SMS provides an instant, universal channel that is critical for real-time operations (e.g., order ready for pickup) and serves diverse user demographics (like those with limited smartphone proficiency).
**Research Report:** Twilio offers the most reliable global SMS delivery. While it introduces variable costs per message, its API maturity and carrier coverage make it the best choice. Strict adherence to compliance (opt-out handling) is required.
**Design Doc:**
- The integration will be managed by the "Customer Success" and "Operations" AI agents.
- Users will toggle "SMS Notifications" in their dashboard.
- Cloud Mode: OHC uses a central Twilio account; tenants are billed for usage.
- Standalone Mode: Users provide their own Twilio credentials.
- The system will intercept relevant internal events (e.g., `OrderPlaced`, `BookingConfirmed`) and format brief SMS messages to send via Twilio's API.
**Implementation Prompt:** Implement a Twilio client wrapper that supports both Cloud (managed) and Standalone (bring-your-own-keys) modes. Integrate it into the existing notification pipeline so that system events can optionally trigger an SMS. Ensure robust error handling, retries for transient carrier issues, and strict compliance with opt-out requests.
**Priority:** P1
**Estimated Scope:** Medium

```yaml
issue_title: "[SMS & Notifications] Integrate Twilio for Universal SMS Alerts"
issue_priority: "P1"
issue_description: "Integrate Twilio to provide reliable SMS notifications for business owners and customers, addressing the need for instant communication in real-time operations (food pre-orders, service bookings)."
issue_todo_list:
  - [ ] Implement Twilio client wrapper (Cloud & Standalone support)
  - [ ] Integrate with notification pipeline
  - [ ] Implement robust error handling and retries
  - [ ] Ensure opt-out (STOP) compliance
issue_label: ["research", "high-impact"]
```

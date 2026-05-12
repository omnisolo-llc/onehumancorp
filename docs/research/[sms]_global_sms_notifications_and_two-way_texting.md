# Global SMS Notifications and Two-Way Texting

## Problem Statement
In many regions, email open rates are plummeting, and customers prefer SMS for order updates, appointment reminders, and quick questions. Business owners find SMS more intuitive. They need a way to send automated alerts and text back and forth with customers without giving out their personal phone number.

### Target Personas
- **Fatima, local restaurant owner: Wants to text customers 'Your order is ready for pickup' to avoid cold food.**
- **James, plumber: Needs to text clients 'I am 15 minutes away' while on the road.**
- **Anita, beauty salon owner: Needs to send SMS appointment reminders to reduce a 20% no-show rate.**

## Research Report
We conducted a comprehensive analysis of the available tools in the market to solve this specific challenge for small businesses.

### Competitive Tool Analysis

#### Twilio
- **Ease of Use**: Medium. Developer API, but industry standard.
- **Pricing Model**: Pay-as-you-go, approx $0.0079/msg in US.
- **Market Reputation**: The undisputed king of CPaaS (Communications Platform as a Service).
- **Key Advantages**: Unparalleled global reach, extreme reliability, massive feature set.
- **Identified Risks**: Complex regulatory compliance (A2P 10DLC in the US) is hard to abstract for SMBs.
- **Architecture Compatibility**: Cloud API.

#### MessageBird
- **Ease of Use**: Medium. Has good no-code tools alongside API.
- **Pricing Model**: Pay-as-you-go, approx $0.008/msg depending on country.
- **Market Reputation**: Strong competitor to Twilio, especially outside the US.
- **Key Advantages**: Omnichannel widget, strong global carrier routing.
- **Identified Risks**: Support can be slow for small accounts.
- **Architecture Compatibility**: Cloud API.

#### Plivo
- **Ease of Use**: Medium. Developer focused.
- **Pricing Model**: Pay-as-you-go, very competitive rates.
- **Market Reputation**: Reliable and cost-effective alternative.
- **Key Advantages**: Great documentation, low latency, cost-effective for high volume.
- **Identified Risks**: Requires developer effort to build user-facing features.
- **Architecture Compatibility**: Cloud API.

#### Sinch
- **Ease of Use**: Low. Enterprise focused.
- **Pricing Model**: Custom enterprise pricing.
- **Market Reputation**: Massive global telecom footprint.
- **Key Advantages**: Direct carrier connections globally, unparalleled reliability.
- **Identified Risks**: Not accessible for SMBs directly; meant for massive scale aggregators.
- **Architecture Compatibility**: Cloud API.

### Market Context
SMS open rates are consistently above 90%, compared to 20% for email, making it critical for time-sensitive alerts.

## Design Doc
A 'Texting' module in OHC. Business owners are assigned a virtual phone number. They can configure automated SMS templates triggered by OHC events (e.g., 'Order #123 is out for delivery'). Additionally, an SMS inbox UI allows them to receive texts from customers and reply directly from their desktop or mobile device. All communication is routed through the chosen SMS API provider.

### Security & Compliance
Must implement opt-in/opt-out (STOP) handling to comply with TCPA and local telecom regulations.

### Resilience Strategy
Implement robust retry logic for failed message delivery due to carrier issues.

## Implementation Prompt
Implement an automated SMS notification system. When a specific event occurs (e.g., an appointment is booked), trigger an SMS to the customer's phone number using a predefined template. Allow the business owner to configure these templates in the settings. Ensure the system handles international phone number formatting (E.164) correctly and logs delivery receipts.

### Acceptance Criteria
- [ ] User can customize SMS templates.
- [ ] System formats numbers to E.164 standard.
- [ ] Event trigger successfully queues and sends an SMS via provider.
- [ ] Incoming 'STOP' replies automatically blacklist the number from future automated texts.

## Priority
P0

## Estimated Scope
Large

## Extended Architectural Considerations

When implementing sms, developers must consider the implications for both the multi-tenant Cloud deployment of OHC and the self-hosted Standalone mode.

In Cloud mode, API rate limiting is a shared concern. A sudden spike in activity from one tenant must not exhaust the API quota for the entire platform. This necessitates a robust queueing system, such as RabbitMQ or AWS SQS, to process outbound requests and ingest incoming webhooks efficiently.

In Standalone mode, the business owner might not have the technical expertise to configure complex OAuth apps or webhook receivers. The UI must guide them through this process with extreme clarity, perhaps utilizing a proxy service maintained by OHC to simplify the webhook routing to dynamic IP addresses typical of self-hosted setups.

Furthermore, data privacy is paramount. Any PII (Personally Identifiable Information) synced from sms tools must be encrypted at rest within the OHC database. Retention policies should automatically purge transient data (like raw webhook payloads) after successful processing to minimize the attack surface.

The user interface must remain mobile-first. Small business owners operate primarily from their smartphones. Therefore, the settings pages, dashboards, and daily interaction elements designed for this integration must be fully responsive and pass the 'Grandmother Test' for usability.

By carefully considering these architectural, security, and usability constraints, we can deliver an integration that not only functions reliably but empowers the user to grow their business without friction.

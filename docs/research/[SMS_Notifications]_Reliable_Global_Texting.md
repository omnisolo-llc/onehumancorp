## [SMS & Notifications] Reliable Global Texting
**Title**: Integrate Twilio / MessageBird for Critical SMS Alerts

**Problem Statement**: Some customers (and business owners) don't use email frequently or have low English proficiency, preferring simple text messages. Businesses need a reliable way to send appointment reminders, pickup notifications, or delivery updates via SMS to reduce no-shows and improve service.

**Research Report**:
- **Persona Context**: Local service providers (salons, mechanics, clinics) and their diverse customer base, including non-tech-savvy individuals.
- **Solution Evaluated**: Twilio and MessageBird. Twilio is the industry standard with massive global reach. MessageBird offers great omnichannel capabilities.
- **Ease of Use**: Invisible to the end-customer. For the business owner, it's just toggling "Send SMS reminder".
- **Advantages**: 98% open rate for SMS. Drastically reduces appointment no-shows.
- **Risks**: Telecom regulations (A2P 10DLC in the US) make onboarding complex for small businesses. High costs for international SMS.
- **Pricing Estimate**: ~$0.0079 per message in the US, higher internationally.
- **Cloud/Standalone Support**: Works in both. Standalone users input their Twilio SID and Auth Token.

**Design Doc**:
- **Triggers**: An appointment is approaching (e.g., 24 hours prior) or an order is ready for pickup.
- **Actions**: OHC sends a templated text message to the customer's phone number.
- **User Interface**: A simple toggle in the scheduling or order management settings: "Send SMS reminders to customers". In Standalone mode, a settings page to input Twilio credentials.

**Implementation Prompt**:
Implement automated SMS notifications for key events like upcoming appointments or order pickups. Provide a straightforward settings panel for users to enable SMS and configure their provider credentials. Ensure the system handles phone number formatting (E.164) gracefully.

**Priority**: P1
**Estimated Scope**: Small

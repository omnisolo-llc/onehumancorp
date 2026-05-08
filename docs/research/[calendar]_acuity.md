## [Calendar] Acuity Scheduling Integration
**Title**: Integrate Acuity Scheduling for Automated Client Booking
**Problem Statement**: Service-based small businesses, like consultants or personal trainers, waste hours going back and forth via email to find meeting times. They need an easy way for clients to book available slots directly.
**Research Report**:
- **Tool**: Acuity Scheduling (Squarespace)
- **Target Persona**: Consultants, Therapists, Fitness Instructors
- **Advantages**: Very popular, powerful customization for appointment types, handles payments.
- **Risks**: Slightly more complex to set up initially than Calendly.
- **Pricing**: Starts around $16/month.
- **Compatibility**: Cloud (OAuth). Standalone (API Keys).
**Design Doc**:
- User connects their Acuity account in OHC integrations.
- OHC agents can view available slots and share booking links in chat.
- When an appointment is booked, a webhook triggers OHC to create a meeting record and notify the owner.
**Implementation Prompt**: Build the Acuity Scheduling integration. Support syncing appointment types and reading availability. Implement webhook handlers to receive new booking notifications and insert them into the OHC calendar view.
**Priority**: P2
**Estimated Scope**: Medium

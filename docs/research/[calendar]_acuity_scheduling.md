## [Calendar] Acuity Scheduling Integration
**Title**: Integrate Acuity Scheduling for Automated Booking
**Problem Statement**: Manual back-and-forth for booking appointments is a significant pain point. Service-based businesses need a streamlined way to handle bookings.
**Research Report**:
- **Tool**: Acuity Scheduling
- **Target Persona**: Service-based businesses
- **Advantages**: Pull appointments into a unified OHC dashboard and allow easy sharing of booking links. Acuity's strength in service businesses makes it preferable to Calendly for our demographic.
- **Risks**: May require syncing complex configuration (buffer times, etc.) between Acuity and OHC.
- **Pricing**: Standard Acuity pricing.
- **Compatibility**: Cloud, Standalone (via API).
**Design Doc**:
- User authenticates and connects their Acuity account.
- OHC imports appointment types and availability via Acuity API.
- Users can share Acuity booking links or use an embedded Acuity widget.
- Bookings are synced back to the unified OHC dashboard.
**Implementation Prompt**: Create an integration to connect an Acuity Scheduling account. Sync appointment types and display a booking widget on the public profile. Ensure that new appointments booked via Acuity are synced to the OHC operations dashboard.
**Priority**: P2
**Estimated Scope**: Medium

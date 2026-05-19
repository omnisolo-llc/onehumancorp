## [Booking] Advanced Open-Source Scheduling Integration with Cal.com

**Title**: Advanced Open-Source Scheduling Integration with Cal.com

**Problem Statement**: Small business owners (like Carlos the Handyman or Leo the Music Tutor) need an affordable, highly customizable scheduling tool that supports multiple event types without immediately forcing them into expensive paid tiers, and allows seamless integration into their standalone or cloud platforms.

**Research Report**:
Based on our research, Cal.com is a powerful open-source Calendly alternative. It provides an open-source version for self-hosting, which gives small business owners full data ownership and avoids recurring SaaS fees if they choose to manage it themselves. Additionally, it offers a robust free API with generous rate limits, allowing up to 120 requests per minute on the free tier. This flexibility, combined with its open-source nature and transparent pricing model, aligns perfectly with OHC's standalone mode and the specific needs of small business owners.

**Design Doc**:
- OHC users will be able to easily connect their existing Cal.com account or point to a self-hosted instance from the Operations dashboard.
- The OHC system will interact seamlessly with the Cal.com API to synchronize availability and read bookings in real-time.
- A customizable Cal.com scheduling widget will be available to embed directly into their OHC storefront, offering a native booking experience for end customers.
- Booked appointments will automatically appear in the OHC platform.

**Implementation Prompt**:
Integrate the Cal.com API into the platform. Provide a user interface in the Operations dashboard for users to authenticate and link their Cal.com accounts (or configure a self-hosted URL). Develop functionality to sync availability data accurately. Finally, implement a module to display the Cal.com embed widget on storefronts and ensure booked events are correctly recorded in the OHC system.

**Priority**: P1

**Estimated Scope**: Medium

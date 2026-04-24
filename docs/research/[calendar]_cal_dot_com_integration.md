# Cal.com API Integration

**Title**: Implement Seamless Booking and Scheduling via Cal.com API
**Problem Statement**: Service-based businesses (like Carlos the handyman or Leo the tutor) need a way for customers to book time slots without back-and-forth emails. Building a robust calendar system from scratch (handling timezones, double-booking, Google/Outlook sync) is error-prone.
**Research Report**:
- **Tool**: Cal.com API v2.
- **Ease of Use (End User)**: Invisible. The business owner sets their availability in OHC, and OHC handles the rest. Customers see a simple date/time picker on the storefront.
- **Pricing**: Open-source/Self-hosted (free, but maintenance overhead) or Managed Platform (starts at $15/user/mo, enterprise pricing available for white-label API usage).
- **Cloud vs. Standalone**: Cal.com offers a self-hosted option, making it viable for OHC's Standalone mode if bundled, or via API for the Cloud mode.
**Design Doc**:
- **Trigger**: User creates a "Service" product type and defines availability hours.
- **Action**: OHC creates a managed user/event type via Cal.com API. The storefront embeds a booking widget or renders a custom UI powered by Cal.com's available slots API.
- **UI**: "Availability" settings in the dashboard. Booking calendar component on the public storefront.
**Implementation Prompt**: Integrate the Cal.com API to power booking functionality for service-based products. Business owners should be able to set their available hours, and customers should be able to select an open slot during checkout. Ensure booked slots are automatically blocked off.
**Priority**: P0
**Estimated Scope**: Medium

# Zoom API Integration

**Title**: Implement Auto-Generated Video Meeting Links via Zoom API
**Problem Statement**: Online service providers (like Leo the tutor) spend unnecessary time manually creating Zoom links for every booked appointment and sending them to clients.
**Research Report**:
- **Tool**: Zoom API (Server-to-Server OAuth or standard OAuth).
- **Ease of Use (End User)**: Seamless. The business owner connects their Zoom account once. Customers receive the link automatically upon booking.
- **Pricing**: Free to build the integration. The business owner needs a Zoom account (Free or Pro depending on their meeting length needs).
- **Cloud vs. Standalone**: Cloud API. Works in both modes.
**Design Doc**:
- **Trigger**: A booking is confirmed for a service marked as "Online/Video".
- **Action**: OHC calls the Zoom API to create a meeting scheduled for the booked time. The join URL is saved to the booking record and emailed to the customer.
- **UI**: "Connect Zoom" button in Integrations. A location dropdown on services including "Zoom Meeting".
**Implementation Prompt**: Integrate the Zoom API to automatically generate meeting links for booked online services. Allow users to authenticate their Zoom accounts. When an online service is booked, create a Zoom meeting and include the join link in the confirmation email and calendar event.
**Priority**: P2
**Estimated Scope**: Small

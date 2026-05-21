### 2. Cal.com

**Title**: Cal.com Integration for Automated Meeting Scheduling

**Problem Statement**:
Small business owners spend too much time going back and forth with clients to find a time to meet (e.g., for consultations, estimates, or online lessons). They need a way to share a link where customers can pick an available time, and have it automatically sync to their calendar without manual intervention.

**Research Report**:
- **Tool**: Cal.com (Calendar & Scheduling).
- **Ease of Use**: Very high. Cal.com has a clean, simple UI. Non-technical users can easily set up meeting types and share their link.
- **Pricing**: Free tier available for individuals (unlimited event types and calendars). Teams tier is $12/month/user for collaborative scheduling.
- **Reputation**: Open-source, highly respected alternative to Calendly, with strong developer support.
- **Compatibility**: Excellent for both Cloud and Standalone modes. Cal.com can be self-hosted, making it a perfect fit for OHC's Standalone (local, private) mode.

**Design Doc**:
- **Trigger**: User configures Cal.com integration in OHC.
- **Action**: OHC syncs customer booking events via Cal.com webhooks.
- **User Interface**: A "Scheduling" tab where the business owner can view upcoming appointments. A "Share Booking Link" button that copies their Cal.com URL to the clipboard. Incoming bookings automatically create or update customer profiles in OHC.
- **Integration Flow**: OAuth connection to Cal.com or webhook URL generation to paste into Cal.com settings.

**Implementation Prompt**:
Integrate Cal.com into the OHC platform. Allow users to connect their Cal.com account. Display upcoming meetings in a new "Schedule" view within OHC. When a customer books a meeting on Cal.com, automatically ingest the webhook to create a new customer record or append the meeting to an existing customer's timeline in the CRM.

**Priority**: P1
**Estimated Scope**: Medium

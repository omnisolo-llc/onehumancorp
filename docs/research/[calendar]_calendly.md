# [Calendar] Calendly Sync

**Title**: Integrate Calendly for Seamless Scheduling
**Problem Statement**: Small business owners manually schedule appointments, leading to double-booking and lost time playing "email tag" with clients.
**Research Report**:
- **Target Persona**: Consultants, salon owners, and service providers who need clients to book time with them easily.
- **Evaluation**: Calendly is the industry standard for scheduling. Non-technical users understand it immediately. It handles timezone conversions and Google/Outlook sync perfectly.
- **Ease of Use**: Very High. The setup wizard is foolproof.
- **Pricing**: Free tier available (1 event type); Paid starts at $10/mo.
- **Key Risks**: Over-reliance on third-party availability logic. If Calendly goes down, bookings stop.
- **Compatibility**: Cloud integration is seamless via OAuth. Standalone might require manual API key input.
**Design Doc**: Users will connect their Calendly account in the OHC settings. OHC will display their upcoming appointments on the main dashboard. No complex setup needed.
**Implementation Prompt**: Create a dashboard widget that displays upcoming appointments from Calendly and allows users to copy their booking link. Acceptance criteria: widget shows next 5 appointments and has a functioning "Copy Link" button.
**Priority**: P1
**Estimated Scope**: Medium

# [Email Marketing] Mailchimp Integration

**Title**: Integrate Mailchimp for easy customer email campaigns

**Problem Statement**: Small business owners collect customer emails but rarely have the time or technical skill to send professional newsletters, promotions, or updates. They need an easy way to market to their existing customer base to drive repeat business.

**Research Report**: Mailchimp (now Intuit Mailchimp) is a dominant marketing automation and email marketing platform.
- **Ease of use**: Very high for non-technical users. Famous for its user-friendly interface, templates, and "Mailkimp" marketing campaigns.
- **Pricing**: Freemium. Good free tier for small lists; paid plans are affordable for small businesses.
- **Reputation**: Highly trusted, used by millions. Known for good deliverability and easy list management.
- **Cloud/Standalone**: API driven. Standalone mode requires internet connectivity.

**Design Doc**:
- **Trigger**: User connects their Mailchimp account in the OHC integrations tab.
- **Action**: OHC automatically syncs new customer emails (from bookings, payments, or manual entry) to a designated Mailchimp audience list.
- **User Experience**: The business owner doesn't have to manually export/import CSVs. When they get a new client, that client magically appears in their Mailchimp list, ready for the next newsletter.

**Implementation Prompt**: Create a one-way sync feature. When a user connects Mailchimp, any new customer added to the OHC contacts list should be automatically added to the user's default Mailchimp audience list.

**Priority**: P2
**Estimated Scope**: Medium
## [Calendar] Issue Brief

**Title**: Scout 🔍: Integrate Calendly for Easy Scheduling
**Problem Statement**:
Scheduling meetings, consultations, or classes involves a lot of back-and-forth emails. Business owners need a simple way to let clients book time without conflict.
**Research Report**:
- **Tool**: Calendly API
- **Evaluation**: Calendly handles calendar sync (Google, Outlook) and timezone conversions. Integrating it allows OHC to embed scheduling directly into the business's storefront or chatbot.
- **Ease of Use**: Users just connect their Calendly account via OAuth. No manual calendar setup required.
- **Pricing**: Has a free tier. Paid tiers offer more features and integrations.
- **Cloud vs. Standalone**: Works in both Cloud and Standalone modes via OAuth.
**Design Doc**:
- User connects their Calendly account.
- OHC fetches available event types.
- A booking widget is embedded in the storefront or shared via AI agents.
**Implementation Prompt**:
Integrate the Calendly API. Provide a way for users to link their account via OAuth. Display their available event types and allow embedding the Calendly booking widget on their site.
**Priority**: P1
**Estimated Scope**: Small

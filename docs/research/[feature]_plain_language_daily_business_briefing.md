# [feature] Plain Language Daily Business Briefing

**Title**: Implement Plain Language Daily Business Briefing

**Problem Statement**:
Analytics dashboards (like Google Analytics or even Shopify's dashboard) are intimidating for small business owners. They don't want to parse charts; they want to be told what happened and what to do next.

**Research Report**:
- Users want to feel in control without data overload.
- No competitor currently offers a plain-text, narrative daily summary.

**Design Doc**:
- **Architecture**:
  - Nightly cron job aggregates daily metrics (sales, visits, abandoned carts, low inventory).
  - LLM summarizes the data into a friendly 3-sentence paragraph.
  - System dispatches a push notification and saves the summary to the user's dashboard feed.
- **UI/UX Flow (Mobile 375px first)**:
  - Push notification at 8:00 AM local time.
  - Tapping opens the app to the Home screen.
  - Top card displays the briefing (e.g., "Good morning! You made $150 yesterday...").

**Implementation Prompt**:
Create a background job that aggregates daily store data and uses an LLM to generate a plain-language summary paragraph. Implement the push notification dispatch and a mobile-first (375px) dashboard card to display the daily briefing.

**Priority**: P2
**Estimated Scope**: Medium

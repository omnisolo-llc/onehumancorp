# Email Marketing: ConvertKit (Kit)

**Problem Statement:** Small business owners want to send newsletters or promotional emails to their customer base but find enterprise tools (like Mailchimp or Salesforce) too complex and bloated.

**Research Report:** ConvertKit (rebranding to Kit) is designed for creators and small businesses. It focuses on simplicity and high deliverability.
- Ease of Use: Extremely straightforward, clean UI, excellent for non-technical users.
- Pricing: Free tier available; paid plans start around $29/month based on list size.
- Reputation: Loved by creators, excellent deliverability rates.
- Cloud vs. Standalone: Cloud-based.

**Design Doc:**
- OHC customer list automatically syncs to ConvertKit as subscribers via API.
- User can trigger basic automations (e.g., "Welcome email") from OHC, which executes in ConvertKit.
- UI wireframes or screen flow description (375px first): A "Marketing" tab in OHC showing total subscribers and recent email open rates. A button to "Create Broadcast" that deep-links to ConvertKit.
- Mobile UX flow: Quick view of campaign performance metrics.

**Implementation Prompt:** Integrate ConvertKit to automatically sync the OHC customer database. Provide high-level metrics (subscribers, open rates) within the OHC dashboard.
- Acceptance Criteria: New customers added in OHC are instantly added to ConvertKit. Unsubscribes are synced back.

**Priority:** P1
**Estimated Scope:** Medium

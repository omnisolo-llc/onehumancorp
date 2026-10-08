import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    // Mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Automated Cart Recovery Focused UI Contract', () => {
  test('Unified Feed - Cart recovery card displays correctly', async ({ adminUser, loginAs, page, request }) => {
    // Setup test data directly using the API, bypassing background workers
    const feedItemPayload = {
      tenant_id: adminUser.organizationId,
      event_source: "sales",
      context_payload: {},
      proposed_action: {
        action_type: "Draft Reply",
        description: "The Assistant recovered 1 abandoned cart this week, securing $45.00 in revenue. The Salesperson drafted a recovery message for Alice.",
        draft_reply: "Hi Alice, noticed you left something in your cart!",
        feature_type: "quote_draft"
      },
      lifecycle_state: "PENDING_APPROVAL"
    };

    const res = await request.post('/api/v1/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    await loginAs(page, adminUser);
    await page.goto('/unified-feed');

    // Wait for the feed item to load and assert contract UI rendering
    await expect(page.getByText('abandoned cart').first()).toBeVisible({ timeout: 15000 });
    await expect(page.getByText('Hi Alice, noticed you left something in your cart!').first()).toBeVisible();
    await expect(page.getByTestId('approve-run-sale').or(page.getByTestId('feed-approve-btn'))).toBeVisible();
  });
});

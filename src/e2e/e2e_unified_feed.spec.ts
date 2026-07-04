import { test as base, expect } from './fixtures';

const test = base.extend({
  page: async ({ page }, use) => {
    // Mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await use(page);
  }
});

test.describe('Unified Action Feed e2e', () => {
  test('Unified Feed - Tap to approve user journey', async ({ adminUser, loginAs, page, request }) => {
    // Setup test data directly using the API
    const feedItemPayload = {
      tenant_id: adminUser.tenantId,
      event_source: "Instagram DM",
      context_payload: {
        msg: "Can I get a custom cake next Tuesday?",
      },
      proposed_action: {
        action_type: "Draft Reply",
        draft_reply: "Hi! Yes, we have availability next Tuesday. The base price for a custom cake is $150. Would you like to proceed with a $50 deposit?"
      },
      lifecycle_state: "PENDING"
    };

    const res = await request.post('/api/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    await loginAs(page, adminUser);

    // We expect to land on unified feed or can navigate there
    await page.goto('/unified-feed');

    // Wait for the feed item to load
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).toBeVisible({ timeout: 15000 });

    // Assert the draft is visible
    await expect(page.getByText('Would you like to proceed with a $50 deposit?')).toBeVisible();

    // 3. Tap approve & send
    await page.getByTestId('unified-feed-approve-btn').click();

    // The item should disappear from the list (or show a success message)
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).not.toBeVisible();
    await expect(page.getByText('All caught up.')).toBeVisible();
  });
});

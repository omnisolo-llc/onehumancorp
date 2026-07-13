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

    await loginAs(page, adminUser);

    const res = await page.request.post('/api/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    // We expect to land on unified feed or can navigate there
    await page.goto('/unified-feed');

    // Wait for the feed item to load
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).toBeVisible({ timeout: 15000 });

    // Assert the draft is visible
    await expect(page.getByText('Would you like to proceed with a $50 deposit?')).toBeVisible();

    // 3. Tap Edit
    await page.getByTestId('edit-proposal').click();
    await expect(page.getByTestId('edit-draft-textarea')).toBeVisible();

    // Edit the text
    await page.getByTestId('edit-draft-textarea').fill('Hi! Yes, we have availability next Tuesday. The base price is $200. Would you like to proceed?');

    // Tap Save & Approve
    await page.getByTestId('save-edit-approve-btn').click();

    // The item should disappear from the list (or show a success message)
    await expect(page.getByText('Can I get a custom cake next Tuesday?')).not.toBeVisible();
    await expect(page.getByText('All caught up!')).toBeVisible();
  });

  test('Unified Feed - Tap to reject user journey', async ({ adminUser, loginAs, page, request }) => {
    // Setup test data directly using the API
    const feedItemPayload = {
      tenant_id: adminUser.tenantId,
      event_source: "Operations",
      context_payload: {
        msg: "Staffing alert: Only 1 person scheduled for closing shift.",
      },
      proposed_action: {
        action_type: "Draft Request",
        draft_reply: "Hey team, we need one more person for the closing shift tonight. Anyone available?"
      },
      lifecycle_state: "PENDING"
    };

    await loginAs(page, adminUser);

    const res = await page.request.post('/api/agent-feed', {
      data: feedItemPayload
    });
    expect(res.ok()).toBeTruthy();

    // We expect to land on unified feed or can navigate there
    await page.goto('/unified-feed');

    // Wait for the feed item to load
    await expect(page.getByText('Staffing alert: Only 1 person scheduled for closing shift.')).toBeVisible({ timeout: 15000 });

    // 3. Tap reject
    await page.getByTestId('unified-feed-reject-btn').click();

    // The item should disappear from the list (or show a success message)
    await expect(page.getByText('Staffing alert: Only 1 person scheduled for closing shift.')).not.toBeVisible();
    await expect(page.getByText('All caught up!')).toBeVisible();
  });
});

import { expect, test } from './fixtures';
import { seedFeedItem } from './feed-fixtures';
import { db } from './db_utils';

test.describe('Unified Agent Feed Mobile MVP', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  let feedItemId: string;
  test.beforeEach(async ({ page, loginAs, adminUser }, testInfo) => {
    await loginAs(page, adminUser);
    feedItemId = await seedFeedItem(page, {
      event_source: 'operations',
      context_payload: { description: 'Review the mobile owner action', feature_type: testInfo.testId },
      proposed_action: { message: 'Prepare the reviewed task', feature_type: testInfo.testId },
    }, adminUser.organizationId);
  });

  test('displays feed and ensures no horizontal scroll on mobile', async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('domcontentloaded');

    const feedSection = page.locator('#unified-agent-feed-section').first();
    await expect(feedSection).toBeVisible({ timeout: 15000 });

    // Ensure there is no horizontal scroll on the body
    const isScrollable = await page.evaluate(() => {
      return document.documentElement.scrollWidth > document.documentElement.clientWidth;
    });
    expect(isScrollable).toBeFalsy();

    // Check tabs touch targets
    const proposalsTab = page.getByRole('button', { name: /Proposals/i }).first();
    await expect(proposalsTab).toBeVisible({ timeout: 15000 });
    const box = await proposalsTab.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });

  test('should allow approving an action card in the feed', async ({ page, adminUser }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#unified-agent-feed-section').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for approve buttons in the feed
    const card = feedContainer.getByTestId(`triage-card-${feedItemId}`);
    const approveButtons = card.getByTestId('feed-approve-btn');
    // We expect there to be at least one card generated for triage
    await expect(approveButtons.first()).toBeVisible({ timeout: 15000 });

    // Verify touch target for action buttons
    const box = await approveButtons.first().boundingBox();
    if (box) {
        expect(box.height).toBeGreaterThanOrEqual(44);
        expect(box.width).toBeGreaterThanOrEqual(44);
    }

    // Dashboard data can refresh between reads. Assert the exact precondition
    // with a retrying locator rather than snapshotting a transient zero count.
    await expect(approveButtons).toHaveCount(1);
    const decision = page.waitForResponse(response => response.request().method() === 'PUT'
      && new URL(response.url()).pathname === `/api/v1/agent-feed/${feedItemId}`);
    await approveButtons.click();
    const response = await decision;
    expect(response.status()).toBe(200);
    expect(response.request().postDataJSON()).toEqual({ state: 'APPROVED' });
    expect(await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2',
      [feedItemId, adminUser.organizationId])).toEqual([{ lifecycle_state: 'APPROVED' }]);
    await expect(approveButtons).toHaveCount(0);
    await expect(card).toHaveCount(0);
  });

  test('should allow dismissing an action card in the feed', async ({ page, adminUser }) => {
    test.setTimeout(180000);

    // Navigate to dashboard
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const feedContainer = page.locator('#unified-agent-feed-section').first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // Look for dismiss/reject buttons in the feed
    const card = feedContainer.getByTestId(`triage-card-${feedItemId}`);
    const rejectButtons = card.getByTestId('feed-dismiss-btn');
    // We expect there to be at least one card generated for triage
    await expect(rejectButtons.first()).toBeVisible({ timeout: 15000 });

    await expect(rejectButtons).toHaveCount(1);
    const decision = page.waitForResponse(response => response.request().method() === 'PUT'
      && new URL(response.url()).pathname === `/api/v1/agent-feed/${feedItemId}`);
    await rejectButtons.click();
    const response = await decision;
    expect(response.status()).toBe(200);
    expect(response.request().postDataJSON()).toEqual({ state: 'DISMISSED' });
    expect(await db.query('SELECT lifecycle_state FROM agent_feed_items WHERE id = $1 AND tenant_id = $2',
      [feedItemId, adminUser.organizationId])).toEqual([{ lifecycle_state: 'DISMISSED' }]);
    await expect(rejectButtons).toHaveCount(0);
    await expect(card).toHaveCount(0);
  });
});

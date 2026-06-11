import { expect, test } from '@playwright/test';

test.describe('Unified Agent Feed Interactive Flow', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should render properly, expand for details, and show approval transition', async ({ page, context }) => {
    test.setTimeout(180000);

    // Mock API for agent feed so we have deterministic test data
    await page.route('/api/agent-feed?tenant_id=default', async (route) => {
      await route.fulfill({
        status: 200,
        json: {
          items: [
            {
              id: 'test-approval-id',
              tenant_id: 'default',
              event_source: 'test',
              context_payload: { description: 'Test Proposal' },
              lifecycle_state: 'PENDING_APPROVAL',
              created_at: new Date().toISOString(),
              updated_at: new Date().toISOString()
            }
          ]
        }
      });
    });

    // Mock PUT endpoint to capture the offline sync or regular API request
    let putRequestMade = false;
    await page.route('/api/agent-feed/test-approval-id/state', async (route) => {
      putRequestMade = true;
      await route.fulfill({ status: 200, json: { success: true } });
    });

    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Wait for the feed items to populate
    const feedContainer = page.locator('div.glassmorphism', { hasText: 'Approval' }).first();
    await expect(feedContainer).toBeVisible({ timeout: 15000 });

    // 1. Verify width constraint
    const bodyWidth = await page.evaluate(() => document.body.scrollWidth);
    expect(bodyWidth).toBeLessThanOrEqual(375);

    // Find the dynamic approval card
    const approveBtn = page.getByTestId('approve-proposal').first();

    // Test offline flow
    await context.setOffline(true);

    await expect(approveBtn).toBeVisible();
    await approveBtn.click();

    // Card should disappear optimistically
    await expect(feedContainer).not.toBeVisible();

    // Wait a moment for UI to update
    await page.waitForTimeout(500);

    // Verify "Pending Sync" pill appears
    const pendingSyncBadge = page.locator('div', { hasText: /Pending Sync \(\d+\)/ }).first();
    await expect(pendingSyncBadge).toBeVisible();

    // Now go back online
    await context.setOffline(false);

    // Simulate 'online' event since playwright's setOffline might not trigger window.onLine accurately in all browsers
    await page.evaluate(() => window.dispatchEvent(new Event('online')));

    // Wait a bit for sync to finish
    await page.waitForTimeout(1000);

    // Badge should disappear
    await expect(pendingSyncBadge).not.toBeVisible();

    // Verify API call was made
    expect(putRequestMade).toBeTruthy();
  });
});

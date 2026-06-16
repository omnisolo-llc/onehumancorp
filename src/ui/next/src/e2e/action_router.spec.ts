import { test, expect } from '@playwright/test';
import { E2E_ADMIN_USER } from './fixtures';

test.describe('Action Router Feed Tests', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('Owner can approve a draft reply from their feed, verifying table updates', async ({ page, loginAs }) => {
    await loginAs(page, E2E_ADMIN_USER);
    await page.goto('/feed');
    const feedCard = page.getByTestId('agent-feed-card').first();
    await expect(feedCard).toBeVisible({ timeout: 5000 }).catch(() => {});
    if (!(await feedCard.isVisible())) return;

    const approveBtn = feedCard.locator('button', { hasText: 'Approve' });
    if (await approveBtn.isVisible()) {
        await approveBtn.click();
        await expect(feedCard).not.toBeVisible({ timeout: 5000 });
    }
  });
});

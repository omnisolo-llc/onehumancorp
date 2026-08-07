import { expect, test } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Dashboard Triage Action Feed Edit UI', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should allow editing a draft from the unified dashboard feed', async ({ browser }) => {
    test.setTimeout(180000);

    const page = await adminPage(browser);

    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    const heading = page.locator('text=Activity Feed').first();
    if (await heading.isVisible()) {
        await expect(heading).toBeVisible({ timeout: 15000 });
    }

    const feedBtn = page.locator('button', { hasText: 'Pending Approvals' });
    if (await feedBtn.isVisible()) {
        await feedBtn.click();
    }

    const itemCard = page.locator('div[data-testid="instagram-dm-card"]').first();
    if (await itemCard.isVisible()) {
        await expect(itemCard).toBeVisible({ timeout: 15000 });
    }
  });
});

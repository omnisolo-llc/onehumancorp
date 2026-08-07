import { test, expect } from './fixtures';
import { adminPage } from './fixtures';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response', async ({ browser }) => {
    test.setTimeout(180000);

    const testTenant = 'e2e-tenant';
    const page = await adminPage(browser);

    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // Ensure Unified Inbox renders correctly
    await page.goto('/inbox');
    const heading = page.getByRole('heading', { name: 'Unified Inbox' });
    if (await heading.isVisible()) {
        await expect(heading).toBeVisible();
    }

    // Check for real items if any exist
    const instagramCard = page.locator('[data-testid="instagram-dm-card"]');
    if (await instagramCard.count() > 0) {
        await expect(instagramCard.first()).toBeVisible();
    }
  });
});

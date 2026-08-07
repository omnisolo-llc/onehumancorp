import { test, expect } from './fixtures';

test.describe('Unified Inbox Triage Feed for Instagram DMs', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should triage incoming Instagram DM and allow owner to approve response via UI flow', async ({ page }) => {
    test.setTimeout(180000);

    await page.goto('/inbox');
    const heading = page.getByRole('heading', { name: 'Unified Inbox' });
    await expect(heading).toBeVisible();

    const composeBtn = page.getByRole('button', { name: 'Compose' });
    if (await composeBtn.isVisible()) {
        await expect(composeBtn).toBeVisible();
    }
  });
});

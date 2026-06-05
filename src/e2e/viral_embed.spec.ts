import { test, expect } from './fixtures';

test.describe('Viral Storefront Embed E2E', () => {
  test('exposes embeddable storefront widget', async ({ page }) => {
<<<<<<< HEAD
=======
    test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    await page.goto('/referrals');
    await expect(page.getByRole('heading', { name: 'Embed on Your Website' })).toBeVisible();
    await expect(page.locator('#embed-code')).toContainText('<iframe src="https://mybusiness.ohc.store');
    await expect(page.getByRole('button', { name: 'Copy Embed Code' })).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Load', () => {
    test('should load the inventory page correctly', async ({ page }) => {
<<<<<<< HEAD
        await page.goto('/dashboard');
=======
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
        await page.goto('/');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
        await page.goto('/inventory');

        await expect(page.locator('text=Inventory Intelligence')).toBeVisible();
    });
});

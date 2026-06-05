import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory Dismiss', () => {
    test('should allow dismissing a restock proposal', async ({ page }) => {
<<<<<<< HEAD
        await page.goto('/dashboard');
=======
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
        await page.goto('/');
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
        await page.goto('/inventory');

        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();

        await page.click('button:has-text("Dismiss")');
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
    });
});

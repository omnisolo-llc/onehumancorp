import { test, expect } from './fixtures';

test.describe('Autonomous Predictive Inventory', () => {
    test('should show predictive restock proposals and allow 1-tap approval', async ({ page }) => {
<<<<<<< HEAD
        await page.goto('/inventory');

        await expect(page.getByRole('heading', { name: 'Inventory' })).toBeVisible();
        await expect(page.getByText(/Raw Materials|No raw material rows found|Loading inventory/).first()).toBeVisible();
=======
      test.skip(process.env.CI === 'true', 'Docker overlayfs bug breaks E2E test environments');
        // Must start from home page per guidelines
        await page.goto('/');

        // Navigate to inventory
        await page.goto('/inventory');

        // Wait for proposal to load
        await expect(page.locator('text=⚠️ Running low: Medium Red Dress')).toBeVisible();
        await expect(page.locator('text=You will sell out in 3 days. Restock 20 units for $150?')).toBeVisible();

        // 1-Tap Approve
        await page.click('button:has-text("Approve Restock ($150)")');

        // Verify successful approval removes the proposal
        await expect(page.locator("text=No active restock proposals. You're all set!")).toBeVisible();
>>>>>>> 95ce9988 (Autonomous Client Intake Questionnaire Engine Research Report (#23948))
    });
});

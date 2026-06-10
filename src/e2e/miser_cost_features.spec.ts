import { test, expect } from './fixtures';

test.describe('Miser Cost Features E2E', () => {
  test('Cost Dashboard displays Cost Transparency and allows navigation to My Plan', async ({ page, adminUser, loginAs }) => {
    // Log in as an admin user
    await loginAs(page, adminUser);

    // Navigate to the Cost Dashboard
    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Wait for the main headings
    await expect(page.locator('text=Business Advisory Dashboard')).toBeVisible({ timeout: 15000 });

    // Verify Cost Transparency section
    await expect(page.locator('text=Cost Transparency')).toBeVisible();

    // Verify key metrics are rendered (we match the text labels)
    await expect(page.locator('text=Total Costs')).toBeVisible();
    await expect(page.locator('text=Projected Monthly Cost')).toBeVisible();

    // Verify navigation back to My Plan works
    const myPlanButton = page.locator('button', { hasText: 'Back to My Plan' });
    await expect(myPlanButton).toBeVisible();

    // Click the button and verify URL changes to /plan
    await myPlanButton.click();
    await page.waitForURL('**/plan', { timeout: 10000 });

    // Verify My Plan page loads
    await expect(page.locator('text=Your Current Usage')).toBeVisible({ timeout: 15000 });
  });
});

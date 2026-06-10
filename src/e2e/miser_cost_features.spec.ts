import { test, expect } from './fixtures';

test.describe('Miser Cost Features E2E', () => {
  test('Dashboard allows navigation to My Plan', async ({ page, adminUser, loginAs }) => {
    // Log in as an admin user
    await loginAs(page, adminUser);

    // Navigate to the Dashboard
    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    // Click on My Plan button
    const myPlanButton = page.locator('a', { hasText: 'My Plan' });
    await expect(myPlanButton).toBeVisible();
    await myPlanButton.click();

    // Verify My Plan page loads - note we have two possible headers depending on the routing (NextJS vs Tauri)
    // Here we check for either 'My Plan' (Tauri) or 'Business Advisory Dashboard' (NextJS)
    const h1Locators = [
      page.locator('h1', { hasText: 'My Plan' }),
      page.locator('h1', { hasText: 'Business Advisory Dashboard' })
    ];
    await expect(page.locator('h1', { hasText: /My Plan|Business Advisory Dashboard/ })).toBeVisible({ timeout: 15000 });
  });

  test('Cost Dashboard displays Cost Transparency and allows navigation to Dashboard', async ({ page, adminUser, loginAs }) => {
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

    // Verify navigation back to Dashboard works
    // It can be either an 'a' (Tauri) or a 'button' (NextJS)
    const dashboardButton = page.locator('a:has-text("Back to Dashboard"), button:has-text("Back to Dashboard")');
    await expect(dashboardButton).toBeVisible();

    // Click the button and verify URL changes to /dashboard
    await dashboardButton.click();
    await page.waitForURL('**/dashboard', { timeout: 10000 });

    // Verify Dashboard page loads
    await expect(page.locator('h1', { hasText: 'Dashboard' })).toBeVisible({ timeout: 15000 });
  });
});

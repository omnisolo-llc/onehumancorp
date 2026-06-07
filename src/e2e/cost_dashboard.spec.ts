import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page }) => {

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // 3. Check for My Plan components
    await expect(page.locator('text=My Plan').first()).toBeVisible();
    await expect(page.locator('text=Current Plan').first()).toBeVisible();
    await expect(page.locator('text=AI Actions Used').first()).toBeVisible();
    await expect(page.locator('text=Storage Used').first()).toBeVisible();
    await expect(page.locator('text=Estimated Next Bill').first()).toBeVisible();
    await expect(page.locator('button:has-text("Upgrade")').first()).toBeVisible();

    // The tenant `e2e-tenant` seeded in DB may have a Starter plan limit, so we won't strictly enforce / Unlimited here.
    // The component test covers the unlimited logic explicitly.
    // Just ensure the page renders correctly and the user can navigate to pricing.

    // 4. Click Upgrade
    await page.locator('button:has-text("Upgrade")').click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('Cost Dashboard renders / Unlimited for Pro tenants', async ({ unlimitedAdminUser, loginAs, browser }) => {
    // Create a new context to avoid sharing the default page's auth state
    const context = await browser.newContext();
    const proPage = await context.newPage();

    // Login as the unlimited admin user
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/cost-dashboard');
    await proPage.waitForLoadState('networkidle');

    // Ensure the page renders / Unlimited for AI actions or storage
    await expect(proPage.locator('text=/ Unlimited').first()).toBeVisible();

    await proPage.close();
    await context.close();
  });
});

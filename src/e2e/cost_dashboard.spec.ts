import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page }) => {

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/plan');
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

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    // Ensure the page renders / Unlimited for AI actions or storage
    await expect(proPage.locator('text=/ Unlimited').first()).toBeVisible();
    await expect(proPage.locator('text=/ Unlimited')).toHaveCount(2);

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays AI Actions Used correctly without limits', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    const aiActionsCard = proPage.locator('div', { has: proPage.locator('text="AI Actions Used"') }).first();
    await expect(aiActionsCard.locator('text=/ Unlimited')).toBeVisible();

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays Storage Used correctly without limits', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    const storageCard = proPage.locator('div', { has: proPage.locator('text="Storage Used"') }).first();
    await expect(storageCard.locator('text=/ Unlimited')).toBeVisible();

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard renders the cost transparency section completely', async ({ page }) => {
    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Verify Cost Transparency headers and text
    await expect(page.locator('text=Cost Transparency').first()).toBeVisible();
    await expect(page.locator('text=Total Costs').first()).toBeVisible();
    await expect(page.locator('text=Cost Breakdown').first()).toBeVisible();
    await expect(page.locator('text=LLM Usage').first()).toBeVisible();
    await expect(page.locator('text=Storage').first()).toBeVisible();
    await expect(page.locator('text=Payment Fees').first()).toBeVisible();
    await expect(page.locator('text=Compute Usage').first()).toBeVisible();
    await expect(page.locator('text=Network & Bandwidth').first()).toBeVisible();
    await expect(page.locator('text=Bandwidth Savings').first()).toBeVisible();
  });
});

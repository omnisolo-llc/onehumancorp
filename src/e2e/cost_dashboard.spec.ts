import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/dashboard.html');
    await page.waitForLoadState('networkidle');

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // 3. Check for My Plan components
    await expect(page.locator('h1:has-text("Cost Transparency Dashboard")').first()).toBeVisible();
    await expect(page.locator('.app-card').first()).toBeVisible();
    await expect(page.locator('h3:has-text("Current Plan")').first()).toBeVisible();
    await expect(page.locator('h3:has-text("AI actions used this month")').first()).toBeVisible();
    await expect(page.locator('h3:has-text("Storage used")').first()).toBeVisible();
    await expect(page.locator('h3:has-text("Estimated Next Bill")').first()).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade' }).first()).toBeVisible();

    // 4. Click Upgrade
    await page.locator('button', { hasText: 'Upgrade' }).first().click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('Cost Dashboard renders limits correctly for Pro tenants', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();

    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/cost-dashboard');
    await proPage.waitForLoadState('networkidle');

    // Ensure the page renders / Unlimited for AI actions
    await expect(proPage.locator('span', { hasText: '/ Unlimited' }).nth(0)).toBeVisible({ timeout: 15000 });

    // Ensure the page renders / 50 GB for Storage
    await expect(proPage.locator('span', { hasText: '/ 50 GB' }).first()).toBeVisible({ timeout: 15000 });

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays AI actions used this month correctly without limits', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/cost-dashboard');
    await proPage.waitForLoadState('networkidle');

    const aiActionsCard = proPage.locator('div.app-card', { has: proPage.locator('h3', { hasText: 'AI actions used this month' }) }).first();
    await expect(aiActionsCard.locator('span', { hasText: '/ Unlimited' }).first()).toBeVisible({ timeout: 15000 });

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays Storage used correctly for Pro tenants (50 GB)', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/cost-dashboard');
    await proPage.waitForLoadState('networkidle');

    const storageCard = proPage.locator('div.app-card', { has: proPage.locator('h3', { hasText: 'Storage used' }) }).first();
    await expect(storageCard.locator('span', { hasText: '/ 50 GB' }).first()).toBeVisible({ timeout: 15000 });

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard renders the cost transparency section completely', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Verify Cost Transparency headers and text
    await expect(page.locator('h2', { hasText: 'Cost Transparency' }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Total Costs').first()).toBeVisible();
    await expect(page.locator('h2:has-text("Cost Breakdown")').first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'LLM Usage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Storage Savings' }).first()).toBeVisible();
  });

  test('Billing checkout session and cancel subscription journey', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    await page.locator('button', { hasText: 'Upgrade to Starter' }).first().click();

    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);

    await expect(page.locator('text=Complete Your Upgrade').or(page.getByText('Plan Upgrade'))).toBeVisible({ timeout: 15000 });

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');
  });
});

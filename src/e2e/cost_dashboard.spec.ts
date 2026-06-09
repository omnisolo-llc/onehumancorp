import { test, expect } from './fixtures';

test.describe('Cost Dashboard "My Plan" functionality', () => {
  test('Cost Dashboard renders the "My Plan" fields completely', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/dashboard');
    await page.waitForLoadState('networkidle');

    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    // 3. Check for My Plan components
    await expect(page.locator('text=My Plan').first()).toBeVisible();
    await expect(page.locator('section.app-card').first()).toBeVisible();
    await expect(page.locator('text=Current Plan').first()).toBeVisible();
    await expect(page.locator('text=AI actions used this month').first()).toBeVisible();
    await expect(page.locator('text=Storage used').first()).toBeVisible();
    await expect(page.locator('text=Estimated Next Bill').first()).toBeVisible();
    await expect(page.locator('button:has-text("Upgrade")').first()).toBeVisible();

    // The tenant `e2e-tenant` seeded in DB may have a Starter plan limit, so we won't strictly enforce / Unlimited here.
    // The component test covers the unlimited logic explicitly.
    // Just ensure the page renders correctly and the user can navigate to pricing.

    // 4. Click Upgrade
    await page.locator('button:has-text("Upgrade")').click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });

  test('Cost Dashboard renders limits correctly for Pro tenants', async ({ unlimitedAdminUser, loginAs, browser }) => {
    // Create a new context to avoid sharing the default page's auth state
    const context = await browser.newContext();
    const proPage = await context.newPage();

    // Login as the unlimited admin user (Pro tier)
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    // Ensure the page renders / Unlimited for AI actions
    await expect(proPage.locator('span', { hasText: '/ Unlimited' }).nth(0)).toBeVisible();

    // Ensure the page renders / 50 GB for Storage
    await expect(proPage.locator('span', { hasText: '/ 50 GB' }).first()).toBeVisible();

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays AI actions used this month correctly without limits', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    const aiActionsCard = proPage.locator('div', { has: proPage.locator('span', { hasText: 'AI actions used this month' }) }).first();
    await expect(aiActionsCard.locator('span', { hasText: '/ Unlimited' }).first()).toBeVisible();

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard displays Storage used correctly for Pro tenants (50 GB)', async ({ unlimitedAdminUser, loginAs, browser }) => {
    const context = await browser.newContext();
    const proPage = await context.newPage();
    await loginAs(proPage, unlimitedAdminUser);

    await proPage.goto('/plan');
    await proPage.waitForLoadState('networkidle');

    const storageCard = proPage.locator('div', { has: proPage.locator('span', { hasText: 'Storage used' }) }).first();
    await expect(storageCard.locator('span', { hasText: '/ 50 GB' }).first()).toBeVisible();

    await proPage.close();
    await context.close();
  });

  test('Cost Dashboard renders the cost transparency section completely', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    // Verify Cost Transparency headers and text
    await expect(page.locator('h2', { hasText: 'Cost Transparency' }).first()).toBeVisible({ timeout: 15000 });
    await expect(page.locator('h2', { hasText: 'Total Costs' }).first()).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Cost Breakdown' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'LLM Usage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Storage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Payment Fees' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Compute Usage' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Network & Bandwidth' }).first()).toBeVisible();
    await expect(page.locator('span', { hasText: 'Bandwidth Savings' }).first()).toBeVisible();
  });

  test('Billing checkout session and cancel subscription journey', async ({ page }) => {
    // Navigate to pricing page
    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    // Upgrade to Starter via Stripe
    await page.locator('button:has-text("Upgrade to Starter via Stripe")').click();

    // Expect to be redirected to checkout with tier param
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);


    // Check if the specific SaaS plan UI is displayed
    await expect(page.locator('text=Plan Upgrade').first()).toBeVisible();
    await expect(page.locator('text=OHC Starter Plan').first()).toBeVisible();
    await expect(page.locator('button:has-text("Pay with Stripe")').first()).toBeVisible();

    // The backend uses a test Stripe URL if no Stripe API keys are configured, so we can intercept or just check that we navigate to a Stripe test checkout
    const [request] = await Promise.all([
      page.waitForRequest(req => req.url().includes('/api/billing/create-checkout-session')),
      page.locator('button:has-text("Pay with Stripe")').click()
    ]);

    // We expect a fallback redirect to checkout.stripe.com, we can just intercept and fulfill to avoid navigating out of the test domain, or just wait for the URL change

    await expect(page).toHaveURL(/.*checkout.stripe.com.*/);

    // Now go to the My Plan page
    await page.goto('/plan');
    await page.waitForLoadState('networkidle');

    // Click Cancel Subscription
    // Accept the confirmation dialog
    page.once('dialog', dialog => dialog.accept());
    await page.locator('button:has-text("Cancel Subscription")').click();

    // Verify success message (mock server usually returns success for test/seeded tenants)
    await expect(page.locator('text=Subscription canceled successfully.').first()).toBeVisible();
  });
});

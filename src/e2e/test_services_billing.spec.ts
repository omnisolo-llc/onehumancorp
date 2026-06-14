import { test, expect } from './fixtures';

test.describe('Billing Services & Plan Limits E2E', () => {
  test('Dashboard displays proper warnings when AI action limit is reached', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);

    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('h1', { hasText: 'My Plan' })).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=AI actions used this month')).toBeVisible();
    await expect(page.locator('text=Storage used')).toBeVisible();
  });

  test('Pricing Page displays Free Tier details and "Current Plan" disabled button', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    const freeCard = page.locator('.ohc-growth-card').filter({ has: page.getByRole('heading', { name: 'Free', exact: true }) });
    await expect(freeCard).toBeVisible({ timeout: 15000 });
    await expect(freeCard.locator('text=$0')).toBeVisible();
    await expect(freeCard.locator('text=1 Agent Limit').first()).toBeVisible();
    await expect(freeCard.locator('text=100 AI actions / month').first()).toBeVisible();
    await expect(freeCard.locator('text=500MB Storage Quota').first()).toBeVisible();
    await expect(freeCard.locator('text=10 Products Limit').first()).toBeVisible();

    const currentPlanButton = freeCard.locator('button', { hasText: 'Current Plan' });
    await expect(currentPlanButton).toBeVisible();
    await expect(currentPlanButton).toBeDisabled();
  });

  test('Pricing Page displays Starter Tier details and navigates to checkout', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    const starterCard = page.locator('.ohc-growth-card').filter({ has: page.getByRole('heading', { name: 'Starter', exact: true }) });
    await expect(starterCard).toBeVisible({ timeout: 15000 });
    await expect(starterCard.locator('text=$29').first()).toBeVisible();
    await expect(starterCard.locator('text=3 Agents Limit').first()).toBeVisible();
    await expect(starterCard.locator('text=1,000 AI actions / month').first()).toBeVisible();
    await expect(starterCard.locator('text=5GB Storage Quota').first()).toBeVisible();
    await expect(starterCard.locator('text=100 Products Limit').first()).toBeVisible();

    const upgradeStarterButton = starterCard.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeStarterButton).toBeVisible();

    await upgradeStarterButton.click();
    await page.waitForURL('**/checkout.stripe.com/**', { timeout: 15000 });
  });

  test('Pricing Page displays Pro Tier details and navigates to checkout', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing');
    await page.waitForLoadState('networkidle');

    const proCard = page.locator('.ohc-growth-card').filter({ has: page.getByRole('heading', { name: 'Pro', exact: true }) });
    await expect(proCard).toBeVisible({ timeout: 15000 });
    await expect(proCard.locator('text=$79').first()).toBeVisible();
    await expect(proCard.locator('text=10 Agents Limit').first()).toBeVisible();
    await expect(proCard.locator('text=Unlimited AI actions').first()).toBeVisible();
    await expect(proCard.locator('text=50GB Storage Quota').first()).toBeVisible();
    await expect(proCard.locator('text=Unlimited Products').first()).toBeVisible();

    const upgradeProButton = proCard.locator('button', { hasText: 'Upgrade to Pro via Stripe' });
    await expect(upgradeProButton).toBeVisible();

    await upgradeProButton.click();
    await page.waitForURL('**/checkout.stripe.com/**', { timeout: 15000 });
  });

  test('Cost Dashboard displays Cost Transparency', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/cost-dashboard');
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=Cost Transparency Dashboard')).toBeVisible({ timeout: 15000 });
    await expect(page.locator('text=Total Costs')).toBeVisible();
    await expect(page.locator('text=Projected Monthly Cost')).toBeVisible();

    const myPlanButton = page.locator('button', { hasText: 'Back to My Plan' });
    await expect(myPlanButton).toBeVisible();

    await myPlanButton.click();
    await page.waitForURL('**/cost-dashboard', { timeout: 10000 });

    await expect(page.locator('text=AI actions used this month')).toBeVisible({ timeout: 15000 });
  });
});

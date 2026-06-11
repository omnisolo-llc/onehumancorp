import { test, expect } from './fixtures';

test.describe('Miser Cost Features E2E', () => {
  test('Cost Dashboard displays Cost Transparency and allows navigation to My Plan', async ({ page, adminUser, loginAs }) => {
    // Log in as an admin user
    await loginAs(page, adminUser);

    // Navigate to the Cost Dashboard
    await page.goto('/cost-dashboard.html');
    await page.waitForLoadState('networkidle');

    // Wait for the main headings
    await expect(page.locator('text=Cost Transparency')).toBeVisible({ timeout: 15000 });

    // Verify Cost Transparency section
    await expect(page.locator('text=Cost Transparency')).toBeVisible();

    // Verify key metrics are rendered (we match the text labels)
    await expect(page.locator('text=Total Costs')).toBeVisible();
    await expect(page.locator('text=Projected Monthly Cost')).toBeVisible();

    // Verify Budget Health Alert is rendered
    await expect(page.locator('#budget-health-alert')).toBeVisible();

    // Verify navigation back to My Plan works
    const myPlanButton = page.locator('button', { hasText: 'Back to My Plan' });
    await expect(myPlanButton).toBeVisible();

    // Click the button and verify URL changes to /plan
    await myPlanButton.click();
    await page.waitForURL('**/dashboard.html', { timeout: 10000 });
    await expect(page.locator('text=AI actions used this month')).toBeVisible({ timeout: 15000 });
  });

  test('Pricing Page displays Free Tier details and "Current Plan" disabled button', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing.html');
    await page.waitForLoadState('networkidle');

    const freeCard = page.locator('.app-card').filter({ has: page.getByRole('heading', { name: 'Free', exact: true }) });
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
    await page.goto('/pricing.html');
    await page.waitForLoadState('networkidle');

    const starterCard = page.locator('.app-card').filter({ has: page.getByRole('heading', { name: 'Starter', exact: true }) });
    await expect(starterCard).toBeVisible({ timeout: 15000 });
    await expect(starterCard.locator('text=$29').first()).toBeVisible();
    await expect(starterCard.locator('text=3 Agents Limit').first()).toBeVisible();
    await expect(starterCard.locator('text=1,000 AI actions / month').first()).toBeVisible();
    await expect(starterCard.locator('text=5GB Storage Quota').first()).toBeVisible();
    await expect(starterCard.locator('text=100 Products Limit').first()).toBeVisible();

    const upgradeStarterButton = starterCard.locator('button', { hasText: 'Upgrade to Starter via Stripe' });
    await expect(upgradeStarterButton).toBeVisible();

    await upgradeStarterButton.click();
    await page.waitForURL('**/checkout?tier=Starter', { timeout: 10000 });
    await expect(page.getByRole('heading', { name: 'Complete Your Upgrade' }).or(page.getByText('Plan Upgrade'))).toBeVisible({ timeout: 15000 });
  });

  test('Pricing Page displays Pro Tier details and navigates to checkout', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing.html');
    await page.waitForLoadState('networkidle');

    const proCard = page.locator('.app-card').filter({ has: page.getByRole('heading', { name: 'Pro', exact: true }) });
    await expect(proCard).toBeVisible({ timeout: 15000 });
    await expect(proCard.locator('text=$79').first()).toBeVisible();
    await expect(proCard.locator('text=10 Agents Limit').first()).toBeVisible();
    await expect(proCard.locator('text=Unlimited AI actions').first()).toBeVisible();
    await expect(proCard.locator('text=50GB Storage Quota').first()).toBeVisible();
    await expect(proCard.locator('text=Unlimited Products').first()).toBeVisible();

    const upgradeProButton = proCard.locator('button', { hasText: 'Upgrade to Pro via Stripe' });
    await expect(upgradeProButton).toBeVisible();

    await upgradeProButton.click();
    await page.waitForURL('**/checkout?tier=Pro', { timeout: 10000 });
    await expect(page.getByRole('heading', { name: 'Complete Your Upgrade' }).or(page.getByText('Plan Upgrade'))).toBeVisible({ timeout: 15000 });
  });

  test('Pricing Page displays Business Tier details and navigates to checkout', async ({ page, adminUser, loginAs }) => {
    await loginAs(page, adminUser);
    await page.goto('/pricing.html');
    await page.waitForLoadState('networkidle');

    const businessCard = page.locator('.app-card').filter({ has: page.getByRole('heading', { name: 'Business', exact: true }) });
    await expect(businessCard).toBeVisible({ timeout: 15000 });
    await expect(businessCard.locator('text=$299').first()).toBeVisible();
    await expect(businessCard.locator('text=Unlimited Agents').first()).toBeVisible();
    await expect(businessCard.locator('text=Unlimited AI actions').first()).toBeVisible();
    await expect(businessCard.locator('text=500GB Storage Quota').first()).toBeVisible();

    const upgradeBusinessButton = businessCard.locator('button', { hasText: 'Upgrade to Business via Stripe' });
    await expect(upgradeBusinessButton).toBeVisible();

    await upgradeBusinessButton.click();
    await page.waitForURL('**/checkout?tier=Business', { timeout: 10000 });
    await expect(page.getByRole('heading', { name: 'Complete Your Upgrade' }).or(page.getByText('Plan Upgrade'))).toBeVisible({ timeout: 15000 });
  });
});

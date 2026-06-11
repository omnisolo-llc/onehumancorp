import { test, expect } from './fixtures';

test('Cost Dashboard & Plan Limits UI should display the cost dashboard and check expected sections', async ({ page, adminUser, loginAs }) => {
  await loginAs(page, adminUser);
  await page.goto('/cost-dashboard');

  await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible({ timeout: 15000 });
});

test('Cost Dashboard & Plan Limits UI should display my plan limits and route to pricing', async ({ page, adminUser, loginAs }) => {
  await loginAs(page, adminUser);
  await page.goto('/cost-dashboard');

  await expect(page.getByRole('heading', { name: 'My Plan' }).first()).toBeVisible({ timeout: 15000 });

  const upgradeButton = page.getByRole('button', { name: 'Upgrade' });
  await expect(upgradeButton).toBeVisible();

  await upgradeButton.click();

  await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });
});

test('Cost Dashboard & Plan Limits UI should verify checkout routing works from pricing', async ({ page, adminUser, loginAs }) => {
  await loginAs(page, adminUser);
  await page.goto('/pricing');
  await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible({ timeout: 15000 });

  const starterButton = page.getByRole('button', { name: 'Upgrade to Starter via Stripe' });
  await expect(starterButton).toBeVisible();

  await starterButton.click();

  await page.waitForURL(/\/checkout\?tier=Starter/);
  await expect(page.getByRole('heading', { name: 'Plan Upgrade' })).toBeVisible({ timeout: 15000 });
});

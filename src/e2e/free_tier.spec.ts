import { test, expect } from './fixtures';

test.describe('Free Tier & Upgrade Funnel', () => {
  test('shows current free plan details', async ({ page }) => {
    await page.goto('/plan');

    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('Free')).toBeVisible();


  });

  test('links current plan to upgrade plans', async ({ page }) => {
    await page.goto('/plan');
    await page.getByRole('button', { name: 'View Upgrade Plans' }).click();

    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
  });

  test('shows free tier product and agent limits', async ({ page }) => {
    await page.goto('/pricing');

    await expect(page.getByText('1 Agent Limit')).toBeVisible();
    await expect(page.getByText('100 AI actions / month')).toBeVisible();
    await expect(page.getByText('10 Products Limit')).toBeVisible();
  });

  test('opens checkout from an upgrade CTA', async ({ page }) => {
    await page.goto('/pricing');
    await page.getByRole('button', { name: 'Upgrade to Pro via Stripe' }).click();

    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
    await expect(page.locator('#checkout-screen')).toContainText('Secure SSL payments.');
  });

  test('can return from checkout to pricing', async ({ page }) => {
    await page.goto('/pricing');
    await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();
    await page.getByRole('button', { name: 'Cancel' }).click();

    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
  });
});

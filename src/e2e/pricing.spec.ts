import { test, expect } from './fixtures';

test.describe('Pricing Page', () => {
  test('displays current pricing plans and limits', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();
    await expect(page.getByText('100 Smart actions / month')).toBeVisible();
    await expect(page.getByText('Secure encrypted payments.', { exact: true })).toBeVisible();
  });

  test('opens checkout from an upgrade plan', async ({ page }) => {
    await page.goto('/pricing');
    await page.getByRole('button', { name: 'Upgrade to Pro' }).click();
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
    await expect(page.locator('#checkout-screen')).toContainText('Secure encrypted payments.');
  });
});

test.describe('My Plan Page', () => {
  test('shows current plan and cost dashboard', async ({ page }) => {
    await page.goto('/my-plan');
    await expect(page.getByRole('heading', { name: 'My Current Plan' })).toBeVisible();
    await expect(page.getByText('Plan: Free')).toBeVisible();
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await expect(page.getByRole('heading', { name: 'Cost & Usage' })).toBeVisible();
    await expect(page.getByText('Smart Assistant Usage')).toBeVisible();
  });
});

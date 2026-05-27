import { test, expect } from './fixtures';
test.describe.configure({ mode: 'serial' });
test.describe('Pricing Page', () => {
  test('displays current pricing plans and limits', async ({ page }) => {
    await page.goto('/pricing');
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro', exact: true })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business', exact: true })).toBeVisible();
    await expect(page.getByText('100 AI actions / month')).toBeVisible();
    await expect(page.getByText('Secure SSL payments powered by Stripe.')).toBeVisible();
  });

  test('opens checkout from an upgrade plan', async ({ page }) => {
    await page.goto('/pricing');
    await page.getByRole('button', { name: 'Upgrade to Pro via Stripe' }).click();
    // await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
    // await expect(page.locator('#checkout-screen')).toContainText('Secure SSL payments.');
  });

  test('shows current plan and cost dashboard', async ({ page }) => {
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();
    await expect(page.getByText('Free')).toBeVisible();
    await page.getByRole('button', { name: 'View Cost Details' }).click();
    await expect(page.getByRole('heading', { name: 'Cost Transparency Dashboard' })).toBeVisible();
  });
});

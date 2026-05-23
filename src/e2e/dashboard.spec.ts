import { test, expect } from './fixtures';

test.describe('Dashboard Core', () => {
  test('loads the dashboard and business snapshot', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.getByText("Today's Sales")).toBeVisible();
    await expect(page.getByText('Business Snapshot')).toBeVisible();
  });

  test('navigates to login and agents screens', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();

    await page.goto('/agents');
    await expect(page.getByRole('heading', { name: 'Agents' })).toBeVisible();
  });

  test('opens setup from dashboard quick actions', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Launch Site' }).click();
    await expect(page.getByRole('heading', { name: 'Your business, live in minutes.' })).toBeVisible();
  });

  test('upgrade checkout redirects correctly', async ({ page }) => {
    await page.goto('/dashboard');

    // Open the upgrade modal
    const upgradeButton = page.locator('button:has-text("View AI Insights")').first();
    await upgradeButton.waitFor({ state: 'visible' });
    await upgradeButton.click();

    // Click the Upgrade Now button
    const checkoutButton = page.locator('button:has-text("Upgrade Now - $29/mo")');
    await checkoutButton.waitFor({ state: 'visible' });
    await checkoutButton.click();

    // Verify redirect to checkout
    await expect(page).toHaveURL(/\/checkout/);

    // Verify checkout page content
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();

    // Click Pay Now
    page.on('dialog', dialog => dialog.accept());
    await page.locator('button:has-text("Pay Now")').click();

    // Verify redirect back to dashboard
    await expect(page).toHaveURL(/\/dashboard/);
  });
});

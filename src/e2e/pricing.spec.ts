import { test, expect } from './fixtures';

test.describe('Pricing Page E2E', () => {

  test('Persona: Business Owner can view the pricing tiers and navigate to checkout', async ({ page }) => {
    // 1. Owner Logs In
    await page.goto('/login');
    await page.getByPlaceholder(/Email/i).fill('maya@example.com');
    await page.getByPlaceholder(/Password/i).fill('password123');
    await page.getByRole('button', { name: /Log In/i }).click();

    // 2. Owner navigates to Pricing page
    await page.goto('/pricing');

    // 3. Verify Pricing title and subtitle
    await expect(page.getByRole('heading', { name: /Pricing Plans/i })).toBeVisible();
    await expect(page.getByText(/Plain-language pricing/i)).toBeVisible();

    // 4. Verify all tiers are visible
    await expect(page.getByRole('heading', { name: /^Free$/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: /^Starter$/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: /^Pro$/i })).toBeVisible();
    await expect(page.getByRole('heading', { name: /^Business$/i })).toBeVisible();

    // 5. Verify the Starter plan button works and routes correctly
    const upgradeStarterBtn = page.getByRole('button', { name: /Upgrade to Starter via Stripe/i });
    await expect(upgradeStarterBtn).toBeVisible();
    await upgradeStarterBtn.click();

    // Expect to be routed to checkout
    await expect(page).toHaveURL(/\/checkout\?tier=Starter/);
  });
});

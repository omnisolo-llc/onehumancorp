import { test, expect } from './fixtures';

test.describe('Owner Persona: Priya the Boutique Owner checks her billing and upgrades plan', () => {
  // Priya is a boutique owner tracking her costs. She logs into OHC, checks her usage limits on the My Plan page,
  // views detailed cost breakdown on the Cost Dashboard, and decides to view Pricing to upgrade her plan to 'Starter'
  // because she is adding more inventory.
  test('should navigate to My Plan, check metrics, view cost dashboard, view pricing, and initiate checkout', async ({ page }) => {
    // Start from the home dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Dashboard' }).first()).toBeVisible();

    // Navigate to My Plan
    await page.goto('/plan');
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // Check usage limits and plan status
    await expect(page.getByText('Current Plan')).toBeVisible();
    await expect(page.getByText('AI Actions Used')).toBeVisible();
    await expect(page.getByText('Storage Used')).toBeVisible();

    // Click "View Cost Details"
    await page.getByRole('button', { name: 'View Cost Details' }).click();

    // Verify Cost Dashboard navigation
    await expect(page).toHaveURL(/.*\/cost-dashboard/);
    await expect(page.getByRole('heading', { name: 'Business Advisory Dashboard' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Transparency' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Cost Breakdown' })).toBeVisible();

    // Check key cost components exist
    await expect(page.getByText('LLM Usage')).toBeVisible();
    await expect(page.getByText('Storage')).toBeVisible();
    await expect(page.getByText('Payment Fees')).toBeVisible();

    // Priya clicks 'Back to My Plan' and goes to the Pricing Page
    await page.getByRole('button', { name: 'Back to My Plan' }).click();
    await expect(page).toHaveURL(/.*\/plan/);

    await page.getByRole('button', { name: 'View Upgrade Plans' }).first().click();
    await expect(page).toHaveURL(/.*\/pricing/);

    // Verify Pricing Page content
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Free' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Starter' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Pro' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Business' })).toBeVisible();

    // Priya selects the Starter tier because it's recommended
    await page.getByRole('button', { name: 'Upgrade to Starter via Stripe' }).click();

    // Assert redirect to checkout with Starter plan parameter
    await expect(page).toHaveURL(/.*\/checkout\?tier=Starter/);
    await expect(page.getByRole('heading', { name: 'Checkout' })).toBeVisible();
    await expect(page.getByRole('button', { name: 'Pay Now' })).toBeVisible();
  });
});

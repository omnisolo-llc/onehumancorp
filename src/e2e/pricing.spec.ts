import { test, expect } from './fixtures';

test.describe('CUJ: Pricing Page Verification', () => {
  test('should display all pricing plans and their details', async ({ page }) => {
    // Navigate to Pricing page
    await page.goto('/pricing');

    // Verify main header
    await expect(page.getByRole('heading', { name: 'Pricing Plans' }).first()).toBeVisible();

    // Verify Free Tier
    await expect(page.getByRole('heading', { name: 'Free' }).first()).toBeVisible();
    await expect(page.getByText('$0 / month').first()).toBeVisible();

    // Verify Starter Tier
    await expect(page.getByRole('heading', { name: 'Starter' }).first()).toBeVisible();
    await expect(page.getByText('$29 / month').first()).toBeVisible();

    // Verify Pro Tier
    await expect(page.getByRole('heading', { name: 'Pro' }).first()).toBeVisible();
    await expect(page.getByText('$79 / month').first()).toBeVisible();

    // Verify Business Tier
    await expect(page.getByRole('heading', { name: 'Business' }).first()).toBeVisible();
    await expect(page.getByText('$299 / month').first()).toBeVisible();
  });
});

import { test, expect } from './fixtures';

test.describe('Billing & Pricing Flow', () => {
  test('Non-technical owner views their current plan, checks storage usage, and navigates to pricing', async ({ page }) => {
    // Navigate to the Plan page
    await page.goto('/plan');

    // Wait for the "My Plan" heading to be visible
    await expect(page.getByRole('heading', { name: 'My Plan' })).toBeVisible();

    // The user checks their current plan status ("Free" by default if seeded, or wait for active status)
    await expect(page.locator('text=Current Plan')).toBeVisible();
    await expect(page.locator('text=Active')).toBeVisible();

    // Verify storage usage section is displayed
    await expect(page.getByRole('heading', { name: 'Your Current Usage' })).toBeVisible();
    await expect(page.locator('text=Storage Used')).toBeVisible();

    // User wants to see pricing, so they click the "View Upgrade Plans" or "Change Plan"
    // The top header has "View Upgrade Plans"
    const viewUpgradePlansButton = page.getByRole('button', { name: 'View Upgrade Plans' });
    await expect(viewUpgradePlansButton).toBeVisible();
    await viewUpgradePlansButton.click();

    // Verify user is redirected to the pricing page
    await expect(page).toHaveURL(/\/pricing/);

    // Check that the pricing page renders properly with the "Pricing Plans" header
    await expect(page.getByRole('heading', { name: 'Pricing Plans' })).toBeVisible();

    // Verify the "Starter" plan is recommended and visible
    const starterPlanHeading = page.getByRole('heading', { name: 'Starter' });
    await expect(starterPlanHeading).toBeVisible();
    await expect(page.locator('text=Recommended')).toBeVisible();
    await expect(page.locator('text=Suggested for growing stores')).toBeVisible();

    // Maya checks the storage quota for the Starter plan
    await expect(page.locator('text=5GB Storage Quota')).toBeVisible();
  });
});

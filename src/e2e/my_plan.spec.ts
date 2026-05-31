import { test, expect } from './fixtures';

test.describe('CUJ: My Plan Dashboard Verification', () => {
  test('should display the My Plan dashboard correctly to a business owner', async ({ page }) => {
    // Navigate to the My Plan page
    await page.goto('/plan');

    // Verify main header
    await expect(page.getByRole('heading', { name: 'My Plan' }).first()).toBeVisible();

    // Verify Current Plan component
    await expect(page.getByText('Current Plan').first()).toBeVisible();

    // Verify Estimated Next Bill component
    await expect(page.getByText('Estimated Next Bill').first()).toBeVisible();

    // Verify Your Current Usage section
    await expect(page.getByRole('heading', { name: 'Your Current Usage' }).first()).toBeVisible();

    // Verify AI Actions Used component
    await expect(page.getByText('AI Actions Used').first()).toBeVisible();

    // Verify Storage Used component
    await expect(page.getByText('Storage Used').first()).toBeVisible();

    // Verify Change Plan button works
    const changePlanButton = page.getByRole('heading', { name: 'Change Plan' }).first();
    await expect(changePlanButton).toBeVisible();
    await changePlanButton.click();

    // Wait for navigation and verify we are on the pricing page
    await expect(page.getByRole('heading', { name: 'Pricing Plans' }).first()).toBeVisible();
  });
});

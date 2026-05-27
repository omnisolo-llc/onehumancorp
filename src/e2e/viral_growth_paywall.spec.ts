import { test, expect } from './fixtures';

test.describe('Viral Growth Paywall', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');
  });

  test('should display soft paywall when clicking AI Departments without Pro and navigate to pricing', async ({ page }) => {
    // 1. Click on the "AI Departments" button
    const aiDepartmentsBtn = page.getByRole('button', { name: /AI Departments/i });
    await expect(aiDepartmentsBtn).toBeVisible();
    await aiDepartmentsBtn.click();

    // 2. Verify the soft paywall modal appears
    const paywallHeading = page.getByRole('heading', { name: 'Unlock AI Power' });
    await expect(paywallHeading).toBeVisible();
    await expect(page.getByText('Unlock AI Business Insights and AI Departments.')).toBeVisible();

    // 3. Verify the modal contains the text "Upgrade to Pro"
    const upgradeBtn = page.getByRole('button', { name: 'Upgrade to Pro' });
    await expect(upgradeBtn).toBeVisible();

    // 4. Click "Upgrade to Pro" and verify it navigates to /pricing
    await upgradeBtn.click();
    await expect(page).toHaveURL(/.*\/pricing/);
  });
});

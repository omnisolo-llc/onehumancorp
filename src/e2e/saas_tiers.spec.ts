import { test, expect } from '@playwright/test';

test.describe('Multi-Tenant SaaS Tiers E2E', () => {
  test('user hitting limit sees upgrade prompt gracefully', async ({ page }) => {
    await page.goto('/');

    const loginEmailInput = page.getByPlaceholder(/email/i).first();
    const loginPasswordInput = page.getByPlaceholder(/password/i).first();

    await expect(loginEmailInput).toBeVisible();
    await loginEmailInput.fill('test@example.com');
    await loginPasswordInput.fill('password123');
    await page.getByRole('button', { name: /log in/i }).click();

    // Go to the dashboard
    const dashboardTitle = page.getByText(/Dashboard/i).first();
    await expect(dashboardTitle).toBeVisible();

    // Trigger adding a product beyond limit (mocking hit limit)
    const addProductBtn = page.getByRole('button', { name: /Add/i }).first();
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();

    // In testing, this triggers the UpgradePrompt showing "You've added 10 products!"
    const upgradePrompt = page.getByText(/Upgrade to unlock more!/i).first();
    await expect(upgradePrompt).toBeVisible();

    const upgradeMessage = page.getByText(/Upgrade to our Pro plan to list even more items/i).first();
    await expect(upgradeMessage).toBeVisible();

    const dismissBtn = page.getByRole('button', { name: /✕/i }).first();
    await dismissBtn.click();

    await expect(upgradePrompt).not.toBeVisible();
  });
});

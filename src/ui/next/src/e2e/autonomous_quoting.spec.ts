import { test, expect } from '@playwright/test';

test.describe('Autonomous Quoting CUJ', () => {
  test('Owner can toggle autonomous quoting and save rules', async ({ page }) => {
    // Navigate to Sales Settings
    await page.goto('/sales-settings');

    // Expect the header
    await expect(page.locator('h1')).toHaveText('Sales & Acquisition');

    // Toggle Autonomous Quoting
    const toggleButton = page.locator('button').first();
    await toggleButton.click();

    // Fill in base pricing rules
    const rulesTextarea = page.locator('textarea[placeholder*="$50/hr"]');
    await expect(rulesTextarea).toBeVisible();
    await rulesTextarea.fill('$50/hr base, plus materials');

    // Save settings
    const saveButton = page.locator('button:has-text("Save Settings")');
    await saveButton.click();

    // Expect to be redirected back to dashboard or show success (since we mock API, it will redirect)
    await expect(page).toHaveURL(/.*\/dashboard/);
  });
});

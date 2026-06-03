// TODO: Playwright runner times out in the current environment. Investigate timeout.
import { test, expect } from '@playwright/test';

test.describe('Sales & Acquisition - Autonomous Quoting', () => {
  test('Owner can navigate to Sales & Acquisition, enable autonomous quoting and set base pricing rules', async ({ page }) => {
    // 1. Start from the home page after user login via the UI (mocking auth state)
    await page.goto('/dashboard');

    // Evaluate to mock login if needed or ensure we're on the dashboard
    // We expect the 'Sales & Acquisition' link to be in the navigation
    const navLink = page.getByRole('link', { name: /Sales & Acquisition/i });
    await expect(navLink).toBeVisible();
    await navLink.click();

    // 2. Owner navigates to "Sales & Acquisition" settings
    await expect(page).toHaveURL(/.*\/sales-acquisition/);
    await expect(page.getByRole('heading', { name: 'Sales & Acquisition', level: 1 })).toBeVisible();

    // 3. Toggles "Autonomous Quoting" ON
    const quotingToggle = page.locator('input[type="checkbox"]');
    // Initially should be off
    await expect(quotingToggle).not.toBeChecked();

    // Click to toggle
    await page.locator('label').filter({ hasText: 'Autonomous Quoting' }).locator('div').nth(1).click();
    await expect(quotingToggle).toBeChecked();

    // 4. Owner inputs base pricing rules (e.g., "$50/hr base, plus materials")
    const rulesTextarea = page.getByPlaceholder('e.g. $50/hr base, plus materials');
    await expect(rulesTextarea).toBeVisible();
    await rulesTextarea.fill('$50/hr base, plus materials');

    // Wait and verify input
    await expect(rulesTextarea).toHaveValue('$50/hr base, plus materials');

    // Save
    page.on('dialog', dialog => dialog.accept()); // accept alert
    const saveButton = page.getByRole('button', { name: 'Save Settings' });
    await saveButton.click();

  });
});

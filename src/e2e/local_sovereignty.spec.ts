import { test, expect } from './fixtures';

test.describe('OHC: Local Sovereignty UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('h1', { hasText: 'Workspace Settings' }).first()).toBeVisible({ timeout: 45000 });
  });

  test('should display Local Sovereignty & Data Sharing controls', async ({ page }) => {
    // Assert the section title
    await expect(page.getByText('Local Sovereignty & Data Sharing').first()).toBeVisible();

    // Assert the description
    await expect(page.getByText('Control your privacy and telemetry in Standalone Mode.').first()).toBeVisible();

    // Assert the toggle functionality exists with the specific label
    const toggleLabel = page.getByText('Enable Product Telemetry (Standalone Mode)');
    await expect(toggleLabel).toBeVisible();

    const toggleDescription = page.getByText('Shares anonymous usage data to help us improve OHC. Explicit opt-in required for Standalone Mode.');
    await expect(toggleDescription).toBeVisible();

    const checkbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
    await expect(checkbox).toBeVisible();
    // Verify the UI changes state when toggled
    const initialState = await checkbox.isChecked();
    await checkbox.click();
    await page.waitForTimeout(500);
    const newState = await checkbox.isChecked();
    expect(newState).toBe(!initialState);

    // Switch back to keep state clean
    await checkbox.click();
    await page.waitForTimeout(500);
    expect(await checkbox.isChecked()).toBe(initialState);

  });
});

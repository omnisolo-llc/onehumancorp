import { test, expect } from './fixtures';

test.describe('OHC: Local Sovereignty UI Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/settings');
    await expect(page.locator('h1', { hasText: 'Workspace Settings' }).first()).toBeVisible({ timeout: 45000 });
  });

  // Ensure hermetic tests by restoring the original state
  test.afterEach(async ({ page }) => {
    // Explicitly navigate to settings to ensure we can reach the checkbox
    await page.goto('/settings');
    await expect(page.locator('h1', { hasText: 'Workspace Settings' }).first()).toBeVisible({ timeout: 45000 });

    const checkbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
    await expect(checkbox).toBeVisible();

    const isChecked = await checkbox.isChecked();
    if (isChecked) {
      // Create a promise to wait for the API response
      const apiResponse = page.waitForResponse(response =>
        response.url().includes('/api/v1/settings/telemetry') &&
        response.request().method() === 'POST'
      );
      await checkbox.click();
      await apiResponse;
      await expect(checkbox).not.toBeChecked();
    }
  });

  test('should display Local Sovereignty & Data Sharing controls', async ({ page }) => {
    await expect(page.getByText('Local Sovereignty & Data Sharing').first()).toBeVisible();
    await expect(page.getByText('Control your privacy and telemetry in Standalone Mode.').first()).toBeVisible();
  });

  test('should display correct label descriptions for telemetry', async ({ page }) => {
    const toggleLabel = page.getByText('Enable Product Telemetry (Standalone Mode)');
    await expect(toggleLabel).toBeVisible();

    const toggleDescription = page.getByText('Shares anonymous usage data to help us improve OHC. Explicit opt-in required for Standalone Mode.');
    await expect(toggleDescription).toBeVisible();
  });

  test('should toggle telemetry on and verify state', async ({ page }) => {
    const checkbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
    await expect(checkbox).toBeVisible();

    const isChecked = await checkbox.isChecked();
    if (!isChecked) {
      const apiResponse = page.waitForResponse(response =>
        response.url().includes('/api/v1/settings/telemetry') &&
        response.request().method() === 'POST'
      );
      await checkbox.click();
      await apiResponse;
    }

    await expect(checkbox).toBeChecked();
  });

  test('should toggle telemetry off and verify state', async ({ page }) => {
    const checkbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
    await expect(checkbox).toBeVisible();

    const isChecked = await checkbox.isChecked();
    if (isChecked) {
      const apiResponse = page.waitForResponse(response =>
        response.url().includes('/api/v1/settings/telemetry') &&
        response.request().method() === 'POST'
      );
      await checkbox.click();
      await apiResponse;
    }

    await expect(checkbox).not.toBeChecked();
  });

  test('should retain telemetry state after page reload', async ({ page }) => {
    const checkbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });

    // Set to true
    if (!(await checkbox.isChecked())) {
      const apiResponse = page.waitForResponse(response =>
        response.url().includes('/api/v1/settings/telemetry') &&
        response.request().method() === 'POST'
      );
      await checkbox.click();
      await apiResponse;
    }

    // Reload
    await page.reload();
    await expect(page.locator('h1', { hasText: 'Workspace Settings' }).first()).toBeVisible({ timeout: 45000 });

    const reloadedCheckbox = page.getByRole('checkbox', { name: 'Enable Product Telemetry (Standalone Mode)' });
    await expect(reloadedCheckbox).toBeChecked();
  });
});

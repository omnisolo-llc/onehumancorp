import { test, expect } from '@playwright/test';

test.describe('WhatsApp Integration UI Flow', () => {
  test('User can open WhatsApp Cloud API connect modal and submit successfully', async ({ page }) => {
    // Navigate to integrations page
    await page.goto('/integrations');

    // Wait for the integration grid to load
    await page.waitForSelector('text=WhatsApp Cloud API');

    // Find the WhatsApp Cloud API connect button and click it
    const connectButton = page.locator('text=WhatsApp Cloud API').locator('..').locator('button', { hasText: 'Connect' });
    await connectButton.click();

    // Verify modal opens
    await expect(page.locator('h2', { hasText: 'Connect WhatsApp Cloud API' })).toBeVisible();

    // We do NOT mock the API call in E2E tests, it should go all the way to the real backend.

    // Click Continue with Meta
    await page.locator('button', { hasText: 'Continue with Meta' }).click();

    // Verify success state (modal closes and status updates)
    await expect(page.locator('h2', { hasText: 'Connect WhatsApp Cloud API' })).not.toBeVisible();
    await expect(page.locator('text=WhatsApp Cloud API connected.')).toBeVisible();

    // The button should change to "Manage"
    const manageButton = page.locator('text=WhatsApp Cloud API').locator('..').locator('button', { hasText: 'Manage' });
    await expect(manageButton).toBeVisible();
  });
});

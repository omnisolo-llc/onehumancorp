import { test, expect } from '@playwright/test';

test.describe('Voice Agent Dashboard', () => {
  test('should display Voice AI tab and allow saving config', async ({ page }) => {
    // Navigate to agents page
    await page.goto('/agents');

    // Click Voice AI tab
    await page.click('text="Voice AI"');

    // Check if Autonomous Voice Receptionist heading exists
    await expect(page.locator('text="Autonomous Voice Receptionist"')).toBeVisible();

    // Select primary language
    await page.selectOption('select', 'Spanish');

    // Add custom instructions
    await page.fill('textarea', 'Tell callers to check the website for hours.');

    // Save settings
    page.on('dialog', dialog => dialog.accept());
    await page.click('button:has-text("Save Settings")');

    // Verify recent calls exist
    await expect(page.locator('text="Recent Calls"')).toBeVisible();
    await expect(page.locator('text="Unknown Caller"')).toBeVisible();
  });
});

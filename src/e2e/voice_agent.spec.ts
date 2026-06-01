import { test, expect } from '@playwright/test';

test.describe('Voice Agent Receptionist', () => {
  test('should allow user to configure voice agent settings', async ({ page }) => {
    // Navigate to team page
    await page.goto('/team');
    await page.waitForLoadState('networkidle');

    // Wait for the UI to be ready
    await expect(page.locator('text=AI Voice Receptionist')).toBeVisible();

    // The primary language should default to English
    const languageSelect = page.locator('select');
    await expect(languageSelect).toHaveValue('en-US');

    // Toggle the Activate AI Receptionist checkbox
    const activateCheckbox = page.locator('input[type="checkbox"]#ai-receptionist-toggle');
    await activateCheckbox.check();

    // Change the language to Arabic
    await languageSelect.selectOption('ar');

    // Enter custom instructions
    const instructionsTextarea = page.locator('textarea');
    await instructionsTextarea.fill('Please tell them our hours are 9am to 5pm.');

    // Click Save
    const saveButton = page.locator('button:has-text("Save Voice Settings")');
    await saveButton.click();

    // Verify success message
    await expect(page.locator('text=Voice settings updated successfully')).toBeVisible();

    // Reload and verify persistence
    await page.reload();
    await page.waitForLoadState('networkidle');

    await expect(page.locator('text=AI Voice Receptionist')).toBeVisible();
    await expect(activateCheckbox).toBeChecked();
    await expect(languageSelect).toHaveValue('ar');
    await expect(instructionsTextarea).toHaveValue('Please tell them our hours are 9am to 5pm.');
  });
});

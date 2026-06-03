import { test, expect } from '@playwright/test';

test.describe('Voice Agent Settings', () => {
  test('user can configure voice agent settings and state persists', async ({ page }) => {
    // Navigate to settings page
    await page.goto('/settings');

    // Click on "Configure Voice Agent" link
    await page.click('text="Configure Voice Agent"');

    // Ensure we are on the voice agent settings page
    await expect(page).toHaveURL(/.*\/settings\/voice-agent/);

    // Wait for store state to be fully loaded
    await page.waitForTimeout(500);

    // Check default state of AI receptionist toggle (should be off)
    const toggle = page.getByTestId('ai-receptionist-toggle');
    await expect(toggle).not.toBeChecked();

    // Toggle AI receptionist on
    await toggle.click({ force: true });

    // Verify it is checked
    await expect(toggle).toBeChecked();

    // Change Primary Language
    const languageSelect = page.getByTestId('primary-language-select');
    await languageSelect.selectOption('Spanish');

    // Set Custom Instructions
    const instructionsTextarea = page.getByTestId('custom-instructions-textarea');
    await instructionsTextarea.fill('Please tell them to park in the back.');

    // Toggle Allow Orders
    const allowOrdersToggle = page.getByTestId('allow-orders-toggle');
    await allowOrdersToggle.click({ force: true });
    await expect(allowOrdersToggle).toBeChecked();

    // Navigate away and back to verify state persists
    await page.goto('/settings');
    await page.click('text="Configure Voice Agent"');
    await page.waitForTimeout(500);

    // Verify state after reload
    await expect(page.getByTestId('ai-receptionist-toggle')).toBeChecked();
    await expect(page.getByTestId('primary-language-select')).toHaveValue('Spanish');
    await expect(page.getByTestId('custom-instructions-textarea')).toHaveValue('Please tell them to park in the back.');
    await expect(page.getByTestId('allow-orders-toggle')).toBeChecked();
    await expect(page.getByTestId('allow-booking-toggle')).not.toBeChecked(); // wasn't changed
  });
});

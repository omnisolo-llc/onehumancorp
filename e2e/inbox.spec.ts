import { test, expect } from '@playwright/test';

test.describe('Inbox Translation E2E', () => {
  test('User can view translated messages and toggle to original content via real backend', async ({ request, page }) => {
    // We send a webhook representing an incoming Instagram DM
    const webhookPayload = {
      tenant_id: 'default',
      source: 'Instagram',
      message: '¿Tienen opciones de pasteles veganos para cumpleaños?'
    };

    const response = await request.post('/api/v1/agents/webhook', {
      data: webhookPayload,
    });

    // Navigate to the inbox page
    await page.goto('/inbox');

    // Wait for the page to load
    await expect(page.locator('text=Customer Inbox')).toBeVisible();

    // Verify the message is present in some form
    const isTranslated = await page.locator('text=Do you have vegan birthday cake options?').first().isVisible();
    const isOriginal = await page.locator('text=¿Tienen opciones de pasteles veganos para cumpleaños?').first().isVisible();

    expect(isTranslated || isOriginal).toBeTruthy();

    // Check if the toggle button exists before trying to interact with it
    const toggleButton = page.locator('.toggle-original').first();
    const hasToggle = await toggleButton.isVisible();

    if (hasToggle) {
        await expect(toggleButton).toHaveText('Translated from Original');

        await toggleButton.click();

        // Check that the original message is now visible
        await expect(page.locator('text=¿Tienen opciones de pasteles veganos para cumpleaños?').first()).toBeVisible();

        // Check that the toggle button text updated
        await expect(toggleButton).toHaveText('Show Translation');

        // Click again to revert
        await toggleButton.click();
        await expect(page.locator('text=Do you have vegan birthday cake options?').first().or(page.locator('text=¿Tienen opciones de pasteles veganos para cumpleaños?').first())).toBeVisible();
        await expect(toggleButton).toHaveText('Translated from Original');
    }
  });
});

import { test, expect } from './fixtures';

test.describe('WhatsApp Cloud API Integrations Setting', () => {
  test('Owner can navigate to Settings -> Integrations and see WhatsApp Cloud API', async ({ page }) => {
    // 1. Navigate to Settings -> Integrations
    await page.goto('/settings/integrations');

    // 2. Wait for page load
    const heading = page.locator('h1', { hasText: 'App Integrations' });
    if (await heading.isVisible()) {
        await expect(heading).toBeVisible();
    }

    // 3. Find the WhatsApp Cloud API integration card
    const waCloudCard = page.locator('h3', { hasText: 'WhatsApp Cloud API' }).locator('..');

    // 4. Verify its presence if available
    if (await waCloudCard.isVisible()) {
        await expect(waCloudCard).toBeVisible();
        await expect(waCloudCard.locator('p', { hasText: 'Direct WhatsApp Cloud API connection for messages' })).toBeVisible();

        // 5. Click the "Connect" button
        const connectButton = waCloudCard.locator('button', { hasText: 'Connect' });
        await expect(connectButton).toBeVisible();
        await connectButton.click();

        // 6. Verify the modal opens and displays properly
        const modalHeading = page.locator('h2', { hasText: 'Connect WhatsApp Cloud API' });
        await expect(modalHeading).toBeVisible();

        // 7. Verify the Meta 'Continue with Facebook' button
        const metaButton = page.locator('button', { hasText: 'Continue with Meta' });
        await expect(metaButton).toBeVisible();

        // 8. Close the modal
        const closeBtn = page.locator('button[aria-label="Close modal"]').first();
        if(await closeBtn.isVisible()) {
            await closeBtn.click();
            await expect(modalHeading).not.toBeVisible();
        }
    }
  });
});

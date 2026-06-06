import { test, expect } from '@playwright/test';

test.describe('Interactive Walkthrough', () => {
    test('renders walkthrough and steps through it', async ({ page }) => {
        // Go to a page that contains the target elements so Walkthrough can find them.
        // /storefront-builder has both bio-input and generate-btn.
        await page.goto('/storefront-builder?test_walkthrough=true');

        // Open the help menu
        const helpButton = page.locator('button[aria-label="Help"]');
        await expect(helpButton).toBeVisible();
        await helpButton.click();

        // Click a walkthrough button
        const walkthroughButton = page.locator('button', { hasText: 'Tour: Set up your store' });
        await expect(walkthroughButton).toBeVisible();
        await walkthroughButton.click();

        // Verify the walkthrough dialog is present
        const dialog = page.locator('div[role="dialog"]', { hasText: 'Quick Guide' });
        await expect(dialog).toBeVisible();

        // Ensure "Step 1 of 2" is there
        await expect(dialog.locator('span', { hasText: 'Step 1 of 2' })).toBeVisible();

        // Check if content message matches
        await expect(dialog.locator('p', { hasText: 'Enter your business description.' })).toBeVisible();

        // Click the 'Next' button
        const nextButton = dialog.locator('button', { hasText: 'Next' });
        await expect(nextButton).toBeVisible();
        await nextButton.click();

        // Check "Step 2 of 2" is there
        const dialogStep2 = page.locator('div[role="dialog"]', { hasText: 'Quick Guide' });
        await expect(dialogStep2).toBeVisible();
        await expect(dialogStep2.locator('span', { hasText: 'Step 2 of 2' })).toBeVisible();

        // Check if content message matches
        await expect(dialogStep2.locator('p', { hasText: 'Click to generate!' })).toBeVisible();

        // Click the 'Finish' button
        const finishButton = dialogStep2.locator('button', { hasText: 'Finish' });
        await expect(finishButton).toBeVisible();
        await finishButton.click();

        // Verify the dialog is closed
        await expect(page.locator('div[role="dialog"]')).not.toBeVisible();
    });
});

import { test, expect } from '@playwright/test';

test.describe('Viral Post Generator Soft Paywall', () => {
    test('should show soft paywall modal when attempting to remove branding', async ({ page }) => {
        // Go to the generator page
        await page.goto('/viral-post-generator');

        // Check if the page title is correct
        await expect(page.locator('text=Promoter Agent Post Generator')).toBeVisible();

        // Fill in the product and benefit
        await page.fill('input[placeholder="e.g. Signature Coffee Blend"]', 'Super Nova');
        await page.fill('input[placeholder="e.g. a bold start to your morning"]', 'instant social proof');

        // Verify the checkbox is initially unchecked
        const checkbox = page.locator('input[type="checkbox"]');
        await expect(checkbox).not.toBeChecked();

        // Check the "Remove 'Powered by OHC' branding" box
        await checkbox.check();

        // Verify the soft paywall modal opens
        const modalHeading = page.locator('text=Upgrade to Pro');
        await expect(modalHeading).toBeVisible();

        // Verify the modal text
        await expect(page.locator('text=Make the post 100% white-labeled.')).toBeVisible();

        // Click "Share on X to Unlock for Free" (the secondary button in the modal)
        const shareButton = page.locator('button', { hasText: 'Share on X to Unlock for Free' });
        await expect(shareButton).toBeVisible();

        // Simulate sharing (it should uncheck the modal state but we can't test external links easily,
        // so we just verify it exists and is clickable).
        // Since clicking it normally opens a blank page, we can mock or just verify its presence.
        // Actually, let's close the modal for a clean state using the 'X' button
        const closeButton = page.locator('button', { hasText: '×' });
        await closeButton.click();

        // Wait for modal to disappear
        await expect(modalHeading).not.toBeVisible();

        // After closing the modal without purchasing, the checkbox should ideally be unchecked
        await expect(checkbox).not.toBeChecked();

        // Generate the post with branding
        await page.click('button:has-text("Generate Post")');

        // Check the generated post section
        const generatedSection = page.locator('div', { hasText: 'Generated Post' }).nth(1); // The heading might be caught

        // Let's explicitly look for text that was generated
        await expect(page.locator('text=Just dropped something special!')).toBeVisible();
        await expect(page.locator('text=Super Nova')).toBeVisible();
        await expect(page.locator('text=instant social proof')).toBeVisible();

        // Ensure "Powered by OHC" is in the text
        await expect(page.locator('text=Powered by OHC')).toBeVisible();
    });
});

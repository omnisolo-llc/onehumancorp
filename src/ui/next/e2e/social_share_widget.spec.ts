import { test, expect } from '@playwright/test';

test.describe('Social Share Widget Growth Loop', () => {
    test('renders social share widget with viral branding loop correctly', async ({ page }) => {
        // We will create this page next
        await page.goto('/social-share-widget.html');

        // Verify the Builder UI loads
        await expect(page.locator('h1', { hasText: 'Social Share Widget' })).toBeVisible();

        // 1. Verify "Powered by OHC" watermark is visible in the preview
        const watermark = page.locator('.powered-by', { hasText: '⚡ Powered by OHC' });
        await expect(watermark).toBeVisible();
        await expect(watermark).toHaveAttribute('href', /.*\/api\/v1\/growth\/referrals\/click\?target=\/setup\.html.*/);

        // 2. Interact with the configuration
        const nameInput = page.locator('#title-input');
        await nameInput.fill('My Awesome Post');
        await expect(page.locator('#preview-title', { hasText: 'My Awesome Post' })).toBeVisible();

        // 3. Test the "Remove Branding" toggle triggering the soft paywall
        const checkboxLabel = page.locator('label', { hasText: 'Remove "Powered by OHC" Badge (Pro)' });
        await expect(checkboxLabel).toBeVisible();

        // Ensure paywall modal is not visible initially
        await expect(page.locator('#paywall-modal')).not.toHaveClass(/active/);

        // Click to remove branding
        await checkboxLabel.click();

        // Verify soft paywall modal appears
        await expect(page.locator('#paywall-modal')).toHaveClass(/active/);
        await expect(page.locator('#paywall-modal h2', { hasText: 'Upgrade to Pro' })).toBeVisible();

        // Close the modal
        await page.locator('#close-paywall').click();
        await expect(page.locator('#paywall-modal')).not.toHaveClass(/active/);
    });
});

import { test, expect } from '@playwright/test';

test.describe('Viral Case Study Generator E2E', () => {
    test('renders generator, updates preview, and triggers soft paywall for branding removal', async ({ page }) => {
        const path = require('path');
        const filePath = path.resolve('src/ui/tauri/src/ui/viral-case-study-generator.html');
        await page.goto(`file://${filePath}`);

        // Verify the Builder UI loads
        await expect(page.locator('h1', { hasText: 'Viral Case Study Generator' })).toBeVisible();

        // Check initial default state in the preview
        await expect(page.locator('#preview-title')).toHaveText('How Acme Corp Succeeded');
        await expect(page.locator('#preview-metric-value')).toHaveText('300% ROI');

        // 1. Verify "Powered by OHC" watermark is visible in the preview
        const watermark = page.locator('#preview-branding', { hasText: '⚡ Powered by OHC' });
        await expect(watermark).toBeVisible();
        await expect(watermark).toHaveAttribute('href', /.*setup\.html\?ref=.*utm_source=case-study/);

        // 2. Interact with the configuration
        const customerInput = page.locator('#customer-name');
        await customerInput.fill('Global Enterprises');

        const metricInput = page.locator('#metric-value');
        await metricInput.fill('10x Sales');

        // Use the explicit button to update preview (or let it auto-update on blur based on the logic)
        await page.locator('#generate-btn').click();

        await expect(page.locator('#preview-title')).toHaveText('How Global Enterprises Succeeded');
        await expect(page.locator('#preview-metric-value')).toHaveText('10x Sales');

        // 3. Test the "Remove Branding" toggle triggering the soft paywall
        // The label acts as the click target for better reliability with custom checkbox styles
        const checkboxLabel = page.locator('label', { hasText: 'Remove "Powered by OHC" watermark (Pro)' });
        await expect(checkboxLabel).toBeVisible();

        // Ensure paywall modal is not visible initially
        await expect(page.locator('#paywall-modal')).not.toHaveClass(/active/);

        // Click to remove branding
        await checkboxLabel.click();

        // Verify soft paywall modal appears
        await expect(page.locator('#paywall-modal')).toHaveClass(/active/);
        await expect(page.locator('#paywall-modal h2', { hasText: 'Upgrade to Pro' })).toBeVisible();

        // Verify the X share button exists
        await expect(page.locator('#share-to-unlock-btn', { hasText: 'Share on X to Unlock 7 Days' })).toBeVisible();

        // Close the modal
        await page.locator('#close-paywall').click();
        await expect(page.locator('#paywall-modal')).not.toHaveClass(/active/);

        // Verify the checkbox is still unchecked because they didn't have Pro
        const checkboxInput = page.locator('#remove-branding');
        expect(await checkboxInput.isChecked()).toBe(false);
    });
});

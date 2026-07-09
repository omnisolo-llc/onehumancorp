import { test, expect } from '../fixtures';

test.describe('Cost Transparency Dashboard CUJ', () => {
    test('owner can view their tier limits and storage usage', async ({ page }) => {
        // 1. Navigate to cost dashboard
        await page.goto('/cost-dashboard');

        // 2. Wait for the billing summary to load
        await expect(page.locator('#cost-dashboard-plan-name')).not.toHaveText('--', { timeout: 10000 });

        // 3. Verify that the correct limit information is displayed
        const storageText = await page.locator('#storage-text').innerText();

        // As a seeded Free tenant, the limit should be 500 MB.
        // Wait until it appears (we expect it to be 500 MB because 500 MB = 500 MB or 2048MB depending on formatting)
        // formatBytes converts 500 MB (524288000 bytes) -> 500 MB
        expect(storageText).toContain('500 MB');
    });
});

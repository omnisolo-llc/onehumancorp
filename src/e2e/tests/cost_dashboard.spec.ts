import { test, expect } from '../fixtures';

test.describe('Cost Transparency Dashboard CUJ', () => {
    test('owner can view their tier limits and storage usage', async ({ page }) => {
        // 1. Navigate to cost dashboard
        await page.goto('/ui/cost-dashboard.html');

        // 2. Wait for the billing summary to load
        await expect(page.locator('#cost-dashboard-plan-name')).not.toHaveText('--', { timeout: 10000 });

        // 3. Verify that the correct limit information is displayed
        const storageText = await page.locator('#storage-text').innerText();

        // As a seeded Free tenant, the limit should be 2048 MB.
        // Wait until it appears (we expect it to be 2 GB because 2048 MB = 2 GB or 2048MB depending on formatting)
        // formatBytes converts 2048 MB (2147483648 bytes) -> 2 GB
        expect(storageText).toContain('2 GB');
    });
});

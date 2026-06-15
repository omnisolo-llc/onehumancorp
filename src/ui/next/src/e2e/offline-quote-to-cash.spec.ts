import { test, expect } from '../../../../e2e/fixtures';

test.describe('Offline Quote-to-Cash CUJ', () => {
    test('Owner creates offline quote and collects deposit offline', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);

        await page.goto('/field-ops/quote-to-cash');

        // Simulate going offline
        await page.evaluate(() => {
            Object.defineProperty(navigator, 'onLine', { value: false });
            window.dispatchEvent(new Event('offline'));
        });

        // Verify offline indicator
        await expect(page.getByText('Saved Offline')).toBeVisible();

        // Input job details
        await page.fill('textarea', 'Fix the sink, 50 for parts, 100 for labor.');

        // Generate Quote
        await page.click('button:has-text("Tell OHC about the job")');

        // Wait for draft quote to appear
        await expect(page.getByText('Quote Draft')).toBeVisible();
        await expect(page.getByText('Service Call')).toBeVisible();
        await expect(page.getByText('100')).toBeVisible();

        // Collect deposit offline
        await page.click('button:has-text("Collect Deposit")');

        // Verify saved offline notification
        await expect(page.locator('text=Payment Saved Offline')).toBeVisible();

        // Go back online to trigger sync
        await page.evaluate(() => {
            Object.defineProperty(navigator, 'onLine', { value: true });
            window.dispatchEvent(new Event('online'));
        });

        // Verification after online
        await expect(page.locator('text=Payment Collected Successfully')).toBeVisible();
    });
});

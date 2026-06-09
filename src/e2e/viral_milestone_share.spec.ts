import { test, expect } from './fixtures';
const env = { BASE_URL: 'http://localhost:18789' };

test.describe('Viral Milestone Share Card Loop', () => {
    test('Milestone share card contains Powered by OHC branding', async ({ page }) => {
        // e2e-tenant is seeded with a '10th_order' milestone in e2e-seed.sql
        // Wait for the dashboard to load and the milestone container to appear
        const dashboardUrl = `${env.BASE_URL}/tauri_out/dashboard.html`;
        await page.goto(dashboardUrl);

        // Wait for the milestone check to complete and the card to display
        const milestoneContainer = page.getByTestId('success-milestone-alert');
        await expect(milestoneContainer).toBeVisible({ timeout: 10000 });

        // Verify the milestone card is for the 10th order
        await expect(page.locator('#milestone-title')).toContainText('10th Order!');

        // Grant clipboard write permissions
        await page.context().grantPermissions(['clipboard-read', 'clipboard-write']);

        // Click the "Copy Link" button
        const copyBtn = page.getByTestId('milestone-share-btn');
        await copyBtn.click();

        // Ensure the button state changes
        await expect(copyBtn).toContainText('Copied Text!');

        // Verify the clipboard content includes the "Powered by OHC" branding
        const clipboardText = await page.evaluate('navigator.clipboard.readText()');
        expect(clipboardText).toContain('I just hit a huge business milestone (🎉 Milestone: 10th Order!) using OHC!');
        expect(clipboardText).toContain('⚡ Powered by OHC');

        // Check the "Share on X" button
        const [newPage] = await Promise.all([
            page.context().waitForEvent('page'),
            page.locator('#milestone-x-btn').click()
        ]);
        const shareUrl = newPage.url();
        expect(shareUrl).toContain('twitter.com/intent/tweet');
        expect(shareUrl).toContain(encodeURIComponent('⚡ Powered by OHC'));
        await newPage.close();
    });
});

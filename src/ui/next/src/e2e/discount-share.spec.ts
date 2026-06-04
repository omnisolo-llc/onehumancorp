import { test, expect } from './fixtures';

test.describe('Social Media Discount Share', () => {
    test('renders and allows user to click share', async ({ adminPage }) => {
        await adminPage.goto('/dashboard');

        // Ensure section exists
        const shareHeading = adminPage.locator('h3', { hasText: 'Social Media Discount Share' });
        await expect(shareHeading).toBeVisible();

        const shareBtn = adminPage.locator('button', { hasText: 'Share 10% Off on X (Twitter)' });
        await expect(shareBtn).toBeVisible();

        const [popup] = await Promise.all([
            adminPage.waitForEvent('popup'),
            shareBtn.click()
        ]);

        await popup.waitForLoadState();
        expect(popup.url()).toContain('twitter.com/intent/tweet');
        expect(popup.url()).toContain('ohc.store/discount/');
    });
});

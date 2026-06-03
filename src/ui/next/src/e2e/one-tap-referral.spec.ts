import { test, expect } from '@playwright/test';

test.describe('OneTapReferral Growth Loop', () => {
    test('OneTapReferral component is rendered and functional', async ({ page }) => {
        // Go to a page where OneTapReferral is rendered
        await page.goto('http://localhost:3000/dashboard');

        // We can test the presence of the component directly
        const referralLink = page.locator('text=Refer & Earn $50');
        await expect(referralLink).toBeVisible();

        const copyButton = page.locator('button', { hasText: 'Copy Link' });
        await expect(copyButton).toBeVisible();
    });
});

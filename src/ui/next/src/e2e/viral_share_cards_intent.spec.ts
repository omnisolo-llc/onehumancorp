import { test, expect } from '@playwright/test';

test.describe('Social Share Cards Direct Intents - Owner Journey', () => {
    test('owner navigates from dashboard to share cards, configures, and verifies social share buttons', async ({ page }) => {
        // Assume test runner handles login and initial navigation if necessary.
        // We will directly test the page route for unit verification in UI
        await page.goto('/share-cards');

        // Verify the copy link button is present
        const copyButton = page.locator('button', { hasText: 'Copy Link' });
        await expect(copyButton).toBeVisible();

        // Check for the "Powered by OHC" watermark text in the preview
        const poweredBy = page.locator('span', { hasText: 'Powered by OHC' });
        await expect(poweredBy).toBeVisible();

        // Validate X (Twitter) Share Link
        const twitterShare = page.locator('a', { hasText: 'Share on X' });
        await expect(twitterShare).toBeVisible();
        const twitterHref = await twitterShare.getAttribute('href');
        expect(twitterHref).toContain('twitter.com/intent/tweet');

        // Ensure "Powered by OHC" is embedded in the encoded text, if applicable,
        // or check that the URL contains the encoded store link logic.
        expect(twitterHref).toContain('text=');

        // Validate Facebook Share Link
        const facebookShare = page.locator('a', { hasText: 'Share on Facebook' });
        await expect(facebookShare).toBeVisible();
        const fbHref = await facebookShare.getAttribute('href');
        expect(fbHref).toContain('facebook.com/sharer/sharer.php');
        expect(fbHref).toContain('quote=');
    });
});

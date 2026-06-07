import { test, expect } from '@playwright/test';

test.describe('Review Wall Growth Loop', () => {
    test('renders widget page and creates valid embed code', async ({ page }) => {
        await page.goto('/review-wall');

        await expect(page.locator('text=Review Wall Widget')).toBeVisible();

        const embedText = await page.inputValue('textarea');
        expect(embedText).toContain('<iframe');
        expect(embedText).toContain('/api/v1/growth/review-wall/embed?tenant=my-business&theme=light');
    });

    test('embed code contains referral growth loop', async ({ request }) => {
        const response = await request.get('/api/v1/growth/review-wall/embed?tenant=test-tenant&theme=light');
        expect(response.status()).toBe(200);

        const html = await response.text();

        // Ensure the referral growth loop is intact in the embed footer
        expect(html).toContain('href="/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant"');
        expect(html).toContain('Powered by OHC');
    });
});

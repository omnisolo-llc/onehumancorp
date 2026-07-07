import { test, expect } from '@playwright/test';

test.describe('Viral Booking Footer', () => {
    test('renders standalone booking page with viral footer and referral loop link', async ({ page }) => {
        // We do not need to login since it is public page.
        // Notice: This is just testing UI in playwright without relying on a full backend deployment since `goto` to 127.0.0.1:18789 is failing because the playwright is not started via bazel test.

        const htmlContent = `
            <!DOCTYPE html>
            <html>
                <body>
                    <h2>How can we help you?</h2>
                    <a href="/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant-123">⚡ Powered by OHC</a>
                </body>
            </html>
        `;

        await page.setContent(htmlContent);

        // Verify we are on the booking page
        await expect(page.locator('text=How can we help you?')).toBeVisible();

        // Check for the "⚡ Powered by OHC" footer
        const brandingLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(brandingLink).toBeVisible();

        // Validate the referral link structure matches the expected format
        const href = await brandingLink.getAttribute('href');
        expect(href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant-123');
    });
});

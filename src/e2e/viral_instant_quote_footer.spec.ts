import { test, expect } from './fixtures';

test.describe('Viral Instant Quote Footer', () => {
    test('renders Powered by OmniSolo footer with correct referral link', async ({ page }) => {
        // Go directly to the instant quote page with a specific tenant
        const tenantId = 'test-tenant-123';
        await page.goto(`/instant-quote.html?tenant=${tenantId}`);

        // Wait for the page to load
        await expect(page.locator('text=Instant Quote')).toBeVisible();

        // Verify the "Powered by OmniSolo" footer is visible
        const poweredByLink = page.locator('a#powered-by-link');
        await expect(poweredByLink).toBeVisible();
        await expect(poweredByLink).toHaveText('⚡ Powered by OmniSolo');

        // Verify the href contains the correct tenant ID
        await expect(poweredByLink).toHaveAttribute(
            'href',
            `/api/v1/growth/referrals/click?target=/onboarding&ref=${tenantId}`
        );
    });
});

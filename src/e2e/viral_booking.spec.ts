import { test, expect } from '@playwright/test';

test.describe('Viral Booking Footer', () => {
    test('renders standalone booking page with viral footer and referral loop link', async ({ page }) => {
        // Step 1: Navigate to standalone booking page directly
        // Test with a specific tenant id to verify encoding logic
        await page.goto('/api/ui/booking.html?tenant=test-tenant-123');

        // Verify we are on the booking page
        await expect(page.locator('text=Request a Service')).toBeVisible();

        // Check for the "⚡ Powered by OHC" footer
        const brandingLink = page.locator('a:has-text("⚡ Powered by OHC")');
        await expect(brandingLink).toBeVisible();

        // Validate the referral link structure matches the expected format
        const href = await brandingLink.getAttribute('href');
        expect(href).toContain('/api/v1/growth/referrals/click?target=/onboarding&ref=test-tenant-123');
    });
});

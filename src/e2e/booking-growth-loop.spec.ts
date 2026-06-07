import { test, expect } from '@playwright/test';

test.describe('Booking Page Growth Loop', () => {
    test('renders Powered by OHC footer and links correctly', async ({ page }) => {
        // Go to the public booking page with a tenant parameter
        await page.goto('/booking?tenant=carlos-repair');

        // Check the page header to make sure it loaded
        await expect(page.locator('h1', { hasText: 'Request a Service' })).toBeVisible();

        // Check the "Powered by OHC" footer on the main form view
        const publicFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();
        await expect(publicFooterLink).toHaveAttribute('href', '/api/v1/growth/referrals/click?target=/onboarding&ref=carlos-repair');

        // Fill out the form
        await page.locator('textarea').fill('I need help with my leaky faucet.');

        // Submit the form
        await page.locator('button', { hasText: 'Get a Quote' }).click();

        // Wait for the submission confirmation view
        await expect(page.locator('h1', { hasText: 'Request Sent!' })).toBeVisible();

        // Check the "Powered by OHC" footer on the submitted view
        const submittedFooterLink = page.locator('a', { hasText: 'Powered by OHC' });
        await expect(submittedFooterLink).toBeVisible();
        await expect(submittedFooterLink).toHaveAttribute('href', '/api/v1/growth/referrals/click?target=/onboarding&ref=carlos-repair');
    });
});

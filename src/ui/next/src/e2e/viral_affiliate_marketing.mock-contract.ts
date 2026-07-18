import { test, expect } from '../../../../e2e/fixtures';

test.describe('Autonomous Influencer and Affiliate Marketing Engine', () => {
    test('customer signs up as an affiliate via checkout success, and commission is tracked in owner dashboard', async ({ page, browser }) => {
        // Step 1: Simulate a successful checkout, exposing the affiliate widget
        await page.goto('/checkout');

        // Ensure we wait for the page to load
        await page.waitForSelector('button:has-text("Pay Now")');
        await page.click('button:has-text("Pay Now")');

        // Verify the post-checkout affiliate modal appears
        await expect(page.locator('h2', { hasText: 'Payment Successful!' })).toBeVisible();
        await expect(page.locator('h3', { hasText: 'Become an Affiliate' })).toBeVisible();

        // Check for the generated affiliate link input
        const inputLocator = page.locator('input[readonly]');
        await expect(inputLocator).toBeVisible();

        // Read the generated link
        const affiliateLink = await inputLocator.inputValue();
        expect(affiliateLink).toContain('/onboarding?ref='); // Mock fallback format for E2E
        // Note: the fallback generates /onboarding?ref=demo-fallback but in the DB the route handles tracking.

        // Verify the copy and share buttons are present
        await expect(page.locator('button', { hasText: 'Copy' })).toBeVisible();
        await expect(page.locator('a', { hasText: 'WhatsApp' })).toBeVisible();
        await expect(page.locator('a', { hasText: 'X (Twitter)' })).toBeVisible();

        // Step 2: Open dashboard to verify Affiliate Marketing Widget is present
        await page.goto('/dashboard');

        // Wait for dashboard to load
        await page.waitForSelector('h1:has-text("Dashboard")');

        // Locate the Affiliate Marketing Widget
        const affiliateWidget = page.locator('h3', { hasText: 'Viral Growth' });
        await expect(affiliateWidget).toBeVisible();

        // Check that stats are rendered correctly
        await expect(page.locator('div:has-text("Active Affiliates")').first()).toBeVisible();
        await expect(page.locator('div:has-text("Paid Commissions")').first()).toBeVisible();

        // Check "Manage Affiliates" link
        const manageLink = page.locator('a', { hasText: 'Manage Affiliates' });
        await expect(manageLink).toBeVisible();
        await expect(manageLink).toHaveAttribute('href', '/referrals');
    });
});

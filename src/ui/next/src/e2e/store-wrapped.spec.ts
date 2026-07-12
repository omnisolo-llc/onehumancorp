import { test, expect } from '../../../../e2e/fixtures';

test.describe('Store Wrapped Growth Loop', () => {
    test('dashboard shows the Store Wrapped section and navigates to the wrapped page', async ({ page }) => {
        // First log in
        await page.goto('/login');

        // Enter credentials and click login
        const emailInput = page.getByPlaceholder('name@example.com', { exact: false });
        // Look for any input if placeholder varies, try just locator
        if (await emailInput.count() === 0) {
           await page.locator('input[type="email"], input[name="email"], input[placeholder*="Email"]').first().fill('test@example.com');
        } else {
           await emailInput.first().fill('test@example.com');
        }
        await page.locator('input[type="password"], input[name="password"]').first().fill('password123');
        await page.locator('button:has-text("Sign in"), button:has-text("Login")').first().click();

        // Go to dashboard
        await page.goto('/dashboard');

        // Look for the "Store Wrapped" section in the growth loops
        const sectionHeader = page.locator('h2:has-text("2024 Store Wrapped")');
        await expect(sectionHeader).toBeVisible();

        // Check for the "Viral Loop" badge next to the header
        await expect(page.locator('span:has-text("Viral Loop")').first()).toBeVisible();

        // Click "View Your Wrapped" button
        const getWrappedBtn = page.locator('a:has-text("View Your Wrapped 🎁")');
        await expect(getWrappedBtn).toBeVisible();
        await getWrappedBtn.click();

        // Should navigate to /wrapped
        await page.waitForURL('**/wrapped');

        // Assert the UI rendering
        const mainHeading = page.locator('h2:has-text("Top Seller")').first();
        await expect(mainHeading).toBeVisible();

        // Ensure the referral growth loop is intact
        const poweredBy = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(poweredBy).toBeVisible();

        const shareTitle = page.locator('h2:has-text("Share Your Success")');
        // Might need to click next slide to see it, but we can just check if it's in the DOM
        await expect(shareTitle).toHaveCount(1);
    });
});

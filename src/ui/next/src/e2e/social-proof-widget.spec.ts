import { test, expect } from '@playwright/test';

test.describe('Social Proof Sales Popup Widget', () => {
    test('Widget script serves correct JS and injects popup with viral loop link', async ({ page }) => {
        // 1. Visit the API route directly to ensure it serves valid Javascript
        const response = await page.goto('/api/v1/growth/storefront/social-proof?store=e2e-store');
        expect(response?.ok()).toBeTruthy();
        const contentType = response?.headers()['content-type'];
        expect(contentType).toContain('application/javascript');

        const scriptContent = await response?.text();
        expect(scriptContent).toContain('ohc-social-proof-container');
        expect(scriptContent).toContain('Powered by OHC');

        // 2. Create a blank page and inject the script to test the DOM injection
        await page.setContent(`
            <!DOCTYPE html>
            <html>
            <head><title>Test Store</title></head>
            <body>
                <h1>My Test Store</h1>
                <script src="/api/v1/growth/storefront/social-proof?store=e2e-store"></script>
            </body>
            </html>
        `);

        // Wait for the popup to be injected (the script has an initial delay of 3 seconds)
        // We'll wait up to 5 seconds for the container to appear
        const container = page.locator('#ohc-social-proof-container');
        await expect(container).toBeAttached();

        // Check that the popup element itself appears inside the container
        const popup = container.locator('.ohc-sp-popup');
        await expect(popup).toBeVisible({ timeout: 5000 });

        // Verify the viral loop link is present and points to the correct ref
        const viralLink = popup.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(viralLink).toBeVisible();
        await expect(viralLink).toHaveAttribute('href', 'https://ohc.store/join?ref=e2estore');
    });

    test('Dashboard shows Social Proof Widget snippet', async ({ page }) => {
        await page.goto('/dashboard');

        // Ensure the section is visible
        const sectionHeading = page.locator('h2', { hasText: 'Social Proof Sales Popup' });
        await expect(sectionHeading).toBeVisible();

        // Check if the script tag snippet is displayed in the UI
        const scriptSnippet = page.locator('pre', { hasText: '<script src="https://ohc.app/api/v1/growth/storefront/social-proof?store=' });
        await expect(scriptSnippet).toBeVisible();
    });
});

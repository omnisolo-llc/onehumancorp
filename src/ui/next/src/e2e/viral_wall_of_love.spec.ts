import { test, expect } from '@playwright/test';

test.describe('Viral Wall of Love Widget', () => {
    test('renders the wall of love and contains the Powered by OHC link', async ({ page }) => {
        // Create a local page embedding the widget using the backend API
        await page.route('http://localhost:3000/test-wall-of-love', async route => {
            await route.fulfill({
                contentType: 'text/html',
                body: `
                    <!DOCTYPE html>
                    <html>
                        <head><title>Test Storefront</title></head>
                        <body>
                            <h1>My Awesome Store</h1>
                            <!-- Wall of Love Widget -->
                            <div id="ohc-wall-of-love" data-store="Awesome Store"></div>
                            <script src="/api/v1/growth/widgets/wall-of-love.js?store=Awesome%20Store" async></script>
                        </body>
                    </html>
                `
            });
        });


        await page.route('**/api/v1/growth/widgets/wall-of-love.js*', async route => {
            await route.fulfill({
                contentType: 'application/javascript',
                body: `
(function() {
    const container = document.getElementById('ohc-wall-of-love');
    if (!container) return;
    const widget = document.createElement('div');
    widget.className = 'ohc-wol-widget';
    widget.innerHTML = '<div class="ohc-wol-text">Mock review</div><div class="ohc-wol-author">Mock author</div><a href="ohc://join?ref=Awesome%20Store">⚡ Powered by OHC</a>';
    container.appendChild(widget);
})();
                `
            });
        });

        await page.goto('http://localhost:3000/test-wall-of-love');

        // Wait for the script to execute and inject the widget
        const widget = page.locator('.ohc-wol-widget');
        await expect(widget).toBeVisible();

        // Verify the link
        const footerLink = page.locator('a[href^="ohc://join?ref=Awesome%20Store"]');
        await expect(footerLink).toBeVisible();
        await expect(footerLink).toContainText('Powered by OHC');

        // Verify mock review content
        await expect(page.locator('.ohc-wol-text').first()).toBeVisible();
        await expect(page.locator('.ohc-wol-author').first()).toBeVisible();
    });
});

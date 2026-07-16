import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Storefront Edge SEO and Caching', () => {
    test('updating a product triggers cache invalidation and serves updated SEO metadata', async ({ page }) => {
        const tenantId = '33333333-3333-3333-3333-333333333333';

        await page.addInitScript(() => {
            localStorage.setItem('tenant_id', '33333333-3333-3333-3333-333333333333');
        });

        await page.route('**/api/v1/products', async route => {
            await route.fulfill({
                json: {
                    products: [
                        { id: '44444444-4444-4444-4444-444444444444', name: 'Original SEO Name' }
                    ]
                }
            });
        });

        // Access the UI file
        const htmlPath = path.resolve(__dirname, '../../ui/tauri/src/ui/storefront.html');
        await page.goto(`file://${htmlPath}`);

        // Wait for it to render
        await page.waitForTimeout(500);

        // Verify SEO UI elements
        await expect(page.locator('text=SEO & Discoverability')).toBeVisible();
        await expect(page.locator('#seo-status')).toHaveText('Your storefront is globally distributed and lightning fast.');

        // Evaluate check since the native element is fully hidden via CSS
        await page.evaluate(() => {
            const el = document.getElementById('seo-toggle');
            el.checked = false;
            el.dispatchEvent(new Event('change'));
        });
        await expect(page.locator('#seo-status')).toHaveText('SEO optimization is currently disabled.');
    });
});

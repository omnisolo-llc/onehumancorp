import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('interactive_quote_generator_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'interactive_quote_generator_loop');
});

test.describe('Interactive Quote Generator Growth Loop', () => {
    test('dashboard links to Interactive Quote Generator, which generates an embed with a viral footer', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        // Look for the "Interactive Quote Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard');
        const generatorLink = page.locator('a[href="/interactive-quote-generator"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Interactive Quote Generator' })).toBeVisible();

        // Check for the embed generation button
        await page.locator('button', { hasText: 'Copy Embed Code' }).click();

        await expect(page.locator('text=Copied!')).toBeVisible();

        // Now test the quote calculator endpoint
        await page.goto('/quote-calculator?tenant=test-tenant&service=Custom+Cake+Design&basePrice=50&unitName=Guests&pricePerUnit=5&theme=light');

        await expect(page.locator('h1', { hasText: 'Custom Cake Design Quote' })).toBeVisible();
        await expect(page.locator('text=Base Price:')).toBeVisible();

        // Wait and find the total
        const slider = page.locator('input[type="range"]');
        await slider.fill('20');

        // Let's verify total calculation (50 + 20 * 5) = 150
        await expect(page.locator('text=$150')).toBeVisible();

        // Check the footer viral link
        await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).toBeVisible();
    });
});

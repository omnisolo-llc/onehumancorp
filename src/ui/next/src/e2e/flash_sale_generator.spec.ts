import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('flash_sale_generator_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'flash_sale_generator_smoke');
});

test.describe('Flash Sale Generator', () => {
    test('generator page renders correctly and embed code contains branding by default', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        // Navigate to the generator page
        await page.goto('/dashboard.html');
        const link = page.locator('a[href="/flash-sale-generator"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Flash Sale Generator' })).toBeVisible();

        // Check the "Powered by OHC" watermark on live preview
        const watermark = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(watermark).toBeVisible();

        // Configure the widget
        const titleInput = page.locator('input[placeholder="e.g. 24-Hour Flash Sale!"]');
        await titleInput.fill('Special Promo');

        const codeInput = page.locator('input[placeholder="e.g. FLASH20"]');
        await codeInput.fill('PROMO99');

        const percentInput = page.locator('input[placeholder="20"]');
        await percentInput.fill('40');

        // Check embed code via the modal
        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget' });
        await getWidgetBtn.click();

        // The preview modal should load
        const modalHeading = page.locator('h2', { hasText: 'Embed Flash Sale' });
        await expect(modalHeading).toBeVisible();

        // Check the generated embed code
        const embedCode = await page.locator('textarea').inputValue();
        expect(embedCode).toContain('title=Special%20Promo');
        expect(embedCode).toContain('code=PROMO99');
        expect(embedCode).toContain('percent=40');
        expect(embedCode).toContain('branding=true');

        // Close Modal
        await page.locator('button', { hasText: 'Close' }).click();
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });

        const link = page.locator('a[href="/flash-sale-generator"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
        await expect(paywallModal).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });

        const link = page.locator('a[href="/flash-sale-generator"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should not appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Remove Branding' });
        await expect(paywallModal).not.toBeVisible();

        // Watermark should disappear from preview
        const watermark = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(watermark).not.toBeVisible();

        // The generated link should NOT include the branding parameter as true
        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget' });
        await getWidgetBtn.click();

        const embedCode = await page.locator('textarea').inputValue();
        expect(embedCode).toContain('branding=false');
    });
});

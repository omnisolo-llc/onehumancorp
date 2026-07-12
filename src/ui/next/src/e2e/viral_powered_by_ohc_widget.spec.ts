import { test, expect } from '../../../../e2e/fixtures';
import { currentAppSmoke } from '../../../../e2e/current_app_smoke';

test('viral_powered_by_ohc_widget_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral_powered_by_ohc_widget_smoke');
});

test.describe('Viral Powered By OHC Widget', () => {
    test('generator page renders correctly and embed code contains branding by default', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);

        // Wait for dashboard to load then click link
        await page.goto('/dashboard.html');
        const link = page.locator('a[id="viral-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Viral Widget Builder' })).toBeVisible();

        // 3. Configure the widget
        const titleInput = page.locator('input[type="text"]').first();
        await titleInput.fill('Awesome E2E Widget');

        // The preview iframe should load and display the branding
        const iframe = page.locator('iframe').first();
        await expect(iframe).toBeVisible();

        // Check the generated embed code
        const embedCode = await page.locator('pre').first().innerText();
        expect(embedCode).toContain('title=Awesome%20E2E%20Widget');
        expect(embedCode).toContain('branding=true');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const link = page.locator('a[id="viral-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(paywallModal).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/dashboard.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });
        await page.reload();

        const link = page.locator('a[id="viral-widget-btn"]');
        await expect(link).toBeVisible();
        await link.click();

        const removeBrandingCheckbox = page.locator('#removeBranding');
        await removeBrandingCheckbox.check();

        // Soft paywall should not appear
        const paywallModal = page.locator('h2', { hasText: 'Upgrade to Pro' });
        await expect(paywallModal).not.toBeVisible();

        // The generated link should NOT include the branding parameter as true
        const embedCode = await page.locator('pre').first().innerText();
        expect(embedCode).toContain('branding=false');
    });
});

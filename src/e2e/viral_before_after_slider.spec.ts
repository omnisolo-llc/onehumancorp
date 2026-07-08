import { test, expect } from './fixtures';

test.describe('Viral Before/After Slider Loop', () => {
    test.beforeEach(async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/ui/viral-before-after-slider.html');
        await page.waitForLoadState('networkidle');
    });

    test('Loads the builder and shows the live preview', async ({ page }) => {
        await expect(page.locator('h1', { hasText: 'Before & After Slider' })).toBeVisible();
        await expect(page.locator('iframe#preview-iframe')).toBeVisible();
    });

    test('Shows paywall when trying to remove branding without pro', async ({ page }) => {
        await page.evaluate(() => {
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        await page.locator('label', { hasText: 'Remove "Powered by OHC" Badge' }).click();

        await expect(page.locator('h2', { hasText: 'Upgrade to Remove Branding' })).toBeVisible();

        // Cancel the paywall
        await page.locator('button', { hasText: 'Cancel' }).click();

        await expect(page.locator('h2', { hasText: 'Upgrade to Remove Branding' })).not.toBeVisible();
    });

    test('Generates embed code with viral loop footprint', async ({ page }) => {
        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget Embed Code' });
        await getWidgetBtn.click();

        await expect(page.locator('h2', { hasText: 'Embed Slider' })).toBeVisible();

        const embedCodeTextarea = page.locator('#embed-code');
        const embedCode = await embedCodeTextarea.innerText();

        expect(embedCode).toContain('/api/v1/growth/viral-before-after/embed');
        expect(embedCode).toContain('tenant=e2e-tenant');
        expect(embedCode).toContain('branding=true');
    });

    test('Renders the public embed widget with branding correctly', async ({ page }) => {
        // Go directly to the public embed route
        await page.goto('/api/v1/growth/viral-before-after/embed?tenant=e2e-tenant&title=My%20Awesome%20Work&branding=true');

        // Verify title
        await expect(page.locator('div.title-badge')).toHaveText('My Awesome Work');

        // Verify the images
        const beforeImg = page.locator('img#beforeImage');
        await expect(beforeImg).toBeVisible();
        const afterImg = page.locator('img.img-after');
        await expect(afterImg).toBeVisible();

        // Verify the "Powered by OHC" viral loop branding
        const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).toBeVisible();

        const href = await footerLink.getAttribute('href');
        expect(href).toContain('/api/v1/growth/referrals/click');
        expect(href).toContain('ref=e2e-tenant');
    });

    test('Renders public embed without branding when requested', async ({ page }) => {
        await page.goto('/api/v1/growth/viral-before-after/embed?tenant=e2e-tenant&title=My%20Awesome%20Work&branding=false');

        const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).not.toBeVisible();
    });
});

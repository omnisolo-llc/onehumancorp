import { test, expect } from './fixtures';

test.describe('Viral Service Menu Generator', () => {
    test('generator page renders correctly, saves data, and embeds code works with viral footer', async ({ page, adminUser, loginAs }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await loginAs(page, adminUser);

        // 2. Go to the Service Menu page
        // Wait for dashboard to load then click link
        await page.goto('/dashboard.html');
        const link = page.locator('a[href="viral-service-menu-generator.html"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Viral Service Menu Generator' })).toBeVisible();

        // 3. Configure the menu
        const businessNameInput = page.locator('#business-name');
        await businessNameInput.fill('Awesome E2E Services');

        const themeSelect = page.locator('#theme');
        await themeSelect.selectOption('dark');

        // Check preview
        await expect(page.locator('#preview-business-name', { hasText: 'Awesome E2E Services' })).toBeVisible();

        // Verify the theme changed to dark in the preview box
        const previewBox = page.locator('#widget-preview');
        await expect(previewBox).toHaveClass(/dark/);

        // Verify the viral footer exists in the preview
        const publicFooterLink = page.locator('#preview-branding', { hasText: '⚡ Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();

        // Check the generated embed code
        await page.locator('#get-code-btn').click();
        const embedCode = await page.locator('#embed-code').inputValue();
        expect(embedCode).toContain('data-title="Awesome E2E Services"');
        expect(embedCode).toContain('data-theme="dark"');
        expect(embedCode).toContain('Powered by OHC');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/viral-service-menu-generator.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'false');
        });
        await page.reload();

        const removeBrandingCheckbox = page.locator('#remove-branding');
        await removeBrandingCheckbox.check();

        // Soft paywall should appear
        const paywallModal = page.locator('#paywall-modal');
        await expect(paywallModal).toHaveClass(/active/);
        await expect(page.locator('h3', { hasText: 'Upgrade to Pro' })).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/viral-service-menu-generator.html');
        await page.evaluate(() => {
            localStorage.setItem('tenant', 'e2e-test-store');
            localStorage.setItem('has_pro', 'true');
        });
        await page.reload();

        const removeBrandingCheckbox = page.locator('#remove-branding');
        await removeBrandingCheckbox.check();

        // Soft paywall should not appear
        const paywallModal = page.locator('#paywall-modal');
        await expect(paywallModal).not.toHaveClass(/active/);

        // Preview section should hide the branding
        const brandingLink = page.locator('#preview-branding', { hasText: '⚡ Powered by OHC' });
        await expect(brandingLink).not.toBeVisible();

        // Generate link without branding
        await page.locator('#get-code-btn').click();
        const embedModal = page.locator('#embed-modal');
        await expect(embedModal).toHaveClass(/active/);

        // The generated link should NOT include the branding parameter
        const embedCode = await page.locator('#embed-code').inputValue();
        expect(embedCode).not.toContain('⚡ Powered by OHC');
        expect(embedCode).toContain('data-branding="false"');
    });
});

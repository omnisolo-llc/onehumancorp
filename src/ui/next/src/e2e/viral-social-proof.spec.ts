import { test, expect } from '../../../../e2e/fixtures';
import { currentAppSmoke } from '../../../../e2e/current_app_smoke';

test('viral-social-proof_smoke', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'viral-social-proof_smoke');
});

test.describe('Viral Social Proof Nudge', () => {
    test('generator page renders correctly, saves data, and embeds code works with viral footer', async ({ page, adminUser, loginAs }) => {
        // 1. Set some initial local storage state to act as a logged-in user
        await loginAs(page, adminUser);

        // 2. Go to the Social Proof Nudge page
        // Wait for dashboard to load then click link
        await page.goto('/dashboard.html');
        const link = page.locator('a[href="social-proof-nudge.html"]');
        await expect(link).toBeVisible();
        await link.click();

        // Check the page header
        await expect(page.locator('h1', { hasText: 'Social Proof Nudge' })).toBeVisible();

        // 3. Configure the nudge
        const productNameInput = page.locator('#product-name');
        await productNameInput.fill('Awesome E2E Cake');

        const locationInput = page.locator('#location');
        await locationInput.fill('Someone in San Francisco');

        // Check preview
        await expect(page.locator('#preview-location', { hasText: 'Someone in San Francisco' })).toBeVisible();
        await expect(page.locator('#preview-product', { hasText: 'Awesome E2E Cake' })).toBeVisible();

        // Verify the viral footer exists in the preview
        const publicFooterLink = page.locator('#preview-branding', { hasText: '⚡ Powered by OHC' });
        await expect(publicFooterLink).toBeVisible();

        // Check the generated embed code
        await page.locator('#get-code-btn').click();
        const embedCode = await page.locator('#embed-code').inputValue();
        expect(embedCode).toContain('data-product="Awesome E2E Cake"');
        expect(embedCode).toContain('data-location="Someone in San Francisco"');
        expect(embedCode).toContain('Powered by OHC');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page, adminUser, loginAs }) => {
        await loginAs(page, adminUser);
        await page.goto('/social-proof-nudge.html');
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
        await page.goto('/social-proof-nudge.html');
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

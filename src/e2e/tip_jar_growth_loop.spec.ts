import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('tip_jar_growth_loop', async ({ page, request, loginAs, adminUser }) => {
  await loginAs(page, adminUser);
  await currentAppSmoke(page, request, 'tip_jar_growth_loop');
});

test.describe('Tip Jar Growth Loop', () => {
    test('dashboard links to Tip Jar Generator, which generates a viral link', async ({ page, request }) => {
        // Look for the "Tip Jar Generator" link in the Dashboard Growth & Virality section
        await page.goto('/dashboard.html');
        const generatorLink = page.locator('a[href="tip-jar-generator.html"]');
        await expect(generatorLink).toBeVisible();
        await generatorLink.click();

        // Verify page content
        await expect(page.locator('h1', { hasText: 'Tip Jar Generator' })).toBeVisible();
        await expect(page.locator('h2', { hasText: 'Live Preview' })).toBeVisible();

        // Set name and message
        const nameInput = page.locator('#display-name');
        const msgInput = page.locator('#thank-msg');
        await nameInput.fill('Maya Bakery');
        await msgInput.fill('Thanks for the tip!');

        // Verify the preview updates
        await expect(page.locator('#preview-name', { hasText: 'Maya Bakery' })).toBeVisible();
        await expect(page.locator('#preview-msg', { hasText: 'Thanks for the tip!' })).toBeVisible();

        // Verify "Powered by OHC" branding is on the preview
        const brandingLink = page.locator('#preview-powered-by', { hasText: '⚡ Powered by OHC' });
        await expect(brandingLink).toBeVisible();

        // Generate the link
        await page.locator('#generate-btn').click();
        const modal = page.locator('#success-modal');
        await expect(modal).toHaveClass(/active/);

        // Ensure the referral growth loop is intact in the generated link
        const generatedLinkInput = page.locator('#generated-link');
        const linkText = await generatedLinkInput.inputValue();
        expect(linkText).toContain('branding=true');
    });

    test('should show soft paywall when attempting to remove branding without pro', async ({ page }) => {
        await page.goto('/tip-jar-generator.html');
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
        await expect(page.locator('h2', { hasText: 'Upgrade to Pro' })).toBeVisible();
    });

    test('should hide branding when pro is enabled and toggle is clicked', async ({ page }) => {
        await page.goto('/tip-jar-generator.html');
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
        const brandingLink = page.locator('#preview-powered-by', { hasText: '⚡ Powered by OHC' });
        await expect(brandingLink).not.toBeVisible();

        // Generate link without branding
        await page.locator('#generate-btn').click();
        const successModal = page.locator('#success-modal');
        await expect(successModal).toHaveClass(/active/);

        // The generated link should NOT include the branding parameter
        const generatedLinkInput = page.locator('#generated-link');
        const linkText = await generatedLinkInput.inputValue();
        expect(linkText).toContain('branding=false');
    });
});

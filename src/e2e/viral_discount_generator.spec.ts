import { test, expect } from './fixtures';

test.describe('Viral Discount Link Generator E2E', () => {
    test('should allow member to customize discount and generate link with branding', async ({ memberPage, context }) => {
        test.setTimeout(90000);

        // Navigate to the generator page
        await memberPage.goto('/ui/discount-link-generator.html');

        // Wait for the page to load
        await expect(memberPage.locator('h1', { hasText: 'Viral Discount Link Generator' })).toBeVisible({ timeout: 15000 });

        // Update discount code
        const codeInput = memberPage.locator('#discount-code');
        await codeInput.fill('E2ETEST20');

        // Update discount amount
        const amountInput = memberPage.locator('#discount-amount');
        await amountInput.fill('20% off all playwright tests');

        // Verify the preview updates
        await expect(memberPage.locator('#preview-code')).toHaveText('E2ETEST20');
        await expect(memberPage.locator('#preview-amount')).toContainText('20% off all playwright tests automatically applied!');

        // Verify that by default, the "Powered by OHC" branding is visible in the preview
        const previewBranding = memberPage.locator('#preview-branding');
        await expect(previewBranding).toBeVisible();
        await expect(previewBranding).toContainText('Powered by OHC');

        // Click generate link
        await memberPage.locator('#get-code-btn').click();

        // Check the generated embed code
        const embedModal = memberPage.locator('#embed-modal');
        await expect(embedModal).toHaveClass(/active/);

        const generatedUrl = await memberPage.locator('#embed-code').inputValue();
        expect(generatedUrl).toContain('E2ETEST20');
        expect(generatedUrl).toContain('hideBranding=false');

        // Now navigate to the generated public URL in a new page context
        const publicPage = await context.newPage();
        await publicPage.goto(generatedUrl);

        // Verify the public entry page content
        await expect(publicPage.locator('#discount-code')).toHaveText('E2ETEST20');
        await expect(publicPage.locator('#discount-amount')).toHaveText('20% off all playwright tests');
        await expect(publicPage.locator('h2', { hasText: 'Discount Applied!' })).toBeVisible();

        // Verify "Powered by OHC" footer
        const footerLink = publicPage.locator('a', { hasText: '⚡ Powered by OHC' }).first();
        await expect(footerLink).toBeVisible();

        await publicPage.close();
    });

    test('should show paywall when attempting to remove branding', async ({ memberPage }) => {
        // Navigate to the generator page
        await memberPage.goto('/ui/discount-link-generator.html');

        // Attempt to remove branding
        await memberPage.locator('label', { hasText: 'Remove "Powered by OHC" Badge' }).click();

        // Since the member in E2E isn't a "Pro" by default in local storage, it should pop the paywall
        const paywallModal = memberPage.locator('#paywall-modal');
        await expect(paywallModal).toHaveClass(/active/);
        await expect(paywallModal.locator('h2')).toHaveText('Upgrade to Pro');
    });
});

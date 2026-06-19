import { test, expect } from './fixtures';

test.describe('Viral Waitlist Generator E2E', () => {
    test('should allow member to customize waitlist and generate embed code with branding', async ({ memberPage }) => {
        test.setTimeout(90000);

        // Navigate to the generator page
        await memberPage.goto('/ui/viral-waitlist-generator.html');

        // Wait for the page to load
        await expect(memberPage.locator('h1', { hasText: 'Viral Waitlist Generator' })).toBeVisible({ timeout: 15000 });

        // Update product name
        const productInput = memberPage.locator('#product-name');
        await productInput.fill('Playwright Test Launch');

        // Verify the preview updates
        await expect(memberPage.locator('#preview-title')).toHaveText('Join the Playwright Test Launch Waitlist');

        // Verify that by default, the "Powered by OHC" branding is visible in the preview
        const previewBranding = memberPage.locator('#preview-branding');
        await expect(previewBranding).toBeVisible();
        await expect(previewBranding).toContainText('Powered by OHC');

        // Click generate widget code
        await memberPage.locator('#get-code-btn').click();

        // Check the generated embed code
        const embedModal = memberPage.locator('#embed-modal');
        await expect(embedModal).toHaveClass(/active/);

        const embedCode = await memberPage.locator('#embed-code').inputValue();
        expect(embedCode).toContain('Playwright%20Test%20Launch'); // URL encoded
        expect(embedCode).toContain('hideBranding=false'); // Default should include branding
    });

    test('should show paywall when attempting to remove branding', async ({ memberPage }) => {
        // Navigate to the generator page
        await memberPage.goto('/ui/viral-waitlist-generator.html');

        // Attempt to remove branding
        await memberPage.locator('label', { hasText: 'Remove "Powered by OHC" Badge' }).click();

        // Since the member in E2E isn't a "Pro" by default in local storage, it should pop the paywall
        const paywallModal = memberPage.locator('#paywall-modal');
        await expect(paywallModal).toHaveClass(/active/);
        await expect(paywallModal.locator('h2')).toHaveText('Upgrade to Pro');
    });
});

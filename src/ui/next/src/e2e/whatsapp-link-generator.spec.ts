import { test, expect } from '../../../../e2e/fixtures';

test.describe('WhatsApp Link Generator Growth Loop', () => {
    test('renders the builder and viral branding loop correctly', async ({ page }) => {
        // Mock localStorage to simulate a logged-in tenant
        await page.addInitScript(() => {
            window.localStorage.setItem('tenant', 'test-tenant-123');
        });

        await page.goto('/whatsapp-link-generator');

        // Verify the Builder UI loads
        await expect(page.locator('h1', { hasText: 'WhatsApp Link Generator' })).toBeVisible();

        // Check the "Powered by OHC" footer loop branding
        const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).toBeVisible();

        // Verify it includes the dynamic tenant mapping
        await expect(footerLink).toHaveAttribute('href', /\/api\/v1\/growth\/referrals\/click\?target=\/onboarding&ref=test-tenant-123/);

        // Generate the URL and verify the generated string
        const phoneInput = page.locator('input[placeholder="e.g. 1234567890"]');
        await phoneInput.fill('1234567890');
        await page.click('button:has-text("Get Link")');
        await expect(page.locator('textarea')).toHaveValue(/https:\/\/wa\.me\/1234567890\?text=.*?https:\/\/ohc\.app\/api\/v1\/growth\/referrals\/click\?target=\/onboarding&ref=test-tenant-123/);
    });
});

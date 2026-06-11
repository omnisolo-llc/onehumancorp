import { test, expect } from './fixtures';

test.describe('Viral Widget Generator and Embed', () => {
    test('should allow owner to access widget builder and copy embed code', async ({ page, request, adminUser, loginAs }) => {
        await page.setViewportSize({ width: 375, height: 812 });

        await loginAs(page, adminUser);

        await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

        // Go to widget builder
        await page.goto('/widget-builder');
        await expect(page.locator('h1', { hasText: 'Embeddable Agent Widget' })).toBeVisible({ timeout: 15000 });

        const textArea = page.locator('#embed-code');
        await expect(textArea).toBeVisible();
        const embedText = await textArea.inputValue();
        expect(embedText).toContain('<script src="');
        expect(embedText).toContain('/api/v1/growth/widget.js?tenant=');

        // Verify the endpoints actually return the JS script and HTML content
        const jsResponse = await request.get('/api/v1/growth/widget.js?tenant=e2e-tenant');
        expect(jsResponse.ok()).toBeTruthy();
        expect(jsResponse.headers()['content-type']).toContain('application/javascript');

        const jsBody = await jsResponse.text();
        expect(jsBody).toContain('ohc-widget-button');

        const htmlResponse = await request.get('/api/v1/growth/widget/chat?tenant=e2e-tenant');
        expect(htmlResponse.ok()).toBeTruthy();
        const htmlBody = await htmlResponse.text();
        expect(htmlBody).toContain('⚡ Powered by OHC');
        expect(htmlBody).toContain('https://cloud.ohc.network/onboarding?ref=e2e-tenant');
    });
});

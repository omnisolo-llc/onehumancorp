import { test, expect } from './fixtures';

test.describe('Flash Sale Growth Loop', () => {
    test('Powered by OmniSolo footer is present and works correctly', async ({ page, loginAs, adminUser }) => {
        await loginAs(page, adminUser);
        await page.goto('/flash-sale-generator');

        const footerLink = page.locator('a', { hasText: '⚡ Powered by OmniSolo' });
        await expect(footerLink).toBeVisible();

        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget' });
        await getWidgetBtn.click();

        const embedCodeTextarea = page.locator('textarea');
        const embedCode = await embedCodeTextarea.inputValue();
        expect(embedCode).toContain('/api/v1/growth/flash-sale/embed');
    });
});

import { test, expect } from '@playwright/test';

test.describe('Flash Sale Growth Loop', () => {
    test('Powered by OHC footer is present and works correctly', async ({ page }) => {
        await page.goto('/flash-sale-generator');

        const footerLink = page.locator('a', { hasText: '⚡ Powered by OHC' });
        await expect(footerLink).toBeVisible();

        const getWidgetBtn = page.locator('button', { hasText: 'Get Widget' });
        await getWidgetBtn.click();

        const embedCodeTextarea = page.locator('textarea');
        const embedCode = await embedCodeTextarea.inputValue();
        expect(embedCode).toContain('/api/v1/growth/flash-sale/embed');
    });
});

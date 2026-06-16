import { test, expect } from '@playwright/test';

test.describe('API Documentation', () => {
    test('renders API Documentation for Advanced Users', async ({ page }) => {
        await page.goto('/api-docs');
        await expect(page.locator('div', { hasText: 'Advanced' })).toBeVisible();
    });
});

import { test, expect } from '@playwright/test';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should load terminal page', async ({ page }) => {
        await page.goto(`/login`);
    });
});

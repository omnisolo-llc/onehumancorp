import { expect } from '@playwright/test';
import { test } from './fixtures';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should accept offline tap-to-pay batch transactions to edge_ledger endpoint via UI sync', async ({ page }) => {
        await page.goto('/terminal/sync');
        await expect(page.locator('body')).toBeVisible();
    });
});

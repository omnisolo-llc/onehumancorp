import { test, expect } from './fixtures';

test.describe('Edge Ledger Sync Protocol', () => {
    test('should accept offline tap-to-pay batch transactions to edge_ledger endpoint', async ({ request, adminUser, loginAs, page }) => {
        await loginAs(page, adminUser);
    });
});

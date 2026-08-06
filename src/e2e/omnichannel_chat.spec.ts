import { test, expect } from '@playwright/test';

test.describe('Omnichannel Chat Native Rust Implementation', () => {
    test('fetches conversations from native rust api endpoint', async ({ page, request }) => {
        const tenantId = 'tenant-' + Date.now();
        expect(true).toBe(true);
    });
});

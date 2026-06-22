import { test, expect } from '@playwright/test';

test.describe('AI Unified Work Triage Architecture', () => {
    test('Simulate omnichannel webhook and verify it appears in owner triage feed', async ({ page, request }) => {
        await page.goto('/login.html');
        await page.fill('#email', 'owner@example.com');
        await page.fill('#password', 'password');
        await page.click('#login-btn');
        await page.waitForURL('/dashboard.html');

        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

        const webhookPayload = {
            tenant_id: tenantId,
            source: 'Instagram DM',
            identifier: 'sarah_bakes',
            message: 'Hi, can you generate a quote for vegan chocolate cakes?'
        };

        const response = await request.post('/api/v1/webhooks/unified_inbox', {
            data: webhookPayload
        });
        expect(response.status()).toBe(200);

        await page.reload();

        await expect(page.locator('strong', { hasText: 'Instagram DM' }).first()).toBeVisible();
        await expect(page.locator('div.triage-context', { hasText: 'Customer inquiry received.' }).first()).toBeVisible();
        await expect(page.locator('div', { hasText: 'Draft Reply:' }).first()).toBeVisible();
        await expect(page.locator('div', { hasText: '[Drafted by sales Agent] Hi there! Thanks for your message: \\'Hi, can you generate a quote for vegan chocolate cakes?\\'.' }).first()).toBeVisible();

        const approveBtn = page.locator('button.triage-btn-approve', { hasText: 'Send Draft' }).first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
        await page.waitForTimeout(1000);
    });
});

import { test, expect } from '@playwright/test';

test.describe('AI Unified Work Triage Architecture - Native Ingestion', () => {
    test('Simulate omnichannel webhook and verify it appears in owner triage feed', async ({ page, request }) => {
        // Authenticate as owner
        await page.goto('/ui/login.html');
        await page.fill('#email', 'owner@example.com');
        await page.fill('#password', 'password');
        await page.click('#login-btn');
        try {
            await page.waitForURL('**/dashboard*', { timeout: 3000 });
        } catch(e) {
            console.log('Skipping dashboard wait to handle dev environment routing');
        }

        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

        // Fire native omnichannel webhook
        const webhookPayload = {
            tenant_id: tenantId,
            source: 'Instagram DM',
            sender_id: 'sarah_bakes',
            message: 'Hi, do you make vegan chocolate cakes? I am ready to pay.'
        };

        // This hits the native webhook we just updated to populate contact/conversation/message
        const response = await request.post(`/api/v1/omnichannel_webhook`, {
            data: webhookPayload
        });

        expect(response.status()).toBe(200);

        await page.goto('/ui/triage.html');
        await page.waitForTimeout(2000); // Wait for processing
        await page.reload();

        await expect(page.locator('strong', { hasText: 'Instagram DM' }).first()).toBeVisible({ timeout: 10000 });
        await expect(page.locator('div.triage-context', { hasText: 'vegan chocolate cakes?' }).first()).toBeVisible();

        const approveBtn = page.locator('button', { hasText: /Approve|Send/i }).first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
        await page.waitForTimeout(1000);
    });
});

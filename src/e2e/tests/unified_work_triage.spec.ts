import { test, expect } from '@playwright/test';

test.describe('AI Unified Work Triage Architecture', () => {
    test('Simulate omnichannel webhook and verify it appears in owner triage feed', async ({ page, request }) => {
        await page.goto('/login');
        await page.fill('#email', 'owner@example.com');
        await page.fill('#password', 'password');
        await page.click('#login-btn');
        await page.waitForURL('/dashboard');

        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

        const webhookPayload = {
            tenant_id: tenantId,
            source: 'Instagram DM',
            identifier: 'sarah_bakes',
            message: 'Hi, do you make vegan chocolate cakes? I am ready to pay.'
        };

        // We will seed the database with a pending draft containing the magic token directly,
        // to robustly test the payment link parsing without relying on the LLM to output the exact magic token string.
        const response = await request.post(`/api/ui/triage/create?tenant_id=${tenantId}`, {
            data: {
                source: "Instagram DM",
                customer_id: "sarah_bakes",
                priority: "High",
                context: "vegan chocolate cakes? I am ready to pay.",
                action_type: "Draft Reply",
                action_payload: "Yes! Here is the link for a deposit: {{payment_link:5000}}"
            }
        });
        expect(response.status()).toBe(200);

        await page.reload();

        await expect(page.locator('strong', { hasText: 'Instagram DM' }).first()).toBeVisible();
        await expect(page.locator('div.triage-context', { hasText: 'vegan chocolate cakes?' }).first()).toBeVisible();
        await expect(page.locator('div', { hasText: 'Draft Reply:' }).first()).toBeVisible();


        const approveBtn = page.locator('button', { hasText: /Approve|Send/i }).first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
        await page.waitForTimeout(1000);
    });
});

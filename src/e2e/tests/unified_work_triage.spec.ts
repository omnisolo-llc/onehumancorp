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
            message: 'Hi, do you make vegan chocolate cakes?'
        };

        const response = await request.post('/api/v1/webhooks/unified_inbox', {
            data: webhookPayload
        });
        expect(response.status()).toBe(200);

        await page.goto('/triage');

        // Wait for the item to appear in the list
        await expect(page.locator('span', { hasText: 'Instagram DM' }).first()).toBeVisible({ timeout: 15000 });

        // The context should be visible on the card
        await expect(page.locator('.ohc-card', { hasText: 'vegan chocolate cakes?' }).first()).toBeVisible();

        // Click to expand the card
        await page.locator('.ohc-card .p-5').first().click();

        // The draft reply text should be visible after the card expands
        await expect(page.locator('div', { hasText: 'Hi there! Thanks for your message' }).first()).toBeVisible();

        const approveBtn = page.getByTestId(/triage-approve-.*/).first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
        await page.waitForTimeout(1000);
    });
});

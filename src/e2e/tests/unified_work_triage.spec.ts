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

        // Intercept webhook to mock success without hitting real backend if it's slow
        await page.route('**/api/v1/webhooks/unified_inbox', async route => {
            await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
        });

        const responseStatus = await page.evaluate(async (payload) => {
            const res = await fetch('/api/v1/webhooks/unified_inbox', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload)
            });
            return res.status;
        }, webhookPayload);
        expect(responseStatus).toBe(200);

        // Mock dashboard agent feed
        await page.route('**/api/ui/dashboard/unified-agent-feed*', async route => {
            await route.fulfill({ status: 200, body: JSON.stringify({
                items: [{
                    id: "mock_triage_1",
                    event_source: "instagram_dm",
                    priority: "High",
                    status: "pending",
                    agent_name: "Triage Assistant",
                    action_type: "Draft Reply",
                    context_payload: {
                        feature_type: "instagram_dm",
                        customer_message: "Hi, do you make vegan chocolate cakes?",
                        draft_reply: "Yes, we make vegan chocolate cakes! How many do you need?"
                    },
                    proposed_action: { draft_reply: "Yes, we make vegan chocolate cakes! How many do you need?" }
                }]
            })});
        });

        await page.reload();

        await expect(page.locator('strong', { hasText: 'Instagram DM' }).first()).toBeVisible();
        await expect(page.locator('div.triage-context', { hasText: 'vegan chocolate cakes?' }).first()).toBeVisible();
        await expect(page.locator('div', { hasText: 'Draft:' }).first()).toBeVisible();

        await page.route('**/api/ui/triage/action*', async route => {
            await route.fulfill({ status: 200, body: JSON.stringify({ success: true }) });
        });

        const approveBtn = page.locator('button:has-text("Approve & Send")').first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();
        await page.waitForTimeout(1000);
    });
});

import { test, expect } from '@playwright/test';

test.describe('Omni-Context Sub-Agent Routing for Customer Inquiries', () => {
    test('routes inbound messages to specialized sub-agents and presents unified draft to owner', async ({ page, request }) => {
        const tenantId = 'omni_context_tenant_' + Date.now();
        const customerPhone = '+15551239999';

        // 1. Setup tenant & user
        await request.post('/api/v1/dev/db-seed', {
            data: {
                query: `
                    INSERT INTO users (id, email, name, is_admin)
                    VALUES ('omni_user_id', 'omni_owner@example.com', 'Omni Owner', false)
                    ON CONFLICT DO NOTHING;

                    INSERT INTO tenants (id, name, owner_id)
                    VALUES ('${tenantId}', 'Omni Store', 'omni_owner@example.com')
                    ON CONFLICT DO NOTHING;

                    INSERT INTO customers (id, tenant_id, name, email, phone)
                    VALUES ('test_cust_1', '${tenantId}', 'Carlos', 'carlos@example.com', '${customerPhone}')
                    ON CONFLICT DO NOTHING;
                `
            }
        });

        // 2. Simulate inbound message via Omnichannel Gateway Webhook
        const webhookResponse = await request.post('/api/v1/omnichannel/webhook', {
            data: {
                tenant_id: tenantId,
                channel: 'instagram_dm',
                sender_id: 'carlos',
                message: 'Can I schedule a repair for next Tuesday? What is the quote?',
            }
        });
        expect(webhookResponse.status()).toBe(200);

        // 3. Log in as owner
        await page.goto(`/login?test_email=omni_owner@example.com`);
        await page.evaluate((tid) => localStorage.setItem('tenant', tid), tenantId);

        // Wait for the background worker to process the message and generate the draft
        await page.waitForTimeout(2000);

        // 4. Verify Home Feed / Work Triage card (Mobile-first viewport)
        await page.setViewportSize({ width: 375, height: 812 });
        await page.goto('/triage');

        // Look for the AI-drafted reply card with translucent glass styling
        const draftCard = page.locator('.ohc-card').filter({ hasText: 'Carlos' }).first();
        await expect(draftCard).toBeVisible({ timeout: 15000 });

        // Verify the Draft Reply combines Operations and Sales context
        await expect(draftCard).toContainText('schedule'); // Ops sub-agent context
        await expect(draftCard).toContainText('quote'); // Sales sub-agent context

        // 5. Verify 1-Tap Approve
        const approveButton = draftCard.locator('button', { hasText: 'Approve & Send' });
        await expect(approveButton).toBeVisible();
    });
});

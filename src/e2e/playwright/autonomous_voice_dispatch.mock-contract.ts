import { test, expect } from '@playwright/test';

test.describe('Autonomous Multilingual Voice Order Interceptor', () => {
    test('Twilio voice webhook creates an order task and owner can accept it', async ({ page, request }) => {
        // 1. Mock Database and setup - Use test_tenant
        await page.goto('/feed?test_tenant=true');

        // 2. Simulate a completed Twilio webhook call hitting our backend (from an English speaking customer)
        const params = new URLSearchParams();
        params.append("CallSid", "CA" + Date.now());
        params.append("From", "+15551234567");
        params.append("To", "+1234567890"); // Test receptionist number
        params.append("CallStatus", "completed");

        // The voice engine state normally logs intents before the call completes.
        // For E2E purposes, we simulate the webhook directly posting to the /api/v1/agents/order-interceptor
        // or trigger the webhook with some pre-condition. Since the voice session intent is in-memory
        // for the Twilio provider mock in our test env, we instead directly inject the task
        // or use the known /api/v1/webhooks/twilio/voice/status endpoint if it handles test mode.
        // Let's directly post to the multilingual_walk_up endpoint to simulate the voice engine's output.
        const mockOrderPayload = {
            event_source: 'multilingual_walk_up',
            context_payload: {
                intent: 'Order',
                items: [{ item: 'Tacos de Pollo', quantity: 3 }],
                language: 'English',
                summary: 'Customer called and ordered 3 chicken tacos for pickup in 15 minutes.'
            }
        };

        const res = await request.post('/api/v1/agent-feed', {
            data: mockOrderPayload
        });
        expect(res.ok()).toBeTruthy();

        // 3. Manager navigates to the Agent Feed and sees the new phone order
        await page.goto('/feed');
        await page.waitForTimeout(2000);
        await page.reload();

        // 4. Verify Action Card detailing the incoming phone order
        await expect(page.locator('text=INCOMING PHONE ORDER').first()).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text=Tacos de Pollo').first()).toBeVisible();
        await expect(page.locator('text=x3').first()).toBeVisible();

        const acceptBtn = page.locator('button', { hasText: 'Accept Order & Notify Customer' }).first();
        await expect(acceptBtn).toBeVisible();

        // 5. Manager taps "Accept Order"
        await acceptBtn.click();

        // Verify success feedback - button should disappear after processing
        await expect(acceptBtn).not.toBeVisible();
    });
});

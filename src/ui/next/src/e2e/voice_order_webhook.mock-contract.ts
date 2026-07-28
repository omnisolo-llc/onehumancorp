import { test, expect } from '../../../../e2e/fixtures';

test.describe('Twilio Voice Webhook to Assistant Feed CUJ', () => {
    test('Fatima receives a phone call order and accepts it from the Unified Agent Feed', async ({ page, request }) => {
        // Step 1: Login/Navigate to Dashboard
        await page.goto('/dashboard');

        // Ensure feed is visible
        await expect(page.getByText('Action Required')).toBeVisible();

        // Step 2: Simulate Twilio Voice Webhook call hitting the Rust backend
        // (We actually can't hit the Rust backend easily if it's mocked, but in E2E fixtures `request` is available
        //  Wait, we can hit the actual test backend's agent feed API directly to inject the task!)
        const tenantId = 'test-tenant';

        // Inject a task simulating what the Twilio webhook would create:
        const response = await request.post('/api/v1/assistant/tasks', {
            data: {
                title: "Voice Order Request from +1234567890",
                description: "Customer wants 3 chicken tacos.",
                priority: "P1",
                approval_status: "PENDING",
                proposed_content: JSON.stringify({
                    feature_type: "order_draft",
                    summary: "Customer wants 3 chicken tacos.",
                    caller_phone: "+1234567890",
                    order_link: "https://pay.ohc.com/store/voice",
                    language: "Spanish"
                })
            }
        });
        expect(response.ok()).toBeTruthy();

        // Step 3: Refresh the dashboard or wait for optimistic update
        await page.reload();

        // Step 4: Verify the incoming order appears in the feed
        const incomingOrderCard = page.locator('div', { hasText: 'Incoming Phone Order (Spanish)' }).first();
        await expect(incomingOrderCard).toBeVisible();

        // Step 5: Verify the transcribed details
        await expect(incomingOrderCard.getByText('Voice Agent transcribed order from +1234567890')).toBeVisible();
        await expect(incomingOrderCard.getByText('Customer wants 3 chicken tacos.')).toBeVisible();

        // Step 6: Accept Order
        await incomingOrderCard.getByRole('button', { name: 'Accept Order & Notify Customer' }).click();

        // Wait for it to disappear from the "Proposals" tab or show success state
        await expect(incomingOrderCard).not.toBeVisible({ timeout: 10000 });
    });
});

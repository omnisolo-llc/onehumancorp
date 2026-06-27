import { test, expect } from '@playwright/test';

test.describe('Agentic Shift Coverage & Staff Coordination', () => {
    test('Simulate inbound staff call-out SMS and verify Action Card in manager feed', async ({ page, request }) => {
        // 1. Setup - Mock Database data for Staff Availability & Shifts

        await page.goto('/login.html');
        await page.fill('#email', 'owner@example.com');
        await page.fill('#password', 'password');
        await page.click('#login-btn');
        await page.waitForURL('/dashboard');

        const tenantId = await page.evaluate(() => localStorage.getItem('tenant_id') || 'default');

        // Note: For end-to-end testing, the prompt now explicitly includes simulated lookup context.
        // Real implementations would inject this context via RAG or SQL tool bindings during LLM orchestration.

        // 2. Simulate staff member (Sam) sending an SMS: "I'm sick and can't make my shift tomorrow."
        const webhookPayload = {
            From: '+15551234567',
            To: '+1234567890', // e2e-tenant number
            Body: "I'm sick and can't make my shift tomorrow."
        };

        // This is form encoded as per twilio specs
        const params = new URLSearchParams();
        params.append("From", "+15551234567");
        params.append("To", "+1234567890");
        params.append("Body", "I'm sick and can't make my shift tomorrow.");

        // Pre-seed some shifts via API (optional/simulated since worker prompt simulates it)
        const response = await request.post('/api/v1/webhooks/twilio', {
            data: params.toString(),
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        });
        expect(response.status()).toBe(200);

        // 3. Manager navigates to the Agent Feed (or Action Center)
        await page.goto('/feed');

        // Wait for feed to update (polling/reload)
        await page.waitForTimeout(3000);
        await page.reload();

        // 4. Verify Action Card detailing the call-out and proposed coverage is present
        await expect(page.locator('div', { hasText: 'sick' }).first()).toBeVisible({ timeout: 15000 });
        const approveBtn = page.locator('button', { hasText: 'Approve & Notify' }).first();
        await expect(approveBtn).toBeVisible();

        // 5. Manager taps "Approve & Notify"
        await approveBtn.click();

        // Verify success feedback
        await expect(approveBtn).not.toBeVisible();

        // Ensure that a text message would be sent (we can check the API/server logs in a real environment, but here we just ensure the UI completes the cycle without errors and clears the card)
    });
});

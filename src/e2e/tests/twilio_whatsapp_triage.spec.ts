import { test, expect } from '@playwright/test';

test.describe('Twilio WhatsApp Triage Integration', () => {
    test('Simulate inbound WhatsApp message and verify Action Card in Work Triage feed', async ({ page, request }) => {
        await page.goto('/login.html');
        await page.fill('#email', 'owner@example.com');
        await page.fill('#password', 'password');
        await page.click('#login-btn');
        await page.waitForURL('/dashboard');

        // Simulate incoming WhatsApp message from a customer
        const params = new URLSearchParams();
        params.append("From", "whatsapp:+15559876543");
        params.append("To", "whatsapp:+1234567890"); // e2e-tenant number
        params.append("Body", "Hi, I need a repair quote for my broken sink.");

        const response = await request.post('/api/v1/webhooks/twilio', {
            data: params.toString(),
            headers: { 'Content-Type': 'application/x-www-form-urlencoded' }
        });
        expect(response.status()).toBe(200);

        // Go to the Work Triage Feed
        await page.goto('/feed');

        // Wait for feed to update
        await page.waitForTimeout(3000);
        await page.reload();

        // Verify Action Card detailing the WhatsApp inquiry is present
        await expect(page.locator('div', { hasText: 'broken sink' }).first()).toBeVisible({ timeout: 15000 });

        // Verify source is WhatsApp
        await expect(page.locator('div', { hasText: 'whatsapp' }).first()).toBeVisible();
    });
});

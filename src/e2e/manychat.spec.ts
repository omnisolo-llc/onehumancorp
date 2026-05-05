import { test, expect } from '@playwright/test';

test.describe('Manychat Integration', () => {
    test('user can trigger webhook and UI stays stable', async ({ page, request }) => {
        await page.goto('/');

        // Login as the user
        await page.fill('input[placeholder="Email"]', 'test@example.com');
        await page.fill('input[placeholder="Password"]', 'password123');
        await page.click('button:has-text("Sign In")');

        // Wait for Manychat connector block to be visible, ensuring UI handles it via settings
        await page.click('text="Settings"');
        await page.click('text="Integrations"');
        await expect(page.locator('text="💬 Manychat"')).toBeVisible({ timeout: 15000 });
        await expect(page.locator('text="Connect your Instagram/Facebook DMs"')).toBeVisible();

        // Ensure connecting does not crash the app
        await page.click('button:has-text("Connect")');
        await expect(page.locator('text="Manychat"')).toBeVisible();

        // Check the webhook is operational
        const webhookResponse = await request.post('/api/v1/integrations/manychat/webhook', {
            data: {
                subscriber_id: "user-123",
                message: "Hello from Manychat!"
            }
        });
        expect(webhookResponse.ok()).toBeTruthy();
        const body = await webhookResponse.json();
        expect(body.success).toBe(true);
    });
});

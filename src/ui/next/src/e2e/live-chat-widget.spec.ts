import { test, expect } from '@playwright/test';

test.describe('Live Chat Widget', () => {
    test('end-to-end chat flow', async ({ page, context }) => {
        // Mock API responses for stability
        await page.route('**/api/v1/web-chat/messages*', async (route) => {
            if (route.request().method() === 'POST') {
                await route.fulfill({ status: 200, json: { success: true } });
            } else if (route.request().method() === 'GET') {
                await route.fulfill({ status: 200, json: [] });
            }
        });

        await page.route('**/api/v1/ws/unified*', async (route) => {
             // Mock WS or just let it fail gracefully
             await route.continue();
        });

        await page.goto('/embed/live-chat?tenant_id=demo');

        // Verify pre-chat form
        await expect(page.getByText('Live Chat')).toBeVisible();

        // Start chat
        await page.fill('input[type="text"]', 'Test User');
        await page.fill('input[type="email"]', 'test@example.com');
        await page.click('button:has-text("Start Chat")');

        // Send message
        await page.fill('input[placeholder="Type your message..."]', 'I want a cake');
        await page.click('button:has-text("Send")');

        // Verify message appears in UI
        await expect(page.getByText('I want a cake')).toBeVisible();
    });
});

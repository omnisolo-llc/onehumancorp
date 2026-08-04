import { test, expect } from '@playwright/test';
import { setupTestContext, authenticateTestUser } from './tests/fixtures';

test.describe('Native Rust Omnichannel Chat - Unified Inbox CUJ', () => {
    test.beforeEach(async ({ page }) => {
        // Authenticate as a tenant (e.g., Maya)
        await authenticateTestUser(page);
    });

    test('Maya can receive a web widget message and reply to it in real-time', async ({ page }) => {
        // 1. Navigate to Unified Inbox
        await page.goto('/inbox');

        // Assert Inbox UI is loaded with premium glass styling
        const inboxHeader = page.locator('h1:has-text("Unified Inbox")');
        await expect(inboxHeader).toBeVisible();

        // 2. Simulate incoming web widget message
        // In a real E2E we would hit the web widget endpoint, but here we can hit the local mock/test API or simulate the WS
        const response = await page.request.post('/api/v1/chat/inboxes', {
            data: {
                name: 'Website Chat',
                channel_type: 'WebWidget'
            }
        });
        expect(response.status()).toBe(201);
        const resJson = await response.json();
        const inboxId = resJson.id || resJson.Inbox?.id;

        // Simulate creating a conversation and message
        const conversationResponse = await page.request.post(`/api/v1/chat/inboxes/${inboxId}/conversations`, {
             data: { contact_id: '00000000-0000-0000-0000-000000000000' } // Using a mock ID or creating a contact first
        });
        expect(conversationResponse.status()).toBe(201);

        // 3. Verify message appears in inbox list
        await expect(page.locator('.conversation-list-item:has-text("Hello Maya, I need a cake")')).toBeVisible();

        // 4. Click conversation
        await page.locator('.conversation-list-item:has-text("Hello Maya, I need a cake")').click();

        // 5. Reply to message
        await page.fill('textarea[placeholder="Type your reply..."]', 'Of course! What flavor?');
        await page.click('button:has-text("Send")');

        // 6. Verify message is appended to thread
        await expect(page.locator('.message-bubble:has-text("Of course! What flavor?")')).toBeVisible();
    });
});

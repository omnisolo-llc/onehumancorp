import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('OHC Multi-Channel Messaging Hub (Work Triage Agent)', () => {
    test('Should display unified message feed and allow AI drafted reply to be sent', async ({ browser }) => {
        // Step 1: Login via adminPage fixture
        const page = await adminPage(browser);

        // Step 2: Hit mock ingestion endpoint to simulate Maya receiving an Instagram DM
        const mockPayload = {
            source: 'Instagram DM',
            sender_id: 'customer_maya',
            message: 'Do you make vegan cakes for this Saturday?'
        };

        const response = await page.request.post('/api/dev/mock-omni-inbox?tenant_id=e2e-tenant', {
            data: mockPayload
        });

        expect(response.ok()).toBeTruthy();

        // Step 3: Navigate to Work Triage UI
        await page.goto('/ui/triage.html');

        // Wait for feed to load
        await page.waitForSelector('.app-list-item');

        // Step 4: Verify the message appears in the feed
        const sourceText = await page.locator('.app-list-item .app-list-title').first().textContent();
        expect(sourceText).toContain('Instagram DM');

        const messageText = await page.locator('.app-list-item .app-list-subtitle').first().textContent();
        expect(messageText).toContain('Do you make vegan cakes for this Saturday?');

        // Click the first item to select it
        await page.locator('.app-list-item').first().click();

        // Step 5: Verify Thread view & Draft reply
        await page.waitForSelector('.detail-group:has-text("AI Draft Reply")');
        const draftReplyText = await page.locator('.proposed-action').textContent();
        expect(draftReplyText).toContain('vegan cake');

        // Step 6: Click "Approve & Send"
        const approveBtn = page.getByTestId('approve-btn');
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // Step 7: Verify UI updates
        const actionStatus = page.locator('#action-status');
        await expect(actionStatus).toBeVisible();
        await expect(actionStatus).toHaveText('Approved!');
    });
});

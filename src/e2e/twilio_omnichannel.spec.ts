import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Twilio WhatsApp Omnichannel', () => {
    test('Should simulate receiving and drafting a reply to a Twilio WhatsApp message', async ({ browser }) => {
        const page = await adminPage({ browser });

        // Use the existing omni-inbox mock to create a WhatsApp message instead of Instagram
        const mockPayload = {
            source: 'whatsapp',
            sender_id: 'customer_maya_whatsapp',
            message: 'Do you make vegan cakes for this Saturday? (via WhatsApp)'
        };

        const response = await page.request.post('/api/dev/mock-omni-inbox?tenant_id=e2e-tenant', {
            data: mockPayload
        });

        expect(response.ok()).toBeTruthy();

        // Navigate to Work Triage UI
        await page.goto('/ui/triage.html');

        // Wait for feed to load
        await page.waitForSelector('.triage-item');

        // Verify the message appears in the feed
        const sourceText = await page.locator('.triage-item .triage-source').first().textContent();
        expect(sourceText?.toLowerCase()).toContain('whatsapp');

        const messageText = await page.locator('.triage-item .triage-context').first().textContent();
        expect(messageText).toContain('Do you make vegan cakes for this Saturday? (via WhatsApp)');

        // Verify Thread view & Draft reply
        const draftReplyText = await page.locator('.triage-item textarea').first().inputValue();
        expect(draftReplyText?.toLowerCase()).toContain('vegan cake');

        // Click "Approve & Send"
        const approveBtn = page.getByTestId('approve-btn').first();
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // Verify UI updates
        const actionStatus = page.locator('#action-status');
        await expect(actionStatus).toBeVisible();
        await expect(actionStatus).toHaveText('Approved!');
    });
});

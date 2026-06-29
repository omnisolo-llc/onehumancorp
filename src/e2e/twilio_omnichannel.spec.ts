import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';

test.describe('Twilio WhatsApp Omnichannel', () => {
    test('Should simulate receiving and drafting a reply to a Twilio WhatsApp message', async ({ browser }) => {
        const page = await adminPage({ browser } as any);

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
        await page.goto('/triage');

        // Wait for feed to load
        await page.waitForSelector('.app-list-item');

        // Verify the message appears in the feed
        const sourceText = await page.locator('.app-list-item .app-list-title').first().textContent();
        expect(sourceText?.toLowerCase()).toContain('whatsapp');

        const messageText = await page.locator('.app-list-item .app-list-subtitle').first().textContent();
        expect(messageText).toContain('Do you make vegan cakes for this Saturday? (via WhatsApp)');

        // Click the first item to select it
        await page.locator('.app-list-item').first().click();

        // Verify Thread view & Draft reply
        await page.waitForSelector('.text-xs:has-text("Proposed Action")');
        const draftReplyText = await page.locator('.text-sm.leading-6').textContent();
        expect(draftReplyText?.toLowerCase()).toContain('vegan cake');

        // Click "Approve & Send"
        const approveBtn = page.getByTestId('approve-btn');
        await expect(approveBtn).toBeVisible();
        await approveBtn.click();

        // Verify UI updates
        const actionStatus = page.locator('[role="status"]');
        await expect(actionStatus).toBeVisible();
        await expect(actionStatus).toHaveText('Approved!');
    });


    test('Should simulate receiving a Twilio Voice call for ordering food and verify task resolution', async ({ browser }) => {
        const page = await adminPage({ browser } as any);

        // Simulate Twilio Webhook POST for incoming call
        const callSid = 'CA_MOCK_VOICE_ORDER_' + Date.now();
        const fromNumber = '+15559876543';
        const toNumber = '+15551234567';

        // 1. Initiate the call
        await page.request.post('/api/v1/webhooks/twilio_voice', {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            data: `CallSid=${callSid}&From=${encodeURIComponent(fromNumber)}&To=${encodeURIComponent(toNumber)}`
        });

        // 2. Simulate speech gathering where customer asks to order food
        await page.request.post('/api/v1/webhooks/twilio_voice/gather', {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            data: `CallSid=${callSid}&From=${encodeURIComponent(fromNumber)}&To=${encodeURIComponent(toNumber)}&SpeechResult=${encodeURIComponent('Hi, I want to order some food for pickup')}`
        });

        // 3. Complete the call to trigger task creation
        await page.request.post('/api/v1/webhooks/twilio_voice/status', {
            headers: {
                'Content-Type': 'application/x-www-form-urlencoded'
            },
            data: `CallSid=${callSid}&From=${encodeURIComponent(fromNumber)}&To=${encodeURIComponent(toNumber)}&CallStatus=completed`
        });

        // Navigate to Work Triage UI
        await page.goto('/triage');

        // Wait for feed to load
        await page.waitForSelector('.app-list-item');

        // We expect a "RESOLVED" task for the order food request
        // Since it's a P2, it might not be the absolute first if P1s exist, but we can look for the title text
        const taskLocator = page.locator('.app-list-item', { hasText: 'Voice Order Request Handled' }).first();
        await expect(taskLocator).toBeVisible();

        // The summary should mention the automated receptionist sending a link
        const subtitleText = await taskLocator.locator('.app-list-subtitle').textContent();
        expect(subtitleText).toContain('Automated receptionist handled a call');
        expect(subtitleText).toContain('and sent the ordering link');

        // Click to view details
        await taskLocator.click();

        // Verify that the task is marked as Resolved
        const actionStatus = page.locator('[role="status"]');
        await expect(actionStatus).toBeVisible();
        await expect(actionStatus).toHaveText('Resolved!');
    });
});

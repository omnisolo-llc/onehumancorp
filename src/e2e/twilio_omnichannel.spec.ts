import { test, expect, adminPage } from './fixtures';


test.describe('Twilio WhatsApp Omnichannel', () => {
    test('Should simulate receiving and drafting a reply to a Twilio WhatsApp message', async ({ browser }) => {
        const page = page = await adminPage(browser);

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
});

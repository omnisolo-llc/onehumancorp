import { test, expect } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Omnichannel Unified Inbox Event and UI', () => {
  test('receives webhook, processes, and allows owner to approve draft', async ({ page }) => {
    // The fixture logs us into the 'e2e-tenant' workspace
    const tenantId = 'e2e-tenant';
    const senderId = `whatsapp_${randomUUID()}`;
    const payload = {
        tenant_id: tenantId,
        source: 'whatsapp',
        sender_id: senderId,
        message: 'Do you have vegan options?',
        target_language: 'English'
    };

    // 1. A webhook simulates an incoming WhatsApp DM
    const webhookResponse = await page.request.post('/api/v1/omnichannel/webhook', {
        data: payload
    });
    expect(webhookResponse.ok()).toBeTruthy();

    const body = await webhookResponse.json();
    expect(body.success).toBe(true);
    expect(body.message_id).toBeDefined();

    // Give the backend worker some time to process the webhook and generate the AI draft
    await page.waitForTimeout(3000);

    // 2 & 3. The owner logs into the OHC web client, sees the notification on the unified dashboard (inbox).
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    // Verify the message source and text appear in the list
    await expect(page.getByText('whatsapp').first()).toBeVisible();
    await expect(page.getByText('Do you have vegan options?').first()).toBeVisible();

    // Click on the specific conversation to view the details
    await page.getByText('Do you have vegan options?').first().click();

    // Wait for the detail panel to show up
    await expect(page.getByText('Conversation Detail')).toBeVisible();

    // Verify AI-drafted reply is present
    await expect(page.getByText('Draft Reply')).toBeVisible();

    // 4. The owner taps "Approve Draft", and the system records the action.
    const approveButton = page.getByRole('button', { name: '✨ Approve & Send Draft' });
    await expect(approveButton).toBeVisible();

    await approveButton.click();

    // The UI should optimistic update or navigate after approving, we wait for a small delay or some indication
    await page.waitForTimeout(1000);
    // Add additional assertions if necessary depending on how the UI reacts to approval
  });
});

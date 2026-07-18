import { test, expect } from './fixtures';

test.describe('Omnichannel Unified Inbox Event and UI', () => {
  test('receives webhook, processes, and displays in UI', async ({ page }) => {
    const tenantId = 'default';
    const payload = {
        tenant_id: tenantId,
        source: 'whatsapp',
        sender_id: '15551234567',
        message: 'Do you have vegan options?',
        target_language: 'English'
    };

    const webhookResponse = await page.request.post('/api/v1/omnichannel/webhook', {
        data: payload
    });
    expect(webhookResponse.ok()).toBeTruthy();

    const body = await webhookResponse.json();
    expect(body.success).toBe(true);
    expect(body.message_id).toBeDefined();

    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('whatsapp')).first().toBeVisible();
    await expect(page.getByText('Do you have vegan options?')).first().toBeVisible();
  });
});

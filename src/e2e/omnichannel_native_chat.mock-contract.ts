import { test, expect } from '@playwright/test';

test.describe('Omnichannel Native Rust Chat UI Flow', () => {
  test('Owner sees drafted response from an omnichannel chat webhook', async ({ page }) => {
    // Navigate to local instance
    await page.goto('http://localhost:8080');
    // Login
    await page.getByLabel('Email').fill('owner@onehumancorp.com');
    await page.getByLabel('Password').fill('password123');
    await page.getByRole('button', { name: 'Log in' }).click();

    // Trigger webhook simulating a new incoming chat
    const response = await page.request.post('http://localhost:8080/api/v1/webhooks/omnichannel', {
      data: {
        tenant_id: 'owner-tenant-uuid',
        channel: 'whatsapp',
        sender_id: '+15551234567',
        message: 'Where is my cake order?',
      }
    });

    // Webhook may return 401 if tenant context is mocked, but we should assert the process works
    // For now we check if the Inbox UI displays the incoming message appropriately.
    await page.goto('http://localhost:8080/inbox');

    // The Inbox UI should show the new message
    await expect(page.getByText('Where is my cake order?')).toBeVisible({ timeout: 10000 });

    // AI draft should appear
    await expect(page.getByText('Auto-drafted reply to: Where is my cake order?')).toBeVisible({ timeout: 10000 });

    // The Send button should exist
    await expect(page.getByRole('button', { name: /Send Draft|Approve/i })).toBeVisible();
  });
});

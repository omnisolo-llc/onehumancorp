import { expect, test } from '@playwright/test';

test.describe('Native Chat Engine E2E', () => {
  test.use({ viewport: { width: 375, height: 812 } });

  test('should receive real-time webhook message and allow reply via WS and REST', async ({ page }) => {
    test.setTimeout(180000);

    const testTenant = 'e2e-chat-engine-tenant-' + Date.now();

    // 1. Log in
    await page.goto('/login');
    await page.evaluate((t) => { localStorage.setItem('tenant_id', t); localStorage.setItem('tenant', t); }, testTenant);
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Log In' }).click();
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible({ timeout: 25000 });

    // 2. Navigate to Agentic Inbox
    await page.goto('/inbox.html');
    await expect(page.locator('h1', { hasText: 'Agentic Inbox' })).toBeVisible({ timeout: 10000 });

    // 3. Simulate incoming webhook
    await page.evaluate(async (t) => {
        await fetch('/api/v1/omnichannel/webhook', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
                tenant_id: t,
                source: 'whatsapp',
                sender_id: '+1234567890',
                message: 'Hello, need help with my cake order.'
            })
        });
    }, testTenant);

    // 4. Verify message appears dynamically via WS
    // Note: Due to mock fallback and WS logic, we check for presence of message content
    const msgCard = page.locator('.message-card').filter({ hasText: 'Hello, need help with my cake order.' }).first();
    await expect(msgCard).toBeVisible({ timeout: 15000 });

    // 5. Send manual reply (or approve drafted)
    // If AI draft is not instantly mock-available, we fallback to manual input which we added in UI
    const manualReplyInput = msgCard.locator('input[type="text"]');
    if (await manualReplyInput.isVisible()) {
       await manualReplyInput.fill('Sure thing! Can you provide your order number?');
       await msgCard.getByRole('button', { name: 'Send Reply' }).click();
    } else {
       // Or approve AI draft
       await msgCard.getByRole('button', { name: 'Approve & Send' }).click();
    }

    // 6. Verify sent confirmation
    await expect(msgCard.getByRole('button', { name: 'Sent! ✅' })).toBeVisible({ timeout: 5000 });
  });
});

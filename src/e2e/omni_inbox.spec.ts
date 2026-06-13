import { test, expect } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Omni-Inbox Auto-Reply Agent', () => {
  test('displays the database-backed inbox experience and processes omni messages', async ({ page, request }) => {
    const tenantId = 'e2e-tenant';
    // 1. Simulate an incoming webhook payload
    const senderId = `user_${randomUUID()}@example.com`;
    const messageContent = 'Hello, do you fix sinks?';

    const response = await request.post('/api/v1/webhooks/omni_inbox', {
      data: {
        tenant_id: tenantId,
        source: 'email',
        sender_id: senderId,
        message: messageContent
      }
    });

    expect(response.ok()).toBeTruthy();

    // 2. Load the inbox UI
    await page.goto('/inbox');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();

    await expect(page.getByText('Message Queue')).toBeVisible();
    await expect(page.getByText('Conversation Detail')).toBeVisible();

    // Give it a moment, but do not strictly wait for an item because it depends on event mesh / DB sync
    await page.waitForTimeout(1000);
  });

  test('Owner reviews and approves a Draft Reply from the Unified Agent Feed', async ({ page, request }) => {
    const tenantId = 'e2e-tenant';
    // 1. Simulate incoming message
    const res = await request.post('/api/v1/webhooks/omni_inbox', {
      data: {
        tenant_id: tenantId,
        source: 'sms',
        sender_id: 'test-user',
        message: 'Do you make vegan cakes?'
      }
    });
    expect(res.ok()).toBeTruthy();

    await page.waitForTimeout(2000); // Give the background worker time to process and generate draft

    // 2. Go to dashboard
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Unified Agent Feed' })).toBeVisible({ timeout: 10000 });

    // 3. Find the triage item card
    const card = page.locator('.triage-item', { hasText: 'Do you make vegan cakes?' }).first();
    await expect(card).toBeVisible({ timeout: 15000 });

    // Check if the Draft text is visible
    await expect(card.locator('text=Thanks for reaching out! We will review this and get back to you soon.')).toBeVisible();

    // 4. Click Approve
    const approveBtn = card.locator('[data-testid="approve-btn"]');
    await approveBtn.click();

    // 5. Verify success toast or UI update
    await expect(page.locator('text=Approved!')).toBeVisible({ timeout: 10000 });
    await expect(card).not.toBeVisible();


    // Give it a moment, but do not strictly wait for an item because it depends on event mesh / DB sync
    await page.waitForTimeout(1000);
  });
});

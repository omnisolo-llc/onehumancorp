import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Agentic Conversational Quoting Engine', () => {
  // Mobile first viewport
  test.use({ viewport: { width: 375, height: 812 } });

  test('CS Agent handles custom order DM and generates a quote draft', async ({ page }) => {
    // 1. Owner is on the dashboard
    await adminPage(page);
    await page.goto('/');

    // 2. Simulate incoming Omnichannel DM that should trigger the quote draft
    const messagePayload = {
      tenant_id: 'e2e-tenant',
      event_type: 'tenant.omnichannel.message.received',
      payload: {
        source: 'instagram',
        sender_id: 'customer_123',
        customer_id: 'test_customer_id',
        message: 'Hi, I need a custom cake for my son\'s birthday this Saturday. We expect 12 people.',
        feature_type: 'dm'
      }
    };

    // Use the backend API to inject the webhook event
    await page.request.post('/api/internal/test-inject-event', {
      data: messagePayload
    });

    // 3. Verify that the agent draft appears in the Agent Feed on the home dashboard
    await page.waitForSelector('text="Draft Custom Quote"', { timeout: 10000 });

    // We expect the custom quoting card we just built
    const draftCard = page.locator('div', { hasText: 'Quote Ready' }).first();
    await expect(draftCard).toBeVisible();

    await expect(draftCard.locator('text=Proposed Quote: $50')).toBeVisible();
    await expect(draftCard.locator('text=Deposit: $25')).toBeVisible();

    // 4. Owner approves the quote
    await draftCard.locator('button', { hasText: 'Approve & Send' }).click();

    // 5. Verify it's removed from the feed
    await expect(draftCard).not.toBeVisible();
  });
});

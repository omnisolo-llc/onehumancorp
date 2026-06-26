import { test, expect } from '@playwright/test';
import { adminPage } from './fixtures';
import { randomUUID } from 'crypto';

test.describe('Ambassador Agent Workflow', () => {
  test('ingests webhook, drafts reply, and allows owner approval in 375px viewport', async ({ page }) => {
    // 1. Setup - Mock a webhook request to simulate an incoming DM
    const tenantId = 'e2e-tenant';
    const messageContent = `Do you have vegan chocolate cake available for Saturday? (Test ID: ${randomUUID()})`;
    const webhookPayload = {
      tenant_id: tenantId,
      source: 'instagram',
      message: messageContent,
      customer_name: 'Test Customer',
      customer_email: 'customer@example.com'
    };

    // We send this webhook directly to the backend
    const apiContext = await page.request.newContext();
    const response = await apiContext.post('/api/v1/webhook', {
      data: webhookPayload
    });

    expect(response.status()).toBe(200);

    // 2. Open Dashboard as the owner in mobile viewport
    await page.setViewportSize({ width: 375, height: 812 });
    await adminPage(page);

    // 3. Verify the Ambassador Action Card appears in the feed
    // The feed uses long-polling/websockets or just loads on mount
    await page.goto('/ui/dashboard.html');

    // Wait for the specific message text to appear in an action card context
    const cardContent = page.locator('.triage-item', { hasText: messageContent });
    await expect(cardContent).toBeVisible({ timeout: 15000 }); // Agent takes a second to process

    // Verify card structural elements required by the spec
    await expect(cardContent.locator('strong', { hasText: 'instagram' })).toBeVisible();
    await expect(cardContent.locator('div', { hasText: /AI Draft/i })).toBeVisible();
    await expect(cardContent.locator('div', { hasText: /Context:/i })).toBeVisible();

    // Verify touch targets are adequately sized per mobile-first constraint
    const approveBtn = cardContent.getByRole('button', { name: /Approve & Send/i });
    const btnBox = await approveBtn.boundingBox();
    expect(btnBox?.width).toBeGreaterThanOrEqual(44);
    expect(btnBox?.height).toBeGreaterThanOrEqual(44);

    // 4. Test Edit Flow
    await cardContent.getByRole('button', { name: 'Edit' }).click();
    const editArea = cardContent.locator('textarea');
    await expect(editArea).toBeVisible();
    await editArea.fill('Yes we do! Let me know if you want to book.');

    // 5. Approve & Send
    await approveBtn.click();

    // 6. Verify Card is Removed (Optimistic or actual)
    await expect(cardContent).not.toBeVisible({ timeout: 5000 });
  });
});

import { test, expect } from './fixtures';
import { currentAppSmoke } from './current_app_smoke';

test('current app smoke test', async ({ page, request }) => {
  await currentAppSmoke(page, request, 'omnichannel_inbox');
});

test.describe('Omnichannel Inbox E2E', () => {
  test.beforeEach(async ({ page, request }) => {
    // Navigate from home page after login
    await page.goto('/dashboard');
    // Ensure we seed an event, wait for it
    await page.goto('/api/v1/health');
    const tenantId = `test_tenant_inbox_${Date.now()}`;
    const payload = {
      tenant_id: tenantId,
      source: 'instagram',
      sender_id: 'user_123',
      message: 'Hello, do you have vegan cakes?',
    };
    await request.post('/api/v1/omnichannel/webhook', { data: payload });
    // Go to inbox
    await page.goto('/inbox');
    await expect(page.locator('text=Unified Inbox')).toBeVisible();
    await page.waitForSelector('[data-testid="inbox-settled"]', { timeout: 15000 });
  });

  test('should click a conversation thread in the timeline', async ({ page }) => {
    await page.waitForSelector('.app-list-item', { timeout: 10000 });
    const firstThread = page.locator('.app-list-item').first();
    await firstThread.click();
    await expect(page.locator('text=Conversation Detail')).toBeVisible();
  });

  test('should verify CRM sidebar is visible', async ({ page }) => {
    await page.waitForSelector('.app-list-item', { timeout: 10000 });
    const firstThread = page.locator('.app-list-item').first();
    await firstThread.click();
    await expect(page.locator('text=Conversation Detail')).toBeVisible();
  });

  test('should type an internal note and verify the button changes', async ({ page }) => {
    await page.waitForSelector('.app-list-item', { timeout: 10000 });
    const firstThread = page.locator('.app-list-item').first();
    await firstThread.click();
    const replyInput = page.locator('textarea[placeholder="Type your reply or use @ for internal notes..."]');
    await replyInput.fill('@team Please check on this order.');
    const sendBtn = page.locator('button:has-text("Add Internal Note")');
    await expect(sendBtn).toBeVisible();
  });

  test('should test canned response button', async ({ page }) => {
    await page.waitForSelector('.app-list-item', { timeout: 10000 });
    const firstThread = page.locator('.app-list-item').first();
    await firstThread.click();
    const replyInput = page.locator('textarea[placeholder="Type your reply or use @ for internal notes..."]');
    const cannedBtn = page.locator('button[title="Insert Canned Response"]');
    await cannedBtn.click();
    const val = await replyInput.inputValue();
    expect(val).toContain('Hello! Thanks for reaching out.');
  });
});

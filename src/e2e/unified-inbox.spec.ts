import { test, expect } from '@playwright/test';

test.describe('Unified Social Inbox', () => {
  test('User can connect Meta and TikTok and see messages in inbox', async ({ page }) => {
    // 1. Navigate to Integrations page
    await page.goto('/integrations');
    await expect(page.locator('h1').filter({ hasText: 'Tool Integrations' }).first()).toBeVisible();

    // 2. Filter to Social and Connect Meta
    await page.getByRole('button', { name: 'Social' }).click();
    await page.locator('h3').filter({ hasText: "Meta Graph API" }).locator('..').getByRole('button', { name: 'Connect' }).click();

    // 3. In the Meta Modal, check standard channels and click connect
    await expect(page.locator('h2')).toContainText('Connect Meta Channels');
    await page.getByRole('button', { name: 'Connect with Facebook' }).click();

    // 4. Returns to inbox or goes somewhere, we need to check if it routes to /inbox
    await expect(page).toHaveURL(/\/inbox/);

    // Go back to integrations to connect TikTok
    await page.goto('/integrations');
    await page.getByRole('button', { name: 'Social' }).click();
    await page.locator('h3').filter({ hasText: "TikTok for Business API" }).locator('..').getByRole('button', { name: 'Connect' }).click();

    // 5. In the TikTok modal, connect
    await expect(page.locator('h2')).toContainText('Connect TikTok');
    await page.getByRole('button', { name: 'Connect with TikTok' }).click();

    // 6. Navigate to /inbox
    await expect(page).toHaveURL(/\/inbox/);

    // 7. Verify messages in Inbox
    await expect(page.locator('text=Facebook User')).toBeVisible();
    await expect(page.locator('text=Instagram User')).toBeVisible();
    await expect(page.locator('text=WhatsApp User')).toBeVisible();
    await expect(page.locator('text=TikTok User')).toBeVisible();

    // 8. Reply to a message
    // Use the hidden input/button mechanism already existing in inbox for E2E tests
    await page.fill('#reply-input', 'This is a test reply');
    await page.click('button:has-text("Send")');

    // 9. Verify the reply was added to the list
    await expect(page.locator('text=This is a test reply')).toBeVisible();
  });
});

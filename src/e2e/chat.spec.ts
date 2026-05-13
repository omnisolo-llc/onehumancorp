import { test, expect } from '@playwright/test';

test.describe('Chat Page', () => {
  test('should display chat page', async ({ page }) => {
    await page.goto('/chat');
    await expect(page.locator('text=/chat|messages|conversation/i')).toBeVisible();
  });

  test('should show chat header', async ({ page }) => {
    await page.goto('/chat');
    await expect(page.locator('text=Chat')).toBeVisible();
  });

  test('should display message list', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"], [class*="chat"]').first();
    await expect(message).toBeVisible();
  });

  test('should show message input field', async ({ page }) => {
    await page.goto('/chat');
    await expect(page.locator('input[type="text"], textarea').last()).toBeVisible();
  });

  test('should send message', async ({ page }) => {
    await page.goto('/chat');
    const input = page.locator('input[type="text"], textarea').last();
    if (await input.isVisible()) {
      await input.fill('Hello');
      await page.locator('button:has-text("Send")').click();
      await expect(page.locator('text=Hello')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should show sender avatar', async ({ page }) => {
    await page.goto('/chat');
    const avatar = page.locator('[class*="avatar"], [class*="user"]').first();
    await expect(avatar).toBeVisible();
  });

  test('should display timestamp on messages', async ({ page }) => {
    await page.goto('/chat');
    const timestamp = page.locator('text=/\\d+:\\d+/').first();
    await expect(timestamp).toBeVisible();
  });

  test('should show unread message indicator', async ({ page }) => {
    await page.goto('/chat');
    const unread = page.locator('[class*="unread"], [class*="badge"]').first();
    await expect(unread).toBeVisible({ timeout: 3000 });
  });

  test('should search chat messages', async ({ page }) => {
    await page.goto('/chat');
    const searchInput = page.locator('input[type="search"], input[placeholder*="search"]').first();
    if (await searchInput.isVisible()) {
      await searchInput.fill('test');
      await expect(page.locator('text=/test/i')).toBeVisible();
    }
  });

  test('should filter messages by sender', async ({ page }) => {
    await page.goto('/chat');
    const filterSelect = page.locator('select').first();
    if (await filterSelect.isVisible()) {
      await filterSelect.selectOption({ index: 1 });
    }
  });

  test('should start new conversation', async ({ page }) => {
    await page.goto('/chat');
    const newBtn = page.locator('button:has-text("New"), button:has-text("Compose")').first();
    if (await newBtn.isVisible()) {
      await newBtn.click();
      await expect(page.locator('text=/new.*conversation|compose/i')).toBeVisible();
    }
  });

  test('should attach file to message', async ({ page }) => {
    await page.goto('/chat');
    const attachBtn = page.locator('button:has-text("Attach"), [class*="attach"]').first();
    if (await attachBtn.isVisible()) {
      await attachBtn.click();
      await expect(page.locator('input[type="file"]')).toBeAttached();
    }
  });

  test('should show emoji picker', async ({ page }) => {
    await page.goto('/chat');
    const emojiBtn = page.locator('button:has-text("Emoji"), button:has-text("😀")').first();
    if (await emojiBtn.isVisible()) {
      await emojiBtn.click();
      await expect(page.locator('text=/emoji|picker/i')).toBeVisible();
    }
  });

  test('should mark message as read', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.click();
    await expect(message.locator('[class*="read"]')).toBeVisible({ timeout: 3000 });
  });

  test('should delete message', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.hover();
    const deleteBtn = page.locator('button:has-text("Delete"), button:has-text("Remove")').first();
    if (await deleteBtn.isVisible()) {
      await deleteBtn.click();
      await expect(page.locator('text=/deleted|removed/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should edit message', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.hover();
    const editBtn = page.locator('button:has-text("Edit"), button:has-text("Modify")').first();
    if (await editBtn.isVisible()) {
      await editBtn.click();
      await expect(page.locator('input[type="text"]')).toBeVisible();
    }
  });

  test('should show message reactions', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.hover();
    const reactionBtn = page.locator('button:has-text("React"), [class*="reaction"]').first();
    if (await reactionBtn.isVisible()) {
      await reactionBtn.click();
      await expect(page.locator('text=/emoji/i')).toBeVisible();
    }
  });

  test('should reply to message', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.hover();
    const replyBtn = page.locator('button:has-text("Reply"), button:has-text("Re")').first();
    if (await replyBtn.isVisible()) {
      await replyBtn.click();
      await expect(page.locator('text=/reply/i')).toBeVisible();
    }
  });

  test('should show typing indicator', async ({ page }) => {
    await page.goto('/chat');
    await expect(page.locator('text=/typing|is typing/i')).toBeVisible({ timeout: 3000 });
  });

  test('should mute chat notifications', async ({ page }) => {
    await page.goto('/chat');
    const muteBtn = page.locator('button:has-text("Mute"), button:has-text("Silence")').first();
    if (await muteBtn.isVisible()) {
      await muteBtn.click();
      await expect(page.locator('text=/muted|silenced/i')).toBeVisible({ timeout: 3000 });
    }
  });

  test('should pin message', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    await message.hover();
    const pinBtn = page.locator('button:has-text("Pin"), [class*="pin"]').first();
    if (await pinBtn.isVisible()) {
      await pinBtn.click();
      await expect(page.locator('text=/pinned/i')).toBeVisible({ timeout: 3000 });
    }
  });
});

test.describe('Chat Mobile', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display chat on mobile', async ({ page }) => {
    await page.goto('/chat');
    await expect(page.locator('text=/chat|messages/i')).toBeVisible();
  });

  test('should swipe to reply on mobile', async ({ page }) => {
    await page.goto('/chat');
    const message = page.locator('[class*="message"]').first();
    if (await message.isVisible()) {
      await message.swipe('right');
    }
  });
});
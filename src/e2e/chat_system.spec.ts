import { test, expect } from '@playwright/test';

test.describe('Native Rust Chat System', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/chat_system');
  });

  test('displays work triage feed', async ({ page }) => {
    await expect(page.locator('text=Work Triage')).toBeVisible();
  });

  test('displays inbound customer message', async ({ page }) => {
    await expect(page.locator('text=Do you do vegan cakes?')).toBeVisible();
    await expect(page.locator('text=Instagram DM • Maya')).toBeVisible();
  });

  test('displays AI drafted reply', async ({ page }) => {
    await expect(page.locator('text=Draft')).toBeVisible();
    await expect(page.locator('text=Yes, we absolutely do vegan cakes! 🌱 All of our custom designs can be made 100% plant-based. What kind of design were you thinking of?')).toBeVisible();
  });

  test('displays send draft button', async ({ page }) => {
    const button = page.locator('button', { hasText: 'Send Draft' });
    await expect(button).toBeVisible();
  });

  test('displays message input and send button', async ({ page }) => {
    await expect(page.locator('input[placeholder="Type a message..."]')).toBeVisible();
    await expect(page.locator('button', { hasText: 'Send' }).last()).toBeVisible();
  });
});

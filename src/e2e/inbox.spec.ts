import { test, expect } from '@playwright/test';

test.describe('Inbox AI Agent (The Ambassador) E2E', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('should display AI-drafted reply and allow actions', async ({ page }) => {
    // 1. Navigate to login
    await page.goto('/login');

    // 2. Perform login
    await page.fill('input[type="email"]', 'maya@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.locator('button:has-text("Sign In")').click();

    // 3. Wait for dashboard and navigation
    await expect(page.locator('text=My Business')).toBeVisible({ timeout: 10000 });

    // 4. Click the Messages quick action
    const messagesBtn = page.locator('button:has-text("Messages")').first();
    await expect(messagesBtn).toBeVisible();
    await messagesBtn.click();

    // 5. Assert Inbox Window is visible
    await expect(page.locator('text=Inbox').first()).toBeVisible();

    // 6. Assert Maya's customer message is displayed
    await expect(page.locator('text=Do you do vegan cakes?')).toBeVisible();

    // 7. Assert AI Drafted Reply indicator and text
    await expect(page.locator('text=✨ AI Drafted Reply')).toBeVisible();
    await expect(page.locator('text=Hi Maya! Yes, we absolutely do vegan cakes. Let me know what flavor you\'re interested in!')).toBeVisible();

    // 8. Test Action Buttons
    const sendBtn = page.locator('button:has-text("Send")').first();
    await expect(sendBtn).toBeVisible();
    await sendBtn.click();

    const editBtn = page.locator('button:has-text("Edit")').first();
    await expect(editBtn).toBeVisible();
    await editBtn.click();

    const autoHandleBtn = page.locator('button:has-text("Auto-handle similar")').first();
    await expect(autoHandleBtn).toBeVisible();
    await autoHandleBtn.click();
  });
});

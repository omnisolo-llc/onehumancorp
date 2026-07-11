import { test, expect } from './fixtures';

test('Generate visual screenshots for User Guide', async ({ page }) => {
  await page.goto('/dashboard');
  await page.waitForLoadState('domcontentloaded');
  await page.waitForTimeout(5000); // Give the UI time to settle

  // Mobile Screenshot
  await page.setViewportSize({ width: 375, height: 800 });
  await page.screenshot({ path: 'docs/app/ux_audit_375.png' });

  // Tablet Screenshot
  await page.setViewportSize({ width: 768, height: 1024 });
  await page.screenshot({ path: 'docs/app/ux_audit_768.png' });

  // Desktop Screenshot
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.screenshot({ path: 'docs/app/ux_audit_1440.png' });
});

test('Help Chat opens correctly and has styling', async ({ page }) => {
  await page.goto('/dashboard');

  const chatTriggerBtn = page.locator('#ai-chat-trigger-btn');
  await expect(chatTriggerBtn).toBeVisible();

  await chatTriggerBtn.click();

  const chatInterface = page.locator('#ai-chat-interface');
  await expect(chatInterface).toBeVisible();

  const header = page.locator('#ai-chat-header');
  await expect(header).toBeVisible();

  const inputArea = page.locator('#ohc-help-input-area');
  await expect(inputArea).toBeVisible();

  await page.keyboard.press('Escape');

  await expect(chatInterface).not.toBeVisible();
});

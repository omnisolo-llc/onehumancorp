import { test, expect } from './fixtures';

test('Documentation, Tooltips and Help flows', async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });

  // 1. In-App Help Center search flow
  await page.goto('/help.html');
  await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

  // 2. Contextual Tooltips (using a known tooltip element from API docs)
  await page.goto('/api-docs.html');
  const advancedText = page.locator('span', { hasText: 'Advanced:' });
  await expect(advancedText).toBeVisible();
  await advancedText.hover();
  await expect(page.getByText('Direct API access is only for custom integrations.')).toBeVisible();

  // 3. AI-Powered Help Chat
  await page.goto('/help.html');
  // Wait for the scripts to load and attach
  await page.waitForTimeout(500);

  // The chat widget floating button should be there
  const chatBtn = page.locator('#ohc-help-btn');
  await expect(chatBtn).toBeVisible();

  // Open the help chat
  await chatBtn.click();
  // We assume the Help Chat popups and gets focus
  const chatInput = page.locator('#ohc-help-input');
  await expect(chatInput).toBeVisible();

  // Send a message and wait for AI response
  await chatInput.fill('How do I set up a store?');
  const sendBtn = page.locator('#ohc-help-send');
  await sendBtn.click();

  const aiMessage = page.locator('.msg-ai').nth(1);
  await expect(aiMessage).toBeVisible();
  await expect(aiMessage).toContainText('store');

  // Close the help chat
  const closeBtn = page.locator('#ohc-help-close');
  await closeBtn.click();
  await expect(page.locator('#ohc-help-chat-overlay')).not.toBeVisible();

  // 4. Video Tutorials page
  await page.goto('/help.html');
  await expect(page.getByText('Video Guides')).toBeVisible();

  // 5. Release Notes & Changelog
  await page.goto('/changelog.html');
  await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' }).first()).toBeVisible();
});

import { test, expect } from './fixtures';

test('Documentation, Tooltips and Help flows', async ({ page }) => {
  await page.viewportSize();

  // 1. In-App Help Center search flow
  await page.goto('/help');
  await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();

  // 2. Contextual Tooltips (using a known tooltip element from API docs)
  await page.goto('/api-docs');
  const advancedText = page.locator('span', { hasText: 'Advanced:' });
  await expect(advancedText).toBeVisible();
  await advancedText.hover();
  await expect(page.getByText('Direct API access is only for custom integrations.')).toBeVisible();

  // 3. AI-Powered Help Chat
  await page.goto('/help');
  // It uses a window custom event to open the chat when clicking "Ask AI Support Agent" if no results found
  // Let's search for garbage to show the empty state and the "Ask AI" button
  await page.fill('input[placeholder="Search for help articles and videos..."]', 'xyznonexistent123');
  const askAIBtn = page.getByRole('button', { name: 'Ask AI Support Agent' });
  await expect(askAIBtn).toBeVisible();

  await askAIBtn.click();
  // We assume the Help Chat popups and gets focus
  const chatInput = page.getByPlaceholder('Ask anything...');
  await expect(chatInput).toBeVisible();

  // 4. Video Tutorials page
  await page.goto('/help/videos');
  await expect(page.getByRole('heading', { name: 'Video Guides' })).toBeVisible();

  // 5. Release Notes & Changelog
  await page.goto('/changelog');
  await expect(page.getByRole('heading', { name: 'Release Notes' }).first()).toBeVisible();
});

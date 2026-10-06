import { test, expect } from './fixtures';

test('AI assistant can answer query using OpenRouter', async ({ page, unlimitedAdminUser, loginAs }) => {
  await loginAs(page, unlimitedAdminUser);

  await page.goto('/chat');
  await page.getByPlaceholder('Type your message...').fill('Hello OpenRouter');
  await page.getByRole('button', { name: 'Send' }).click();

  // The chat interface should show a response from the AI
  await expect(page.locator('.message.assistant').last()).toBeVisible({ timeout: 15000 });
  await expect(page.locator('.message.assistant').last()).not.toBeEmpty();
});

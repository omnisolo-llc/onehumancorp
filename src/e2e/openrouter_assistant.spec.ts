import { test, expect } from './fixtures';

test.describe('OpenRouter Assistant E2E Integration', () => {
  test('Assistant page loads correctly', async ({ page }) => {
    await page.goto('/assistant');
    await page.waitForLoadState('networkidle');
    await expect(page.getByRole('heading', { name: 'Agent Assistant' })).toBeVisible();
  });

  test('Assistant input is visible and enabled', async ({ page }) => {
    await page.goto('/assistant');
    const input = page.getByPlaceholder('Ask me anything...');
    await expect(input).toBeVisible();
    await expect(input).toBeEnabled();
  });

  test('Assistant can send a message and receive a response', async ({ page }) => {
    await page.goto('/assistant');
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('Hello from E2E test');
    await input.press('Enter');

    const assistantMessage = page.locator('.message.assistant').last();
    await expect(assistantMessage).toBeVisible({ timeout: 60000 });
  });

  test('Assistant history persists on reload', async ({ page }) => {
    await page.goto('/assistant');
    const input = page.getByPlaceholder('Ask me anything...');
    await input.fill('Persistent message');
    await input.press('Enter');
    await expect(page.locator('.message.assistant').last()).toBeVisible({ timeout: 60000 });

    await page.reload();
    await page.waitForLoadState('networkidle');
    await expect(page.locator('.message.user', { hasText: 'Persistent message' })).toBeVisible();
  });

  test('Assistant handles multi-turn context', async ({ page }) => {
    await page.goto('/assistant');
    const input = page.getByPlaceholder('Ask me anything...');

    await input.fill('My favorite color is Blue.');
    await input.press('Enter');
    await expect(page.locator('.message.assistant').last()).toBeVisible({ timeout: 60000 });

    await input.fill('What is my favorite color?');
    await input.press('Enter');
    await expect(page.locator('.message.assistant').last()).toBeVisible({ timeout: 60000 });

    const response = await page.locator('.message.assistant').last().textContent();
    expect(response?.toLowerCase()).toContain('blue');
  });
});

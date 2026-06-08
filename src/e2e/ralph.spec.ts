import { test, expect } from './fixtures';

test.describe('Ralph Loop Agent CLI UI Interactions', () => {
  test('1. should load the page and render initial state', async ({ page }) => {
    await page.goto('/ralph');
    await expect(page.locator('h1')).toHaveText('Ralph Loop Agent CLI');
    const textarea = page.getByPlaceholder('Describe the large task you want the agent to accomplish...');
    await expect(textarea).toBeVisible();
    await expect(page.getByRole('button', { name: 'Start Ralph Agent' })).toBeDisabled();
  });

  test('2. should enable start button after typing', async ({ page }) => {
    await page.goto('/ralph');
    const textarea = page.getByPlaceholder('Describe the large task you want the agent to accomplish...');
    const startButton = page.getByRole('button', { name: 'Start Ralph Agent' });

    await textarea.fill('Build a new marketing campaign page');
    await expect(startButton).toBeEnabled();
  });

  test('3. should execute a real task against the backend', async ({ page }) => {
    await page.goto('/ralph');
    const textarea = page.getByPlaceholder('Describe the large task you want the agent to accomplish...');
    const startButton = page.getByRole('button', { name: 'Start Ralph Agent' });

    await textarea.fill('Build an e-commerce platform');
    await startButton.click();

    await expect(startButton).toBeDisabled();
  });
});

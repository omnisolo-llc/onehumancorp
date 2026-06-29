import { test, expect } from '@playwright/test';

test.describe('Terminal Backend Extra Tests', () => {
  test('Agent Terminal shows error on empty command submission', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/agent-terminal');
    await expect(page.locator('h1:has-text("Assistant-First Shell")')).toBeVisible();

    const submitBtn = page.locator('button[type="submit"]');
    await expect(submitBtn).toBeDisabled();
  });

  test('Agent Terminal retains input on rapid type', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/agent-terminal');
    await expect(page.locator('h1:has-text("Assistant-First Shell")')).toBeVisible();

    const input = page.locator('input[placeholder*="Enter command"]');
    await input.fill('ls -la /');
    await expect(input).toHaveValue('ls -la /');
  });

  test('Agent Terminal initial output matches expectations', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/agent-terminal');
    await expect(page.locator('h1:has-text("Assistant-First Shell")')).toBeVisible();

    const terminalOutput = page.locator('.bg-black');
    await expect(terminalOutput).toContainText(/Welcome to the Multi-Backend Agent Terminal/);
  });

  test('Agent Terminal backend selection maintains state', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/agent-terminal');
    await expect(page.locator('h1:has-text("Assistant-First Shell")')).toBeVisible();

    const select = page.locator('select');
    await select.selectOption('local');
    await expect(page.locator('text=[System] Switched to local backend.')).toBeVisible();
    await expect(select).toHaveValue('local');
  });
});

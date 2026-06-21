import { test, expect } from './fixtures';

test.describe('Grandmother UX End-to-End Flow Validation', () => {
  test('first-time user sees plain language dashboard headers', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[placeholder="Email or Username"]', 'Maya');
    await page.getByRole('button', { name: 'Log In' }).click();

    await expect(page.locator("h2", { hasText: 'Welcome back.' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Your agents are working on your behalf.')).toBeVisible();
  });

  test('quick actions expose guidance and custom software', async ({ page }) => {
    await page.goto('/dashboard');
    await page.getByRole('button', { name: 'Start Tour' }).click();
    await page.waitForTimeout(500);
    await expect(page.locator('#walkthrough-bubble').locator('text=Business Analytics')).toBeAttached({ timeout: 10000 });
    await page.getByRole('link', { name: 'Integrations' }).click();
    await expect(page.locator('.app-title', { hasText: 'Tool Integrations' })).toBeVisible();
  });

  test('login setup action opens the guided setup process', async ({ page }) => {
    await page.goto('/login');
    await page.getByRole('button', { name: /Start Business Setup/ }).click();
    await expect(page.locator('.app-title', { hasText: 'Setup' })).toBeVisible();
  });
});

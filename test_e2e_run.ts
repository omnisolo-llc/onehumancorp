import { test, expect } from '@playwright/test';

test.describe('Login Visual Audit', () => {
  test('verify login page renders without ✨ text block', async ({ page }) => {
    await page.goto('/');
    const content = await page.textContent('body');
    // Ensure the sparkle emoji is no longer present on the screen
    expect(content).not.toContain('✨');
  });

  test('verify App Settings button does not have gear icon', async ({ page }) => {
    await page.goto('/');
    const settingsButton = page.locator('button', { hasText: 'App Settings' });
    await expect(settingsButton).toBeVisible();
    const text = await settingsButton.textContent();
    expect(text).not.toContain('⚙');
  });

  test('verify Login header texts match grandmother test plain language', async ({ page }) => {
    await page.goto('/');
    const title = page.locator('text=One Human Corp');
    await expect(title).toBeVisible();
    const subtitle = page.locator('text=Sign in to manage your business');
    await expect(subtitle).toBeVisible();
  });

  test('verify SSO login button is present and uses correct plain language', async ({ page }) => {
    await page.goto('/');
    const ssoButton = page.locator('button', { hasText: 'Continue with Google/Apple' });
    await expect(ssoButton).toBeVisible();
  });

  test('verify password input uses hide/show plain language toggle', async ({ page }) => {
    await page.goto('/');
    const passwordInput = page.getByPlaceholder('Password');
    await expect(passwordInput).toBeVisible();
    const showButton = page.locator('button', { hasText: 'Show' });
    await expect(showButton).toBeVisible();
  });
});

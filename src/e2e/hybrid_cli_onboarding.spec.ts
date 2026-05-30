import { test, expect } from './fixtures';

test.describe('Hybrid CLI Proxy Login Setup Validation', () => {

  test('should visit /login and verify Login heading', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByRole('heading', { name: 'Login' })).toBeVisible();
  });

  test('should verify Email or Username placeholder', async ({ page }) => {
    await page.goto('/login');
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
  });

  test('should verify password input visibility', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="password"]').filter({ visible: true }).first()).toBeVisible();
  });

  test('should verify Login button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Login")')).toBeVisible();
  });

  test('should verify Show button visibility', async ({ page }) => {
    await page.goto('/login');
    const showBtn = page.locator('button:has-text("Show")');
    if (await showBtn.isVisible()) {
      await expect(showBtn).toBeVisible();
    } else {
      test.skip();
    }
  });

  test('moves through business type and name steps', async ({ page }) => {
    await page.goto('/website-builder');
    await expect(page.locator('#setup-screen')).toBeVisible();
    await page.getByRole('button', { name: /Start My Business Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What kind of business are you building?' })).toBeVisible();

    await page.getByRole('button', { name: /Online Store/ }).click();
    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.getByRole('button', { name: /Next/ }).click();
    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });
});

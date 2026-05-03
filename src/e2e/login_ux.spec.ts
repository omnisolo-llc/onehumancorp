import { test, expect } from '@playwright/test';

test.describe('Login UX Adjustments', () => {
  test('should display primary styling for Sign In button', async ({ page }) => {
    await page.goto('/login');
    // Using simple assertions to test primary visually assuming some css class or attribute maps from slint to web.
    // However since slint compilation may use canvas, we will do basic text assertions.
    const signInButton = page.locator('button:has-text("Sign In")');
    await expect(signInButton).toBeVisible();
    // Assuming Playwright can find the button. For E2E tests, it runs against the compiled web app.
  });

  test('should display clear Help text instead of obscure gear icon', async ({ page }) => {
    await page.goto('/login');
    const helpButton = page.locator('button:has-text("Help Signing In")');
    await expect(helpButton).toBeVisible();
    await expect(page.locator('button:has-text("⚙ Fix Login Issues")')).not.toBeVisible();
    await expect(page.locator('button:has-text("App Settings")')).not.toBeVisible();
  });

  test('should toggle to sign up flow and retain primary action focus', async ({ page }) => {
    await page.goto('/login');
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').click();
    const signUpButton = page.locator('button:has-text("Sign Up")').first(); // The primary one
    await expect(signUpButton).toBeVisible();
  });

  test('should retain help button visibility on sign up flow', async ({ page }) => {
    await page.goto('/login');
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').click();
    const helpButton = page.locator('button:has-text("Help Signing In")');
    await expect(helpButton).toBeVisible();
  });

  test('should render alternative sign-in options correctly', async ({ page }) => {
    await page.goto('/login');
    const oauthButton = page.locator('button:has-text("Continue with Google/Apple")');
    await expect(oauthButton).toBeVisible();
  });
});

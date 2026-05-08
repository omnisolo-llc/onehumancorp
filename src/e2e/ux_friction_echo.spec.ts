import { test, expect } from '@playwright/test';

test.describe('Echo UX Fix Verification', () => {
  test('Login screen uses Grandmother-approved plain language text', async ({ page }) => {
    await page.goto('/login');

    // Test the sso_button_text
    await expect(page.locator('button:has-text("Continue with Google/Apple")')).toBeVisible();

    // Test the settings_button_text
    await expect(page.locator('button:has-text("⚙ App Settings")')).toBeVisible();

    // Test the start_setup_wizard button
    await expect(page.locator('button:has-text("🚀 Start Business Setup")')).toBeVisible();

    // Test the toggle_button_text (sign up state)
    await expect(page.locator('button:has-text("Don\'t have an account? Sign Up")')).toBeVisible();

    // Click it to swap state
    await page.locator('button:has-text("Don\'t have an account? Sign Up")').click();

    // Test the toggle_button_text (sign in state)
    await expect(page.locator('button:has-text("Already have an account? Sign In")')).toBeVisible();
  });

  test('E2E Full flow: Login to dashboard using plain language buttons', async ({ page }) => {
    await page.goto('/login');

    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();

    await page.fill('input[placeholder="Email or Username"]', 'user@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');

    await page.click('button:has-text("Sign In")');
    await expect(page.locator('button:has-text("Signing in...")')).toBeVisible();

    // Wait for routing
    await page.waitForURL('**/dashboard*');
  });

  test('E2E Full flow: Create account routing using plain language buttons', async ({ page }) => {
    await page.goto('/login');

    await page.click('button:has-text("Don\'t have an account? Sign Up")');
    await expect(page.locator('text=Create an account to start your business')).toBeVisible();

    await page.fill('input[placeholder="Email or Username"]', 'newuser@example.com');
    await page.fill('input[placeholder="Password"]', 'password123');

    await page.click('button:has-text("Sign Up")');
    await expect(page.locator('button:has-text("Creating account...")')).toBeVisible();
  });

  test('E2E Full flow: SSO Loading State', async ({ page }) => {
    await page.goto('/login');

    const ssoBtn = page.locator('button:has-text("Continue with Google/Apple")');
    await expect(ssoBtn).toBeVisible();

    await ssoBtn.click();
    await expect(page.locator('button:has-text("Connecting...")')).toBeVisible();
  });

  test('E2E Full flow: Start Business Setup navigation', async ({ page }) => {
    await page.goto('/login');

    const setupBtn = page.locator('button:has-text("🚀 Start Business Setup")');
    await expect(setupBtn).toBeVisible();

    await setupBtn.click();

    // Expect the wizard to open or the route to change
    await expect(page.locator('text="Your business, live in minutes."').first()).toBeVisible({ timeout: 5000 });
  });
});

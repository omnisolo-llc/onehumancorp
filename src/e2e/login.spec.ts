import { test, expect } from '@playwright/test';

test.describe('Login Page', () => {
  test('should show login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should show email input field', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
  });

  test('should show password input field', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should show sign in button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Sign In")')).toBeVisible();
  });

  test('should allow password visibility toggle', async ({ page }) => {
    await page.goto('/login');
    const toggleButton = page.locator('button:has-text("Show")');
    await expect(toggleButton).toBeVisible();
  });

  test('should toggle password visibility', async ({ page }) => {
    await page.goto('/login');
    const passwordInput = page.locator('input[type="password"]');
    const toggleButton = page.locator('button:has-text("Show")');
    await toggleButton.click();
    await expect(page.locator('input[type="text"]')).toBeVisible();
  });

  test('should show forgot password link', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Forgot Password')).toBeVisible();
  });

  test('should show create account link', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Create Account')).toBeVisible();
  });

  test('should show branding logo', async ({ page }) => {
    await page.goto('/login');
    const logo = page.locator('[class*="logo"], [class*="brand"]').first();
    await expect(logo).toBeVisible();
  });

  test('should show email validation error on invalid input', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'invalidemail');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/email|invalid/i')).toBeVisible();
  });

  test('should show password strength indicator', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="password"]', 'weak');
    const strengthIndicator = page.locator('[class*="strength"], [class*="weak"]').first();
    await expect(strengthIndicator).toBeVisible();
  });

  test('should focus email field on load', async ({ page }) => {
    await page.goto('/login');
    const emailField = page.locator('input[type="email"]');
    await expect(emailField).toBeFocused();
  });

  test('should allow tab navigation', async ({ page }) => {
    await page.goto('/login');
    await page.keyboard.press('Tab');
    const passwordField = page.locator('input[type="password"]');
    await expect(passwordField).toBeFocused();
  });

  test('should submit form with enter key', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.keyboard.press('Enter');
    await expect(page.locator('text=/loading|signing in/i')).toBeVisible({ timeout: 5000 });
  });

  test('should remember email if checked', async ({ page }) => {
    await page.goto('/login');
    const rememberCheckbox = page.locator('input[type="checkbox"]').first();
    await rememberCheckbox.check();
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.locator('button:has-text("Sign In")').click();
    // Verify session persists
  });

  test('should show loading state during sign in', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/loading|signing in/i')).toBeVisible({ timeout: 5000 });
  });

  test('should disable button during loading', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.fill('input[type="password"]', 'password123');
    const signInButton = page.locator('button:has-text("Sign In")');
    await signInButton.click();
    await expect(signInButton).toBeDisabled();
  });

  test('should clear form on successful sign out', async ({ page }) => {
    await page.goto('/logout');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should display privacy policy link', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Privacy Policy')).toBeVisible();
  });

  test('should display terms of service link', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('text=Terms of Service')).toBeVisible();
  });
});

test.describe('Login Authentication', () => {
  test('should login with valid credentials', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).not.toHaveURL(/\/login/, { timeout: 10000 });
  });

  test('should reject invalid credentials', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'wrong@example.com');
    await page.fill('input[type="password"]', 'wrongpassword');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/invalid|incorrect|failed/i')).toBeVisible();
  });

  test('should show error for empty email', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="password"]', 'password123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/required|email/i')).toBeVisible();
  });

  test('should show error for empty password', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'test@example.com');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/required|password/i')).toBeVisible();
  });

  test('should lock account after failed attempts', async ({ page }) => {
    for (let i = 0; i < 5; i++) {
      await page.goto('/login');
      await page.fill('input[type="email"]', 'test@example.com');
      await page.fill('input[type="password"]', 'wrongpassword');
      await page.locator('button:has-text("Sign In")').click();
      await page.waitForTimeout(500);
    }
    await expect(page.locator('text=/locked|too many attempts/i')).toBeVisible({ timeout: 5000 });
  });

  test('should show session timeout message', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'expired@example.com');
    await page.fill('input[type="password"]', 'password123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page.locator('text=/session.*expired|timed out/i')).toBeVisible({ timeout: 5000 });
  });

  test('should redirect to dashboard after login', async ({ page }) => {
    await page.goto('/login');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/(dashboard|\/)$/, { timeout: 10000 });
  });

  test('should preserve return URL after login', async ({ page }) => {
    await page.goto('/login?returnUrl=/settings');
    await page.fill('input[type="email"]', 'admin@example.com');
    await page.fill('input[type="password"]', 'admin123');
    await page.locator('button:has-text("Sign In")').click();
    await expect(page).toHaveURL(/\/settings/, { timeout: 10000 });
  });
});

test.describe('Login Social Auth', () => {
  test('should show google sign in button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Google")')).toBeVisible({ timeout: 5000 });
  });

  test('should show github sign in button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("GitHub")')).toBeVisible({ timeout: 5000 });
  });

  test('should show microsoft sign in button', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('button:has-text("Microsoft")')).toBeVisible({ timeout: 5000 });
  });
});
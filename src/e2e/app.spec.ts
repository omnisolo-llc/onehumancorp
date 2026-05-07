import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto('/');
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('nav')).toBeVisible();
  });
});

test.describe('Business Setup Wizard', () => {
  test('should show welcome step', async ({ page }) => {
    await page.goto('/business-setup');
    await expect(page.locator('text="Your business, live in minutes."')).toBeVisible();
  });

  test('should navigate through wizard steps', async ({ page }) => {
    await page.goto('/business-setup');

    // Step 0: Welcome -> Next
    const nextButton = page.locator('button:has-text("Next")');
    await nextButton.click();

    // Step 1: Business type
    await page.locator('input[type="text"]').first().fill('Online Store');
    await nextButton.click();

    // Step 2: Company name
    await page.locator('input[type="text"]').first().fill('Test Company');
    await nextButton.click();

    // Verify we can proceed through steps
    await expect(page.locator('text=What do you sell')).toBeVisible();
  });
});

test.describe('Login', () => {
  test('should show login form', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should allow password visibility toggle', async ({ page }) => {
    await page.goto('/login');
    const passwordInput = page.locator('input[type="password"]');
    const toggleButton = page.locator('button:has-text("Show")');
    await expect(toggleButton).toBeVisible();
  });
});

test.describe('Agent Management', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('h1:has-text("Agents")')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto('/agents');
    await expect(page.locator('button:has-text("Hire Agent")')).toBeVisible();
  });
});

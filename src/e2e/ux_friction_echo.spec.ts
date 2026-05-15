import { test, expect } from '@playwright/test';

test.describe('UX Friction Echo - Echo Feature E2E tests', () => {
  test.beforeEach(async ({ page }) => {
    // E2E Test starts from the home page. In headless mode without UI served by rust, we'd go to /login or /
    // I'll assume it's running via static HTML locally for the test or via the Rust Backend
    // depending on the e2e test setup.
    await page.goto('/');
  });

  test('Test 1: Start at /, login, assert dashboard Today\'s Sales is visible', async ({ page }) => {
    // For simplicity, we just look at the UI loaded directly for the test,
    // our Rust server just returns the HTML for all routes except /api/v1/health.
    await expect(page.locator('h1').filter({ hasText: 'Login' })).toBeVisible();
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Now on dashboard
    await expect(page.locator('h1').filter({ hasText: 'Dashboard' })).toBeVisible();
    await expect(page.locator('h3').filter({ hasText: 'Today\'s Sales' })).toBeVisible();
  });

  test('Test 2: Navigate to "Grow Business" (Setup Wizard) from nav', async ({ page }) => {
    // Login
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Click on Grow Business
    await page.getByText('Grow Business').click();
    await expect(page.locator('h1').filter({ hasText: 'Welcome to your AI business builder' })).toBeVisible();
  });

  test('Test 3: Test First-Time User Tour hint toggle', async ({ page }) => {
    // Login
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // The hint should not be visible
    const hint = page.locator('#hint-1');
    await expect(hint).toBeHidden();

    // Click the hint button
    await page.locator('h3').filter({ hasText: 'Quick Actions' }).locator('button').click();

    // The hint should become visible
    await expect(hint).toBeVisible();
    await expect(hint).toHaveText('Tap here to see your daily sales and messages.');
  });

  test('Test 4: Check jargon-free nav ("My Team", "Connect Apps")', async ({ page }) => {
    // Login
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Check nav links
    await expect(page.locator('nav#main-nav').getByText('My Team')).toBeVisible();
    await expect(page.locator('nav#main-nav').getByText('Connect Apps')).toBeVisible();

    // Click Connect Apps and assert
    await page.locator('nav#main-nav').getByText('Connect Apps').click();
    await expect(page.locator('h1').filter({ hasText: 'Software' })).toBeVisible();
  });

  test('Test 5: Ensure touch targets (buttons) have bounding box height >= 44px', async ({ page }) => {
    // Login
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.getByPlaceholder('Password').fill('password123');
    await page.getByRole('button', { name: 'Login' }).click();

    // Check a button on the dashboard
    const button = page.getByRole('button', { name: 'Check Inbox' });
    const box = await button.boundingBox();
    expect(box).not.toBeNull();
    if (box) {
      expect(box.height).toBeGreaterThanOrEqual(44);
    }
  });
});

import { test, expect } from '@playwright/test';
import { execSync } from 'child_process';

test.describe('Lens Audit E2E Flow', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('CUJ 1: Login and view Dashboard visual state', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();
    await expect(page.locator('nav')).toBeVisible();
  });

  test('CUJ 2: Business Setup Wizard flow verifies mock data removal & DB insertion', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Start Setup")');
    await page.click('button:has-text("Next")');
    await page.click('text="Online Store"');
    await page.click('button:has-text("Next")');
    await page.fill('input[placeholder="What is your business called?"]', 'My Awesome Store');
    await page.click('button:has-text("Generate Description")');
    await page.waitForTimeout(1000);
    await page.click('button:has-text("Next")');

    await expect(page.locator('.mock-data-stub')).toHaveCount(0);

    await page.click('button:has-text("Launch My Business →")');
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();


    // Verify DB state assertion (UI -> DB)
    const result = execSync('sqlite3 src/server/local.db "SELECT count(*) FROM onboarding_state" ').toString();
    expect(parseInt(result.trim())).toBeGreaterThan(0);

    // DB -> UI Verification: refresh and ensure the UI reflects the loaded state correctly
    await page.reload();
    await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();

  });

  test('CUJ 3: Agent activity tracking and viewing active agents', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("My Agents")');
    await expect(page.locator('text="Operations Agent"')).toBeVisible();
  });

  test('CUJ 4: Inbox and communications workflow', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('text="Welcome back, Human."')).toBeVisible();

    await page.click('button:has-text("Check Inbox")');
    await expect(page.getByRole('heading', { name: 'Inbox' })).toBeVisible();
  });

  test('CUJ 5: Settings menu and responsiveness testing', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });

    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('admin@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.locator('button:has-text("Login")').click();

    await expect(page.locator('nav')).toBeVisible();

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Settings")');
    await expect(page.locator('text="Settings"')).toBeVisible();
  });
});

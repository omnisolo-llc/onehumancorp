import { test, expect } from '@playwright/test';

test.describe('Dashboard Canvas E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').fill('test@example.com');
    await page.locator('input[type="password"]').fill('password123');
    await page.getByRole('button', { name: /Login|Sign In/i }).click();
    await page.waitForURL('**/*');
  });

  test('Test 1: Dashboard loads correctly and shows all primary stats', async ({ page }) => {
    await expect(page.locator('text=Business Owner Dashboard')).toBeVisible();
    await expect(page.locator('text=Revenue')).toBeVisible();
    await expect(page.locator('text=$1,200')).toBeVisible();
    await expect(page.locator('text=Orders')).toBeVisible();
    await expect(page.locator('text=45')).toBeVisible();
    await expect(page.locator('text=Active Customers')).toBeVisible();
    await expect(page.locator('text=120')).toBeVisible();
  });

  test('Test 2: Drill down into Revenue stat card', async ({ page }) => {
    await expect(page.locator('text=Business Owner Dashboard')).toBeVisible();
    await page.locator('text=Revenue').click();
    await expect(page.locator('text=Drill down view for Revenue showing details...')).toBeVisible();
  });

  test('Test 3: Drill down into Orders stat card', async ({ page }) => {
    await expect(page.locator('text=Business Owner Dashboard')).toBeVisible();
    await page.locator('text=Orders').click();
    await expect(page.locator('text=Drill down view for Orders showing details...')).toBeVisible();
  });

  test('Test 4: Drill down into Active Customers stat card', async ({ page }) => {
    await expect(page.locator('text=Business Owner Dashboard')).toBeVisible();
    await page.locator('text=Active Customers').click();
    await expect(page.locator('text=Drill down view for Active Customers showing details...')).toBeVisible();
  });

  test('Test 5: Verify AI Agent Status section is visible and contains correct activities', async ({ page }) => {
    await expect(page.locator('text=AI Agent Status')).toBeVisible();
    await expect(page.locator('text=✅ Support Agent replied to 3 customers')).toBeVisible();
    await expect(page.locator('text=📦 Order Manager updated stock for 12 items')).toBeVisible();
  });
});

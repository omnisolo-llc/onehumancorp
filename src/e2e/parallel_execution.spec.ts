import { test, expect } from '@playwright/test';

test.describe('Parallel Execution UI Tests', () => {

  test('Kitchen Command Center loads orders and menu concurrently', async ({ page }) => {
    // 1. Visit the Kitchen Command Center page
    await page.goto('/kitchen');

    // 2. Wait for the page title to be visible
    await expect(page.locator('h1', { hasText: 'Kitchen Command Center' })).toBeVisible();

    // 3. Verify Active Orders section
    await expect(page.locator('h2', { hasText: 'Active Orders' })).toBeVisible();

    // 4. Verify Daily Menu section
    await expect(page.locator('h2', { hasText: 'Daily Menu' })).toBeVisible();
  });

  test('mPOS page loads catalog correctly', async ({ page }) => {
    await page.goto('/pos/mpos');
    await expect(page.locator('h1', { hasText: 'mPOS' })).toBeVisible();
    await expect(page.getByTestId('mpos-quick-charge')).toBeVisible();
  });

  test('KDS page renders header correctly', async ({ page }) => {
    await page.goto('/pos/kds');
    // It should have either Kitchen Display System or نظام عرض المطبخ
    await expect(page.locator('h1').first()).toBeVisible();
    await expect(page.getByTestId('lang-toggle')).toBeVisible();
  });

  test('Omnichannel cart page renders successfully', async ({ page }) => {
    await page.goto('/pos/omnichannel');
    await expect(page.locator('h1', { hasText: 'New In-Store Sale' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Create Omnichannel Cart' })).toBeVisible();
  });

  test('Dashboard loads unified feed without errors', async ({ page }) => {
    await page.goto('/dashboard');
    await expect(page.locator('h1', { hasText: 'Dashboard' }).first()).toBeVisible();
    await expect(page.locator('text=Activity Feed').first()).toBeVisible();
  });

});

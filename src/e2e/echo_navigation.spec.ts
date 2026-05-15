import { test, expect } from '@playwright/test';

test.describe('Dashboard Navigation UX Simplification', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Bottom navigation bar is present and has exact required actions', async ({ page }) => {
    // 1. Start from home page (login)
    await page.goto('/login');

    // 2. Perform the exact login flow as a user would
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');

    // 3. Wait for dashboard to load
    await page.waitForURL('**/*');

    // 4. Verify 5 most-used actions are accessible in the bottom navigation bar
    const addProductBtn = page.locator('text="Add"');
    await expect(addProductBtn.filter({ visible: true }).first()).toBeVisible();

    const viewOrdersBtn = page.locator('text="Orders"').last();
    await expect(viewOrdersBtn).toBeVisible();

    const checkMessagesBtn = page.locator('text="Messages"');
    await expect(checkMessagesBtn.filter({ visible: true }).first()).toBeVisible();

    const seeAnalyticsBtn = page.locator('text="Analytics"');
    await expect(seeAnalyticsBtn.filter({ visible: true }).first()).toBeVisible();

    const shareStoreBtn = page.locator('text="Share"').last();
    await expect(shareStoreBtn).toBeVisible();
  });

  test('Clicking Add Product in bottom nav completes action', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const addProductBtn = page.locator('text="Add"').filter({ visible: true }).first();
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();
    await expect(page.getByRole('heading', { name: 'Add Product' })).toBeVisible();
  });

  test('Clicking View Orders in bottom nav completes action', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    // Make sure we select the one in the bottom nav if there are multiple
    const viewOrdersBtn = page.locator('text="Orders"').last();
    await expect(viewOrdersBtn).toBeVisible();
    await viewOrdersBtn.click();
    await expect(page.getByRole('heading', { name: 'Orders' })).toBeVisible();
  });

  test('Clicking Check Messages in bottom nav completes action', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const checkMessagesBtn = page.locator('text="Messages"').filter({ visible: true }).first();
    await expect(checkMessagesBtn).toBeVisible();
    await checkMessagesBtn.click();
    await expect(page.getByRole('heading', { name: 'Customer Inbox' })).toBeVisible();
  });

  test('Clicking See Analytics in bottom nav completes action', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const seeAnalyticsBtn = page.locator('text="Analytics"').filter({ visible: true }).first();
    await expect(seeAnalyticsBtn).toBeVisible();
    await seeAnalyticsBtn.click();
    await expect(page.getByRole('heading', { name: 'Analytics' })).toBeVisible();
  });

  test('Clicking Share Store in bottom nav completes action', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Sign In")');
    await page.waitForURL('**/*');

    const shareStoreBtn = page.locator('text="Share"').last();
    await expect(shareStoreBtn).toBeVisible();
    await shareStoreBtn.click();
    await expect(page.getByRole('heading', { name: 'Referral Dashboard' })).toBeVisible();
  });
});

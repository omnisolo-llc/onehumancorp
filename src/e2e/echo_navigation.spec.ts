import { test, expect } from '@playwright/test';

test.describe('Dashboard Navigation UX Simplification', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Bottom navigation bar is present and has exact required actions', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const addProductBtn = page.locator('button.nav-item', { hasText: /^Add$/ });
    await expect(addProductBtn.filter({ visible: true }).first()).toBeVisible();

    const viewOrdersBtn = page.locator('button.nav-item', { hasText: /^Orders$/ });
    await expect(viewOrdersBtn.filter({ visible: true }).first()).toBeVisible();

    const checkMessagesBtn = page.locator('button.nav-item', { hasText: /^Messages$/ });
    await expect(checkMessagesBtn.filter({ visible: true }).first()).toBeVisible();

    const seeAnalyticsBtn = page.locator('button.nav-item', { hasText: /^Analytics$/ });
    await expect(seeAnalyticsBtn.filter({ visible: true }).first()).toBeVisible();

    const shareStoreBtn = page.locator('button.nav-item', { hasText: /^Share$/ });
    await expect(shareStoreBtn.filter({ visible: true }).first()).toBeVisible();
  });

  test('Clicking Add in bottom nav routes properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const addProductBtn = page.locator('button.nav-item', { hasText: /^Add$/ }).filter({ visible: true }).first();
    await addProductBtn.click();
    await expect(page.locator('#setup-screen')).toBeVisible();
  });

  test('Clicking Orders in bottom nav routes properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const viewOrdersBtn = page.locator('button.nav-item', { hasText: /^Orders$/ }).filter({ visible: true }).first();
    await viewOrdersBtn.click();
    // Assuming Orders points to dashboard-screen based on the code
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });

  test('Clicking Messages in bottom nav routes properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const checkMessagesBtn = page.locator('button.nav-item', { hasText: /^Messages$/ }).filter({ visible: true }).first();
    await checkMessagesBtn.click();
    await expect(page.locator('#inbox-screen')).toBeVisible();
  });

  test('Clicking Analytics in bottom nav routes properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const seeAnalyticsBtn = page.locator('button.nav-item', { hasText: /^Analytics$/ }).filter({ visible: true }).first();
    await seeAnalyticsBtn.click();
    // Assuming Analytics points to dashboard-screen based on the code
    await expect(page.locator('#dashboard-screen')).toBeVisible();
  });

  test('Clicking Share in bottom nav routes properly', async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator('input[type="password"]').filter({ visible: true }).first().fill( 'password123');
    await page.click('button:has-text("Login")');
    await page.waitForURL('**/*');

    const shareStoreBtn = page.locator('button.nav-item', { hasText: /^Share$/ }).filter({ visible: true }).first();
    await shareStoreBtn.click();
    await expect(page.locator('#referral-dashboard-screen')).toBeVisible();
  });
});

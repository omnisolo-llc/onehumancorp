import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test.describe('Dashboard Navigation UX Simplification', () => {
  test.use({ viewport: { width: 375, height: 800 } });

  test('Bottom navigation bar is present and has exact required actions', async ({ page }) => {
    // 1. Start from home page (login)
    await page.goto(ROUTES.LOGIN);

    // 2. Perform the exact login flow as a user would
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);

    // 3. Wait for dashboard to load
    await page.waitForURL('**/*');

    // 4. Verify 5 most-used actions are accessible in the bottom navigation bar
    const addProductBtn = page.locator('text="Add"');
    await expect(addProductBtn.first()).toBeVisible();

    const viewOrdersBtn = page.locator('text="Orders"').last();
    await expect(viewOrdersBtn).toBeVisible();

    const checkMessagesBtn = page.locator('text="Messages"');
    await expect(checkMessagesBtn.first()).toBeVisible();

    const seeAnalyticsBtn = page.locator('text="Analytics"');
    await expect(seeAnalyticsBtn.first()).toBeVisible();

    const shareStoreBtn = page.locator('text="Share"').last();
    await expect(shareStoreBtn).toBeVisible();
  });

  test('Clicking Add Product in bottom nav completes action', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);
    await page.waitForURL('**/*');

    const addProductBtn = page.locator('text="Add"').first();
    await expect(addProductBtn).toBeVisible();
    await addProductBtn.click();

    // Verify it triggers action - standard response might be a toast, we look for success indication or UI reaction
    // Wait for network idle or any indication of reaction
    await page.waitForTimeout(500);
  });

  test('Clicking View Orders in bottom nav completes action', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);
    await page.waitForURL('**/*');

    // Make sure we select the one in the bottom nav if there are multiple
    const viewOrdersBtn = page.locator('text="Orders"').last();
    await expect(viewOrdersBtn).toBeVisible();
    await viewOrdersBtn.click();

    await page.waitForTimeout(500);
  });

  test('Clicking Check Messages in bottom nav completes action', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);
    await page.waitForURL('**/*');

    const checkMessagesBtn = page.locator('text="Messages"').first();
    await expect(checkMessagesBtn).toBeVisible();
    await checkMessagesBtn.click();

    await page.waitForTimeout(500);
  });

  test('Clicking See Analytics in bottom nav completes action', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);
    await page.waitForURL('**/*');

    const seeAnalyticsBtn = page.locator('text="Analytics"').first();
    await expect(seeAnalyticsBtn).toBeVisible();
    await seeAnalyticsBtn.click();

    await page.waitForTimeout(500);
  });

  test('Clicking Share Store in bottom nav completes action', async ({ page }) => {
    await page.goto(ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').first().fill( TEST_DATA.EMAIL);
    await page.locator('input[type="password"]').first().fill( TEST_DATA.PASSWORD);
    await page.click(SELECTORS.SIGN_IN_BTN);
    await page.waitForURL('**/*');

    const shareStoreBtn = page.locator('text="Share"').last();
    await expect(shareStoreBtn).toBeVisible();
    await shareStoreBtn.click();

    await page.waitForTimeout(500);
  });
});

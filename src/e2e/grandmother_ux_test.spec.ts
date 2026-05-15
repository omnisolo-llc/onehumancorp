import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Grandmother UX Fixes E2E tests', () => {
  test('Login screen shows plain language Fix App Issues button', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.locator('text=Sign in to manage your business')).toBeVisible();
    await expect(page.locator(UI_LOCATORS.LOGIN_BUTTON)).toBeVisible();
  });

  test('Login screen shows plain language brand name', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.locator('text="One Human Corp"').filter({ visible: true }).first()).toBeVisible();
  });

  test('Integrations screen uses plain language for external tools', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill( 'password123');
    await page.click(UI_LOCATORS.SIGN_IN_BTN);

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Connect Custom Software').last()).toBeVisible();
  });

  test('API Docs screen uses Custom Integration label', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill( 'password123');
    await page.click(UI_LOCATORS.SIGN_IN_BTN);

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Custom Integration')).toBeVisible();
  });

  test('API Docs screen replaces GET /v1/products with Read Product List', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await page.getByPlaceholder('Email or Username').filter({ visible: true }).first().fill( 'test@example.com');
    await page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first().fill( 'password123');
    await page.click(UI_LOCATORS.SIGN_IN_BTN);

    await page.click('button:has-text("Menu")');
    await page.click('button:has-text("Connect Custom Software")');

    await expect(page.locator('text=Product Data Access').last()).toBeVisible();
    await expect(page.locator('text=Read Product List')).toBeVisible();
  });
});

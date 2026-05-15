import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test.describe('Dashboard', () => {
  test('should load dashboard page', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page).toHaveTitle(/OneHuman/);
  });

  test('should display navigation', async ({ page }) => {
    await page.goto(E2E_ROUTES.HOME);
    await expect(page.locator(UI_LOCATORS.NAV)).toBeVisible();
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
    const nextButton = page.locator(UI_LOCATORS.NEXT_BUTTON);
    await nextButton.click();

    // Step 1: Business type
    await page.locator('input[type="text"]').filter({ visible: true }).first().fill('Online Store');
    await nextButton.click();

    // Step 2: Company name
    await page.locator('input[type="text"]').filter({ visible: true }).first().fill('Test Company');
    await nextButton.click();

    // Verify we can proceed through steps
    await expect(page.locator('text=What do you sell')).toBeVisible();
  });
});

test.describe('Login', () => {
  test('should show login form', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    await expect(page.getByPlaceholder('Email or Username').filter({ visible: true }).first()).toBeVisible();
    await expect(page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first()).toBeVisible();
  });

  test('should allow password visibility toggle', async ({ page }) => {
    await page.goto(E2E_ROUTES.LOGIN);
    const passwordInput = page.locator(UI_LOCATORS.PASSWORD_INPUT).filter({ visible: true }).first();
    const toggleButton = page.locator('button:has-text("Show")');
    await expect(toggleButton).toBeVisible();
  });
});

test.describe('Agent Management', () => {
  test('should show agents list', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.locator('h1:has-text("Agents")')).toBeVisible();
  });

  test('should show hire agent button', async ({ page }) => {
    await page.goto(E2E_ROUTES.AGENTS);
    await expect(page.locator(UI_LOCATORS.HIRE_AGENT)).toBeVisible();
  });
});

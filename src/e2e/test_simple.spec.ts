import { E2E_ROUTES, UI_LOCATORS } from "./playwright_test_constants";
import { test, expect } from '@playwright/test';

test('simple test', async ({ page }) => {
  await page.goto(E2E_ROUTES.LOGIN);
  await expect(page.getByRole('heading', { name: 'Login' }).first()).toBeVisible();
});

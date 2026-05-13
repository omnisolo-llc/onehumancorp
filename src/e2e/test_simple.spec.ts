import { test, expect } from '@playwright/test';
import { ROUTES, SELECTORS, TEST_DATA } from './constants';

test('simple test', async ({ page }) => {
  await page.goto(ROUTES.LOGIN);
  await expect(page.locator('h1').first()).toBeVisible();
});

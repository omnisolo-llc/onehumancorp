import { test, expect } from '@playwright/test';
import * as path from 'path';

test.describe('Setup Wizard', () => {
  let fileUrl: string;

  test.beforeAll(() => {
    const filePath = path.resolve(__dirname, '../ui/tauri/src/ui/setup.html');
    fileUrl = `file://${filePath}`;
  });

  test.beforeEach(async ({ page }) => {
    await page.goto(fileUrl);
  });

  test('should display only the active step initially', async ({ page }) => {
    await expect(page.locator('#step-intro')).toBeVisible();
    await expect(page.locator('#step-context')).toBeHidden();
    await expect(page.locator('#step-categories')).toBeHidden();
  });

  test('should navigate to the next step when next button is clicked', async ({ page }) => {
    await page.locator('#step-intro .secondary-btn').click();
    await expect(page.locator('#step-intro')).toBeHidden();
    await expect(page.locator('#step-context')).toBeVisible();
  });

  test('should show error when required name is empty', async ({ page }) => {
    await page.locator('#step-intro .secondary-btn').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();

    // We are at step-name now
    await expect(page.locator('#step-name')).toBeVisible();
    await page.locator('#business-name').fill('');
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.locator('#name-error')).toBeVisible();
    await expect(page.locator('#step-name')).toBeVisible(); // Still on the same step
  });

  test('should navigate to next step when name is filled', async ({ page }) => {
    await page.locator('#step-intro .secondary-btn').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();

    // We are at step-name now
    await page.locator('#business-name').fill('Jules Bakery');
    await page.locator('#step-name .next-step-btn').click();

    await expect(page.locator('#step-name')).toBeHidden();
    await expect(page.locator('#step-assistant')).toBeVisible();
  });

  test('should be able to navigate to the final step', async ({ page }) => {
    await page.locator('#step-intro .secondary-btn').click();
    await page.locator('#step-context .next-step-btn').click();
    await page.locator('#step-categories .next-step-btn').click();
    await page.locator('#business-name').fill('Jules Bakery');
    await page.locator('#step-name .next-step-btn').click();
    await page.locator('#step-assistant .next-step-btn').click();
    await page.locator('#step-admin .next-step-btn').click();
    await page.locator('#step-offer .next-step-btn').click();
    await page.locator('#step-location .next-step-btn').click();
    await page.locator('#step-target-audience .next-step-btn').click();
    await page.locator('#step-domain .next-step-btn').click();

    await expect(page.locator('#step-template')).toBeVisible();
  });
});

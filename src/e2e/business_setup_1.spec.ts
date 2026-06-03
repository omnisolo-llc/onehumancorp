import { test, expect } from './fixtures';

test.describe('Business Setup Wizard', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate straight to the setup page rather than going through the entire login flow if it's flakey
    await page.goto('/business-setup');
    await page.waitForLoadState('networkidle');
  });

  test('shows the current setup welcome step', async ({ page }) => {
    await expect(page.locator('text=Your business, live in minutes')).toBeVisible();
    await expect(page.locator('text=Let’s get your new OHC storefront set up.')).toBeVisible();
    await expect(page.getByRole('button', { name: /Get Started/ })).toBeVisible();
  });

  test('moves through business type and name steps', async ({ page }) => {
    await page.getByRole('button', { name: /Get Started/ }).click();

    await expect(page.getByRole('heading', { name: 'Give your business a name' })).toBeVisible();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cookies and cakes');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();

    await expect(page.getByRole('heading', { name: 'What do you sell?' })).toBeVisible();
  });

  test('completes the publish path to the checklist', async ({ page }) => {
    await page.getByRole('button', { name: /Get Started/ }).click();

    // Step 2 & 3
    await page.getByRole('button', { name: /Online Store/ }).click();
    await page.getByPlaceholder('What is your business called?').fill('Test Company');
    await page.getByPlaceholder("e.g. Maya's Cakes").fill('Custom cookies and cakes');
    await page.locator('#step-3').getByRole('button', { name: /Next/ }).click();

    // Step 4
    await page.getByLabel(/Physical Products/).check();
    await page.locator('#step-4').getByRole('button', { name: /Next/ }).click();

    // Final Step
    await page.getByRole('button', { name: 'Done' }).click();

    await expect(page.getByRole('heading', { name: 'Welcome to One Human Corp' })).toBeVisible({ timeout: 15000 });
  });
});

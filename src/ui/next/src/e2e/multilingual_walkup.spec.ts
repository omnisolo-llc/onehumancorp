import { test, expect } from '../../../../e2e/fixtures';

test.describe('Multilingual Order Interceptor Walk-up', () => {
  test.beforeEach(async ({ page, context }) => {
    await context.clearCookies();
    await page.goto('/pos/walkup');
    await page.evaluate(() => localStorage.clear());
    await page.reload();
  });

  test('Submits raw Spanish order and verifies processing state', async ({ page }) => {
    // Wait for the UI to load
    await expect(page.getByTestId('input-walkup-text')).toBeVisible();

    // The Persona is a small business owner handing over the phone or speaking Spanish input for the customer
    await page.getByTestId('input-walkup-text').fill('Quiero 3 tacos de pollo');

    // Click submit
    await page.getByTestId('btn-submit-walkup').click();

    // Verify processing overlay
    await expect(page.getByTestId('processing-overlay')).toBeVisible();

    // Wait for success
    await expect(page.getByTestId('success-card')).toBeVisible({ timeout: 15000 });
  });
});

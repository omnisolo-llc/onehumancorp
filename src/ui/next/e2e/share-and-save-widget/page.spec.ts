import { test, expect } from '@playwright/test';

test.describe('Share & Save Widget Page', () => {
  test.beforeEach(async ({ page }) => {
    // Go to the widget configuration page
    await page.goto('/share-and-save-widget');
  });

  test('should render the configuration form', async ({ page }) => {
    // Verify main headings
    await expect(page.locator('h1', { hasText: 'Share & Save Widget' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Configure Incentive' })).toBeVisible();
    await expect(page.locator('h2', { hasText: 'Brand Settings' })).toBeVisible();
  });

  test('should render the live preview with default values', async ({ page }) => {
    // Verify preview card contents
    await expect(page.locator('h4', { hasText: 'Love our store?' })).toBeVisible();

    // Verify "Powered by OHC" watermark is visible
    const watermark = page.locator('a', { hasText: '⚡ Powered by OHC' });
    await expect(watermark).toBeVisible();
  });

  test('should update preview when discount is changed', async ({ page }) => {
    // Change discount value
    const input = page.locator('input[type="number"]');
    await input.fill('20');

    // Change discount type to $
    const select = page.locator('select');
    await select.selectOption('$');

    // The preview text should update
    await expect(page.locator('p', { hasText: '$20 off' })).toBeVisible();
  });

  test('should show soft paywall when trying to remove branding without Pro plan', async ({ page }) => {
    // Ensure we are not on a Pro plan (the local storage mock in page sets has_pro based on actual localStorage which is unset by default in e2e)
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'false');
    });
    await page.reload();

    // Click the checkbox label to remove branding
    const checkboxLabel = page.locator('text=Remove "Powered by OHC" branding');
    await checkboxLabel.click();

    // Verify the soft paywall modal appears
    await expect(page.locator('h3', { hasText: 'Make it Yours' })).toBeVisible();
    await expect(page.locator('button', { hasText: 'Upgrade to Pro' })).toBeVisible();
  });

  test('should allow removing branding if user has Pro plan', async ({ page }) => {
    // Simulate having a Pro plan
    await page.evaluate(() => {
        localStorage.setItem('has_pro', 'true');
    });
    await page.reload();

    // Click the checkbox to remove branding
    const checkbox = page.locator('input[type="checkbox"]');
    await checkbox.check();

    // Verify the "Powered by OHC" watermark is no longer visible in the preview
    await expect(page.locator('a', { hasText: '⚡ Powered by OHC' })).not.toBeVisible();
  });
});

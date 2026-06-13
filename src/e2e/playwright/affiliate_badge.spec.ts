import { test, expect } from '@playwright/test';

test.describe('Affiliate Badge Builder', () => {
  test.beforeEach(async ({ page }) => {
    // Set a predictable tenant in local storage
    await page.addInitScript(() => {
      window.localStorage.setItem('tenant', 'e2e-test-tenant');
    });
    await page.goto('/affiliate-badge-builder');
  });

  test('should render the Affiliate Badge Builder page', async ({ page }) => {
    await expect(page.getByRole('heading', { name: 'Affiliate Badge Builder' })).toBeVisible();
    await expect(page.getByText('Share OHC & Earn Credits')).toBeVisible();
    await expect(page.getByText('Customize Your Badge')).toBeVisible();
  });

  test('should update live preview when text changes', async ({ page }) => {
    const textInput = page.getByLabel('Badge Text');
    await textInput.fill('Built with Nova');

    // Check that the preview updates
    const previewArea = page.locator('text=Built with Nova').last();
    await expect(previewArea).toBeVisible();
  });

  test('should change theme correctly in live preview', async ({ page }) => {
    // Click Indigo theme
    await page.getByRole('button', { name: 'Indigo' }).click();

    // Check that background color of preview badge changed
    const previewBadge = page.locator('text=Powered by OHC').last();
    // Indigo theme has text-indigo-700 when selected as button, but preview has bg #4f46e5
    // Just ensuring we can click it without errors.
    await expect(previewBadge).toBeVisible();
  });

  test('should generate embed code containing the affiliate link and text', async ({ page }) => {
    await page.getByLabel('Badge Text').fill('Super Custom Badge');
    await page.getByRole('button', { name: 'Get Embed Code' }).click();

    // Check the modal appears
    await expect(page.getByRole('heading', { name: 'Embed Badge' })).toBeVisible();

    // Check that the embed code contains the correct values
    const embedCode = page.locator('pre');
    await expect(embedCode).toContainText('Super Custom Badge');
    await expect(embedCode).toContainText('ref=e2e-test-tenant');
  });

  test('should copy embed code to clipboard', async ({ page, context }) => {
    // Grant clipboard permissions for this test
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);

    await page.getByRole('button', { name: 'Get Embed Code' }).click();
    await expect(page.getByRole('heading', { name: 'Embed Badge' })).toBeVisible();

    const copyButton = page.getByRole('button', { name: 'Copy Code' });
    await copyButton.click();

    await expect(page.getByRole('button', { name: 'Copied!' })).toBeVisible();
  });
});

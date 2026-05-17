import { test, expect } from '@playwright/test';

test.describe('Website Builder Full E2E', () => {
  test('renders editable storefront blocks', async ({ page }) => {
    await page.goto('/storefront-builder');
    await expect(page.getByRole('heading', { name: 'Edit Website' })).toBeVisible();
    await page.getByRole('button', { name: 'Rearrange' }).click();
    await expect(page.locator('#builder-preview-container')).toContainText('Hero');
    await expect(page.locator('#builder-preview-container')).toContainText('Product Grid');
  });

  test('opens publish workflow for a free subdomain', async ({ page }) => {
    await page.goto('/storefront-builder');
    await page.getByRole('button', { name: 'Publish Changes' }).click();
    await page.getByRole('button', { name: /Free OHC Subdomain/ }).click();
    await page.getByPlaceholder('mybusiness').fill('test-store');
    await expect(page.getByPlaceholder('mybusiness')).toHaveValue('test-store');
  });
});

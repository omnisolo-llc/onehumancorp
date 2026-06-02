import { test, expect } from '@playwright/test';

test('viral_storefront', async ({ page }) => {
  await page.goto('/dashboard');

  // Find embed section
  await expect(page.getByRole('heading', { name: 'Embed Your Store' })).toBeVisible();

  // Click copy code
  await page.getByRole('button', { name: 'Get Widget' }).click();

  // Modal should appear
  await expect(page.getByRole('heading', { name: 'Embed Storefront' })).toBeVisible();

  // Check the copy code button inside modal
  const copyBtn = page.getByRole('button', { name: 'Copy Code' });
  await expect(copyBtn).toBeVisible();
});


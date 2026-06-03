import { test, expect } from '@playwright/test';

test.describe('Documentation Navigation CUJ', () => {
  test('User can navigate between Help Center, Video Tutorials, Changelog, and API Docs', async ({ page }) => {
    // 1. Visit the dashboard where the navigation links exist
    await page.goto('/dashboard');
    await expect(page.getByRole('heading', { name: 'Business Analytics' })).toBeVisible();

    // 2. Navigate to Help Center via global sidebar UI link
    await page.locator('a[title="Help Center"]').click();
    await expect(page.getByRole('heading', { name: 'Help Center' })).toBeVisible();
    await expect(page.getByRole('heading', { name: 'Articles' })).toBeVisible({ timeout: 10000 });
    await expect(page.getByText('Getting Started')).toBeVisible();

    // 3. Navigate to Video Tutorials via global sidebar UI link
    await page.locator('a[title="Tutorials"]').click();
    await expect(page.getByRole('heading', { name: 'Video Guides' })).toBeVisible();

    // 4. Navigate to Changelog via global sidebar UI link
    await page.locator('a[title="Changelog"]').click();
    await expect(page.getByRole('heading', { name: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.getByText('Version 1.0 (Latest)')).toBeVisible();

    // 5. Navigate to API Docs via global sidebar UI link
    await page.locator('a[title="API Docs"]').click();
    await expect(page.getByText('Advanced: This section is for developers')).toBeVisible();
    await expect(page.locator('.swagger-ui')).toBeVisible({ timeout: 15000 });
  });
});

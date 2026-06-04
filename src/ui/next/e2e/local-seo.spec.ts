import { test, expect } from '@playwright/test';

test.describe('Local SEO - Google Business Profile', () => {
  test('should display Google Business Profile page and allow connect', async ({ page }) => {
    await page.goto('/local-seo');

    await expect(page.locator('h2').filter({ hasText: 'Google Business Profile' })).toBeVisible();
    await expect(page.locator('text=Connect Profile')).toBeVisible();
  });

  test('should show pending reviews when connected', async ({ page }) => {
    // Assuming our API returns connected=true and 2 pending reviews as written in local_seo.rs
    await page.goto('/local-seo');

    // We expect the connected badge and pending reviews to be visible.
    await expect(page.locator('text=Connected')).toBeVisible();
    await expect(page.locator('text=Pending Reviews')).toBeVisible();
    await expect(page.locator('text=Carlos fixed my sink perfectly!')).toBeVisible();
  });
});

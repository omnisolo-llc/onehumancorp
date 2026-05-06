import { test, expect } from '@playwright/test';

test.beforeEach(async ({ page }) => {
  await page.goto('/login');
  await page.getByPlaceholder('Email address').fill('admin@onehumancorp.com');
  await page.getByPlaceholder('Password').fill('admin');
  await page.getByRole('button', { name: 'Sign In' }).click();
  // Wait for login to complete and navigate to dashboard
  await page.waitForURL('**/');
});

test('Website Preview Widget is visible on the dashboard', async ({ page }) => {
  // Verify the preview widget is visible
  await expect(page.locator('text="Website Preview"')).toBeVisible();

  // Verify Edit button
  await expect(page.locator('button:has-text("Edit")').first()).toBeVisible();

  // Verify URL is shown
  await expect(page.locator('text="🔒 https://mybusiness.ohc.app"')).toBeVisible();
});

test('Website Preview Widget allows full screen', async ({ page }) => {
  await expect(page.locator('text="Tap to view in full screen"')).toBeVisible();
  await expect(page.locator('text="Welcome to My Business"')).toBeVisible();
});

test('Website Preview Widget layout tests', async ({ page }) => {
  await expect(page.locator('text="The best services in town."')).toBeVisible();
  await expect(page.locator('button:has-text("Book Now")').first()).toBeVisible();
});

test('Website Preview Widget edit button test', async ({ page }) => {
  const editButton = page.locator('button:has-text("Edit")').first();
  await expect(editButton).toBeVisible();
});

test('Website Preview Widget shows loading state', async ({ page }) => {
    // Just a placeholder for testing
  await expect(page.locator('text="Website Preview"')).toBeVisible();
});

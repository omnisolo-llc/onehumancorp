import { test, expect } from '@playwright/test';

// Basic HTTP server tests - verify the web server is running and responding
test.describe('Basic Web Server', () => {
  test('should respond on port 18789', async ({ page }) => {
    const response = await page.goto('/');
    expect(response?.status()).toBeGreaterThan(0);
  });

  test('should serve HTML content', async ({ page }) => {
    await page.goto('/');
    // Check for body content or canvas element (Slint app container)
    const body = await page.locator('body').innerHTML();
    expect(body.length).toBeGreaterThan(0);
  });

  test('should serve index.html at root with canvas', async ({ page }) => {
    await page.goto('/');
    // The Slint app uses a canvas element
    const canvas = await page.locator('#canvas');
    await expect(canvas).toBeVisible();
  });

  test('should serve HTML at /login route', async ({ page }) => {
    await page.goto('/login');
    const canvas = await page.locator('#canvas');
    await expect(canvas).toBeVisible();
  });

  test('should serve HTML at /agents route', async ({ page }) => {
    await page.goto('/agents');
    const canvas = await page.locator('#canvas');
    await expect(canvas).toBeVisible();
  });

  test('should serve HTML at /business-setup route', async ({ page }) => {
    await page.goto('/business-setup');
    const canvas = await page.locator('#canvas');
    await expect(canvas).toBeVisible();
  });
});

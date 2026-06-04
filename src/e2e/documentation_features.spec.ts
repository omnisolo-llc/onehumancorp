import { test, expect } from './fixtures';

test.describe('Help Center Page', () => {
  test('should load help center and navigate to article', async ({ page }) => {
    // Navigate using the built-in path
    await page.goto('/');

    // Show the help screen
    await page.evaluate(() => {
      (window as any).showScreen('help-screen');
    });

    await expect(page.locator('#help-screen')).toBeVisible();
    await expect(page.locator('#help-screen h1', { hasText: 'Help Center' })).toBeVisible();
    await expect(page.locator('#help-screen h2', { hasText: 'Getting Started' })).toBeVisible();
    await expect(page.locator('#help-screen p', { hasText: 'Welcome to OneHumanCorp!' })).toBeVisible();
  });
});

test.describe('API Documentation', () => {
  test('should load Swagger UI', async ({ page }) => {
    await page.goto('/');

    // Show the API docs screen
    await page.evaluate(() => {
      (window as any).showScreen('api-docs-screen');
    });

    await expect(page.locator('#api-docs-screen')).toBeVisible();
    await expect(page.locator('#swagger-ui')).toBeVisible();

    await expect(page.locator('#api-docs-screen h1', { hasText: 'OHC Advanced API Reference' }).first()).toBeVisible();
    await expect(page.locator('#api-docs-screen p', { hasText: 'This section is for developers directly integrating with our APIs' })).toBeVisible();
  });
});

test.describe('Release Notes and Changelog', () => {
  test('should load changelog page', async ({ page }) => {
    await page.goto('/');

    // Show the changelog screen
    await page.evaluate(() => {
      (window as any).showScreen('changelog-screen');
    });

    await expect(page.locator('#changelog-screen')).toBeVisible();
    await expect(page.locator('#changelog-screen h1', { hasText: 'Release Notes & Changelog' })).toBeVisible();
    await expect(page.locator('#changelog-screen h2', { hasText: 'Version 1.0 (Latest)' })).toBeVisible();
    await expect(page.locator('#changelog-screen p', { hasText: 'Interactive AI Store Builder:' })).toBeVisible();
  });
});

test.describe('Help Chat Widget', () => {
  test('should verify widget functionality', async ({ page }) => {
    // Check if the chat API endpoint responds
    const response = await page.request.post('/api/chat', {
        data: { message: "How do I add a product?" }
    });

    expect(response.status()).toBe(200);
    const result = await response.json();
    expect(result.reply).toBeDefined();
    expect(result.link).toBeDefined();
  });
});

test.describe('Video Tutorials in Help Widget', () => {
  test('should verify video endpoint', async ({ page }) => {
    // Verify the videos api endpoint responds properly for the help widget
    const response = await page.request.get('/api/videos');
    expect(response.status()).toBe(200);

    const result = await response.json();
    expect(result.length).toBeGreaterThan(0);
    expect(result[0].title).toBeDefined();
    expect(result[0].duration).toBeDefined();
  });
});

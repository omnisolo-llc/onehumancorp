import { test, expect } from '@playwright/test';

test.describe('E2E Chaos - ML Resilience Fallbacks', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/login');
    await page.getByPlaceholder('Email or Username').first().fill('ai_chaos@example.com');
    await page.locator('input[type="password"]').first().fill('password123');
    await page.locator('button:has-text("Sign In"), button:has-text("Login")').click();
    await page.waitForURL('**/dashboard**');
  });

  test('should fallback gracefully when LLM API returns malformed JSON', async ({ page }) => {
    // Navigate to a feature that relies heavily on LLM generations (e.g. AI Store Builder)
    await page.locator('button:has-text("StoreBuilder"), button:has-text("Website")').first().click();

    // Mock the backend API to return garbage string instead of expected JSON payload
    await page.route('**/api/ai/generate/**', async route => {
        await route.fulfill({
            status: 200,
            contentType: 'application/json',
            body: '{ "invalid": json, syntax_error: true '
        });
    });

    await page.locator('button:has-text("Generate AI Store")').first().click();

    // Ensure the frontend catches the parsing error and displays a visual excellence degraded state
    const errorNotice = page.locator('.error-notice, text=/We encountered an issue|Try again/i');
    await expect(errorNotice).toBeVisible();
    await expect(page.locator('text=/unexpected response/i')).toBeVisible();
  });

  test('should halt agent jobs and not retry indefinitely upon 401 Unauthorized API keys', async ({ page }) => {
    await page.locator('button:has-text("Helpers"), button:has-text("Agents")').first().click();

    // Mock API to return a hard failure like 401
    await page.route('**/api/agents/run/**', async route => {
        await route.fulfill({
            status: 401,
            body: '{"error": "Invalid API Key"}'
        });
    });

    await page.locator('button:has-text("Start Task")').first().click();

    // It should immediately pause/fail, rather than endlessly showing "Retrying..."
    await expect(page.locator('text=/Paused|Failed/i')).toBeVisible();
    await expect(page.locator('text=/API Key|Configuration Error/i')).toBeVisible();
  });
});

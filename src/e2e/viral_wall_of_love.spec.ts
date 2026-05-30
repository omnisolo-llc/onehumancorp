import { test, expect } from '@playwright/test';

test.describe('Viral Wall of Love Growth Loop', () => {
  test('displays Wall of Love section in dashboard and returns valid HTML from API endpoint', async ({ page, request }) => {
    // Navigate to dashboard
    await page.goto('/dashboard');

    // Verify Wall of Love section is present
    const wallOfLoveHeading = page.locator('h2:has-text("Wall of Love")');
    await expect(wallOfLoveHeading).toBeVisible();

    // Verify iframe snippet contains correct endpoint
    const embedInput = page.locator('input[value*="wall-of-love/embed"]');
    await expect(embedInput).toBeVisible();

    // Verify Copy Code button is present
    const copyButton = page.locator('button:has-text("Copy Code")').filter({ hasText: 'Copy Code' }).first();
    await expect(copyButton).toBeVisible();

    // Verify API endpoint directly
    const apiResponse = await request.get('http://127.0.0.1:3000/api/v1/growth/wall-of-love/embed?tenant=test-tenant');
    expect(apiResponse.ok()).toBeTruthy();
    const htmlBody = await apiResponse.text();
    expect(htmlBody).toContain('What our customers say');
    expect(htmlBody).toContain('⚡ Powered by OHC');
    expect(htmlBody).toContain('test-tenant');
  });
});

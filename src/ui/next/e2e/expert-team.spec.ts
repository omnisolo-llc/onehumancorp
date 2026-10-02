import { test, expect } from '@playwright/test';

test.describe('Expert Team', () => {
  test('rejects duplicate expert outputs without inventing a delivered report', async ({ page }) => {
    // Navigate to the expert team page
    await page.goto('/expert-team');

    // Check that the title exists
    await expect(page.locator('h1', { hasText: 'Collaborative Expert Team' }).first()).toHaveText('Collaborative Expert Team');

    // Fill in the task context
    await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Write a comprehensive business plan for a new AI startup. Chart: Required. Analysis: Deep. Chapter 1, Chapter 2, Chapter 3, Chapter 4, Chapter 5, Chapter 6, Chapter 7, Chapter 8');

    // Click the execute button
    const execution = page.waitForResponse(response => response.url().endsWith('/api/v1/expert-team') && response.request().method() === 'POST');
    await page.click('button:has-text("Execute Task via Expert Team")');

    // Verify that the loading state appears
    await expect(page.locator('button:has-text("Orchestrating Expert Team...")')).toBeVisible();

    // The isolated runtime's deterministic fallback gives the experts duplicate
    // outputs. The real pre-merge gate must reject them, as the CI trace shows.
    // Successful synthesis is exercised at the provider boundary by unit tests.
    const response = await execution;
    expect(response.status()).toBe(502);
    const failure = await response.json();
    expect(failure.error).toBe('Pre-merge Gate Failed: High similarity detected (>75%) between expert outputs. Deduplication required.');
    await expect(page.locator('.expert-error-content')).toContainText(failure.error);
    await expect(page.getByRole('heading', { name: 'Final Delivered Output' })).toHaveCount(0);
    await expect(page.getByRole('button', { name: 'Execute Task via Expert Team' })).toBeEnabled();
  });

  test('should fail when API returns an error', async ({ page }) => {
    await page.goto('/expert-team');

    // Fill in the task context that will fail a quality gate (e.g., missing chapters or short)
    await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Short task');

    // Click the execute button
    await page.click('button:has-text("Execute Task via Expert Team")');

    // Wait for the error message
    await expect(page.locator('h3:has-text("Quality Gate or Execution Error:")')).toBeVisible({ timeout: 15000 });
  });
});

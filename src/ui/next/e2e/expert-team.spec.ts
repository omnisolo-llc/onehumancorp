import { test, expect } from '@playwright/test';

test.describe('Expert Team', () => {
  test('should execute a task via the expert team and pass quality gates', async ({ page }) => {
    // Navigate to the expert team page
    await page.goto('/expert-team');

    // Check that the title exists
    await expect(page.locator('h1')).toHaveText('Collaborative Expert Team');

    // Fill in the task context
    await page.fill('textarea[placeholder*="Write a comprehensive business plan"]', 'Write a comprehensive business plan for a new AI startup. Chart: Required. Analysis: Deep. Chapter 1, Chapter 2, Chapter 3, Chapter 4, Chapter 5, Chapter 6, Chapter 7, Chapter 8');

    // Click the execute button
    await page.click('button:has-text("Execute Task via Expert Team")');

    // Verify that the loading state appears
    await expect(page.locator('button:has-text("Orchestrating Expert Team...")')).toBeVisible();

    // The backend should return the final delivered output
    await expect(page.locator('h2:has-text("Final Delivered Output")')).toBeVisible({ timeout: 15000 });

    // Check that the output contains the required elements from the backend stub
    const outputText = await page.locator('pre').textContent();
    expect(outputText).toContain('Combined Executive Summary');
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

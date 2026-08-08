import { test, expect } from '@playwright/test';

test.describe('Dynamic Workflows Orhestrator', () => {
  test('should render the form and handle a basic generation flow', async ({ page }) => {
    // We navigate directly to the dynamic-workflows page
    await page.goto('/dynamic-workflows');

    // Verify title is visible
    await expect(page.locator('text=Dynamic Workflows Orchestrator')).toBeVisible();

    // Fill in a prompt
    await page.fill('textarea[placeholder="e.g. Audit every route handler under src/routes/ for missing authentication checks..."]', 'Test dynamic workflow prompt');

    // Submit
    await page.click('button:has-text("Generate Workflow")');

    // It should at least enter loading state since the request will be pending
    await expect(page.locator('text=Processing...')).toBeVisible();

    // Wait for the state to be awaiting_confirmation
    await expect(page.locator('h2', { hasText: 'Workflow Status: awaiting_confirmation' })).toBeVisible({ timeout: 10000 });

    // Click the approve button
    await page.click('button:has-text("Approve & Run Workflow")');

    // Wait for the state to transition to queued
    await expect(page.locator('h2', { hasText: 'Workflow Status: queued' })).toBeVisible({ timeout: 10000 });
  });
});

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
  });
});

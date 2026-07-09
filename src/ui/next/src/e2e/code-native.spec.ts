import { test, expect } from '@playwright/test';

test('Code-Native Execution E2E CUJ', async ({ page }) => {
  await page.goto('/code-native-execution');

  // Verify UI elements
  await expect(page.locator('h1')).toContainText('Code-Native Execution Pipeline');

  // Verify the Run button
  const button = page.locator('button:has-text("Run Pipeline")');
  await expect(button).toBeVisible();
  await expect(button).toBeEnabled();

  // Run the pipeline
  await button.click();

  // Button should show loading state briefly, then return to normal
  await expect(page.locator('button:has-text("Executing pipeline...")')).toBeVisible();

  // Verify success message appears and contains expected mock output
  const successMessage = page.locator('[data-testid="success-message"]');
  await expect(successMessage).toBeVisible({ timeout: 10000 });
  await expect(successMessage).toContainText('Generated rich data with ID: test_id');
  await expect(successMessage).toContainText('Processed data natively. New record count: 2');
});
